#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from fastapi import APIRouter
from starlette.requests import Request

from core.route_log import LogRoute
from models.result import Result
from service.sysService import getSysDict, UpdateSysDictForm, updateSysDict, getSysDictHash, getSysDictForUser, \
    getSysDictHashForUser, updateSysDictForUser

router = APIRouter(route_class=LogRoute)


@router.get("/dict/{key}")
def getDict(key: str):
    res = getSysDict(key)
    return Result.success(data=res if res is not None else None)


@router.get("/dict/{user}/{key}")
def getDictForUser(user: str, key: str):
    res = getSysDictForUser(user, key)
    return Result.success(data=res if res is not None else None)


@router.get("/dict_hash/{key}")
def getDictHash(key: str):
    res = getSysDictHash(key)
    return Result.success(data=res if res is not None else None)


@router.get("/dict_hash/{user}/{key}")
def getDictHash(user: str, key: str):
    res = getSysDictHashForUser(user, key)
    return Result.success(data=res if res is not None else None)


@router.post("/dict")
def upadteDict(updateForm: UpdateSysDictForm):
    row = updateSysDict(updateForm)
    return Result.success(data=row)


@router.post("/dict/{user}")
def upadteDict(user: str, updateForm: UpdateSysDictForm):
    row = updateSysDictForUser(user, updateForm)
    return Result.success(data=row)
