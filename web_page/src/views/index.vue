<template>
  <MainPanel no-pad="1">
    <div class="flex flex-col h-full bg-white overflow-x-hidden pa-1">
      <el-dropdown class="menu-panel" placement="bottom-start" trigger="click" @command="handleMenu">
        <ep-Menu></ep-Menu>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="info">Info</el-dropdown-item>
            <el-dropdown-item command="dict">Dict</el-dropdown-item>
            <el-dropdown-item command="login">Login</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <div class="flex flex-row h-50">
        <div class="w-1/2 flex items-center justify-center">
          <el-card class="w-max info-panel ma-1 max-w-110">
            <template #header>
              <div class="card-header text-center flex items-center justify-between">
                <span class="text-xs hand" @click="refreshCarelinkToken">状态:{{ status }}</span>
                <span class="text-2xl font-bold hand" @click="reloadCarelinkData">{{ time.format('HH:mm') }}</span>
              </div>
            </template>
            <div class="flex justify-between flex-wrap">
              <el-tag class="mb-1 mr-1" size="small" type="primary">活性:
                {{ data.activeInsulin.amount }}
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                平均:
                {{ sugarCalc.calcSG(data.averageSG) }},
                {{ timeInRange[0] }}%,
                CV:
                {{ sugarCalc.calcCV(data.sgs, data.averageSG) }}%
              </el-tag>
              <el-tag class="mb-1 mr-1" size="small" type="primary">
                <span class="text-rose">Low:{{ timeInRange[1] }}%,</span>&nbsp;
                <span class="text-rose">Hight:{{ timeInRange[2] }}%</span>
              </el-tag>
              <el-tag class="mb-1 mr-1" size="small" type="warning">泵:
                {{ data.reservoirRemainingUnits }}U,
                {{ data.medicalDeviceBatteryLevelPercent }}%
              </el-tag>
              <el-tag class="mb-1 mr-1" size="small" type="warning">探头:
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
        <div class="w-1/2 flex items-center justify-center flex-col">
          <div class="flex items-center justify-between">
            <div :style="{color:sugarCalc.sgColor(sugarCalc.getLastSg(data.lastSG))}" class="text-7xl font-bold ">{{
                sugarCalc.getLastSg(data.lastSG)
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
            <span class="mx-2">{{ lastUpdateTime.sg }}</span>
            <span class="mx-2">{{ lastOffset }}</span>
            <span class="ml-2 text-xs">
              <ep-Refresh class="hand" @click="reload"></ep-Refresh>
            </span>
          </div>
          <div class="flex items-center justify-center time-range">
            <el-radio-group v-model="setting.startPercent" size="small" @change="changeTimeRange">
              <el-radio-button v-for="item in TIME_RANGE_CONFIG" :label="item.label" :value="item.value"/>
            </el-radio-group>
            <!--
                        <el-segmented v-model="startPercent" :options="TIME_RANGE_CONFIG" size="default" @change="changeTimeRange">
                          <template #default="{ item }">
                            <div>{{ item.label }}</div>
                          </template>
                        </el-segmented>-->
          </div>

          <div class="flex text-xs items-center justify-between align-center my-1">
            <span class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
            <div :style="{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}" class="mr-2">
              {{ SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name }}
            </div>
            <div>
              <el-checkbox v-model="setting.showAR2" label="AR2" size="small" @change="refreshChart"/>
            </div>
          </div>
        </div>
      </div>
      <div class="flex-1">
        <div ref="myChart" class="border-grey border-grey mb-4 h-full"></div>
      </div>
      <div class="h-20 px-2 flex items-center justify-around">
        <el-tag :type="modeObj.mode.type" class="" size="small">
          {{ modeObj.mode.name }}
          <span v-if="modeObj.mode.key===PUMP_STATUS.sport.key">
            剩余:{{ modeObj.timeRemaining }}
            </span>
          <span v-if="modeObj.mode.key===PUMP_STATUS.manuel.key">
              ,基础:{{ modeObj.basalRate }}
              <span v-if="modeObj.isTemp">
                ,剩余:{{ modeObj.timeRemaining }}
                </span>
            </span>
        </el-tag>
        <el-tag class="hand" size="small" type="warning" @click="updateConduitTime">管路:
          {{ lastUpdateTime.conduit || '--' }}
        </el-tag>
      </div>
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
import {
  CARELINK_DICT_KEY,
  COLORS,
  CONST_VAR,
  DIRECTIONS,
  INSULIN_TYPE,
  PUMP_STATUS,
  REFRESH_INTERVAL,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import defaultSettings from "@/settings";
import {DictService} from "@/service/dict-service";
import {forEach} from "lodash-es";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)
const sugarService = new SugarService()
const dictService = new DictService()

const myChart = ref<HTMLElement>();
let chart: any
let resizeObj: any = null
const lastStatus: any = Tools.getLastStatus('sugar-setting', {
  startPercent: TIME_RANGE_CONFIG[1].value,
  showAR2: true
})
const sugarCalc = useSugarCalc()
const setting = lastStatus.value['sugar-setting']

const state: any = reactive({
  tokenData: {},
  status: 200,
  updateDatetime: '--',//数据更新时间
  interval: {
    time: null,
    data: null,
    myData: null,
  },
  myData: {},
  forecast: {},
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
    basal: {},
    therapyAlgorithmState: {},
    markers: [],
    gstBatteryLevel: 0,
    sensorDurationMinutes: 0,
  },
  time: dayjs()//当前系统时间
})

const {data, time, updateDatetime, status} = toRefs(state)

onMounted(async () => {
  await loadCarelinkData()
  drawLine()
  await loadCarelinkMyData()
  startInterval()
})

onBeforeUnmount(() => {
  if (resizeObj) {
    resizeObj.beforeDestroy()
  }
})

function fromNow(time: any) {
  if (!time) return
  return dayjs().from(time)
}

function toNow(time: any) {
  if (!time) return
  return dayjs().to(time)
}

function updateConduitTime() {
  updateMyData(
      "是否确认更新管路更换时间?",
      () => {
        state.myData.lastConduitTime = dayjs().format(DATE_FORMAT.datetime)
      },
      () => Msg.successMsg('更新管路更换时间成功'))
}

function updateMyData(confirmStr: string, sureFunc = () => {
}, afterFunc = () => {
}) {
  Msg.confirm(confirmStr, async () => {
    sureFunc()
    const result = await dictService.updateDict({
      key: CARELINK_DICT_KEY.carelinkMyData,
      val: JSON.stringify(state.myData)
    })
    if (result) {
      afterFunc()
    }
  })
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
  forEach(state.interval, (v, k) => {
    clearInterval(v)
  })
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

async function loadCarelinkMyData() {
  const result: any = await dictService.getDict(CARELINK_DICT_KEY.carelinkMyData)
  if (result) {
    state.myData = JSON.parse(result);
  }
}

//获取数据库数据,不是去 carelink 刷新数据
async function loadCarelinkData(mask = true) {
  try {
    if (state.tokenData) {
      const result = await sugarService.loadData(mask)
      if (result) {
        state.data = result.data
        state.status = result.status
        state.forecast = result.forecast || {ar2: []}
        state.updateDatetime = dayjs(state.data.update_time).format("MM-DD HH:mm")
        state.data.lastSG.datetime = sugarCalc.cleanTime(state.data.lastSG.datetime)
        document.title = `${defaultSettings.title} ${sugarCalc.calcSG(state.data.lastSG.sg)}, ${lastOffset.value > 0 ? '+' + lastOffset.value : lastOffset.value}`
        // state.data.therapyAlgorithmState = null
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
        /* Object.assign(state.data, {
           "therapyAlgorithmState": {
             "autoModeShieldState": "FEATURE_OFF",
             "autoModeReadinessState": "NO_ACTION_REQUIRED",
             "plgmLgsState": "MONITORING",
             "safeBasalDuration": 0,
             "waitToCalibrateDuration": 0
           },
           "pumpBannerState": [
             {
               "type": "TEMP_BASAL",
               "timeRemaining": 35
             }
           ],
           "basal": {
             "basalRate": 0.175,
             "tempBasalRate": 1,
             "tempBasalType": "ABSOLUTE",
             "tempBasalDurationRemaining": 35
           }
         })*/
      }
    }
  } catch (e) {
    console.log(e);
  }
}

function handleMenu(command) {
  if (command === 'login') {
    window.open("https://carelink.minimed.eu/patient/sso/login?country=hk&lang=zh")
  } else {
    location.href = `/${command}`
  }
}

function changeTimeRange() {
  // setting.startPercent = state.setting.startPercent
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
  return {
    sg: fromNow(state.data.lastSG.datetime),
    conduit: toNow(state.myData.lastConduitTime)
  }
})

const modeObj = computed(() => {
  return sugarCalc.getModeObj(state.data)
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
        const isInsulin = params[0].data[2].key === INSULIN_TYPE.INSULIN.key
        return [point[0] - 100, point[1] - 110 * (isInsulin ? 1.5 : 1)]  //返回x、y（横向、纵向）两个点的位置
      },
      formatter: params => {
        // 获取xAxis data中的数据
        const param = params[0]
        let dataStr = `<div class="text-sm font-bold flex justify-between w-30">
          <span>${param.data[2].name}</span>
          <span>${dayjs(param.data[0]).format("HH:mm")}</span>
        </div>`
        params.forEach((item, i) => {
          const type = item.data[2]
          const isInsulin = type.key === INSULIN_TYPE.INSULIN.key
          dataStr += `
            <div class="flex items-center justify-between my-1">
              <span style="width:10px;height:10px;background-color:${type.key === INSULIN_TYPE.SG.key ? sugarCalc.sgColor(item.data[1]) : type.color};"></span>
              <span class="flex-1 ml-1 text-sm">${isInsulin ? type.text[0] : (params.length > 1 ? type.name : '')}</span>
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
              <span class="flex-1 ml-1 text-sm">${type.text[2]}</span>
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
      boundaryGap: false,
      axisLabel: {
        formatter: function (value, index) {
          return dayjs(value).format('HH:mm');
        }
      }
    },
    yAxis: [
      {
        name: 'mmol/L',
        type: 'value',
        ...sugarCalc.calcSgYValueLimit(),
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
        id: 'sliderX',
        start: 100 - setting.startPercent,
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
        type: 'line',
        data: sugarCalc.loadSgData(state.data.sgs, state.forecast.ar2, setting),
        smooth: true,
        yAxisIndex: 0,
        symbol: (value: any, params: Object) => {
          return value[2].symbol
        },
        symbolSize: 6,
        label: {
          show: false,
          position: 'bottom'
        },
        labelLine: {
          smooth: true,
        },
        markArea: {
          emphasis: {disabled: true},
          data: sugarCalc.getSGMarkArea(state.data.lastSG, setting)
        },
        markLine: {
          symbol: ['none', 'none'],
          animation: false,
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
              name: '高值警告',
              yAxis: CONST_VAR.maxWarnSg,
              lineStyle: {
                color: COLORS[0],
              }
            },
            {
              name: '高值严重警告',
              yAxis: CONST_VAR.maxSeriousSg,
              lineStyle: {
                color: COLORS[6],
              }
            },
            {
              name: '低值警告',
              yAxis: CONST_VAR.minWarnSg,
              lineStyle: {
                color: COLORS[0],
              }
            },
            {
              name: '低值严重警告',
              yAxis: CONST_VAR.minSeriousSg,
              lineStyle: {
                color: COLORS[6],
              }
            }
          ],

        }
      },
      {
        name: '校准',
        data: sugarCalc.loadCalibrationData(state.data.markers),
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
        yAxisIndex: 2,
        step: 'middle',
        connectNull: false,
        markArea: {
          silent: true,
          itemStyle: {
            opacity: 0.3
          },
        },
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
        data: sugarCalc.loadBaselData(state.data.markers)
      },
      {
        name: '大剂量',
        type: "scatter",
        yAxisIndex: 3,
        symbolSize: 15,
        label: {
          formatter: (item) => {
            return item.data[3]
          },
          position: 'top',
          show: true,
        },
        emphasis: {
          scale: true
        },
        itemStyle: {
          borderColor: COLORS[2],
          color: (item) => {
            if (item.data) {
              let percent = Number((item.data[3] / item.data[1]).toFixed(1))
              return percent === 1 ? COLORS[2] : percent === 0 ? 'white' : new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                {
                  offset: 1 - percent,
                  color: 'white'
                },
                {
                  offset: percent,
                  color: COLORS[2]
                }
              ])
            }
          }
        },
        data: sugarCalc.loadInsulinData(state.data.markers)
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
.menu-panel {
  position: absolute;
  right: 0;

  svg {
    width: 30px;
    height: 30px;
  }
}

.info-panel {
  :deep(.el-card__header) {
    padding: 4px;
  }

  :deep(.el-card__body) {
    padding: 4px;
  }
}

.arrow {
  svg {
    width: 25px;
  }
}

.time-range {
  .el-segmented {
    --el-segmented-item-selected-color: white;
    --el-segmented-item-selected-bg-color: var(--el-color-primary-light-3);
    --el-border-radius-base: 16px;
  }
}
</style>
