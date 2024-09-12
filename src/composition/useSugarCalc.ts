import {COLORS, CONST_VAR} from "@/views/const";
import dayjs from "dayjs";

export default function () {
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


  const cleanTime = (time: string) => {
    if (!time) return
    return dayjs(time.replaceAll('T', ' ').replaceAll('.000Z', '').replaceAll(".000-00:00", "")).valueOf()
  }
  return {
    calcSgYValueLimit,
    calcTimeInRange,
    calcLastOffset,
    calcSG,
    sgColor,
    cleanTime
  }
}
