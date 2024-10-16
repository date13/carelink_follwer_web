from core.redis_core import RedisTool

rds = RedisTool()
# value = rds.get_json('carelinkAuth')
# value2 = rds.get_json('carelinkData')
# print(value)
# print(type(value2["data"]))
rds.set_hash_kv('food','米饭','28')
rds.set_hash_kv('food','炝饼','48')
list = rds.get_all_hash_kv('food')
for key in list:
    print(str(key,'utf-8'), ":", list[key])