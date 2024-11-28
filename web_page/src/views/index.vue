<template>
  <MainPanel no-pad="1">
    <div class="flex flex-col h-full bg-white overflow-x-hidden pa-1">
      <el-dropdown class="menu-panel" placement="bottom-start" trigger="click" @command="handleMenu">
        <ep-Menu></ep-Menu>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="big">Big</el-dropdown-item>
            <el-dropdown-item command="info">Info</el-dropdown-item>
            <el-dropdown-item command="dict">Dict</el-dropdown-item>
            <el-dropdown-item command="food">Food</el-dropdown-item>
            <el-dropdown-item command="login">Login</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <div class="flex flex-row h-45">
        <div class="w-1/2 flex items-center justify-center">
          <el-card class="w-max info-panel ma-1 max-w-110">
            <template #header>
              <div class="card-header text-center flex items-center justify-between">
                <span :class="{'text-red':status!==200}" class="text-xs hand"
                      @click="refreshCarelinkToken">状态:{{ status }}</span>
                <span class="text-2xl font-bold hand" @click="reloadCarelinkData">{{
                    time.format('HH:mm')
                  }}</span>
              </div>
            </template>
            <div class="flex flex-wrap">
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                IOB:
                {{ data.activeInsulin.amount }}&nbsp;
                CV:
                {{ sugarCalc.calcCV(data.sgs, data.averageSG) }}%
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                Avg:
                {{ sugarCalc.calcSG(data.averageSG) }}&nbsp;
                GMI:
                {{ GMI }}
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                <span class="text-red">Wav:
                  {{ sugarCalc.maxWave(data.sgs, setting) }}</span>&nbsp;
                <span class="text-red">Min:
                  {{ minMaxSG[0] }}</span>&nbsp;
                <span class="text-red">Max:
                  {{ minMaxSG[1] }}</span>
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                TIR:
                {{ timeInRange[0] }}%
                <span class="text-rose mx-1">L:
                  {{ timeInRange[1] }}%
                </span>
                <span class="text-rose">H:
                  {{ timeInRange[2] }}%
                </span>
              </el-tag>
              <el-tag class="" size="small" type="primary">
                TTIR:
                {{ tightTimeInRange[0] }}%
                <span class="text-rose mx-1">L:
                  {{ tightTimeInRange[1] }}%
                </span>
                <span class="text-rose">H:
                  {{ tightTimeInRange[2] }}%
                </span>
              </el-tag>
              <el-tag v-if="data.notificationHistory.activeNotifications.length>0" class="mb-1 mr-1" size="small"
                      type="danger">
                <div v-for="{messageId,sg} in data.notificationHistory.activeNotifications">
                  {{ NOTIFICATION_MAP[messageId] ? sugarCalc.showNotificationMsg(messageId, sg) : messageId }}
                </div>
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
          <div class="flex text-xs items-center justify-between align-center">
            <span :class="{'text-red':lastUpdateTime.sgDiff>=15}" class="mx-2">
              {{
                lastUpdateTime.sg
              }}
            </span>
            <span class="mx-2">{{ lastOffset }}</span>
            <!--            <span class="ml-2 text-xs">
                          <ep-Refresh class="hand" @click="reload"></ep-Refresh>
                        </span>-->
          </div>
          <div class="flex items-center justify-center time-range mt-1">
            <el-radio-group v-model="setting.startPercent" size="small" @change="refreshChart">
              <el-radio-button v-for="item in TIME_RANGE_CONFIG" :label="item.label" :value="item.value"/>
            </el-radio-group>
          </div>
          <div class="flex text-xs items-center justify-between align-center my-1">
            <span v-if="data.systemStatusMessage===SYSTEM_STATUS_MAP.WARM_UP.key" class="mx-2">预计启动:&nbsp;{{
                Tools.toNow(nextStartTime)
              }}</span>
            <span v-else class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
            <div :style="{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}" class="mr-2">
              {{ SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name }}
            </div>
          </div>
          <div class="flex items-center justify-around align-center info-panel">
            <el-tag class="mb-1 mr-1" size="small" type="primary">
              泵:
              {{ data.reservoirRemainingUnits }}U
              {{ data.medicalDeviceBatteryLevelPercent }}%&nbsp;
              探头:
              {{
                sugarCalc.sensorState(data)
              }}
              {{
                data.gstBatteryLevel || '--'
              }}%
            </el-tag>
          </div>
        </div>
      </div>
      <div class="flex-1 chart-panel flex">
        <div ref="myChart" class="flex-1 border-grey border-grey h-full"></div>
        <div class="w-1/28"></div>
      </div>
      <div class="h-10 px-2 flex items-center justify-around">
        <el-tag :type="modeObj.mode.type" class="" size="small">
          {{ modeObj.mode.name }}
          <span v-if="modeObj.mode.key===PUMP_STATUS.safe.key">
            ,闭环退出:{{ modeObj.timeRemaining }}
            </span>
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
        <el-tag size="large" type="primary">
          <div class="flex flex-col ">
              <span>剂量(昨):
              {{ lastUpdateTime.sumInsulin || '--' }}U
              </span>
            <span>基础(昨):
              {{ lastUpdateTime.sumBaseDelivery || '--' }}U
              </span>
          </div>
        </el-tag>
      </div>
    </div>
    <div class="float-panel flex flex-col items-center justify-center">
      <div class="item flex items-center justify-center border-solid border-1 hand no-bottom"
           @click="handleMenu('notification')">
        <el-badge :is-dot="setting.notification.hasNew">
          <ep-Bell></ep-Bell>
        </el-badge>
      </div>
      <div class="item flex items-center justify-center border-solid border-1 hand no-bottom"
           @click="showDrawer">
        <ep-KnifeFork></ep-KnifeFork>
      </div>
      <div :class="{'only-right':showSetting}"
           class="item flex items-center justify-center border-solid border-1 hand no-bottom">
        <div v-show="showSetting"
             class="flex float-item-panel items-center justify-between border-solid border-1 no-right">
          <div class="float-item ">
            <el-checkbox v-model="setting.realWave" label="实时" size="small"/>
          </div>
          <div class="float-item">
            <el-checkbox v-model="setting.showAR2" label="AR2" size="small" @change="switchAR2"/>
          </div>
          <div class="float-item">
            <el-checkbox v-model="setting.showYesterday" label="昨日" size="small" @change="refreshChart"/>
          </div>
          <div class="float-item">
            <el-checkbox v-model="setting.showPeak" label="峰值" size="small" @change="refreshChart"/>
          </div>
        </div>
        <ep-Setting class="hand" @click="triggerSetting"></ep-Setting>
      </div>
      <div :class="{'no-top':showSetting}" class="item flex items-center justify-center border-solid border-1 hand"
           @click="reload">
        <ep-Refresh></ep-Refresh>
      </div>
    </div>
    <el-drawer
        v-model="drawer"
        direction="rtl"
        size="80%"
    >
      <Info :close-drawer="closeDrawer" :is-dialog="true" title="碳水计算"></Info>
    </el-drawer>
    <NotificationDialog v-if="showNotificationDialog" v-model:show="showNotificationDialog"
                        :notificationHistory="data.notificationHistory"></NotificationDialog>
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
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {
  CHART_LEGEND,
  COLORS,
  CONST_VAR,
  INSULIN_TYPE,
  NOTIFICATION_MAP,
  PUMP_STATUS,
  SG_STATUS,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import NotificationDialog from "@/views/components/notificationDialog.vue";
import Info from "@/views/info.vue"
import useSugarCommon from "@/composition/useSugarCommon";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)

