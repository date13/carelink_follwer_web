import Base from "@/model/classes/Base";
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";
import {TIME_RANGE_CONFIG} from "@/views/const";

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
      isClear: false
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
