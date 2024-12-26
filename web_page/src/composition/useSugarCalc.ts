import {
  COLORS,
  CONST_VAR,
  INSULIN_TYPE,
  NOTIFICATION_MAP,
  PUMP_STATUS,
  SENSOR_STATUS,
  SG_STATUS,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import dayjs from "dayjs";
import {std} from "mathjs";
import {compact, isNumber} from "lodash-es";
import echarts from "@/plugins/echart";

export default function () {

  const getLastSg = (lastSG) => {
    return lastSG.sensorState === SG_STATUS.NO_ERROR_MESSAGE.key && lastSG.sg !== 0 ? calcSG(lastSG.sg) : SG_STATUS[lastSG.sensorState]?.name || '--'
  }

  const calcSgYValueLimit = () => {
    return {
      min: 2,
      max: (value) => {//取最大值向上取整为最大刻度
        return value.max < 10 ? 10 + CONST_VAR.maxSgYValueOffset : Math.ceil(value.max) + CONST_VAR.maxSgYValueOffset
      },
    }
  }

  const sgColor = (sg) => {
    if (sg <= CONST_VAR.minSeriousSg || sg >= CONST_VAR.maxSeriousSg) {
      return COLORS[5]
    } else if ((sg > CONST_VAR.minSeriousSg && sg <= CONST_VAR.minWarnSg) || (sg < CONST_VAR.maxSeriousSg && sg >= CONST_VAR.maxWarnSg)) {
      return COLORS[6]
    } else {
      return COLORS[0]
    }
  }

  const calcTimeInRange = (list, isTight = false) => {
    const validSgs = list.filter(item => validItem(item))
    if (validSgs.length > 0) {
      const lt = ((validSgs.filter(item => item.sg < (isTight ? CONST_VAR.minTightWarnSg : CONST_VAR.minWarnSg) * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
      const gt = ((validSgs.filter(item => item.sg > (isTight ? CONST_VAR.maxTightWarnSg : CONST_VAR.maxWarnSg) * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
      return [(100 - (Number(lt) + Number(gt))).toFixed(1), lt, gt]
    }
    return [0, 0, 0]
  }


  const calcLastOffset = (list): number | string => {
    const listDeal = list.filter(item => validItem(item))
    const len = listDeal.length
    return len > 2 ? (calcSG(listDeal[len - 1].sg - listDeal[len - 2].sg, 2)) : 0
  }

  const calcSG = (sg: number, defaultDecision = 1) => {
    return (isNumber(sg) && Math.abs(sg) !== Infinity) ? Number((sg / CONST_VAR.exchangeUnit)).toFixed(defaultDecision) : 0
  }

  const calcCV = (list, avgSg) => {
    if (list.length === 0) return 0
    const stdNumber = std(list.filter(item => item.kind === 'SG' && validItem(item)).map(item => item.sg))
    return ((stdNumber / avgSg) * 100).toFixed(1)
  }

  const cleanTime = (time: string | number) => {
    if (!time) return
    if (typeof time === 'number') return time
    return dayjs(time.replaceAll('T', ' ').replaceAll('.000Z', '').replaceAll(".000-00:00", "")).valueOf()
  }

  const shouldHaveAR2 = (data) => {
    return data.systemStatusMessage === SYSTEM_STATUS_MAP.NO_ERROR_MESSAGE.key && data.lastSG.sensorState === SENSOR_STATUS.NO_ERROR_MESSAGE.key
  }

  function getStartPercent(startPercent) {
    return TIME_RANGE_CONFIG.find(item => item.value === startPercent) ?? TIME_RANGE_CONFIG[1]
  }

  const loadSgData = (data, forecast, setting) => {
    const sgList = compact(data.sgs.map(item => {
      //获取有效数据
      if (validItem(item)) {
        return [
          cleanTime(item.datetime),
          calcSG(item.sg),
          INSULIN_TYPE.SG
        ]
      }
    }))

    if (!shouldHaveAR2(data)) return sgList
    const lastSg = sgList[sgList.length - 1]
    lastSg.push(true)//标记最后一个symbol
    const forcastArr: any = []
    for (let i = 0; i < getStartPercent(setting.startPercent)?.offset / 5; i++) {
      forcastArr.push([
        dayjs(lastSg[0]).add((i + 1) * 5, 'minutes').valueOf(),
        setting.showAR2 ? calcSG(forecast[i]) : null,
        INSULIN_TYPE.SG_FORECAST
      ])
    }
    return sgList.concat(forcastArr)
  }

  const loadYesterdaySgData = (data, setting) => {
    if (!data) return
    if (setting.showYesterday) {
      const curDatetime = dayjs().add(getStartPercent(setting.startPercent)?.offset, 'minutes').valueOf()
      return data.sgs.filter(item => {
        return validItem(item) && item.datetime <= curDatetime
      }).map(item => {
        return [
          item.datetime,
          calcSG(item.sg),
          INSULIN_TYPE.SG_YESTERDAY
        ]
      })
    }
    return []
  }

  const loadCalibrationData = (list) => {
    return list.map(item => {
      //获取校准数据
      if (item.type === INSULIN_TYPE.CALIBRATION.key) {
        return [
          cleanTime(item.dateTime),
          calcSG(item.value),
          INSULIN_TYPE.CALIBRATION,
          'image://blood.png',
          13
        ]
      }
      if (item.type === INSULIN_TYPE.TIME_CHANGE.key) {
        return [
          cleanTime(item.dateTime),
          11,
          INSULIN_TYPE.TIME_CHANGE,
          'image://time.png',
          20
        ]
      }
    })
  }

  const loadBaselData = (list) => {
    let max = 0
    const result = list.map(item => {
      if (item.type === INSULIN_TYPE.AUTO_BASAL_DELIVERY.key || (item.type === INSULIN_TYPE.INSULIN.key && item.activationType === INSULIN_TYPE.AUTOCORRECTION.key)) {
        const isBasal = item.type === INSULIN_TYPE.AUTO_BASAL_DELIVERY.key
        const val = isBasal ? item.bolusAmount.toFixed(3) : item.deliveredFastAmount.toFixed(3)
        if (val > max) {
          max = Number(val)
        }
        return [
          cleanTime(item.dateTime),
          val,
          isBasal ? INSULIN_TYPE.AUTO_BASAL_DELIVERY : INSULIN_TYPE.AUTOCORRECTION
        ]
      }
    })
    return {
      max,
      list: result
    }
  }

  function showBaselPeak(list, setting) {
    const result: any = []
    if (setting.showPeak) {
      let newList = list.filter(item => {
        return item.type === 'INSULIN' && item.activationType === INSULIN_TYPE.AUTOCORRECTION.key
      })
      newList = newList.splice(newList.length > CONST_VAR.peakPoint ? -CONST_VAR.peakPoint : 0)
      newList.forEach((item, i) => {
        const start = cleanTime(item.dateTime)
        result.push({
              name: '开始',
              xAxis: start,
              type: 'start',
              lineStyle: {
                color: COLORS[2],
                width: 0.5
              },
              last: i === newList.length - 1
            },
            {
              name: '峰值',
              type: 'end',
              xAxis: dayjs(start).add(CONST_VAR.peakMinutes, 'minutes').valueOf(),
              lineStyle: {
                color: COLORS[5],
                width: 0.5
              },
              last: i === newList.length - 1
            })
      })
    }
    return result
  }

  const loadInsulinData = (list, setting, type = INSULIN_TYPE.INSULIN) => {
    if (type === INSULIN_TYPE.INSULIN_YESTERDAY) {
      if (!setting.showYesterday) {
        return []
      } else {
        const curDatetime = dayjs().add(getStartPercent(setting.startPercent)?.offset, 'minutes').valueOf()
        list = list.filter(item => {
          return item.dateTime <= curDatetime
        })
      }
    }
    return list.map(item => {
      if (item.type === 'INSULIN' && (item.activationType === 'RECOMMENDED' || item.activationType === 'MANUAL')) {
        const plan = item.programmedFastAmount.toFixed(2)
        const delivered = item.deliveredFastAmount.toFixed(2)
        const meal = list.find(mark => mark.type === 'MEAL' && item.index === mark.index)
        return [
          cleanTime(item.dateTime),
          plan,
          type,
          delivered,
          meal ? meal.amount : 0
        ]
      }
    })
  }


  function showInsulinPeak(list, setting) {
    const result: any = []
    if (setting.showPeak) {
      let newList = list.filter(item => {
        return item.type === 'INSULIN' && (item.activationType === 'RECOMMENDED' || item.activationType === 'MANUAL')
      })
      newList = newList.splice(newList.length > CONST_VAR.peakPoint ? -CONST_VAR.peakPoint : 0)
      newList.forEach((item, i) => {
        const start = cleanTime(item.dateTime)
        result.push({
              name: '开始',
              xAxis: start,
              type: 'start',
              lineStyle: {
                color: COLORS[2],
                width: 0.5
              },
              data: {
                delivered: item.deliveredFastAmount.toFixed(2),
                plan: item.programmedFastAmount.toFixed(2)
              },
              last: i === newList.length - 1
            },
            {
              name: '峰值',
              type: 'end',
              xAxis: dayjs(start).add(CONST_VAR.peakMinutes, 'minutes').valueOf(),
              lineStyle: {
                color: COLORS[5],
                width: 0.5
              },
              last: i === newList.length - 1
            })
      })
    }
    return result
  }

  function getTimeRemaining(time, units = "minutes") {
    //@ts-ignore
    return dayjs.duration(time, units).humanize(true);
  }

  const getModeObj = (data) => {
    let result = {
      mode: PUMP_STATUS.none,
      basalRate: '',
      isTemp: false,
      timeRemaining: '--'
    }
    if (data.therapyAlgorithmState) {
      const therapyAlgorithmState = data.therapyAlgorithmState
      if (therapyAlgorithmState.autoModeShieldState === 'AUTO_BASAL') {
        result.mode = PUMP_STATUS.auto
        if (!data.pumpBannerState || data.pumpBannerState.length > 0) {
          const pumpState = data.pumpBannerState[0]
          if (pumpState.type === 'TEMP_TARGET') {
            result.mode = PUMP_STATUS.sport
            result.timeRemaining = getTimeRemaining(data.pumpBannerState[0].timeRemaining)
          }
          if (pumpState.type === 'DELIVERY_SUSPEND') {
            result.mode = PUMP_STATUS.stop
          }
        }
      } else if (therapyAlgorithmState.autoModeShieldState === 'SAFE_BASAL') {
        result.mode = PUMP_STATUS.safe
        result.timeRemaining = getTimeRemaining(therapyAlgorithmState.safeBasalDuration)
      } else if (therapyAlgorithmState.autoModeShieldState === 'FEATURE_OFF') {
        result.mode = PUMP_STATUS.manuel
        if (data.basal.tempBasalRate) {
          result.isTemp = true
          result.basalRate = data.basal.tempBasalRate
          result.timeRemaining = getTimeRemaining(data.basal.tempBasalDurationRemaining)
        } else {
          result.basalRate = data.basal.basalRate
        }
      }
    }

    return result
  }

  const getSGMarkArea = (data, setting) => {
    const options: any = [
      [
        {
          yAxis: CONST_VAR.maxSeriousSg,
          itemStyle: {
            color: COLORS[8],
            opacity: 0.3
          }
        },
        {
          yAxis: CONST_VAR.maxWarnSg
        }
      ],
      [
        {
          yAxis: CONST_VAR.maxWarnSg,
          itemStyle: {
            color: COLORS[7],
            opacity: 0.3
          }
        },
        {
          yAxis: CONST_VAR.minWarnSg
        }
      ],
      [{
        yAxis: CONST_VAR.minWarnSg,
        itemStyle: {
          color: COLORS[8],
          opacity: 0.3
        }
      },
        {
          yAxis: CONST_VAR.minSeriousSg
        }
      ],
      [
        {
          xAxis: dayjs(cleanTime(data.lastSG.datetime)).add(2.5, 'minute').valueOf(),
          itemStyle: {
            color: COLORS[2],
            opacity: 0.1
          }
        },
        {
          xAxis: dayjs(cleanTime(data.lastSG.datetime)).add(3, 'hour').valueOf()
        }
      ]
    ]
    return options
  }

  function showNotificationMsg(messageId, sg) {
    const item = NOTIFICATION_MAP[messageId]
    if (sg) return item.text.replace(item.replace, Number(calcSG(sg)) > 30 ? '无法探测' : calcSG(sg))
    return item.text
  }

  function calcSgsLen(sgs, setting) {
    if (setting.realWave) {
      const percent = getStartPercent(setting.startPercent).value
      // 计算需要跳过的元素数量
      const startIndex = Math.floor(sgs.length * (1 - percent / 100));

      // 返回从startIndex开始到数组末尾的所有元素
      return sgs.slice(startIndex);
    }
    return sgs
  }

  function validItem(item) {
    return (item.sensorState === SG_STATUS.NO_ERROR_MESSAGE.key && item.sg !== 0) || item.sensorState === SG_STATUS.SG_BELOW_40_MGDL.key
    // return item.sensorState === 'NO_ERROR_MESSAGE' && item.sg !== 0
  }

  function maxWave(sgs, setting, size = 12) {
    const list = calcSgsLen(sgs, setting)
    if (list.length < size || size <= 0) {
      return 0
    }
    const arr = list.filter(item => validItem(item)).map(item => item.sg)
    let maxChange = 0;

    for (let i = 0; i <= arr.length - size; i++) {
      const subArray = arr.slice(i, i + size);
      const minVal = Math.min(...subArray);
      const maxVal = Math.max(...subArray);
      const change = maxVal - minVal;

      if (change > maxChange) {
        maxChange = change;
      }
    }
    return calcSG(maxChange);
  }

  function minMaxSG(sgs, setting) {
    const list = calcSgsLen(sgs, setting)
    const arr = list.filter(item => validItem(item)).map(item => item.sg)
    return [calcSG(Math.min(...arr)), calcSG(Math.max(...arr))]
  }

  function showInsulin(item) {
    if (item.data) {
      const percent = Number((item.data[3] / item.data[1]).toFixed(1))
      const color = item.data[2].color2
      return percent === 1 ? color : percent === 0 ? 'white' : new echarts.graphic.LinearGradient(0, 0, 0, 1, [
        {
          offset: 1 - percent,
          color: 'white'
        },
        {
          offset: percent,
          color: color
        }
      ])
    }
  }

  function sensorState(data, isHumanize = true) {
    return data.sensorState === SENSOR_STATUS.NO_ERROR_MESSAGE.key && data.sensorDurationMinutes ?
        (isHumanize ? dayjs.duration(data.sensorDurationMinutes, 'minutes').humanize(true) : (data.sensorDurationMinutes / 60).toFixed(1) + '小时') :
        SENSOR_STATUS[data.sensorState] ? SENSOR_STATUS[data.sensorState].name : data.sensorState
  }

  return {
    getLastSg,
    calcSgYValueLimit,
    calcTimeInRange,
    calcLastOffset,
    calcSG,
    calcCV,
    sgColor,
    cleanTime,
    loadSgData,
    loadYesterdaySgData,
    loadCalibrationData,
    loadBaselData,
    loadInsulinData,
    getModeObj,
    getSGMarkArea,
    shouldHaveAR2,
    showNotificationMsg,
    maxWave,
    minMaxSG,
    showInsulin,
    showInsulinPeak,
    showBaselPeak,
    sensorState,
    getStartPercent
  }
}