const sugarService = new SugarService()

const myChart = ref<HTMLElement>();
let chart: any
let resizeObj: any = null
const sugarCalc = useSugarCalc()
const sugarCommon = useSugarCommon({
  refreshChart,
  dealSelfData: (result) => {
    state.forecast = result.forecast || {ar2: []}
  },
  alarmNotification: () => {
  }
})
const state: any = reactive({
  drawer: false,
  forecast: {},
  showSetting: false
})
const {
  reload,
  refreshCarelinkToken,
  reloadCarelinkData,
  startTimeInterval,
  dealCarelinkData,
  updateConduitTime,
  handleMenu,
  setting,
  timeInRange,
  tightTimeInRange,
  lastOffset,
  lastUpdateTime,
  modeObj,
  trendObj,
  minMaxSG
} = sugarCommon

const {
  drawer,
  showSetting
} = toRefs(state)

const {
  prepare,
  data,
  updateDatetime,
  status,
  showNotificationDialog,
  nextStartTime,
  GMI,
  time
}: any = toRefs(sugarCommon.state)

onBeforeMount(() => {
  initSetting()
})

onMounted(async () => {
  // await onLoadCarelinkData()
  startTimeInterval()
  await sugarService.initSugarSSE((res) => {
    dealCarelinkData(res)
    refreshChart()
  })
})

