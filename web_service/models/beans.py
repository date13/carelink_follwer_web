from peewee import *
from peewee import SQL, DateTimeField

from core.db_core import BaseModel


class SysDict(BaseModel):
    key = CharField()
    val = TextField(null=True)

    class Meta:
        table_name = 'sys_dict'


class SysUser(BaseModel):
    avatar = CharField(null=True)
    gender = IntegerField(null=True)
    insert_time = DateTimeField(null=True)
    is_deleted = IntegerField(constraints=[SQL("DEFAULT 0")], null=True)
    name = CharField()
    password = CharField()
    phone = CharField(null=True)
    update_time = DateTimeField(null=True)

    class Meta:
        table_name = 'sys_user'
