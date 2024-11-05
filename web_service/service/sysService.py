import json
from typing import Optional

from pydantic import BaseModel

from core.auth import Auth
from core.redis_core import RedisTool
from models.result import SysUserForm


class UpdateSysDictForm(BaseModel):
    key: str = ""
    subKey: Optional[str] = None
    val: str | None


rds = RedisTool()


def getSysDict(key: str):
    return rds.get_value(key)


def getSysDictForUser(user, key: str):
    return rds.get_value("%s:%s" % (user, key))


def getSysDictHash(key: str):
    return rds.get_all_hash_kv(key)


def getSysDictHashForUser(user, key: str):
    return rds.get_all_hash_kv("%s:%s" % (user, key))


def updateSysDict(updateForm: UpdateSysDictForm):
    # dictObj = await getSysDict(updateForm.key)
    # if dictObj is None:
    #     sysDict = SysDict(key=updateForm.key, val=updateForm.val)
    #     return sysDict.save()
    # else:
    #     return await async_db.execute(SysDict.update(val=updateForm.val).where(
    #         SysDict.key == updateForm.key))
    if updateForm.subKey is None:
        return rds.set_value(updateForm.key, updateForm.val)
    else:
        return rds.set_hash_kv(updateForm.key, updateForm.subKey, updateForm.val)


def updateSysDictForUser(user, updateForm: UpdateSysDictForm):
    # dictObj = await getSysDict(updateForm.key)
    # if dictObj is None:
    #     sysDict = SysDict(key=updateForm.key, val=updateForm.val)
    #     return sysDict.save()
    # else:
    #     return await async_db.execute(SysDict.update(val=updateForm.val).where(
    #         SysDict.key == updateForm.key))
    if updateForm.subKey is None:
        return rds.set_value("%s:%s" % (user, updateForm.key), updateForm.val)
    else:
        return rds.set_hash_kv("%s:%s" % (user, updateForm.key), updateForm.subKey, updateForm.val)


def login(loginForm: SysUserForm):
    user = json.loads(rds.get_hash_kv("user", loginForm.name))
    password = user["password"]
    if password is not None and password == loginForm.password.upper():
        return {
            "name": loginForm.name,
            "admin": user["admin"],
            "token": Auth().encode_token(loginForm.name)
        }
    else:
        return None

# form = SysUserForm(name='alex', password='201205')
# print(login(form))
# print(Auth().decode_token('eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzA0Mzk4MDgsImlhdCI6MTcyOTU3NTgwOCwic2NvcGUiOiJhY2Nlc3NfdG9rZW4iLCJzdWIiOiJhbGV4In0.G-HS6OI7v9l2tDc069l6chI58I5vT2Q6pSkIhkQgSdc'))
# print(rds.get_all_hash_kv("history"))
