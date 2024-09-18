from core.redis_core import RedisTool

rds = RedisTool()
value = rds.get_json('carelinkAuth')
value2 = rds.get_json('carelinkData')
print(value)
print(type(value2["data"]))
