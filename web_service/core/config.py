#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# @desc : 配置文件
import argparse

from pydantic_settings import BaseSettings, SettingsConfigDict

from utils.tools import Tools

parser = argparse.ArgumentParser(description='manual to this script')
parser.add_argument("--mode", type=str, default="dev")
args = parser.parse_args()
IS_DEV = True if args.mode != 'prod' else False  # 是否开发环境
print(f'MODE:{args.mode} ,IS_DEV:{IS_DEV} ,root:{Tools.getRoot()}')
config_map = {
    "dev": '/.dev.env',
    "prod": "/.prod.env",
    "local": "/.local.env"
}


# https://docs.pydantic.dev/usage/settings/
class Settings(BaseSettings):
    PROJECT_DESC: str  # 描述
    PROJECT_VERSION: int | str  # 版本
    API_PREFIX: str
    SALT: str  # md5 加密盐
    TOKEN_EXP: int
    HOST_PORT: int
    TASK_RUN: bool
    MAIL_SEND: bool

    DB_HOST: str  # MySQL
    DB_USER: str
    DB_PASSWORD: str
    DB_PORT: int
    DB_SCHEME: str
    DB_MAX_CONNECTIONS: int

    REDIS_HOST: str
    REDIS_PASSWORD: str
    REDIS_PORT: int
    REDIS_DATABASE: int

    CARELINK_API_DOMAIN: str
    CARELINK_DATA_URL: str
    CARELINK_TOKEN_REFRESH_INTERVAL: int
    CARELINK_DATA_REFRESH_INTERVAL: int
    CARELINK_PATIENTID: str
    CARELINK_USERNAME: str
    CARELINK_ROLE: str
    LOGGER_DIR: str  # 日志文件夹名
    LOGGER_NAME: str  # 日志文件名 (时间格式)
    LOGGER_LEVEL: str  # 日志等级: ['DEBUG' | 'INFO']
    LOGGER_ROTATION: str  # 日志分片: 按 时间段/文件大小 切分日志. 例如 ["500 MB" | "12:00" | "1 week"]
    LOGGER_RETENTION: str  # 日志保留的时间: 超出将删除最早的日志. 例如 ["1 days"]

    MAIL_USER: str
    MAIL_PWD: str
    MAIL_TO: str
    MAIL_SMTP_HOST: str

    model_config = SettingsConfigDict(
        env_file=f'{Tools.getRoot()}{config_map[args.mode]}',
        env_file_encoding='utf-8')


config = Settings()
if IS_DEV:
    print(config.model_dump())
# config = config.model_dump()
