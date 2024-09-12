<template>
  <MainPanel no-pad="1">
    <div class="flex flex-col h-full bg-white overflow-x-hidden pa-1">
      <div class="flex flex-row h-50 pt-2">
        <div class="w-1/2 flex items-center justify-center flex-col">
          <div class="flex items-center justify-between">
            <div :style="{color:sugarCalc.sgColor(sugarCalc.calcSG(data.lastSG.sg))}" class="text-7xl font-bold ">{{
                sugarCalc.calcSG(data.lastSG.sg)
              }}
            </div>
            <div class="text-3xl flex font-thin arrow">
              <template v-if="trendObj?.direction">
                <template v-if="trendObj.direction === 1">
                  <ep-Top v-for="i in trendObj.num"></ep-Top>
                </template>
                <template v-if="trendObj.direction === 2">
                  <ep-Top-Right></ep-Top-Right>
                </template>
                <template v-if="trendObj.direction === 3">
                  <ep-Right></ep-Right>
                </template>
                <template v-if="trendObj.direction === 4">
                  <ep-Bottom-Right></ep-Bottom-Right>
                </template>
                <template v-if="trendObj.direction === 5">
                  <ep-Bottom v-for="i in trendObj.num"></ep-Bottom>
                </template>
              </template>
            </div>
          </div>
          <div class="flex text-xs items-center justify-between align-center mb-1">
            <span class="mx-2">{{ lastUpdateTime }}</span>
            <span class="mx-2">{{ lastOffset }}</span>
            <span class="ml-2 text-xs">
              <ep-Refresh class="hand" @click="reload"></ep-Refresh>
            </span>
          </div>
          <div class="flex items-center justify-center">
            <el-radio-group v-model="startPercent" size="small" @change="changeTimeRange">
              <el-radio-button :value="8" label="2"/>
              <el-radio-button :value="13" label="3"/>
              <el-radio-button :value="17" label="4"/>
              <el-radio-button :value="25" label="6"/>
              <el-radio-button :value="50" label="12"/>
            </el-radio-group>
          </div>
          <div class="flex text-xs items-center justify-between align-center my-1">
            <span class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
            <el-tag class="mb-1" type="info" @click="login">登录</el-tag>
          </div>
        </div>
        <div class="w-1/2 flex items-center justify-center ">
          <el-card class="w-max info-panel ma-1 max-w-100">
            <template #header>
              <div class="card-header text-center flex items-center justify-between">
                <span class="text-2xl font-bold">{{ time.format('HH:mm') }}</span>
                <span class="text-xs hand" @click="reloadCarelinkData">状态:{{ status }}</span>
              </div>
            </template>
            <div class="flex justify-between flex-wrap">
              <el-tag class="mb-1 mr-2" type="primary">活性:
                {{ data.activeInsulin.amount }}
              </el-tag>
              <el-tag class="mb-1 mr-2" type="primary">平均:
                {{ sugarCalc.calcSG(data.averageSG) }},
                {{ timeInRange[0] }}%
              </el-tag>
              <el-tag class="mb-1 mr-2" type="primary">
                <span class="text-rose">Low:{{ timeInRange[1] }}%,</span>&nbsp;
                <span class="text-rose">Hight:{{ timeInRange[2] }}%</span>
              </el-tag>
              <el-tag class="mb-1 mr-2" type="warning">泵:
                {{ data.reservoirRemainingUnits }}U,
                {{ data.medicalDeviceBatteryLevelPercent }}%
              </el-tag>
              <el-tag class="mb-1 mr-2" type="warning">探头:
                {{
                  dayjs.duration(data.sensorDurationMinutes, 'minutes').humanize(true)
                }},
                {{
                  data.gstBatteryLevel || '--'
                }}%
              </el-tag>
            </div>
          </el-card>
        </div>
      </div>
      <div class="flex-1">
        <div ref="myChart" class="border-grey border-grey mb-4 h-full"></div>
      </div>
      <div class="h-10"></div>
    </div>
  </MainPanel>
</template>
<script lang="ts" name="mySugar" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'
import relativeTime from 'dayjs/plugin/relativeTime'
import duration from 'dayjs/plugin/duration'
import echarts from "@/plugins/echart"
import useChartResize from "@/composition/useChartResize";
import {DATE_FORMAT} from "@/model/model-type";
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {COLORS, CONST_VAR, DIRECTIONS, INSULIN_TYPE, REFRESH_INTERVAL} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)
const sugarService = new SugarService()

const myChart = ref<HTMLElement>();
let chart: any
let resizeObj: any = null
const lastStatus: any = Tools.getLastStatus('sugar-setting', {
  startPercent: 13
})
const sugarCalc = useSugarCalc()
const setting = lastStatus.value['sugar-setting']
const state: any = reactive({
  tokenData: {},
  status: 200,
  startPercent: setting.startPercent,
  updateDatetime: '--',//数据更新时间
  interval: {
    time: null,
    data: null
  },
  data: {
    lastSG: {
      //最后获取的数据时间
      datetime: dayjs().format(DATE_FORMAT.datetime),
      sg: 0
    },
    sgs: [],
    activeInsulin: {
      amount: 0
    },
    markers: [],
    gstBatteryLevel: 0,
    sensorDurationMinutes: 0,
  },
  time: dayjs()//当前系统时间
})

const {data, time, startPercent, updateDatetime, status} = toRefs(state)

function fromNow(time: any) {
  if (!time) return
  return dayjs().from(time)
}

onMounted(async () => {
  await loadCarelinkData()
  drawLine()
  startInterval()
})
onBeforeUnmount(() => {
  if (resizeObj) {
    resizeObj.beforeDestroy()
  }
})

function login() {
  window.open("https://carelink.minimed.eu/patient/sso/login?country=hk&lang=zh")
}

async function reloadCarelinkData() {
  //后端去 carelink 刷新数据
  const result = await sugarService.refreshCarelinkData()
  if (result) {
    Msg.successMsg('远程数据刷新成功')
    await loadCarelinkData()
    refreshChart()
  }
}

async function reload() {
  clearInterval(state.interval.time)
  clearInterval(state.interval.data)
  try {
    await loadCarelinkData()
    refreshChart()
    startInterval()
    Msg.successMsg('刷新数据成功')
  } catch (e) {
    console.log(e);
  }
}

function startInterval() {
  startTimeInterval()
  startDataLoadInterval()
}

function startTimeInterval() {
  state.interval.time = setInterval(() => {
    state.time = dayjs()
    // state.data.lastSG.updateDatetime = state.data.lastSG.updateDatetime.add(1, 'second')
    // console.log("refresh,", state.time);
  }, 1000)
}

function startDataLoadInterval() {
  state.interval.data = setInterval(async () => {
    await loadCarelinkData(false)
    refreshChart()
    // chart.refresh()
    // chart.setOption(charOption)
  }, REFRESH_INTERVAL.loadData * 60 * 1000)
}

//获取数据库数据,不是去 carelink 刷新数据
async function loadCarelinkData(mask = true) {
  try {
    if (state.tokenData) {
      const result = await sugarService.loadData(CONST_VAR.isDemo, mask)
      if (result) {
        state.data = result.data
        state.status = result.status
        state.updateDatetime = dayjs(data.update_time).format("MM-DD HH:mm")
        state.data.lastSG.datetime = sugarCalc.cleanTime(state.data.lastSG.datetime)
        /*state.data.markers.push({
          "type": "CALIBRATION",
          "index": 148,
          "value": 130,
          "kind": "Marker",
          "version": 1,
          "dateTime": "2024-09-11T14:18:00.000-00:00",
          "relativeOffset": -41836,
          "calibrationSuccess": true
        },)*/
      }
    }
  } catch (e) {
    console.log(e);
  }
}

function changeTimeRange() {
  setting.startPercent = state.startPercent
  refreshChart()
}

function refreshChart() {
  chart.setOption(charOption.value, true);
}

//计算入框率
const timeInRange = computed(() => {
  return sugarCalc.calcTimeInRange(state.data.sgs)
})

//计算最后的数据升降幅度
const lastOffset = computed(() => {
  return sugarCalc.calcLastOffset(state.data.sgs)
})

const lastUpdateTime = computed(() => {
  return fromNow(state.data.lastSG.datetime)
})

//获取升降趋势
const trendObj = computed(() => {
  return state.data?.lastSGTrend && DIRECTIONS[state.data.lastSGTrend]
})

