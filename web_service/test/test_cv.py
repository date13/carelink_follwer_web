from core.redis_core import RedisTool
from service.sugarService import dictKey
import numpy as np

user = "alex"
rds = RedisTool()
data = rds.get_json("%s:%s" % (user, dictKey["data"]))
sgData = data["data"]
sgs = sgData["sgs"]
avg = sgData["averageSG"]

arr = list(map(lambda n: n["sg"],
               filter(lambda n: (n["kind"] == "SG" and n["sensorState"] == "NO_ERROR_MESSAGE" and n["sg"] != 0) or n[
                   "sensorState"] == "SG_BELOW_40_MGDL", sgs)))
cv = (np.std(arr) / avg) * 100
print(cv)
print(cv>25)
