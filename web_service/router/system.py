#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from fastapi import APIRouter
from core.route_log import LogRoute
from models.result import Result
from service.sysService import getSysDict, UpdateSysDictForm, updateSysDict

router = APIRouter(route_class=LogRoute)


@router.get("/dict/{key}")
def getDict(key: str):
    # 从文件中加载对象
    res = getSysDict(key)
    return Result.success(data=res if res is not None else None)


@router.post("/dict")
def upadteDict(updateForm: UpdateSysDictForm):
    row = updateSysDict(updateForm)
    return Result.success(data=row) if row else Result.fail(msg='没有更新任何数据')


