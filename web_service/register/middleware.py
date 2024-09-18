#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 中间件
import asyncio

from fastapi import FastAPI, HTTPException
from starlette import status
from starlette.requests import Request
from starlette.responses import Response, JSONResponse

from core.auth import Auth
from core.db_core import db
from models.const import CROS_HEADER
from models.result import Result

NOT_CHECK: list[str] = ["/api/user/login", "/favicon.ico", "/api/test", '/docs',
                        '/openapi.json']  # 不校验 Cookie


def register_middleware(app: FastAPI):
    """ 请求拦截与响应拦截 -- https://fastapi.tiangolo.com/tutorial/middleware/ """

    @app.middleware("http")
    async def intercept(request: Request, call_next) -> Response:
        """
            在这里获取 request.body() 出现了问题...
            使用了自定义路由APIRoute https://fastapi.tiangolo.com/advanced/custom-request-and-route/#custom-apiroute-class-in-a-router
        """
        print(request.get("path"), request.method)
        path: str = request.get("path")
        if request.method == "OPTIONS" or path.startswith("/api/public/") or request.get(
                "path") in NOT_CHECK:  # 不校验Cookie
            return await asyncio.wait_for(call_next(request), 60)

        try:
            token = request.headers["Authorization"]
            request.state.user = Auth().decode_token(token)
        except HTTPException as e:
            return JSONResponse(status_code=e.status_code, headers=CROS_HEADER,
                                content=Result.fail(msg=f'当前用户无效,{e.detail}').model_dump())
        except Exception as e:
            raise e

        try:
            response = await asyncio.wait_for(call_next(request), 60)
            return response
        except asyncio.TimeoutError as e:
            if not db.is_closed():
                db.close()
                db.close_async()
            return JSONResponse(status_code=status.HTTP_200_OK, headers=CROS_HEADER,
                                content=Result.fail(msg=f'请求超时,{e}').model_dump())