onBeforeUnmount(() => {
  if (resizeObj) {
    resizeObj.beforeDestroy()
  }
})

function initSetting() {
  if (!setting.notification) {
    setting.notification = {
      hasNew: false,
      lastKey: null,
      lastAlarm: {
        key: '',
        isClear: false
      }
    }
  }
  if (!setting.legend) {
    const legendOptions = {}
    CHART_LEGEND.forEach(item => {
      legendOptions[item.name] = true
    })
    setting.legend = {
      selected: legendOptions
    }
  }
}

function triggerSetting() {
  state.showSetting = !state.showSetting
}


function showDrawer() {
  state.drawer = true
}

function closeDrawer() {
  state.drawer = false
}

function switchAR2() {
  if (!sugarCalc.shouldHaveAR2(sugarCommon.state.data) && setting.showAR2) {
    setting.showAR2 = false
    Msg.warnMsg(`当前系统状态为:${SYSTEM_STATUS_MAP[sugarCommon.state.data.systemStatusMessage]?.name},最近血糖为:${SG_STATUS[sugarCommon.state.data.lastSG.sensorState]?.name}`)
  } else {
    refreshChart()
  }
}

function refreshChart() {
  if (!prepare) return
  if (!chart) drawLine()
  chart.setOption(charOption.value, true);
}

const chartTimeOption: any = computed(() => {
  const {interval} = sugarCalc.getStartPercent(setting.startPercent)
  return {
    interval
  }
})

