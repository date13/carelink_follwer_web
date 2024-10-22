#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 用户接口

from fastapi import APIRouter
from pydantic import BaseModel
from starlette.responses import Response

from core.route_log import LogRoute
from models.error import UserErrors
from models.result import ResultSchema, Result, SysUserForm
from service.sysService import login

router = APIRouter(route_class=LogRoute)

class SysUserChangePwdForm(BaseModel):
    oldPassword: str
    newPassword1: str
    newPassword2: str


@router.post("/login")
async def user_login(loginForm: SysUserForm, response: Response) -> ResultSchema:
    sysUser = login(loginForm)
    if sysUser is not None:
        return Result.success(data=sysUser)
    else:
        raise UserErrors(err_desc="用户名或密码错误")

#
# @router.put("/changePwd")
# async def user_change_pwd(changePwd: SysUserChangePwdForm, request: Request) -> ResultSchema:
#     user = SysUser()
#     Tools.parseToObj(user.__data__, request.state.user)
#     sysUser = SysUser.get_or_none(SysUser.id == user.id)
#     if changePwd.newPassword1 != changePwd.newPassword2:
#         raise UserErrors(err_desc="用户两次输入密码不一致错误")
#
#     if changePwd.oldPassword != sysUser.password:
#         raise UserErrors(err_desc="用户原密码错误")
#
#     sysUser.password = changePwd.newPassword1
#     sysUser.save()
#     return Result.success(data=True)
