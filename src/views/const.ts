export const COLORS = [
  '#09d254',//血糖
  '#8A2BE2',//基础率
  '#0fb1ec',//大剂量
  '#0df5db',//自动修正
  '#e333c6',//已输注
  '#f00',//警告线,
  '#ffe166',//警告颜色
]
export const REFRESH_INTERVAL = {
  loadData: parseInt(import.meta.env.VITE_APP_LOAD_INTERVAL),
}
export const CONST_VAR = {
  maxSeriousSg: 13.5,
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