//画图的参数
const charOption = computed(() => {
  return {
    toolbox: {
      show: false,
      feature: {
        mark: {show: true},
        dataView: {show: true, readOnly: false},
        magicType: {show: true, type: ['line', 'bar', 'stack']},
        restore: {show: true},
        saveAsImage: {show: true}
      }
    },
    tooltip: {
      trigger: 'axis',
      confine: true,
      position: (point, params, dom, rect, size) => {
        return [point[0] - 100, point[1] - 110]  //返回x、y（横向、纵向）两个点的位置
      },
      formatter: params => {
        // 获取xAxis data中的数据
        const param = params[0]
        let dataStr = `<div class="text-sm font-bold flex justify-between w-30">
          <span>${param.data[2].name}</span>
          <span>${dayjs(param.data[0]).format("HH:mm")}</span>
        </div>`
        params.forEach(item => {
          const type = item.data[2]
          const isInsulin = type.key === INSULIN_TYPE.INSULIN.key
          dataStr += `
            <div class="flex items-center justify-between my-1">
              <span style="width:10px;height:10px;background-color:${type.key === INSULIN_TYPE.SG.key ? sugarCalc.sgColor(item.data[1]) : type.color};"></span>
              <span class="flex-1 ml-1">${isInsulin ? type.text[0] : ''}</span>
              <span>${item.data[1]}</span>
            </div>`
          if (isInsulin) {
            dataStr += `<div class="flex items-center justify-between mb-1">
              <span style="width:10px;height:10px;background-color:${type.color2};"></span>
              <span class="flex-1 ml-1">${type.text[1]}</span>
              <span>${item.data[3]}</span>
            </div>`
            dataStr += `<div class="flex items-center justify-between mb-1">
              <span style="width:10px;height:10px;background-color:${type.color3};"></span>
              <span class="flex-1 ml-1">${type.text[2]}</span>
              <span>${item.data[4]}</span>
            </div>`
          }
        })
        return dataStr
      }
    },
    grid: {
      left: '1%',
      top: '60',
      right: '0',
      containLabel: true
    },
    xAxis: {
      type: 'time',
      splitLine: {show: true},
      axisLabel: {
        // interval: 5 * 60,
        formatter: function (value, index) {
          return dayjs(value).format('HH:mm');
        }
      }
    },
    yAxis: [
      {
        name: 'mmol',
        type: 'value',
        ...sugarCalc.calcSgYValueLimit(),
        connectNulls: false,
        splitLine: {show: true}
      },
      {
        name: '校准',
        type: 'value',
        ...sugarCalc.calcSgYValueLimit(),
        show: false
      },
      {
        name: '基础率',
        nameLocation: 'start',
        type: 'value',
        inverse: true,
        axisTick: {
          alignWithLabel: true
        },
        min: 0,
        max: 2,
      },
      {
        name: '大剂量',
        type: 'value',
        max: 20,
        show: false
      },
    ],
    dataZoom: [
      {
        type: 'slider',
        start: 100 - state.startPercent,
        end: 100,
        labelFormatter: (value) => {
          return `${dayjs(value).format('MM-DD')}\n${dayjs(value).format('HH:mm')}`;
        },
      }
    ],
    visualMap: {
      show: false,
      dimension: 1,
      type: 'piecewise',
      precision: 1,
      seriesIndex: 0,
      pieces: [
        {
          lte: CONST_VAR.minSeriousSg,
          color: COLORS[5]
        },
        {
          lte: CONST_VAR.minWarnSg,
          gt: CONST_VAR.minSeriousSg,
          color: COLORS[6]
        },
        {
          gt: CONST_VAR.minWarnSg,
          lt: CONST_VAR.maxWarnSg,
          color: COLORS[0]
        },
        {
          gte: CONST_VAR.maxWarnSg,
          lt: CONST_VAR.maxSeriousSg,
          color: COLORS[6]
        },
        {
          gt: CONST_VAR.maxSeriousSg,
          color: COLORS[5]
        }
      ]
    },
    series: [
      {
        name: '血糖',
        data: state.data.sgs.map(item => {
          //获取有效数据
          if (item.sensorState === 'NO_ERROR_MESSAGE') {
            return [
              sugarCalc.cleanTime(item.datetime),
              sugarCalc.calcSG(item.sg),
              INSULIN_TYPE.SG
            ]
          }
        }),
        type: 'line',
        smooth: true,
        yAxisIndex: 0,
        symbol: 'circle',
        symbolSize: 6,
        label: {
          show: false,
          position: 'bottom'
        },
        markLine: {
          symbol: ['none', 'none'],
          label: {
            show: true,
            position: 'end',
          },
          emphasis: {
            lineStyle: {
              width: 1,	// hover时的折线宽度
            }
          },
          data: [
            {
              name: '高值',
              yAxis: CONST_VAR.maxWarnSg
            },
            {
              name: '低值',
              yAxis: CONST_VAR.minWarnSg
            }
          ],
          lineStyle: {
            color: COLORS[5]
          }
        }
      },

      {
        name: '校准',
        data: state.data.markers.map(item => {
          //获取校准数据
          if (item.type === 'CALIBRATION') {
            return [
              sugarCalc.cleanTime(item.dateTime),
              sugarCalc.calcSG(item.value),
              INSULIN_TYPE.CALIBRATION
            ]
          }//排序
        }),
        type: 'line',
        yAxisIndex: 1,
        symbol: 'circle',
        symbolSize: 8,
        lineStyle: 'none',
        label: {
          show: false,
          position: 'bottom'
        },
        itemStyle: {
          color: INSULIN_TYPE.CALIBRATION.color
        }
      },
      {
        name: '基础率',
        type: "bar",
        step: 'middle',
        yAxisIndex: 2,
        connectNulls: true,
        areaStyle: {},
        itemStyle: {
          color: item => {
            if (item.data) {
              return item.data[2].color
            }
          }
        },
        lineStyle: {
          width: 1
        },
        emphasis: {
          focus: 'series'
        },
        data: state.data.markers.map(item => {
          if (item.type === 'AUTO_BASAL_DELIVERY' || (item.type === 'INSULIN' && item.activationType === 'AUTOCORRECTION')) {
            const isBasal = item.type === 'AUTO_BASAL_DELIVERY'
            return [
              sugarCalc.cleanTime(item.dateTime),
              isBasal ? item.bolusAmount.toFixed(3) : item.deliveredFastAmount.toFixed(3),
              isBasal ? INSULIN_TYPE.AUTO_BASAL_DELIVERY : INSULIN_TYPE.AUTOCORRECTION
            ]
          }
        })
      },
      {
        name: '大剂量',
        type: "scatter",
        yAxisIndex: 3,
        symbolSize: 15,
        itemStyle: {
          borderColor: COLORS[2],
          color: (item) => {
            if (item.data) {
              let percent = Number((item.data[3] / item.data[1]).toFixed(1))
              return percent === 1 ? COLORS[2] : percent === 0 ? 'white' : new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                {
                  offset: percent,
                  color: 'white'
                },
                {
                  offset: 1 - percent,
                  color: COLORS[2]
                }
              ])
            }
          }
        },
        data: state.data.markers.map(item => {
          if (item.type === 'INSULIN' && item.activationType === 'RECOMMENDED') {
            const plan = item.programmedFastAmount.toFixed(2)
            const delivered = item.deliveredFastAmount.toFixed(2)
            const meal = state.data.markers.find(mark => mark.type === 'MEAL' && item.index === mark.index)
            return [
              sugarCalc.cleanTime(item.dateTime),
              plan,
              INSULIN_TYPE.INSULIN,
              delivered,
              meal ? meal.amount : 0
            ]
          }
        })
      },
    ]
  }
})

function drawLine() {
  // 基于准备好的dom，初始化echarts实例
  chart = echarts.getInstanceByDom(<HTMLElement>myChart.value);
  if (!chart) { // 如果不存在，就进行初始化。
    chart = echarts.init(<HTMLElement>myChart.value)
  }
  // 绘制图表
  chart.setOption(charOption.value, true);
  resizeObj = useChartResize(chart)
  resizeObj.mounted()
}
</script>
<style lang="scss" scoped>
.info-panel {
  :deep(.el-card__header) {
    padding: 8px;
  }

  :deep(.el-card__body) {
    padding: 8px;
  }
}

.arrow {
  svg {
    width: 25px;
  }
}
</style>
