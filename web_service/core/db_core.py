from contextvars import ContextVar

from fastapi import Depends
from peewee import _ConnectionState, Model
# 异步数据库 # 连接池
from peewee_async import PooledMySQLDatabase as AsyncPooledMySQLDatabase, Manager
from playhouse.shortcuts import ReconnectMixin

from core.config import config

db_state_default = {"closed": None, "conn": None, "ctx": None, "transactions": None}
db_state = ContextVar("db_state", default=db_state_default.copy())


class PeeweeConnectionState(_ConnectionState):
    def __init__(self, **kwargs):
        super().__setattr__("_state", db_state)
        super().__init__(**kwargs)

    def __setattr__(self, name, value):
        self._state.get()[name] = value

    def __getattr__(self, name):
        return self._state.get()[name]


class ReconnectAsyncPooledMySQLDatabase(ReconnectMixin, AsyncPooledMySQLDatabase):
    _instance = None

    @classmethod
    def get_db_instance(cls):
        if not cls._instance:
            cls._instance = cls(**{
                'host': config.DB_HOST,
                'port': config.DB_PORT,
                'user': config.DB_USER,
                'password': config.DB_PASSWORD,
                'database': config.DB_SCHEME,
                'charset': 'utf8',
                'sql_mode': 'PIPES_AS_CONCAT',
                'use_unicode': True,
            }, max_connections=config.DB_MAX_CONNECTIONS)
        return cls._instance


db = ReconnectAsyncPooledMySQLDatabase.get_db_instance()
async_db = Manager(db)
async_db._state = PeeweeConnectionState()
db._state = PeeweeConnectionState()


async def reset_db_state():
    db._state._state.set(db_state_default.copy())
    db._state.reset()


async def get_db(db_state=Depends(reset_db_state)):
    try:
        await async_db.connect()
    finally:
        await async_db.close()


class BaseModel(Model):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        """将事务改成atomic_async"""
        self.trans = db.aio_atomic
        """添加一个Manager类"""
        # self.object = Manager(db)

    class Meta:
        database = db


class UnknownField(object):
    def __init__(self, *_, **__): pass
