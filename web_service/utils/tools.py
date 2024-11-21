import json
import operator
from datetime import datetime, date
from functools import reduce
from os.path import dirname, abspath

from peewee import Model, ModelAlias
from starlette.requests import Request

from models.result import Pagination


def json_serial(obj):
    if isinstance(obj, (datetime, date)):
        return obj.isoformat()
    if isinstance(obj, set):
        return tuple(obj)
    raise TypeError("Type {} s not serializable".format(type(obj)))


class Tools:
    @staticmethod
    def conditions(baseModel: Model, con: list, page: Pagination, aliasStr='t1', fields: tuple = ()):
        alias = baseModel.alias(aliasStr)
        orderBy = Tools.orderByGenerate(alias, page)
        return alias.select(*fields).where(Tools.conditionGenerate(con)).order_by(
            *orderBy)

    @staticmethod
    def conditionGenerate(con: list):
        return reduce(operator.and_, con) if len(con) > 0 else None

    @staticmethod
    def orderByGenerate(aliasObj: ModelAlias, page: Pagination):
        orderBy = []
        for i, val in enumerate(page.sortBy):
            sort = page.sortDesc[i]
            orderBy.append(aliasObj.__getattr__(val).asc() if sort == 'asc' else aliasObj.__getattr__(val).desc())
        return tuple(orderBy)

    @staticmethod
    def getRoot():
        return dirname(dirname(abspath(__file__)))

    @staticmethod
    def getCurrentUserDict(request: Request):
        return json.loads(request.state.user)

    @staticmethod
    def json(obj):
        return json.dumps(obj if isinstance(obj, dict) else obj.__dict__, default=json_serial, sort_keys=True, indent=4,
                          ensure_ascii=False)

    @staticmethod
    def parseToDict(obj_str: str):
        return json.loads(obj_str)

    @staticmethod
    def parseToObj(obj: object, obj_str: str):
        return Tools.dictToObj(obj, **Tools.parseToDict(obj_str))

    @staticmethod
    def dictToObj(obj: object, **data):
        obj.update(data)

    @staticmethod
    def parseValidation(errs: list):
        for err in errs:
            del err["input"]
            if "url" in err:
                del err["url"]
            if len(err['loc']) > 1:
                err['field'] = err['loc'][1]
                del err["loc"]
            if err["type"] == 'missing':
                err["msg"] = "字段必填"
        return errs

    @staticmethod
    def priceToPosition(ownerMoney, percent, price):
        ret_count = ownerMoney * percent / price
        return (int(ret_count / 100)) * 100

    @staticmethod
    def float(f):
        return float('{:.2f}'.format(f))
