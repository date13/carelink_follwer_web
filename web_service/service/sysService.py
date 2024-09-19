from pydantic import BaseModel

# from core.db_core import async_db
from core.redis_core import RedisTool


# from models.beans import SysDict


class UpdateSysDictForm(BaseModel):
    key: str = ""
    val: str | None


rds = RedisTool()


def getSysDict(key: str):
    return rds.get_value(key)


def updateSysDict(updateForm: UpdateSysDictForm):
    # dictObj = await getSysDict(updateForm.key)
    # if dictObj is None:
    #     sysDict = SysDict(key=updateForm.key, val=updateForm.val)
    #     return sysDict.save()
    # else:
    #     return await async_db.execute(SysDict.update(val=updateForm.val).where(
    #         SysDict.key == updateForm.key))
    return rds.set_value(updateForm.key, updateForm.val)
