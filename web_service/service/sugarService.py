import asyncio
import json
import time
from datetime import datetime, timedelta

import pandas as pd
import requests
from statsmodels.tsa.ar_model import AutoReg

from core.config import config
from core.redis_core import RedisTool
from models.result import RedisHashObj
from utils.http_utils import HttpUtils, HTTP_TIMEOUT
from utils.logutils import my_logger
from utils.mail import MailObject

refreshCarelinkTokenUrl = config.CARELINK_API_DOMAIN + 'patient/sso/reauth'
UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0"
dictKey = {
    "luck": "luck",
    "auth": "carelinkAuth",
    "data": "carelinkData",
    "myData": "carelinkMyData",
    "history": "history"
}
mailBody = {
    'subject': 'carelink_follower_web警报',
    'content_text': '',  # 纯文本或者HTML内容
}
session = HttpUtils().getSession()
mailObj = MailObject()
rds = RedisTool()


def sendMail(text):
    mailBody["content_text"] = text
    mailObj.send(mailBody)


def loadCarelinkData(token):
    # session.keep_alive = False
    params = {
        "patientId": config.CARELINK_PATIENTID,
        "username": config.CARELINK_USERNAME,
        "role": config.CARELINK_ROLE
    }
    headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer " + token,
        "User-Agent": UA
    }
    try:
        response = session.post(config.CARELINK_DATA_URL, json=params, headers=headers, timeout=HTTP_TIMEOUT)
        if response.status_code == 200:
            result = json.loads(response.text)
            # response.close()
            my_logger.info("读取 carelinkData 数据成功!!!")
            # print(result)
            return response.status_code, result
        text = "读取 carelinkData 数据失败!!!" + str(response.status_code)
        my_logger.info(text)
        sendMail(text)
        return response.status_code, None
    except requests.exceptions.RequestException as e:
        text = "读取 carelinkData 数据异常!!!" + str(e)
        my_logger.info(text)
        # sendMail(text)
        return None, None


async def getCarelinkToken():
    authKey = dictKey["auth"]
    return rds.get_json(authKey)


forcastCount = 30


def forcastAR2Sg(list):
    series = pd.Series(list)

    # 拟合AR(2)模型
    model = AutoReg(series, lags=2)
    model_fit = model.fit()

    # 使用模型对未来5个时间点进行预测
    forecast = model_fit.predict(start=len(series), end=len(series) + forcastCount, dynamic=False)
    result = []
    for glucose in enumerate(forecast, start=1):
        result.append(round(glucose[1]))

    my_logger.info("ar2预测数据:" + ",".join(map(lambda x: str(x), result)))
    return result


def refreshCarelinkData():
    dataKey = dictKey["data"]
    authKey = dictKey["auth"]
    tokenObj = rds.get_json(authKey)
    dataObj = rds.get_json(dataKey)
    token = tokenObj["token"]
    status = tokenObj["status"]
    if status != 200:
        if dataObj["status"] != status:
            dataObj["status"] = status
            text = "refreshCarelinkData token失效,请手动刷新Token,当前状态:" + str(status)
            my_logger.info(text)
            sendMail(text)
            updateCarelinkDataToRedis(dataObj)
            # await updateCarelinkDataToDB(dataObj)
    else:
        status, data = loadCarelinkData(token)
        if status is not None:
            dataObj["status"] = status
            if data is not None:
                dealWarmUp(data, dataObj)
                dataObj["data"] = data
                # dataObj["forecast"] = {"ar2": forcastAR2Sg(list(map(lambda x: x["sg"], data["sgs"])))}
                dataObj["forecast"] = {
                    "ar2": forcastAR2Sg(filter(lambda x: x != 0, list(map(lambda x: x["sg"], data["sgs"]))))}
        else:
            dataObj["status"] = 404

        updateCarelinkDataToRedis(dataObj)
        # await updateCarelinkDataToDB(dataObj)


nextStartTimeKey = "nextStartTime"


def dealWarmUp(data, dataObj):
    status = data["systemStatusMessage"]
    nextStart = dataObj[nextStartTimeKey]
    if status == "WARM_UP" and nextStart == -1:
        dataObj[nextStartTimeKey] = (datetime.now() + timedelta(hours=2)).strftime("%Y-%m-%d %H:%M:%S")

    if status == "NO_ERROR_MESSAGE" and nextStart != -1:
        dataObj[nextStartTimeKey] = -1


