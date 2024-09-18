#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 解决跨域请求
from fastapi import FastAPI
from starlette.middleware.cors import CORSMiddleware


def register_cors(app: FastAPI):
    """ 跨域请求 -- https://fastapi.tiangolo.com/zh/tutorial/cors/ """

    app.add_middleware(
        CORSMiddleware,
        allow_origins=['*'],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )
