import {COLORS, CONST_VAR, INSULIN_TYPE, PUMP_STATUS} from "@/views/const";
import dayjs from "dayjs";
import {std} from "mathjs";

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

  const calcTimeInRange = (list) => {
    const validSgs = list.filter(item => item.sensorState === 'NO_ERROR_MESSAGE')
    const lt = ((validSgs.filter(item => item.sg <= CONST_VAR.minWarnSg * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
    const gt = ((validSgs.filter(item => item.sg >= CONST_VAR.maxWarnSg * CONST_VAR.exchangeUnit).length / validSgs.length) * 100).toFixed(1)
    return [100 - (Number(lt) + Number(gt)), lt, gt]
  }

  const calcLastOffset = (list) => {
    const len = list.length
    return len > 2 ? (calcSG(list[len - 1].sg - list[len - 2].sg, 2)) : 0
  }

  const calcSG = (sg: number, defaultDecision = 1) => {
    return (sg / CONST_VAR.exchangeUnit).toFixed(defaultDecision)
  }

  const calcCV = (list, avgSg) => {
    if (list.length === 0) return 0
    const stdNumber = std(list.filter(item => item.sg !== 0).map(item => item.sg))
    return ((stdNumber / avgSg) * 100).toFixed(1)
  }

  const cleanTime = (time: string) => {
    if (!time) return
    return dayjs(time.replaceAll('T', ' ').replaceAll('.000Z', '').replaceAll(".000-00:00", "")).valueOf()
  }

  const loadSgData = (list) => {
    return list.map(item => {
      //获取有效数据
      if (item.sensorState === 'NO_ERROR_MESSAGE') {
        return [
          cleanTime(item.datetime),
          calcSG(item.sg),
          INSULIN_TYPE.SG
        ]
      }
    })
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

  const getModeObj = (data) => {
    let result = {
      mode: {},
      basalRate: '',
      isTemp: false,
      timeRemaining: '--'
    }
    if (data.therapyAlgorithmState.autoModeShieldState === 'AUTO_BASAL') {
      result.mode = PUMP_STATUS.auto
      if (!data.pumpBannerState || data.pumpBannerState.length > 0) {
        result.mode = PUMP_STATUS.sport
        result.timeRemaining = dayjs.duration(data.pumpBannerState[0].timeRemaining, 'minutes').humanize(true)
      }
    } else if (data.therapyAlgorithmState.autoModeShieldState === 'SAFE_BASAL') {
      result.mode = PUMP_STATUS.safe
    } else if (data.therapyAlgorithmState.autoModeShieldState === 'FEATURE_OFF') {
      result.mode = PUMP_STATUS.manuel
      if (data.basal.tempBasalRate) {
        result.isTemp = true
        result.basalRate = data.basal.tempBasalRate
        result.timeRemaining = dayjs.duration(data.basal.tempBasalDurationRemaining, 'minutes').humanize(true)
      } else {
        result.basalRate = data.basal.basalRate
      }
    }
    return result
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
    loadCalibrationData,
    loadBaselData,
    loadInsulinData,
    getModeObj
  }
}
