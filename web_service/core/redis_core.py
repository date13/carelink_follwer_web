import json

import redis

from core.config import config


class RedisTool(object):
    def __init__(self):
        # pool = redis.ConnectionPool(host=config.REDIS_HOST, password=config.REDIS_PASSWORD, port=config.REDIS_PORT,
        #                             db=config.REDIS_DATABASE)
        # self.client = redis.Redis(connection_pool=pool)
        self.client = redis.StrictRedis(host=config.REDIS_HOST, password=config.REDIS_PASSWORD, port=config.REDIS_PORT,
                                        db=config.REDIS_DATABASE)

    def set_value(self, key, value):
        return self.client.set(key, value)

    def get_value(self, key):
        return self.client.get(key)

    def del_key(self, key):
        return self.client.delete(key)

    def set_hash_kv(self, name, key, value):
        return self.client.hset(name, key, value)

    def get_all_hash_kv(self, name):
        return self.client.hgetall(name)

    def get_hash_kv(self, name, key):
        return self.client.hget(name, key)

    def del_hash_kv(self, name, *keys):
        return self.client.hdel(name, *keys)

    def set_json(self, key, json_obj):
        json_str = json.dumps(json_obj)
        return self.client.set(key, json_str)

    def get_json(self, key):
        json_bytes = self.client.get(key)
        if json_bytes:
            return json.loads(json_bytes)
        return None
