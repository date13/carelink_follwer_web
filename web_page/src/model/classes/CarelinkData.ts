import Base from "@/model/classes/Base";
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";

export default class CarelinkData extends Base {
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
