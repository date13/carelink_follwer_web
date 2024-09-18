from core.auth import Auth
from models.beans import SysUser
from utils.tools import Tools

query = SysUser.select().where(SysUser.name == 'admin')
user = query[0]
auth = Auth()
print(Tools.json(user.__data__))
print(Tools.json({"a": 1, "b": 2}))
a = '''{
    "avatar": null,
    "gender": null,
    "id": 87,
    "insert_time": "2023-07-27T16:18:14",
    "is_deleted": 0,
    "name": "admin",
    "password": "30780cc6f2e56945aaf9c9578c932e22",
    "phone": null,
    "update_time": null
}'''
dictA = Tools.parseToDict(a)
userA = SysUser()
userA.name = "test"
print(dictA)
print(userA.name, type(userA))
# Tools.dictToObj(userA.__data__, **dictA)
Tools.parseToObj(userA.__data__, a)
print(userA.name)
