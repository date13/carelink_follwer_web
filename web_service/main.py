import asyncio
from contextlib import asynccontextmanager

import uvicorn
from fastapi import FastAPI
from slowapi import _rate_limit_exceeded_handler
from slowapi.errors import RateLimitExceeded

from core.config import config, IS_DEV
# from core.db_core import get_db
from core.denpends import limiter
from register import register_cors, register_exception, register_middleware
from register.router import register_router
from service.sugarService import refreshCarelinkTokenInterval, refreshCarelinkDataInterval, \
    refreshCarelinkTaskIntervalMinutes
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
@asynccontextmanager
async def lifespan(app: FastAPI):
    if config.TASK_RUN:
        my_logger.info("启动定时任务")
        asyncio.ensure_future(refreshCarelinkTokenInterval())
        asyncio.ensure_future(refreshCarelinkDataInterval())
        asyncio.ensure_future(refreshCarelinkTaskIntervalMinutes())
    else:
        my_logger.info("定时任务未启动")
    yield


loop = asyncio.get_event_loop()
loop.set_debug(enabled=True)
app = FastAPI(
    title="Carelink follower API",
    summary="Provide Carelink follower API.",
    description=config.PROJECT_DESC,
    version=config.PROJECT_VERSION,
    openapi_tags=tags_metadata,
    lifespan=lifespan,
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


# 按间距中的绿色按钮以运行脚本。
if __name__ == '__main__':
    my_logger.info("项目启动成功！！！")
    if IS_DEV:
        uvicorn.run(app="main:app", host="0.0.0.0", port=config.HOST_PORT, reload=False)
        # uvicorn.run(app="main:app", host="0.0.0.0", port=8000, reload=False, ssl_keyfile="./ssl/key.pem",
        #             ssl_certfile="./ssl/cert.pem")
    else:
        uvicorn.run(app="main:app", host="0.0.0.0", port=config.HOST_PORT)
# 访问 https://www.jetbrains.com/help/pycharm/ 获取 PyCharm 帮助
