export const COLORS = [
  '#09d254',//0血糖
  '#8A2BE2',//1基础率
  '#0fb1ec',//2大剂量
  '#0df5db',//3自动修正
  '#e333c6',//4已输注
  '#ff7474',//5警告线,
  '#ffd539',//6警告颜色
  '#e6f3fb',//7正常区域背景
  '#f1eacf',//8警告区域背景
  '#eeb1c9'//9严重警告区域背景
]

export const REFRESH_INTERVAL = {
  loadData: parseInt(import.meta.env.VITE_APP_LOAD_INTERVAL),
  loadMyData: 60
}

export const CONST_VAR = {
  maxSeriousSg: 12.5,
  maxWarnSg: 9.5,
  maxTightWarnSg: 7.8,
  minWarnSg: 3.9,
  minSeriousSg: 3.2,
  minTightWarnSg: 4,
  exchangeUnit: 18,
  minSgYValueLimit: 2,
  maxSgYValueOffset: 5,
  peakPoint: 3,//显示峰值的大剂量数
  peakMinutes: 35,//峰值时间
  isDemo: import.meta.env.VITE_APP_IS_DEMO === 'true'
}

export const CHART_LEGEND = [
  {name: '血糖', itemStyle: {color: COLORS[0]}},
  {name: '血糖(昨)', itemStyle: {color: COLORS[9]}},
  {name: '基础', itemStyle: {color: COLORS[1]}},
  {name: '大剂量', itemStyle: {color: COLORS[2]}},
  {name: '大剂量(昨)', itemStyle: {color: COLORS[6]}}
]
export const NOTIFICATION_HASH_KEY = 'carelink_follower'
export const SG_STATUS = {
  NO_ERROR_MESSAGE: {
    key: 'NO_ERROR_MESSAGE',
    name: '正常'
  },
  SG_BELOW_40_MGDL: {
    key: 'SG_BELOW_40_MGDL',
    name: '低于3.2'
  },
  NO_DATA_FROM_PUMP: {
    key: 'NO_DATA_FROM_PUMP',
    name: 'n/a'
  },
  WARM_UP: {
    key: 'WARM_UP',
    name: '--'
  },
  CHANGE_SENSOR: {
    key: 'CHANGE_SENSOR',
    name: '--'
  }
}

export const SENSOR_STATUS = {
  SENSOR_DISCONNECTED: {
    key: 'SENSOR_DISCONNECTED',
    name: '探头断开'
  },
  UNKNOWN: {
    key: 'UNKNOWN',
    name: '未知'
  },
  NO_ERROR_MESSAGE: {
    key: 'NO_ERROR_MESSAGE',
    name: '正常'
  },
  SEARCHING_FOR_SENSOR_SIGNAL: {
    key: 'SEARCHING_FOR_SENSOR_SIGNAL',
    name: '寻找探头'
  },
  WARM_UP: {
    key: 'WARM_UP',
    name: '预热'
  },
  DO_NOT_CALIBRATE: {
    key: 'DO_NOT_CALIBRATE',
    name: '不要校准'
  },
  CALIBRATING: {
    key: 'CALIBRATING',
    name: '校准中'
  },
  NO_DATA_FROM_PUMP: {
    key: 'NO_DATA_FROM_PUMP',
    name: '泵无数据'
  },
  CHANGE_SENSOR: {
    key: 'CHANGE_SENSOR',
    name: '更换探头'
  },
  SG_BELOW_40_MGDL: {
    key: 'SG_BELOW_40_MGDL',
    name: '低于3.2'
  },
}
export const PUMP_STATUS = {
  sport: {
    key: "sport",
    name: '运动模式',
    type: 'success'
  },
  manuel: {
    key: "manuel",
    name: '手动模式',
    type: 'info'
  },
  stop: {
    key: "stop",
    name: '暂停输注',
    type: 'danger'
  },
  auto: {
    key: "auto",
    name: '闭环模式',
    type: 'success'
  },
  safe: {
    key: "safe",
    name: '安全模式',
    type: 'success'
  },
  none: {
    key: "none",
    name: '状态未知',
    type: 'info'
  }
}

export const CARELINK_DICT_KEY = {
  carelinkAuth: "carelinkAuth",
  carelinkData: "carelinkData",
  carelinkMyData: "carelinkMyData"
}