# async def updateCarelinkDataToDB(dataObj):
#     try:
#         dataObj["update_time"] = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
#         await updateCarelinkData(dictKey["data"], dataObj)
#         my_logger.info("carelinkData 更新完成!!!" + str(dataObj["status"]))
#     except Exception as e:
#         text = "更新 carelinkData 数据错误!!!" + str(e)
#         my_logger.info(text)
#         sendMail(text)


def updateCarelinkDataToRedis(dataObj):
    try:
        dataObj["update_time"] = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        rds.set_json(dictKey["data"], dataObj)
        my_logger.info("carelinkData 更新完成!!!" + str(dataObj["status"]))
    except Exception as e:
        text = "更新 carelinkData 数据错误!!!" + str(e)
        my_logger.info(text)
        sendMail(text)


def refreshCarelinkToken():
    tokenObj = rds.get_json(dictKey["auth"])
    token = tokenObj["token"]
    status = tokenObj["status"]
    if status != 200:
        my_logger.info("refreshCarelinkToken token失效,请手动刷新Token,当前状态:" + str(status))
    else:
        headers = {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
            "User-Agent": UA
        }
        params = {
            "country": "HK",
            "locale": "zh"
        }
        try:
            response = session.post(refreshCarelinkTokenUrl, json=params, headers=headers, timeout=HTTP_TIMEOUT)
            tokenObj["status"] = response.status_code
            if response.status_code == 200:
                my_logger.info("carelinkUserToken刷新成功!!!")
                cookies = response.cookies.get_dict()
                tokenObj["token"] = cookies["auth_tmp_token"]
            else:
                text = "carelinkUserToken刷新失败!!!" + str(tokenObj["status"])
                my_logger.info(text)
                sendMail(text)
            tokenObj["update_time"] = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            # await updateCarelinkData(dictKey["auth"], tokenObj)
            rds.set_json(dictKey["auth"], tokenObj)
            # response.close()
        except requests.exceptions.RequestException as e:
            text = "刷新carelinkUserToken错误!!!" + str(e)
            my_logger.info(text)
            sendMail(text)


# async def updateCarelinkData(key, dataObj):
#     params: UpdateSysDictForm = UpdateSysDictForm(key=key, val=json.dumps(dataObj))
#     result = await updateSysDict(params)
#     return result
yesterdayKey = "yesterday"
datetimeFormat = "%Y-%m-%d %H:%M:%S"
dateFormat = "%Y-%m-%d"
hourOffset = 23
luckLimit = 90


def refreshCarelinkYesterdayData(data, localtime):
    # yesterdayDatatime = datetime.strptime(myData[yesterdayKey]["time"], dataFormat)
    # print((localtime - yesterdayDatatime).total_seconds()/3600)
    # 运行条件
    # 1. 0小时0分
    # 2. 没有数据
    myData = rds.get_json(dictKey["myData"])
    yesterdayData = data["data"]
    sgsData = yesterdayData["sgs"]
    markersData = yesterdayData["markers"]
    if yesterdayKey not in myData:
        myData[yesterdayKey] = {}
        myData[yesterdayKey]["sgs"] = [sgsData]
        myData[yesterdayKey]["markers"] = [markersData]
    # elif ((localtime - datetime.strptime(myData[yesterdayKey]["update_time"],
    #                                      datetimeFormat)).total_seconds() / 3600) > hourOffset:
    else:
        # 先备份一下前一天的数据
        # rds.set_json("carelinkMyData_Backup", myData)
        yesSgsArr = myData[yesterdayKey]["sgs"]
        yesMarkersArr = myData[yesterdayKey]["markers"]
        my_logger.info(
            "刷新carelinkYesterdayData数据,sgsArr:" + str(len(yesSgsArr)) + " markersArr:" + str(len(yesMarkersArr)))
        dealYesData(yesSgsArr, sgsData)
        dealYesData(yesMarkersArr, markersData)
        myData[yesterdayKey]["update_time"] = localtime.strftime(datetimeFormat)
        rds.set_json(dictKey["myData"], myData)
        my_logger.info("刷新carelinkYesterdayData数据成功")


