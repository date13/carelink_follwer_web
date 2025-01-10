import json
import numpy as np
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
userConfig = {}
forcastCount = 30
luckLimit = 92
minLuckCV = 25
yesterdayKey = "yesterday"
datetimeFormat = "%Y-%m-%d %H:%M:%S"
dateFormat = "%Y-%m-%d"


# hourOffset = 23

def addUserCofig(user, config):
    userConfig[user] = config


def sendMail(text):
    mailBody["content_text"] = text
    mailObj.send(mailBody)


def loadCarelinkData(user, token):
    # session.keep_alive = False
    options = userConfig[user]
    params = {
        "patientId": options["patientId"],
        "username": options["username"],
        "role": options["role"],
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
            my_logger.info("用户:%s 读取 carelinkData 数据成功!!!" % user)
            # print(result)
            return response.status_code, result
        text = "用户:%s 读取 carelinkData 数据失败!!!:%s" % (user, str(response.status_code))
        my_logger.info(text)
        sendMail(text)
        return response.status_code, None
    except requests.exceptions.RequestException as e:
        text = "用户:%s 读取 carelinkData 数据异常!!!:%s" % (user, str(e))
        my_logger.info(text)
        # sendMail(text)
        return None, None


async def getCarelinkToken(user):
    authKey = "%s:%s" % (user, dictKey["auth"])
    return rds.get_json(authKey)


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


def refreshCarelinkDataForUser(user):
    dataKey = "%s:%s" % (user, dictKey["data"])
    authKey = "%s:%s" % (user, dictKey["auth"])

    tokenObj = rds.get_json(authKey)
    dataObj = rds.get_json(dataKey)
    token = tokenObj["token"]
    status = tokenObj["status"]
    if status != 200:
        if dataObj["status"] != status:
            dataObj["status"] = status
            text = "用户:%s refreshCarelinkData token失效,请手动刷新Token,当前状态:%s" % (user, str(status))
            my_logger.info(text)
            sendMail(text)
            updateCarelinkDataToRedis(user, dataObj)
            # await updateCarelinkDataToDB(dataObj)
    else:
        status, data = loadCarelinkData(user, token)
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

        updateCarelinkDataToRedis(user, dataObj)
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


def updateCarelinkDataToRedis(user, dataObj):
    try:
        dataObj["update_time"] = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        rds.set_json("%s:%s" % (user, dictKey["data"]), dataObj)
        my_logger.info("用户:%s carelinkData 更新完成!!!:%s" % (user, str(dataObj["status"])))
    except Exception as e:
        text = "用户:%s 更新 carelinkData 数据错误!!!:%s" % (user, str(e))
        my_logger.info(text)
        sendMail(text)


def refreshCarelinkTokenForUser(user):
    authKey = "%s:%s" % (user, dictKey["auth"])
    tokenObj = rds.get_json(authKey)
    token = tokenObj["token"]
    status = tokenObj["status"]
    if status != 200:
        my_logger.info("用户:%s refreshCarelinkToken token失效,请手动刷新Token,当前状态:%s" % (user, str(status)))
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
                my_logger.info("用户:%s carelinkUserToken刷新成功!!!" % user)
                cookies = response.cookies.get_dict()
                tokenObj["token"] = cookies["auth_tmp_token"]
            else:
                text = "用户:%s carelinkUserToken刷新失败!!!:%s" % (user, str(tokenObj["status"]))
                my_logger.info(text)
                sendMail(text)
            tokenObj["update_time"] = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            # await updateCarelinkData(dictKey["auth"], tokenObj)
            rds.set_json(authKey, tokenObj)
            # response.close()
        except requests.exceptions.RequestException as e:
            text = "用户:%s 刷新carelinkUserToken错误!!!:%s" % (user, str(e))
            my_logger.info(text)
            sendMail(text)


# async def updateCarelinkData(key, dataObj):
#     params: UpdateSysDictForm = UpdateSysDictForm(key=key, val=json.dumps(dataObj))
#     result = await updateSysDict(params)
#     return result

def refreshCarelinkYesterdayData(user, data, localtime):
    # yesterdayDatatime = datetime.strptime(myData[yesterdayKey]["time"], dataFormat)
    # print((localtime - yesterdayDatatime).total_seconds()/3600)
    # 运行条件
    # 1. 0小时0分
    # 2. 没有数据
    myDataKey = "%s:%s" % (user, dictKey["myData"])
    myData = rds.get_json(myDataKey)
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
            "用户:%s 刷新carelinkYesterdayData数据,sgsArr:%s markersArr:%s" % (
                user, str(len(yesSgsArr)), str(len(yesMarkersArr))))
        dealYesData(yesSgsArr, sgsData)
        dealYesData(yesMarkersArr, markersData)
        myData["update_time"] = localtime.strftime(datetimeFormat)
        rds.set_json(myDataKey, myData)
        my_logger.info("用户:%s 刷新carelinkYesterdayData数据成功" % user)


