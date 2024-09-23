import json

import numpy as np
import pandas as pd
from filterpy.kalman import KalmanFilter
from statsmodels.tsa.ar_model import AutoReg

from core.redis_core import RedisTool
from service.sugarService import forcastAR2Sg

# 假设我们有一些历史数据
blood_glucose = np.array([
    96,
    98,
    103,
    108,
    110,
    116,
    126,
    131,
    137,
    143,
    148,
    150,
    148,
    147,
    144,
    139,
    134,
    135,
    139,
    137,
    132,
    126,
    121,
    119,
    118,
    119,
    119,
    119,
    118,
    116,
    112,
    109,
    107,
    105,
    104,
    103,
    102,
    102,
    102,
    102,
    103,
    104,
    105,
    107,
    107,
    109,
    110,
    111,
    110,
    108,
    106,
    104,
    103,
    102,
    100,
    98,
    96,
    95,
    94,
    93,
    92,
    92,
    91,
    90,
    90,
    89,
    91,
    92,
    94,
    95,
    98,
    102,
    100,
    102,
    101,
    100,
    99,
    98,
    97,
    96,
    95,
    94,
    94,
    94,
    93,
    92,
    91,
    92,
    92,
    92,
    94,
    93,
    93,
    93,
    93,
    93,
    95,
    96,
    98,
    99,
    102,
    105,
    103,
    106,
    107,
    105,
    105,
    104,
    103,
    102,
    103,
    103,
    101,
    100,
    97,
    95,
    96,
    96,
    93,
    89,
    87,
    86,
    87,
    88,
    89,
    92,
    99,
    102,
    100,
    100,
    99,
    99,
    98,
    98,
    97,
    94,
    93,
    92,
    88,
    87,
    86,
    87,
    88,
    93,
    99,
    101,
    98,
    98,
    97,
    95,
    94,
    95,
    91,
    90,
    90,
    93,
    96,
    105,
    111,
    118,
    119,
    116,
    122,
    121,
    122,
    123,
    124,
    121,
    116,
    108,
    100,
    98,
    95,
    89,
    86,
    82,
    78,
    78,
    74,
    71,
    74,
    79,
    89,
    100,
    106,
    107,
    113,
    111,
    107,
    104,
    98,
    90,
    83,
    85,
    91,
    99,
    115,
    121,
    125,
    123,
    119,
    118,
    117,
    117,
    115,
    115,
    116,
    118,
    119,
    123,
    123,
    125,
    125,
    126,
    127,
    126,
    129,
    132,
    133,
    139,
    145,
    150,
    151,
    148,
    143,
    136,
    130,
    129,
    130,
    135,
    139,
    143,
    142,
    139,
    139,
    135,
    127,
    122,
    123,
    121,
    115,
    114,
    109,
    102,
    97,
    96,
    96,
    99,
    101,
    103,
    106,
    110,
    112,
    113,
    116,
    120,
    128,
    136,
    131,
    123,
    120,
    115,
    106,
    96,
    91,
    90,
    90,
    96,
    101,
    102,
    100,
    95,
    88,
    84,
    84,
    87,
    88,
    87,
    91,
    94,
    93,
    92,
    95,
    97,
    97,
    98,
    98,
    98
])


def ar1():
    # AR(1)模型参数（这些参数通常是通过训练得到的）
    alpha = 10  # 截距项
    beta = 0.8  # 滞后一阶的系数

    # 初始化预测数组
    predictions = []

    # 使用最后一个历史数据作为第一个预测的基础
    last_value = blood_glucose[-1]

    # 预测未来5个时间点的数据
    for _ in range(10):
        # 计算下一个时间点的预测值
        next_prediction = alpha + beta * last_value
        predictions.append(next_prediction)
        # 更新用于下一次预测的最后一个值
        last_value = next_prediction

    # 将预测结果转换为NumPy数组
    predictions = np.array(predictions)

    # 打印预测结果
    print("未来血糖预测:")
    for i, glucose in enumerate(predictions, start=1):
        print(f"时间: {i} 时间单位后, 血糖: {glucose:.2f} mg/dL")


def ar2():
    series = pd.Series(blood_glucose)

    # 拟合AR(2)模型
    model = AutoReg(series, lags=2)
    model_fit = model.fit()

    # 打印出模型的摘要信息
    # print(model_fit.summary())

    # 使用模型对未来5个时间点进行预测
    forecast = model_fit.predict(start=len(series), end=len(series) + 9, dynamic=False)

    # 将原始数据与预测结果合并，以便绘图
    # full_series = series.append(pd.Series(forecast, index=np.arange(len(series), len(series) + 5)))

    # 打印预测结果
    print("未来血糖预测:")
    for i, glucose in enumerate(forecast, start=1):
        print(f"时间: {i} 时间单位后, 血糖: {glucose:.0f} mg/dL")


def filterpyCalc():
    # 给定的时间序列数据
    data = np.array(
        [113, 116, 120, 128, 136, 131, 123, 120, 115, 106, 96, 91, 90, 90, 96, 101, 102, 100, 95, 88, 84, 84, 87, 88,
         87, 91, 94, 93, 92, 95, 97, 97, 98, 98, 98])

    # 初始化卡尔曼滤波器
    kf = KalmanFilter(dim_x=1, dim_z=1)

    # 初始状态（初始血糖值）
    kf.x = np.array([[data[0]]])

    # 状态转移矩阵
    kf.F = np.array([[1.]])

    # 观测矩阵
    kf.H = np.array([[1.]])

    # 测量噪声方差
    kf.R = np.array([[1.]])

    # 过程噪声方差
    kf.Q = np.array([[0.01]])

    # 预测列表
    predictions = []

    # 使用卡尔曼滤波器进行预测
    for i in range(len(data)):
        kf.predict()
        kf.update(np.array([[data[i]]]))
        predictions.append(kf.x[0][0])

    # 预测未来5个时间点的数据
    future_predictions = []
    for _ in range(5):
        kf.predict()
        future_predictions.append(kf.x[0][0])

    # 打印预测结果
    print("未来血糖预测:")
    for i, glucose in enumerate(future_predictions, start=1):
        print(f"时间: {i} 时间单位后, 血糖: {glucose:.2f} mg/dL")


# filterpyCalc()
# ar1()
# ar2()
def readData():
    rds = RedisTool()
    dataObj = rds.get_json("carelinkData")
    sgsObj = json.loads(dataObj['data'])

    print(sgsObj["sgs"])
    sgsList = filter(list(map(lambda x: x["sg"], sgsObj["sgs"])))
    print(sgsList)

    forcastAR2Sg(sgsList)

# readData()