def dealYesData(yesArr, yesData):
    if len(yesArr) < 2:
        yesArr.append(yesData)
    elif len(yesArr) == 2:
        yesArr[0] = yesArr[1]
        yesArr[1] = yesData


def saveHistoryData(data, localtime):
    yesterdayData = data["data"]
    historyData = {
        "averageSG": yesterdayData["averageSG"],
        "belowHypoLimit": yesterdayData["belowHypoLimit"],
        "aboveHyperLimit": yesterdayData["aboveHyperLimit"],
        "timeInRange": yesterdayData["timeInRange"],
        "averageSGFloat": yesterdayData["averageSGFloat"]
    }
    rds.set_hash_kv(dictKey["history"], (localtime - timedelta(days=1)).strftime("%Y-%m-%d"),
                    json.dumps(historyData))
    my_logger.info("历史数据保存成功")


def updateLuckData(localtime):
    luck = rds.get_json(dictKey["luck"])
    # if ((localtime - datetime.strptime(luck["update_time"], datetimeFormat)).total_seconds() / 3600) > hourOffset:
    data = rds.get_json(dictKey["data"])
    sgData = data["data"]
    tir = sgData["timeInRange"]
    if tir >= luckLimit:
        luck["yes"] += 1
    else:
        luck["no"] += 1
    luck["update_time"] = localtime.strftime(datetimeFormat)
    rds.set_json(dictKey["luck"], luck)
    my_logger.info("刷新luck数据成功")


async def refreshCarelinkTokenInterval():
    # while True:
    my_logger.info("==============开始carelinkUserToken刷新任务==============")
    try:
        refreshCarelinkToken()
    except Exception as ex:
        text = "carelinkUserToken刷新任务错误!!!" + str(ex)
        my_logger.info(text)
        sendMail(text)
    my_logger.info("!!!结束carelinkUserToken刷新任务!!!")
    # await asyncio.sleep(config.CARELINK_TOKEN_REFRESH_INTERVAL)


async def refreshCarelinkDataInterval():
    # while True:
    my_logger.info("==============开始carelinkData刷新任务==============")
    try:
        refreshCarelinkData()
    except Exception as ex:
        text = "carelinkData刷新任务错误!!!" + str(ex)
        my_logger.info(text)
        sendMail(text)
    my_logger.info("!!!结束carelinkData刷新任务!!!")
    # await asyncio.sleep(config.CARELINK_DATA_REFRESH_INTERVAL)


async def refreshCarelinkTaskIntervalMinutes():
    # my_logger.info("==============开始refreshCarelinkTaskIntervalMinutes刷新任务==============")
    # while True:
    try:
        my_logger.info("==============refreshCarelinkTaskIntervalMinutes任务开始==============")
        localtime = datetime.now()
        data = rds.get_json(dictKey["data"])
        updateLuckData(localtime)
        saveHistoryData(data, localtime)
        refreshCarelinkYesterdayData(data, localtime)
    except Exception as ex:
        text = "refreshCarelinkTaskIntervalMinutes刷新任务刷新任务错误!!!" + str(ex)
        my_logger.info(text)
        sendMail(text)
        # await asyncio.sleep(60)


def getAllFood():
    return rds.get_all_hash_kv('food')


def updateFood(hashObj: RedisHashObj):
    return rds.set_hash_kv('food', hashObj.key, hashObj.val)


def delFood(key):
    return rds.del_hash_kv('food', key)

# asyncio.run(refreshCarelinkToken())
# asyncio.run(refreshCarelinkData())
# result = getToken()
# token = result["data"]["tokenObj"]["token"]
# print(token)
# loadCarelinkData(token)
# updateLuckData(datetime.now())
# refreshCarelinkData()
# refreshCarelinkYesterdayData(datetime.now())
# updateLuckData()
# data = rds.get_json(dictKey["data"])
# saveHistoryData(data)
# localtime = datetime.now()
# print((localtime - timedelta(days=1)).strftime("%Y-%m-%d"))
# print(localtime.strftime(datetimeFormat))