def dealYesData(yesArr, yesData):
    if len(yesArr) < 2:
        yesArr.append(yesData)
    elif len(yesArr) == 2:
        yesArr[0] = yesArr[1]
        yesArr[1] = yesData


def saveHistoryData(user, data, localtime):
    yesterday_data = data["data"]
    sum_recommended = 0
    sum_auto_correction = 0
    sum_basal = 0
    for item in yesterday_data["markers"]:
        if item["type"] == 'INSULIN':
            if item["activationType"] == 'RECOMMENDED':
                sum_recommended += item["deliveredFastAmount"]
            elif item["activationType"] == 'AUTOCORRECTION':
                sum_auto_correction += item["deliveredFastAmount"]
        elif item["type"] == 'AUTO_BASAL_DELIVERY':
            sum_basal += item["bolusAmount"]

    history_data = {
        "averageSG": yesterday_data["averageSG"],
        "belowHypoLimit": yesterday_data["belowHypoLimit"],
        "aboveHyperLimit": yesterday_data["aboveHyperLimit"],
        "timeInRange": yesterday_data["timeInRange"],
        "averageSGFloat": yesterday_data["averageSGFloat"],
        "insulin": {
            "recommended": sum_recommended,
            "autoCorrection": sum_auto_correction,
            "basal": sum_basal
        }
    }
    rds.set_hash_kv("%s:%s" % (user, dictKey["history"]), (localtime - timedelta(days=1)).strftime("%Y-%m-%d"),
                    json.dumps(history_data))
    my_logger.info("用户:%s 历史数据保存成功" % user)


def updateLuckData(user, localtime):
    luckKey = "%s:%s" % (user, dictKey["luck"])
    luck = rds.get_json(luckKey)
    # if ((localtime - datetime.strptime(luck["update_time"], datetimeFormat)).total_seconds() / 3600) > hourOffset:
    data = rds.get_json("%s:%s" % (user, dictKey["data"]))
    sgData = data["data"]
    tir = sgData["timeInRange"]
    arr = list(map(lambda n: n["sg"],
                   filter(
                       lambda n: (n["kind"] == "SG" and n["sensorState"] == "NO_ERROR_MESSAGE" and n["sg"] != 0) or n[
                           "sensorState"] == "SG_BELOW_40_MGDL", sgData["sgs"])))

    cv = (np.std(arr) / sgData["averageSG"]) * 100
    if tir >= luckLimit and cv <= minLuckCV:
        luck["yes"] += 1
    else:
        luck["no"] += 1
    luck["update_time"] = localtime.strftime(datetimeFormat)
    rds.set_json(luckKey, luck)
    my_logger.info("用户:%s tir:%s cv:%s 刷新luck数据成功" % (user, tir, '{:.2f}'.format(cv)))


def getSumObj():
    return {
        "avg": 0,
        "below": 0,
        "above": 0,
        "recommended": 0,
        "autoCorrection": 0,
        "basal": 0,
    }


def updateStatistics(user, data):
    historyData = rds.get_all_hash_kv("%s:%s" % (user, dictKey["history"]))
    # print(historyData)
    sumAvg = 0
    sumBelow = 0
    sumAbove = 0
    day30 = {
        "sum": getSumObj(),
        "count": 0,
        "insulinCount": 0
    }
    day90 = {
        "sum": getSumObj(),
        "count": 0,
        "insulinCount": 0
    }
    i = 0
    for key in sorted(historyData, reverse=True):
        item = json.loads(historyData[key])
        # keyDate = time.mktime(time.strptime(key.decode(), dateFormat))
        if i < 30:
            calcDayObjData(day30, item)
        if i < 90:
            calcDayObjData(day90, item)

        sumAvg += item["averageSGFloat"]
        sumBelow += item["belowHypoLimit"]
        sumAbove += item["aboveHyperLimit"]
        i += 1

    totalAvgSg = sumAvg / len(historyData)
    data["GMI"] = '{:.2f}'.format(3.31 + 0.02392 * totalAvgSg)
    if "statistics" not in data:
        data["statistics"] = {
            "day30": getSumObj(),
            "day90": getSumObj()
        }

    data["statistics"]["day30"]["avg"] = '{:.2f}'.format(day30["sum"]["avg"] / day30["count"])
    data["statistics"]["day30"]["below"] = '{:.2f}'.format(day30["sum"]["below"] / day30["count"])
    data["statistics"]["day30"]["above"] = '{:.2f}'.format(day30["sum"]["above"] / day30["count"])
    data["statistics"]["day30"]["recommended"] = '{:.2f}'.format(day30["sum"]["recommended"] / day30["insulinCount"])
    data["statistics"]["day30"]["autoCorrection"] = '{:.2f}'.format(
        day30["sum"]["autoCorrection"] / day30["insulinCount"])
    data["statistics"]["day30"]["basal"] = '{:.2f}'.format(day30["sum"]["basal"] / day30["insulinCount"])

    data["statistics"]["day90"]["avg"] = '{:.2f}'.format(day90["sum"]["avg"] / day90["count"])
    data["statistics"]["day90"]["below"] = '{:.2f}'.format(day90["sum"]["below"] / day90["count"])
    data["statistics"]["day90"]["above"] = '{:.2f}'.format(day90["sum"]["above"] / day90["count"])
    data["statistics"]["day90"]["recommended"] = '{:.2f}'.format(day90["sum"]["recommended"] / day90["insulinCount"])
    data["statistics"]["day90"]["autoCorrection"] = '{:.2f}'.format(
        day90["sum"]["autoCorrection"] / day90["insulinCount"])
    data["statistics"]["day90"]["basal"] = '{:.2f}'.format(day90["sum"]["basal"] / day90["insulinCount"])

    updateCarelinkDataToRedis(user, data)
    my_logger.info("用户:%s 刷新GMI数据成功" % user)


