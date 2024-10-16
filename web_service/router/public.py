#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from fastapi import APIRouter
from starlette.requests import Request

from core.denpends import limiter
from core.route_log import LogRoute
from models.result import Result, ResultSchema, RedisHashObj
from service.sugarService import loadCarelinkData, getCarelinkToken, refreshCarelinkData, refreshCarelinkToken, \
    getAllFood, updateFood, delFood
from service.sysService import getSysDict, UpdateSysDictForm, updateSysDict

router = APIRouter(route_class=LogRoute)


@router.get("/test")
@limiter.limit("3/minute")
async def root(request: Request):
    return Result.success(data={"message": "Hello World"})


@router.get("/users/")
async def read_users():
    return Result.success(data=[{"username": "Rick"}, {"username": "Morty"}])


@router.get("/dict/{key}")
def getDict(key: str):
    # 从文件中加载对象
    res = getSysDict(key)
    return Result.success(data=res if res is not None else None)


@router.post("/dict")
def upadteDict(updateForm: UpdateSysDictForm):
    row = updateSysDict(updateForm)
    return Result.success(data=row) if row else Result.fail(msg='没有更新任何数据')


@router.put("/sugar")
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
