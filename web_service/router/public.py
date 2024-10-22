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

