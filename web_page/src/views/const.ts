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

export const CARELINK_DICT_KEY = {
  carelinkAuth: "carelinkAuth",
  carelinkData: "carelinkData",
  carelinkMyData: "carelinkMyData"
}

export const PUMP_STATUS = {
  sport: {
    key: "sport",
    name: '运动模式',
    type: 'warning'
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
  }
}

export const TIME_RANGE_CONFIG = [
  {
    label: '2',
    value: 8
  },
  {
    label: '3',
    value: 13
  },
  {
    label: '4',
    value: 17
  },
  {
    label: '6',
    value: 25
  },
  {
    label: '12',
    value: 50
  }
]

export const SYSTEM_STATUS_MAP = {
  NO_ERROR_MESSAGE: {
    name: '正常',
    color: COLORS[0]
  },
  WARNING: {
    name: '警告',
    color: COLORS[6]
  },
  ERROR: {
    name: '错误',
    color: COLORS[5]
  },
  WARM_UP: {
    name: '预热',
    color: COLORS[4]
  }
}
export const REFRESH_INTERVAL = {
  loadData: parseInt(import.meta.env.VITE_APP_LOAD_INTERVAL),
  loadMyData: 60
}
export const CONST_VAR = {
  maxSeriousSg: 12.5,
  maxWarnSg: 9.5,
  minWarnSg: 3.9,
  minSeriousSg: 3.2,
  exchangeUnit: 18,
  minSgYValueLimit: 2,
  maxSgYValueOffset: 5,
  isDemo: import.meta.env.VITE_APP_IS_DEMO === 'true'
}
export const INSULIN_TYPE = {
  SG: {
    name: '血糖',
    color: COLORS[0],
    key: 'SG'
  },
  CALIBRATION: {
    name: '校准',
    color: COLORS[5],
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
    direction: 2,
  }
  , FLAT: {
    direction: 3,
  }
  , DOWN_TRIPLE: {
    direction: 4,
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
