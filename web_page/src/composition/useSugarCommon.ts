import {Msg, Tools} from "@/utils/tools";
import {SugarService} from "@/service/sugar-service";
import dayjs from "dayjs";
import Carelink, {SugarSetting} from "@/model/classes/Carelink";
import {cloneDeep, flatten, forEach} from "lodash-es";
import defaultSettings from "@/settings";
import {CARELINK_DICT_KEY, DIRECTIONS, INSULIN_TYPE, SG_STATUS} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import {DATE_FORMAT} from "@/model/model-type";
import {DictService} from "@/service/dict-service";
import router from "@/router";
import {API_URL} from "@/utils/http-client";

export default function (funcObj: any = {}) {
  const sugarCalc = useSugarCalc()
  const sugarService = new SugarService()
  const dictService = new DictService()
  const lastStatus: any = Tools.getLastStatus('sugar-setting', new SugarSetting())
  const setting = lastStatus.value['sugar-setting']

  const state: any = reactive({
    status: 200,
    prepare: false,
    showNotificationDialog: false,
    showBasalDialog: false,
    updateDatetime: '--',//数据更新时间
    interval: {
      time: null,
    },
    orgMyData: {},
    GMI: 0,
    nextStartTime: -1,
    data: new Carelink(),
    myData: {},
    time: dayjs(),//当前系统时间
  })

  function startTimeInterval() {
    state.interval.time = setInterval(() => {
      state.time = dayjs()
      // state.data.lastSG.updateDatetime = state.data.lastSG.updateDatetime.add(1, 'second')
      // console.log("refresh,", state.time);
    }, 1000)
  }

  function clearStartTimeInterval() {
    clearInterval(state.interval.time)
  }

  async function onLoadCarelinkData(isMask = true) {
    await loadCarelinkData(isMask)
    // await loadCarelinkMyData(isMask)
    funcObj.refreshChart()
  }


  //获取数据库数据,不是去 carelink 刷新数据
  async function loadCarelinkData(mask = true) {
    try {
      const result = await sugarService.loadData()
      dealCarelinkData(result)
    } catch (e) {
      console.log(e);
    }
  }

  let i = 0

  function dealCarelinkData(result) {
    if (result) {
      state.data = result.data
      state.status = result.status
      state.GMI = result.GMI
      state.nextStartTime = result.nextStartTime
      // state.data.systemStatusMessage = SYSTEM_STATUS_MAP.WARM_UP.key
      // state.nextStartTime = '2025-02-05 13:35:30'
      state.updateDatetime = dayjs(state.data.update_time).format("MM-DD HH:mm")
      funcObj.dealSelfData(result)
      // state.data.lastSG.datetime = sugarCalc.cleanTime(state.data.lastSG.datetime)
      // setting.notification.hasNew = true
      dealNewNotification()
      dealMyData(result.myData)
      // console.log(result);
      state.prepare = true
      document.title = `${defaultSettings.title} ${sugarCalc.calcSG(state.data.lastSG.sg)}, ${Number(lastOffset.value) > 0 ? '+' + lastOffset.value : lastOffset.value}`
      i++
    } else {
      state.prepare = false
    }
  }


  function dealMyData(myData) {
    // console.log(myData);
    state.orgMyData = cloneDeep(myData)
    state.myData = myData
    if (state.myData.yesterday) {
      flattenYesterdayData('sgs', 'datetime', () => {
        state.myData.yesterday.sgs = state.myData.yesterday.sgs.filter(item => {
          return item.sensorState === SG_STATUS.NO_ERROR_MESSAGE.key && item.datetime >= dayjs().add(-1, 'day').valueOf()
        })
      })
      flattenYesterdayData('markers', 'dateTime', () => {
        state.myData.yesterday.markers = state.myData.yesterday.markers.filter(item => {
          return (item.type === 'INSULIN' || item.type === 'MEAL') && item.dateTime >= dayjs().add(-1, 'day').valueOf()
        })
      })
    }
  }

  function flattenYesterdayData(key, timeKey = 'datetime', suffixFunc = () => {
  }) {
    state.myData.yesterday[key] = flatten(state.myData.yesterday[key])
    state.myData.yesterday[key].forEach(item => {
      item[timeKey] = dayjs(sugarCalc.cleanTime(item[timeKey])).add(1, 'day').valueOf()
    })
    suffixFunc()
  }

  function dealNewNotification() {
    const {notification} = setting;
    let notificationKey = '';
    let notificationData = null;
    if (state.data.notificationHistory.activeNotifications.length > 0) {
      notificationKey = state.data.notificationHistory.activeNotifications[0].instanceId
      notificationData = state.data.notificationHistory.activeNotifications[0]
    } else if (state.data.notificationHistory.clearedNotifications.length > 0) {
      const clearedNotifications = state.data.notificationHistory.clearedNotifications.sort((a: any, b: any) => {
        return sugarCalc.cleanTime(b.triggeredDateTime)! - sugarCalc.cleanTime(a.triggeredDateTime)!
      })
      notificationKey = clearedNotifications[0].instanceId
      notificationData = clearedNotifications[0]
    }

    if (notification && notificationKey !== notification.lastKey) {
      notification.hasNew = true;
      setting.notification.lastKey = notificationKey
    }
    funcObj.alarmNotification(notificationData, notification)
    // if (state.data.notificationHistory.activeNotifications.length > 0) {
    //   notification.hasNew = true;
    //   setting.notification.lastKey = notificationKey
    // }
    // state.data.notificationHistory.clearedNotifications.push({
    //   "referenceGUID": new Date().getTime(),
    //   "dateTime": "2024-12-29T19:50:15.000-00:00",
    //   "type": "ALERT",
    //   "faultId": 805,
    //   "instanceId": 8149,
    //   "messageId": "BC_SID_LOW_SD_CHECK_BG",
    //   "pumpDeliverySuspendState": false,
    //   "relativeOffset": -83697,
    //   "alertSilenced": false,
    //   "triggeredDateTime": "2024-12-29T19:50:06.000-00:00"
    // })
    // const clearedNotifications = state.data.notificationHistory.clearedNotifications.sort((a: any, b: any) => {
    //   return sugarCalc.cleanTime(b.triggeredDateTime)! - sugarCalc.cleanTime(a.triggeredDateTime)!
    // })
    // const notificationKey = CryptoJS.HmacSHA1(JSON.stringify(
    //     clearedNotifications.map(item => {
    //       return {
    //         referenceGUID: item.referenceGUID,
    //         triggeredDateTime: item.triggeredDateTime
    //       }
    //     }).slice(0, len > 4 ? 4 : len)
    // ), NOTIFICATION_HASH_KEY).toString()
    // setting.notification.lastKey = notificationKey
  }

  async function reload() {
    forEach(state.interval, (v, k) => {
      clearInterval(v)
    })
    try {
      await onLoadCarelinkData()
      startTimeInterval()
      Msg.successMsg('刷新数据成功')
    } catch (e) {
      console.log(e);
    }
  }

  async function refreshCarelinkToken() {
    //后端去 carelink 刷新token
    const result = await sugarService.refreshCarelinkToken()
    if (result) {
      Msg.successMsg('远程Token刷新成功')
      // await loadCarelinkData()
      // refreshChart()
    }
  }

  async function restartSSE() {
    //后端去 carelink 刷新token
    sugarService.SSERestart()
    Msg.successMsg('SSE重启成功')
  }

  function reloadPage() {
    router.go(0)
  }

  async function reloadCarelinkData() {
    //后端去 carelink 刷新数据
    const result = await sugarService.refreshCarelinkData()
    if (result) {
      Msg.successMsg('远程数据刷新成功')
      await onLoadCarelinkData()
      Msg.closeMsg()
    }
  }

  function updateConduitTime(params: any) {
    const datetime = params ? dayjs(params.datetime).format(DATE_FORMAT.datetime) : ''
    Msg.confirm(`是否确认更新管路更换时间为:${datetime ? datetime : '当前时间'}`, async () => {
      state.orgMyData.lastConduitTime = datetime ? datetime : dayjs().format(DATE_FORMAT.datetime)
      const result = await dictService.updateDict({
        key: CARELINK_DICT_KEY.carelinkMyData,
        val: JSON.stringify(state.orgMyData)
      }, {user: true})
      if (result) {
        Msg.successMsg('更新管路更换时间成功')
      }
    })
  }

  //计算入框率
  const timeInRange = computed(() => {
    return sugarCalc.calcTimeInRange(state.data.sgs)
  })

  //计算入框率
  const tightTimeInRange = computed(() => {
    return sugarCalc.calcTimeInRange(state.data.sgs, true)
  })
  //计算最后的数据升降幅度
  const lastOffset = computed(() => {
    return sugarCalc.calcLastOffset(state.data.sgs)
  })

  const lastUpdateTime = computed(() => {
    const lastSgUpdateTime = sugarCalc.cleanTime(state.data.lastSG.datetime)
    let sumInsulin = 0
    let sumBaseDelivery = 0
    if (state.orgMyData.yesterday?.markers) {
      const len = state.orgMyData.yesterday?.markers.length
      state.orgMyData.yesterday?.markers[len === 2 ? 1 : 0].forEach(item => {
        if (item.type === 'INSULIN') {
          sumInsulin += item.deliveredFastAmount
        }
        if (item.type === INSULIN_TYPE.AUTO_BASAL_DELIVERY.key) {
          sumBaseDelivery += item.bolusAmount
        }
      })
    }

    return {
      sg: Tools.toNow(lastSgUpdateTime),
      sgDiff: dayjs().diff(lastSgUpdateTime, 'minute'),
      conduit: Tools.toNow(state.orgMyData.lastConduitTime),
      conduitDatetime: state.orgMyData.lastConduitTime,
      sumInsulin: sumInsulin.toFixed(2),
      sumBaseDelivery: sumBaseDelivery.toFixed(2)
    }
  })

  const modeObj = computed(() => {
    return sugarCalc.getModeObj(state.data)
  })
  //获取升降趋势
  const trendObj = computed(() => {
    return state.data?.lastSGTrend && DIRECTIONS[state.data.lastSGTrend]
  })

  async function handleMenu(command) {
    if (command === 'login') {
      window.open("https://carelink.minimed.eu/patient/sso/login?country=hk&lang=zh")
    } else if (command === 'loginDexcom') {
      window.open(`${API_URL}public/dexcomLogin/${Tools.getUser().name}`)
    } else if (command === 'refreshDexToken') {
      const result = await sugarService.refreshDexcomToken()
      console.log(result);
    } else if (command === 'refreshDexData') {
      const result = await sugarService.refreshDexcomData()
      console.log(result);
    } else if (command === 'notification') {
      state.showNotificationDialog = true
      setting.notification.hasNew = false
    } else if (command === 'basal') {
      state.showBasalDialog = true
    } else {
      router.push(`/${command}`)
    }
  }

  //计算最大和最小值
  const minMaxSG = computed(() => {
    return sugarCalc.minMaxSG(state.data.sgs, setting)
  })

  async function loadSettings() {
    const result = await new DictService().getDict('setting', true, {user: true})
    if (result) {
      return result
    }
  }

  return {
    reload,
    reloadPage,
    startTimeInterval,
    clearStartTimeInterval,
    refreshCarelinkToken,
    restartSSE,
    reloadCarelinkData,
    dealCarelinkData,
    updateConduitTime,
    handleMenu,
    loadSettings,
    state,
    setting,
    timeInRange,
    tightTimeInRange,
    lastOffset,
    lastUpdateTime,
    modeObj,
    trendObj,
    minMaxSG,
  }
}
