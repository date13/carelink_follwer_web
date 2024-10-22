#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from fastapi import APIRouter
from core.route_log import LogRoute
from models.result import Result, ResultSchema, RedisHashObj
from service.sugarService import loadCarelinkData, getCarelinkToken, refreshCarelinkData, refreshCarelinkToken, \
    getAllFood, updateFood, delFood
from service.sysService import getSysDict

router = APIRouter(route_class=LogRoute)


@router.put("/")
def sugar() -> ResultSchema:
    data = getSysDict('carelinkData')
    myData = getSysDict('carelinkMyData')
    try:
        return Result.success(data={
            "data": data,
            "myData": myData
        })
    except Exception as ex:
        return Result.fail(msg='获取 carelink 数据错误:' + ex)


@router.put("/loadCarelinkData")
async def load_carelink_data() -> ResultSchema:
    result = getCarelinkToken()
    try:
        token = result["token"]
        return Result.success(data=loadCarelinkData(token))
    except Exception as ex:
        return Result.fail(msg='获取 carelink 数据错误:' + ex)


@router.put("/refreshCarelinkData")
def refresh_carelink_data() -> ResultSchema:
    refreshCarelinkData()
    return Result.success(data=True)


@router.put("/refreshCarelinkToken")
def refresh_carelink_token() -> ResultSchema:
    refreshCarelinkToken()
    return Result.success(data=True)



@router.get("/foods")
def foods() -> ResultSchema:
    return Result.success(data=getAllFood())


@router.put("/food")
def setFood(hashObj: RedisHashObj) -> ResultSchema:
    return Result.success(data=updateFood(hashObj))


@router.delete("/food/{key}")
def setFood(key: str) -> ResultSchema:
    return Result.success(data=delFood(key))