//画图的参数
const charOption = computed(() => {
  return {
    legend: {
      icon: 'rect',
      itemGap: 5,
      itemWidth: 20,
      data: CHART_LEGEND,
      selected: setting.legend.selected,
    },
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
    axisPointer: {
      show: true,
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      },
      confine: true,
      position: (point, params, dom, rect, size) => {
        // const isInsulin = params[0].data[2].key === INSULIN_TYPE.INSULIN.key
        return [point[0] - 100, point[1] - 150]  //返回x、y（横向、纵向）两个点的位置
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
          const isInsulin = (type.key === INSULIN_TYPE.INSULIN.key || type.key === INSULIN_TYPE.INSULIN_YESTERDAY.key)
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
      right: 0,
      containLabel: true
    },
    xAxis: {
      type: 'time',
      splitLine: {show: false},
      boundaryGap: false,
      interval: chartTimeOption.value.interval,
      axisLabel: {
        formatter: function (value, index) {
          return dayjs(value).format('HH:mm');
        },
      }
    },
    yAxis: [
      {
        // name: 'mmol/L',
        type: 'value',
        ...sugarCalc.calcSgYValueLimit(),
        splitLine: {show: false},
        axisTick: {show: true},
        axisLine: {show: false},
      },
      {
        name: '校准',
        type: 'value',
        ...sugarCalc.calcSgYValueLimit(),
        show: false
      },
      {
        name: '基础',
        nameLocation: 'start',
        show: false,
        type: 'value',
        inverse: true,
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
        xAxisIndex: [0],
        bottom: 25,
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
      hoverLink: false,
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
        data: sugarCalc.loadSgData(sugarCommon.state.data, state.forecast.ar2, setting),
        smooth: true,
        connectNulls: false,
        yAxisIndex: 0,
        symbol: (value: any, params: Object) => {
          return value[2].symbol
        },
        symbolSize: (rawValue, params) => {
          return params.value.length === 3 ? 6 : 10
        },
        label: {
          show: false,
          position: 'bottom'
        },
        labelLine: {
          smooth: true,
        },
        emphasis: {
          disabled: true
        },
        markArea: {
          emphasis: {disabled: true},
          data: sugarCalc.getSGMarkArea(sugarCommon.state.data, setting)
        },
        markLine: {
          symbol: ['none', 'none'],
          animation: false,
          label: {
            color: "inherit",
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
              name: '高值TTIR警告',
              yAxis: CONST_VAR.maxTightWarnSg,
              lineStyle: {
                color: COLORS[1],
              }
            },
            {
              name: '高值警告',
              yAxis: CONST_VAR.maxWarnSg,
              lineStyle: {
                color: COLORS[6],
              }
            },
            {
              name: '高值严重警告',
              yAxis: CONST_VAR.maxSeriousSg,
              lineStyle: {
                color: COLORS[5],
              }
            },
            {
              name: '低值警告',
              yAxis: CONST_VAR.minWarnSg,
              lineStyle: {
                color: COLORS[6],
              }
            },
            {
              name: '低值严重警告',
              yAxis: CONST_VAR.minSeriousSg,
              lineStyle: {
                color: COLORS[5],
              }
            }
          ],
        }
      },
      {
        name: '校准',
        data: sugarCalc.loadCalibrationData(sugarCommon.state.data.markers),
        type: 'scatter',
        yAxisIndex: 1,
        symbol: 'circle',
        symbolSize: 10,
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
        name: '基础',
        type: "bar",
        yAxisIndex: 2,
        connectNull: true,
        barMaxWidth: 30,
        markArea: {
          silent: true,
          itemStyle: {
            opacity: 0.3
          },
        },
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
        data: sugarCalc.loadBaselData(sugarCommon.state.data.markers)
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
          color: sugarCalc.showInsulin
        },
        // markArea: {
        //   data: sugarCalc.showInsulinPeak(data.markers, setting),
        // },
        markLine: {
          symbol: ['none', 'none'],
          animation: false,
          label: {
            show: true,
            color: 'inherit',
            formatter: (item) => {
              return item.data.last ? `${dayjs(item.value).format("HH:mm")}` : ''
            },
          },
          emphasis: {
            label: {
              show: true,
              position: 'end',
              formatter: (item) => {
                return `${item.name}:${dayjs(item.value).format("HH:mm")}`
              },
            },
            lineStyle: {
              width: 1,	// hover时的折线宽度
            }
          },
          data: sugarCalc.showInsulinPeak(sugarCommon.state.data.markers, setting),
        },
        data: sugarCalc.loadInsulinData(sugarCommon.state.data.markers, setting)
      },
      {
        name: '血糖(昨)',
        type: 'line',
        data: sugarCalc.loadYesterdaySgData(sugarCommon.state.myData.yesterday, setting),
        smooth: true,
        connectNulls: false,
        symbol: 'none',
        yAxisIndex: 0,
        label: {
          show: false,
        },
        lineStyle: {
          opacity: 0.4
        },
        labelLine: {
          smooth: true,
        }
      },
      {
        name: '大剂量(昨)',
        type: "scatter",
        yAxisIndex: 3,
        symbolSize: 10,
        label: {
          color: COLORS[4],
          formatter: (item) => {
            return item.data[3]
          },
          fontSize: 10,
          position: 'top',
          show: true,
        },
        emphasis: {
          scale: true
        },
        itemStyle: {
          borderColor: COLORS[6],
          color: sugarCalc.showInsulin
        },
        data: sugarCalc.loadInsulinData(sugarCommon.state.myData.yesterday.markers, setting, INSULIN_TYPE.INSULIN_YESTERDAY)
      },
    ]
  }
})

function drawLine() {
  if (!chart) { // 如果不存在，就进行初始化。
    chart = echarts.init(<HTMLElement>myChart.value)
  }
  chart.on('legendselectchanged', function (params) {
    Object.assign(setting.legend.selected, params.selected)
  });
  resizeObj = useChartResize(chart)
  resizeObj.mounted()
}
</script>
<style lang="scss" scoped>
.menu-panel {
  position: absolute;
  right: 5px;

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

  :deep(.el-tag--small) {
    padding: 0 3px;
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

.float-panel {
  right: 5px;
  position: absolute;
  bottom: 30px;
  background: white;

  .float-item-panel {
    background: white;
    position: absolute;
    right: 35px;

    .float-item {
      height: 35px;
      padding: 0 5px;
      display: flex;
      align-items: center;
    }
  }

  .item {
    width: 35px;
    height: 35px;
  }

  .no-right {
    border-right: none;
  }

  .no-top {
    border-top: none;
  }

  .only-right {
    border-left: none;
    border-top: none;
    border-bottom: none;
  }

  .no-bottom {
    border-bottom: none;
  }
}
</style>
