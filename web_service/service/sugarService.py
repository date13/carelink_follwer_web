import asyncio
import json
from datetime import datetime

import requests

from core.config import config
from core.redis_core import RedisTool
from utils.http_utils import HttpUtils, HTTP_TIMEOUT
from utils.logutils import my_logger
from utils.mail import MailObject

refreshCarelinkTokenUrl = config.CARELINK_API_DOMAIN + 'patient/sso/reauth'
UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0"
dictKey = {
    "auth": "carelinkAuth",
    "data": "carelinkData"
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
            return [response.status_code, result]
        text = "读取 carelinkData 数据失败!!!" + str(response.status_code)
        my_logger.info(text)
        sendMail(text)
        return [response.status_code, None]
    except requests.exceptions.RequestException as e:
        text = "读取 carelinkData 数据异常!!!" + str(e)
        my_logger.info(text)
        sendMail(text)
        return [None, None]


async def getCarelinkToken():
    authKey = dictKey["auth"]
    return rds.get_json(authKey)


def refreshCarelinkData():
    dataKey = dictKey["data"]
    authKey = dictKey["auth"]
    tokenObj = rds.get_json(authKey)
    dataObj = rds.get_json(dataKey)
    token = tokenObj["token"]
    if tokenObj["status"] != 200:
        if dataObj["status"] != tokenObj["status"]:
            dataObj["status"] = tokenObj["status"]
            text = "token失效,请手动刷新Token!!!"
            my_logger.info(text)
            sendMail(text)
            updateCarelinkDataToRedis(dataObj)
            # await updateCarelinkDataToDB(dataObj)
    else:
        result = loadCarelinkData(token)
        status = result[0]
        if status is not None:
            dataObj["status"] = status
            if result[1] is not None:
                dataObj["data"] = json.dumps(result[1])
        else:
            dataObj["status"] = 404

        updateCarelinkDataToRedis(dataObj)
        # await updateCarelinkDataToDB(dataObj)


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
    # print(tokenObj["token"])
    if token:
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


async def refreshCarelinkTokenInterval():
    while True:
        my_logger.info("==============开始carelinkUserToken刷新任务==============")
        try:
            refreshCarelinkToken()
        except Exception as ex:
            text = "carelinkUserToken刷新任务错误!!!" + str(ex)
            my_logger.info(text)
            sendMail(text)
        my_logger.info("!!!结束carelinkUserToken刷新任务!!!")
        await asyncio.sleep(config.CARELINK_TOKEN_REFRESH_INTERVAL)


async def refreshCarelinkDataInterval():
    while True:
        my_logger.info("==============开始carelinkData刷新任务==============")
        try:
            refreshCarelinkData()
        except Exception as ex:
            text = "carelinkData刷新任务错误!!!" + str(ex)
            my_logger.info(text)
            sendMail(text)
        my_logger.info("!!!结束carelinkData刷新任务!!!")
        await asyncio.sleep(config.CARELINK_DATA_REFRESH_INTERVAL)

# asyncio.run(refreshCarelinkToken())
# asyncio.run(refreshCarelinkData())
# result = getToken()
# token = result["data"]["tokenObj"]["token"]
# print(token)
# loadCarelinkData(token)
