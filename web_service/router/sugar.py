#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from fastapi import APIRouter
from starlette.requests import Request

from core.route_log import LogRoute
from models.result import Result, ResultSchema, RedisHashObj
from service.sugarService import loadCarelinkData, getCarelinkToken, refreshCarelinkDataForUser, \
    refreshCarelinkTokenForUser, \
    getAllFood, updateFood, delFood
from service.sysService import getSysDict, getSysDictForUser

router = APIRouter(route_class=LogRoute)


@router.put("/")
def sugar(request: Request) -> ResultSchema:
    user = request.state.user
    data = getSysDictForUser(user, 'carelinkData')
    myData = getSysDictForUser(user, 'carelinkMyData')
    try:
        return Result.success(data={
            "data": data,
            "myData": myData
        })
    except Exception as ex:
        return Result.fail(msg='用户:%s 获取 carelink 数据错误:%s' % (user, str(ex)))


@router.put("/loadCarelinkData")
async def load_carelink_data(request: Request) -> ResultSchema:
    user = request.state.user
    result = getCarelinkToken(user)
    try:
        token = result["token"]
        return Result.success(data=loadCarelinkData(user, token))
    except Exception as ex:
        return Result.fail(msg='用户:%s 获取 carelink 数据错误:%s' % (user, str(ex)))


@router.put("/refreshCarelinkData")
def refresh_carelink_data(request: Request) -> ResultSchema:
    refreshCarelinkDataForUser(request.state.user)
    return Result.success(data=True)


@router.put("/refreshCarelinkToken")
def refresh_carelink_token(request: Request) -> ResultSchema:
    refreshCarelinkTokenForUser(request.state.user)
    return Result.success(data=True)


@router.get("/foods")
def foods(request: Request) -> ResultSchema:
    return Result.success(data=getAllFood(request.state.user))


@router.put("/food")
def setFood(hashObj: RedisHashObj, request: Request) -> ResultSchema:
    return Result.success(data=updateFood(request.state.user, hashObj))


@router.delete("/food/{key}")
def setFood(key: str, request: Request) -> ResultSchema:
    return Result.success(data=delFood(request.state.user, key))