def calcDayObjData(dayObj, item):
    dayObj["sum"]["avg"] += item["timeInRange"]
    dayObj["sum"]["below"] += item["belowHypoLimit"]
    dayObj["sum"]["above"] += item["aboveHyperLimit"]
    dayObj["count"] += 1
    if "insulin" in item:
        dayObj["sum"]["recommended"] += item["insulin"]["recommended"]
        dayObj["sum"]["autoCorrection"] += item["insulin"]["autoCorrection"]
        dayObj["sum"]["basal"] += item["insulin"]["basal"]
        dayObj["insulinCount"] += 1


def refreshCarelinkTokenIntervalForUser(user):
    # while True:
    my_logger.info("==============开始用户:%s carelinkUserToken刷新任务==============" % user)
    try:
        refreshCarelinkTokenForUser(user)
    except Exception as ex:
        text = "用户:%s carelinkUserToken刷新任务错误!!!" % user + str(ex)
        my_logger.info(text)
        sendMail(text)
    my_logger.info("!!!结束用户:%s carelinkUserToken刷新任务!!!" % user)
    # await asyncio.sleep(config.CARELINK_TOKEN_REFRESH_INTERVAL)


def refreshCarelinkDataIntervalForUser(user):
    # while True:
    my_logger.info("==============开始用户:%s carelinkData刷新任务==============" % user)
    try:
        refreshCarelinkDataForUser(user)
    except Exception as ex:
        text = "用户:%s carelinkData刷新任务错误!!!:%s" % (user, str(ex))
        my_logger.info(text)
        sendMail(text)
    my_logger.info("用户:%s !!!结束carelinkData刷新任务!!!" % user)
    # await asyncio.sleep(config.CARELINK_DATA_REFRESH_INTERVAL)


def refreshCarelinkTaskIntervalMinutesForUser(user):
    # my_logger.info("==============开始refreshCarelinkTaskIntervalMinutes刷新任务==============")
    # while True:
    try:
        my_logger.info("==============开始用户:%s refreshCarelinkTaskIntervalMinutes任务开始==============" % user)
        localtime = datetime.now()
        data = rds.get_json("%s:%s" % (user, dictKey["data"]))
        refreshCarelinkYesterdayData(user, data, localtime)
        saveHistoryData(user, data, localtime)
        updateStatistics(user, data)
        updateLuckData(user, localtime)
    except Exception as ex:
        text = "用户:%s refreshCarelinkTaskIntervalMinutes刷新任务刷新任务错误!!!%s" % (user, str(ex))
        my_logger.info(text)
        sendMail(text)
        # await asyncio.sleep(60)


def getAllFood(user):
    return rds.get_all_hash_kv("%s:food" % user)


def updateFood(user, hashObj: RedisHashObj):
    return rds.set_hash_kv("%s:food" % user, hashObj.key, hashObj.val)


def delFood(user, key):
    return rds.del_hash_kv("%s:food" % user, key)

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
# user = "alex"
# localtime = datetime.now()
# data = rds.get_json("%s:%s" % (user, dictKey["data"]))
# updateStatistics(user, data)
# updateLuckData(user,localtime)
# saveHistoryData(user, data, localtime)
# refreshCarelinkYesterdayData(data, localtime)
# print((localtime - timedelta(days=1)).strftime("%Y-%m-%d"))
# print(localtime.strftime(datetimeFormat))
# refreshCarelinkDataIntervalForUser('alex')