export const TIME_RANGE_CONFIG = [
  {
    label: '2',
    value: 10,
    offset: 50,
    interval: 1200
  },
  {
    label: '3',
    value: 13,
    offset: 60,
    interval: 1800
  },
  {
    label: '4',
    value: 17,
    offset: 70,
    interval: 3600
  },
  {
    label: '6',
    value: 25,
    offset: 80,
    interval: 3600
  },
  {
    label: '12',
    value: 52,
    offset: 100,
    interval: 3600
  }
]

export const SYSTEM_STATUS_MAP = {
  NO_ERROR_MESSAGE: {
    key: 'NO_ERROR_MESSAGE',
    name: '正常',
    color: COLORS[0]
  },
  WARM_UP: {
    name: '预热',
    key: 'WARM_UP',
    color: COLORS[4]
  },
  UPDATING: {
    name: '更新中',
    key: 'UPDATING',
    color: COLORS[2]
  },
  SEARCHING_FOR_SENSOR_SIGNAL: {
    name: '寻找探头',
    key: 'SEARCHING_FOR_SENSOR_SIGNAL',
    color: COLORS[5]
  },
  RECONNECTING_TO_PUMP: {
    name: '重连中',
    key: 'RECONNECTING_TO_PUMP',
    color: COLORS[1]
  },
  SENSOR_DISCONNECTED: {
    name: '探头断开',
    key: 'SENSOR_DISCONNECTED',
    color: COLORS[5]
  },
  CHANGE_SENSOR: {
    name: '更换探头',
    key: 'CHANGE_SENSOR',
    color: COLORS[5]
  },
  DO_NOT_CALIBRATE: {
    name: '不要校准',
    key: 'DO_NOT_CALIBRATE',
    color: COLORS[4]
  },
  SG_BELOW_40_MGDL: {
    name: '低探头值',
    key: 'SG_BELOW_40_MGDL',
    color: COLORS[5]
  }
}
export const INSULIN_TYPE = {
  SG: {
    name: '血糖',
    color: COLORS[0],
    key: 'SG',
    symbol: 'circle',
  },
  SG_YESTERDAY: {
    name: '昨日血糖',
    color: COLORS[9],
    key: 'SG_YESTERDAY',
    symbol: 'circle',
  },
  SG_FORECAST: {
    name: '血糖预测',
    color: COLORS[6],
    key: 'SG_FORECAST',
    symbol: 'emptyCircle',
  },
  CALIBRATION: {
    name: '校准血糖',
    color: COLORS[9],
    key: 'CALIBRATION'
  },
  TIME_CHANGE: {
    name: '校准时间',
    color: COLORS[9],
    key: 'TIME_CHANGE'
  },
  AUTO_BASAL_DELIVERY: {
    name: '基础率',
    color: COLORS[1],
    key: 'AUTO_BASAL_DELIVERY'
  },
  INSULIN: {
    name: '大剂量',
    color: COLORS[4],
    color2: COLORS[2],
    color3: COLORS[0],
    key: 'INSULIN',
    text: ['手动输注', '已输注', '碳水']
  },
  INSULIN_YESTERDAY: {
    name: '昨日大剂量',
    color: COLORS[4],
    color2: COLORS[6],
    color3: COLORS[0],
    key: 'INSULIN_YESTERDAY',
    text: ['手动输注', '已输注', '碳水']
  },
  AUTOCORRECTION: {
    name: '自动修正',
    color: COLORS[3],
    key: 'AUTOCORRECTION'
  }
}
export const DIRECTIONS = {
  NONE: {
    direction: 0,
  }
  , UP_DOUBLE: {
    direction: 1,
    num: 2
  }
  , UP: {
    direction: 1,
    num: 1
  }
  , UP_TRIPLE: {
    direction: 1,
    num: 3
  }
  , FLAT: {
    direction: 3,
  }
  , DOWN_TRIPLE: {
    direction: 5,
    num: 3
  }
  , DOWN: {
    direction: 5,
    num: 1
  }
  , DOWN_DOUBLE: {
    direction: 5,
    num: 2
  }
  , 'NOT COMPUTABLE': {
    direction: 0
  }
  , 'RATE OUT OF RANGE': {
    direction: 0
  }
};
