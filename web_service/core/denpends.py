#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 依赖项
from slowapi import Limiter
from slowapi.util import get_remote_address
from starlette.requests import Request

limiter = Limiter(key_func=get_remote_address)


def check_cookie(request: Request):
    pass
    # """ 校验cookie -- 认证"""
    # if request.get("path") in config.COOKIE_NOT_CHECK:  # 不校验Cookie
    #     return
    #
    # session = request.cookies.get(config.COOKIE_KEY)
    # try:
    #     # _user = LocalUser.get(session)
    #     # user_obj = user_crud.get(db, _user.id)  # noqa
    #     # assert user_obj, "用户不存在"  # 判断 user_obj 是否存在
    #     #
    #     # old_user = LocalUserSchema(**_user.dict())  # 旧用户信息
    #     # new_user = LocalUserSchema.from_orm(user_obj)  # 新用户信息
    #     # assert old_user.version == new_user.version, "用户权限变更"  # 判断用户权限是否变更
    #
    #     assert old_user.dict() == new_user.dict(), "用户信息变更"  # 判断用户信息是否变更
    #
    #     return _user
    # except NotFoundError:
    #     raise BadCredentials()
    # except AssertionError as e:
    #     raise BadCredentials(err_desc=str(e))


# CheckCookie = Annotated[LocalUser, Depends(check_cookie)]


def check_permission(code: list[str] | None = None):
    """ 校验权限code -- 鉴权"""

    def wrapper(request: Request):
        pass

        # if not code:
        #     resource = resource_crud.get_resource_by_url(db, request.get("path").replace(settings.API_PREFIX, ""))
        #     if resource and resource.permission_code not in _user.permission_codes:  # 查不到资源, 表示不需要权限
        #         raise UserErrors(code=status.HTTP_403_FORBIDDEN, err_desc="没有权限")
        # else:
        #     if not set(code).issubset(set(_user.permission_codes)):  # 判断一个列表中是否包含另一个列表中的元素
        #         raise UserErrors(code=status.HTTP_403_FORBIDDEN, err_desc="没有权限")

    return wrapper
#
#
# def page_query(page: int = 1, page_size: int = 10):
#     """ 获取查询参数 """
#     return PageSchema(page=page, page_size=page_size)
#
#
# PageQuery = Annotated[PageSchema, Depends(page_query)]
