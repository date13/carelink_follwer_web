export const COLORS = [
  '#09d254',//0血糖
  '#8A2BE2',//1基础率
  '#0fb1ec',//2大剂量
  '#0df5db',//3自动修正
  '#e333c6',//4已输注
  '#f00',//5警告线,
  '#ffe166',//6警告颜色
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
    name: '探头值低于2.2'
  },
  NO_DATA_FROM_PUMP: {
    key: 'NO_DATA_FROM_PUMP',
    name: 'n/a'
  },
  WARM_UP: {
    key: 'WARM_UP',
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
    name: '无需校准'
  },
  CALIBRATING: {
    key: 'CALIBRATING',
    name: '校准中'
  },
  NO_DATA_FROM_PUMP: {
    key: 'NO_DATA_FROM_PUMP',
    name: '泵无数据'
  }
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

export const NOTIFICATION_MAP = {
  BC_SID_BOLUS_ENTRY_TIMED_OUT: {
    text: `大剂量输入超时`,
    replace: '',
    type: 'warning'
  },
  BC_SID_SG_APPROACH_LOW_LIMIT_CHECK_BG: {
    text: `低探头值前报警`,
    replace: '',
    type: 'warning'
  },
  BC_SID_SG_APPROACH_HIGH_LIMIT_CHECK_BG: {
    text: '高探头值前报警',
    replace: '',
    type: 'warning'
  },
  BC_SID_LOW_SD_CHECK_BG: {
    text: '低探头值报警: sg',
    replace: 'sg',
    type: 'error'
  },
  BC_SID_HIGH_SG_CHECK_BG: {
    text: '高探头值报警: sg',
    replace: 'sg',
    type: 'error'
  },
  BC_MESSAGE_SG_UNDER_50_MG_DL: {
    text: '探头值低于3: sg',
    replace: 'sg',
    type: 'error'
  },
  BC_SID_REPLACE_BATTERY_SOON: {
    text: '请立即更换电池',
    replace: null,
    type: 'error'
  },
  BC_SID_INSERT_NEW_SENSOR: {
    text: '请更换新探头',
    replace: null,
    type: 'error'
  },
  BC_SID_MOVE_PUMP_CLOSER_TO_MINILINK: {
    text: '请将泵靠近MINILINK',
    replace: null,
    type: 'warning'
  },
  BC_SID_SG_RISE_RAPID: {
    text: '血糖快速上升',
    replace: null,
    type: 'error'
  },
  BC_SID_IF_NEW_SENSR_SELCT_START_NEW_ELSE_REWIND: {
    text: '如果是新探头,请选择开启新探头,否则请选择继续使用',
    replace: null,
    type: 'warning'
  },
  BC_MESSAGE_TIME_REMAINING_CHANGE_RESERVOIR: {
    text: '等待更换储药器',
    replace: null,
    type: 'warning'
  },
  BC_SID_SMART_GUARD_MINIMUM_DELIVERY: {
    text: '闭环安全模式最小输注',
    replace: null,
    type: 'warning'
  },
  BC_SID_ENTER_BG_TO_CONTINUE_IN_SMART_GUARD: {
    text: '请输入血糖值,继续使用闭环模式',
    replace: null,
    type: 'warning'
  },
  BC_SID_SENSOR_RELATED_ISSUE_INSERT_NEW: {
    text: '请更换新探头',
    replace: null,
    type: 'error'
  },
  BC_SID_BASAL_STARTED_SMART_GUARD: {
    text: '基础率开启，请继续使用闭环模式',
    replace: null,
    type: 'warning'
  },
  BC_SID_BATTERY_LIFE_LESS_30_MINUTES: {
    text: '电池剩余时间不足30分钟',
    replace: null,
    type: 'warning'
  },
  BC_SID_DELIVERY_STOPPED_INSERT_NEW_BATTERY: {
    text: '输注暂停,请更换新电池',
    replace: null,
    type: 'error'
  },
  BC_SID_SELECT_FILL_CANNULA_OR_SKIP: {
    text: '请选择是否使用新管路或跳过',
    replace: null,
    type: 'warning'
  },
  BC_SID_UPDATING_CAN_TAKE_UP_TO_THREE_HOURS: {
    text: '更新中,预计耗时3小时',
    replace: null,
    type: 'warning'
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
    interval: 1200,
    right: 20
  },
  {
    label: '3',
    value: 13,
    offset: 60,
    interval: 1800,
    right: 20
  },
  {
    label: '4',
    value: 17,
    offset: 70,
    interval: 3600,
    right: 20
  },
  {
    label: '6',
    value: 25,
    offset: 80,
    interval: 3600,
    right: -25
  },
  {
    label: '12',
    value: 52,
    offset: 100,
    interval: 3600,
    right: -25
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
    name: '校准',
    color: COLORS[9],
    key: 'CALIBRATION'
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
