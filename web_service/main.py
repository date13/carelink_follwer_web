import json

from apscheduler.executors.pool import ThreadPoolExecutor
from datetime import datetime

import uvicorn
from apscheduler.schedulers.background import BackgroundScheduler
from fastapi import FastAPI
from slowapi import _rate_limit_exceeded_handler
from slowapi.errors import RateLimitExceeded

from core.config import config, IS_DEV
from core.denpends import limiter
from register import register_cors, register_exception, register_middleware
from register.router import register_router
from service.sugarService import addUserCofig, refreshCarelinkTokenIntervalForUser, \
    refreshCarelinkDataIntervalForUser, refreshCarelinkTaskIntervalMinutesForUser
from service.sysService import getSysDictHash
from utils.logutils import my_logger

tags_metadata = [
    {
        "name": "User",
        "description": "用户登录,修改密码接口.",
    },
    {
        "name": "Public",
        "description": "开放接口.",
    }
]

# 初始化 slowapi，注册进 fastapi


# 初始化 slowapi，注册进 fastapi
# @asynccontextmanager
# async def lifespan(app: FastAPI):
#     if config.TASK_RUN:
#         my_logger.info("启动定时任务")
#         asyncio.ensure_future(refreshCarelinkTokenInterval())
#         asyncio.ensure_future(refreshCarelinkDataInterval())
#         asyncio.ensure_future(refreshCarelinkTaskIntervalMinutes())
#     else:
#         my_logger.info("定时任务未启动")
#     yield


# loop = asyncio.get_event_loop()
# loop.set_debug(enabled=True)
app = FastAPI(
    title="Carelink follower API",
    summary="Provide Carelink follower API.",
    description=config.PROJECT_DESC,
    version=config.PROJECT_VERSION,
    openapi_tags=tags_metadata,
    # lifespan=lifespan,
    # dependencies=[Depends(get_db)]
)

app.state.limiter = limiter
app.add_exception_handler(RateLimitExceeded, _rate_limit_exceeded_handler)

register_cors(app)  # 注册跨域请求
register_middleware(app)
register_exception(app)  # 注册异常捕获
register_router(app)


# init_table(False)  # 初始化表
# init_data()  # 初始化表数据
# router = APIRouter()
#
# @app.get("/")
# async def root():
#     return {"message": "Hello Bigger Applications!"}
# @router.get("/test", tags=["test"])
# async def root():
#     return {"message": "Hello World"}
#
# @router.get("/users/", tags=["users"])
# async def read_users():
#     return [{"username": "Rick"}, {"username": "Morty"}]
# app.include_router(router,prefix=config.API_PREFIX, tags=["Public"])
def test(user):
    print("hello %s time:%s" % (user, datetime.now()))


def startTask():
    my_logger.info("启动计划任务！！！")
    executors = {
        'default': ThreadPoolExecutor(20),
    }
    scheduler = BackgroundScheduler(executors=executors)
    users = getSysDictHash("user")
    for key in users:
        user = key.decode()
        options = json.loads(users[key])
        addUserCofig(user, options)
        if options["cgm"] == 'carelink':
            addCarelinkJob(user, scheduler)
    scheduler.start()
    my_logger.info("启动计划任务成功！！！")


def addCarelinkJob(user, scheduler):
    refreshCarelinkTokenIntervalForUser(user)
    refreshCarelinkDataIntervalForUser(user)
    scheduler.add_job(refreshCarelinkTokenIntervalForUser, 'interval', max_instances=3,
                      seconds=config.CARELINK_TOKEN_REFRESH_INTERVAL, args=[user])
    scheduler.add_job(refreshCarelinkDataIntervalForUser, 'interval', max_instances=3,
                      seconds=config.CARELINK_DATA_REFRESH_INTERVAL, args=[user])
    scheduler.add_job(refreshCarelinkTaskIntervalMinutesForUser, 'cron', hour=0, minute=0, args=[user])
    my_logger.info("用户:%s 添加carelink计划任务成功！！！" % user)


# 按间距中的绿色按钮以运行脚本。
if __name__ == '__main__':
    startTask()
    if IS_DEV:
        uvicorn.run(app="main:app", host="0.0.0.0", port=config.HOST_PORT, reload=False)
        # uvicorn.run(app="main:app", host="0.0.0.0", port=8000, reload=False, ssl_keyfile="./ssl/key.pem",
        #             ssl_certfile="./ssl/cert.pem")
    else:
        uvicorn.run(app="main:app", host="0.0.0.0", port=config.HOST_PORT)
    my_logger.info("项目启动成功！！！")
# 访问 https://www.jetbrains.com/help/pycharm/ 获取 PyCharm 帮助
