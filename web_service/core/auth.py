import hashlib
from datetime import datetime, timedelta  # used to handle expiry time for tokens

import jwt
from fastapi import HTTPException  # used to handle error handling
from passlib.context import CryptContext  # used for hashing the password

from core.config import config


class Auth():
    # hasher= CryptContext()
    hasher = CryptContext(schemes=["sha512_crypt"])
    # secret = os.getenv("APP_SECRET_STRING")
    secret = config.SALT

    def md5(self, password):
        hl = hashlib.md5()
        # 更新hash对象的值，如果不使用update方法也可以直接md5构造函数内填写
        # md5_obj=hashlib.md5("123456".encode("utf-8")) 效果一样
        hl.update(password.encode("utf-8"))
        return hl.hexdigest()

    def encode_password(self, password):
        return self.hasher.hash(password)

    def verify_password(self, password, encoded_password):
        return self.hasher.verify(password, encoded_password)

    def encode_token(self, payload):
        payload = {
            'exp': datetime.utcnow() + timedelta(days=0, seconds=config.TOKEN_EXP),
            'iat': datetime.utcnow(),
            'scope': 'access_token',
            'sub': payload
        }
        return jwt.encode(
            payload,
            self.secret,
            algorithm='HS256'
        )

    def decode_token(self, token):
        try:
            payload = jwt.decode(token, self.secret, algorithms=['HS256'])
            if payload['scope'] == 'access_token':
                return payload['sub']
            raise HTTPException(status_code=401, detail='Scope for the token is invalid')
        except jwt.ExpiredSignatureError:
            raise HTTPException(status_code=401, detail='Token expired')
        except jwt.InvalidTokenError:
            raise HTTPException(status_code=401, detail='Invalid token')

    def encode_refresh_token(self, username):
        payload = {
            'exp': datetime.utcnow() + timedelta(days=0, hours=10),
            'iat': datetime.utcnow(),
            'scope': 'refresh_token',
            'sub': username
        }
        return jwt.encode(
            payload,
            self.secret,
            algorithm='HS256'
        )

    def refresh_token(self, refresh_token):
        try:
            payload = jwt.decode(refresh_token, self.secret, algorithms=['HS256'])
            if (payload['scope'] == 'refresh_token'):
                username = payload['sub']
                new_token = self.encode_token(username)
                return new_token
            raise HTTPException(status_code=401, detail='Invalid scope for token')
        except jwt.ExpiredSignatureError:
            raise HTTPException(status_code=401, detail='Refresh token expired')
        except jwt.InvalidTokenError:
            raise HTTPException(status_code=401, detail='Invalid refresh token')
