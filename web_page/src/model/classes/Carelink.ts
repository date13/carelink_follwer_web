import Base from "@/model/classes/Base";
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";
import {COLORS, TIME_RANGE_CONFIG} from "@/views/const";

export default class Carelink extends Base {
  lastSG = {
    //最后获取的数据时间
    datetime: dayjs().format(DATE_FORMAT.datetime),
    sg: 0
  }
  sgs = []
  notificationHistory = {
    activeNotifications: [],
    clearedNotifications: []
  }
  activeInsulin = {
    amount: 0
  }
  basal = {}
  therapyAlgorithmState = {}
  markers = []
  gstBatteryLevel = 0
  sensorDurationMinutes = 0

  constructor(obj?: any | undefined) {
    super(obj)
    if (obj) {
      Object.assign(this, obj)
    }
  }
}

export class Log extends Base {
  content: string = ''
  time: number = new Date().getTime()
  type: "success" | "warning" | "info" | "error" = 'info'

  constructor(obj?: any | undefined) {
    super(obj)
    if (obj) {
      Object.assign(this, obj)
    }
  }
}

export class SugarSetting extends Base {
  startPercent = TIME_RANGE_CONFIG[1].value
  showAR2 = true
  showYesterday = true
  logs: Log[] = []
  notification = {
    hasNew: false,
    lastKey: null,
    lastAlarm: {
      key: '',
      isClear: false,
      isActive: false
    }
  }
  realWave = true
  showPeak = true

  constructor(obj?: any | undefined) {
    super(obj)
    if (obj) {
      Object.assign(this, obj)
    }
  }
}

export class InTimeBarChartData extends Base {
  backgroundColor = '' //设置无背景色
  title = {
    text: '',
    left: 'center',
    textStyle: {
      fontSize: 16
    }
  }
  tooltip = {
    trigger: 'item'
  }
  label = {
    show: true, // 显示标签
    position: 'inside',
    formatter: '{d}%' // 设置标签格式
  }
  series = [
    {
      name: '',
      type: 'pie',
      radius: '50%',
      data: [
        {value: '', name: '框内', itemStyle: {color: COLORS[0]}},
        {value: '', name: '低于', itemStyle: {color: COLORS[1]}},
        {value: '', name: '高于', itemStyle: {color: COLORS[5]}}
      ]
    }
  ]

  constructor(title, timeInRange, obj?: any | undefined) {
    super(obj)
    if (obj) {
      Object.assign(this, obj)
    }
    this.title.text = title
    this.series[0].name = title
    timeInRange.forEach((item, index) => {
      this.series[0].data[index].value = item
    })
  }
}