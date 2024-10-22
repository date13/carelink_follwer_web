#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 注册路由
from fastapi import FastAPI, Depends

from core.config import config
from core.denpends import check_permission
from router import public, user, sugar, system


def register_router(app: FastAPI):
    """ 注册路由 """

    app.include_router(public.router, prefix=config.API_PREFIX + '/public', tags=["Public"])

    app.include_router(system.router, prefix=config.API_PREFIX + '/system', tags=["System"])

    app.include_router(sugar.router, prefix=config.API_PREFIX + '/sugar', tags=["Sugar"])

    app.include_router(user.router, prefix=config.API_PREFIX + "/user", tags=["User"],
                       dependencies=[Depends(check_permission([]))])
