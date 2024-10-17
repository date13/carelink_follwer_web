#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from typing import Any, TypeVar, Generic

from pydantic import BaseModel

T = TypeVar("T")  # 泛型 T ：https://docs.pydantic.dev/usage/models/#generic-models


class RedisHashObj(BaseModel):
    key: str
    val: str | float | int


class ResultSchema(BaseModel, Generic[T]):
    """ 结果数据模型 """
    code: int
    data: T | list[T] | None
    msg: str | None


class PageResult(BaseModel, Generic[T]):
    list: list[T] | None
    total: int = 0


class Pagination(BaseModel):
    """ 分页查询参数 """
    page: int = 1
    size: int = 15
    total: int = 0
    sortDesc: list = []
    sortBy: list = []


class Result:
    """ 结果数据结构 """

    @staticmethod
    def success(*, code: int = 0, data: T = None, total: int = 0, msg: str = "Success") -> ResultSchema[T]:
        """ 成功响应数据结构 """
        return ResultSchema(code=code, data=data, total=total, msg=msg)

    @staticmethod
    def fail(*, code: int = -1, data: Any = None, msg: str = "Fail") -> ResultSchema[T]:
        """ 失败响应数据结构 """
        return ResultSchema(code=code, data=data, msg=msg, total=None)
