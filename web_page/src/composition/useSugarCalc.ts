import {
  COLORS,
  CONST_VAR,
  INSULIN_TYPE,
  NOTIFICATION_MAP,
  PUMP_STATUS,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import dayjs from "dayjs";
import {std} from "mathjs";
import {compact} from "lodash-es";

export default function () {

  const getLastSg = (lastSG) => {
    return lastSG.sensorState === "NO_ERROR_MESSAGE" ? calcSG(lastSG.sg) : "--"
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
    const validSgs = list.filter(item => item.sensorState === 'NO_ERROR_MESSAGE' && item.sg !== 0)
    const lt = ((validSgs.filter(item => item.sg < (isTight ? CONST_VAR.minTightWarnSg : CONST_VAR.minWarnSg) * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
    const gt = ((validSgs.filter(item => item.sg > (isTight ? CONST_VAR.maxTightWarnSg : CONST_VAR.maxWarnSg) * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
    return [100 - (Number(lt) + Number(gt)), lt, gt]
  }


  const calcLastOffset = (list): number | string => {
    const listDeal = list.filter(item => item.sensorState === 'NO_ERROR_MESSAGE')
    const len = listDeal.length
    return len > 2 ? (calcSG(listDeal[len - 1].sg - listDeal[len - 2].sg, 2)) : 0
  }

  const calcSG = (sg: number, defaultDecision = 1) => {
    return (sg / CONST_VAR.exchangeUnit).toFixed(defaultDecision)
  }

  const calcCV = (list, avgSg) => {
    if (list.length === 0) return 0
    const stdNumber = std(list.filter(item => item.kind === 'SG' && item.sensorState === 'NO_ERROR_MESSAGE' && item.sg !== 0).map(item => item.sg))
    return ((stdNumber / avgSg) * 100).toFixed(1)
  }

  const cleanTime = (time: string) => {
    if (!time) return
    return dayjs(time.replaceAll('T', ' ').replaceAll('.000Z', '').replaceAll(".000-00:00", "")).valueOf()
  }

  const shouldHaveAR2 = (data) => {
    return data.systemStatusMessage === SYSTEM_STATUS_MAP.NO_ERROR_MESSAGE.key
  }
  const loadSgData = (data, forecast, setting) => {
    const start = TIME_RANGE_CONFIG.find(item => item.value === setting.startPercent) ?? TIME_RANGE_CONFIG[1]
    const sgList = compact(data.sgs.map(item => {
      //获取有效数据
      if (item.sensorState === 'NO_ERROR_MESSAGE') {
        return [
          cleanTime(item.datetime),
          calcSG(item.sg),
          INSULIN_TYPE.SG
        ]
      }
    }))

    if (!shouldHaveAR2(data)) return sgList
    // console.log(sgList);
    const lastSg = sgList[sgList.length - 1]
    // console.log(dayjs(lastSg[0]).add(( 1) * 5, 'minutes'));
    const forcastArr: any = []

    for (let i = 0; i < start?.offset / 5; i++) {
      forcastArr.push([
        dayjs(lastSg[0]).add((i + 1) * 5, 'minutes').valueOf(),
        setting.showAR2 ? calcSG(forecast[i]) : null,
        INSULIN_TYPE.SG_FORECAST
      ])
    }
    // console.log(sgList.concat(forcastArr));
    return sgList.concat(forcastArr)
  }

  const loadYesterdaySgData = (data, setting) => {
    if (!data) return
    return setting.showYesterday ? compact(data.sgs.map(item => {
      //获取有效数据
      if (item.sensorState === 'NO_ERROR_MESSAGE') {
        return [
          item.datetime,
          calcSG(item.sg),
          INSULIN_TYPE.SG_YESTERDAY
        ]
      }
    })) : []
  }

  const loadCalibrationData = (list) => {
    return list.map(item => {
      //获取校准数据
      if (item.type === 'CALIBRATION') {
        return [
          cleanTime(item.dateTime),
          calcSG(item.value),
          INSULIN_TYPE.CALIBRATION
        ]
      }//排序
    })
  }

  const loadBaselData = (list) => {
    return list.map(item => {
      if (item.type === 'AUTO_BASAL_DELIVERY' || (item.type === 'INSULIN' && item.activationType === 'AUTOCORRECTION')) {
        const isBasal = item.type === 'AUTO_BASAL_DELIVERY'
        return [
          cleanTime(item.dateTime),
          isBasal ? item.bolusAmount.toFixed(3) : item.deliveredFastAmount.toFixed(3),
          isBasal ? INSULIN_TYPE.AUTO_BASAL_DELIVERY : INSULIN_TYPE.AUTOCORRECTION
        ]
      }
    })
  }

  const loadInsulinData = (list) => {
    return list.map(item => {
      if (item.type === 'INSULIN' && item.activationType === 'RECOMMENDED') {
        const plan = item.programmedFastAmount.toFixed(2)
        const delivered = item.deliveredFastAmount.toFixed(2)
        const meal = list.find(mark => mark.type === 'MEAL' && item.index === mark.index)
        return [
          cleanTime(item.dateTime),
          plan,
          INSULIN_TYPE.INSULIN,
          delivered,
          meal ? meal.amount : 0
        ]
      }
    })
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
      ]
    ]
    if (shouldHaveAR2(data) && setting.showAR2) {
      options.push([
        {
          xAxis: dayjs(data.lastSG.datetime).add(2.5, 'minute').valueOf(),
          itemStyle: {
            color: COLORS[2],
            opacity: 0.1
          }
        },
        {
          xAxis: dayjs(data.lastSG.datetime).add(3, 'hour').valueOf()
        }
      ])
    }
    return options
  }

  function showNotificationMsg(messageId, sg) {
    const item = NOTIFICATION_MAP[messageId]
    if (sg) return item.text.replace(item.replace, calcSG(sg))
    return item.text
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
    showNotificationMsg
  }
}
