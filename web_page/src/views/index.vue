<template>
  <MainPanel no-pad="1">
    <div class="flex flex-col h-full bg-white overflow-x-hidden pa-1">
      <el-badge :is-dot="setting.notification.hasNew" class="menu-panel">
        <el-dropdown placement="bottom-start" trigger="click" @command="handleMenu">
          <ep-Menu></ep-Menu>
          <template #dropdown>
            <el-dropdown-menu>
              <el-badge :is-dot="setting.notification.hasNew" :offset="[-13,8]">
                <el-dropdown-item command="notification">Notification</el-dropdown-item>
              </el-badge>
              <el-dropdown-item command="info">Info</el-dropdown-item>
              <el-dropdown-item command="dict">Dict</el-dropdown-item>
              <el-dropdown-item command="login">Login</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-badge>
      <div class="flex flex-row h-50">
        <div class="w-1/2 flex items-center justify-center">
          <el-card class="w-max info-panel ma-1 max-w-110">
            <template #header>
              <div class="card-header text-center flex items-center justify-between">
                <span :class="{'text-red':status!==200}" class="text-xs hand"
                      @click="refreshCarelinkToken">状态:{{ status }}</span>
                <span class="text-2xl font-bold hand" @click="reloadCarelinkData">{{ time.format('HH:mm') }}</span>
              </div>
            </template>
            <div class="flex flex-wrap">
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                IOB:
                {{ data.activeInsulin.amount }}&nbsp;&nbsp;
                CV:
                {{ sugarCalc.calcCV(data.sgs, data.averageSG) }}%
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                Avg:
                {{ sugarCalc.calcSG(data.averageSG) }}&nbsp;&nbsp;
                Wav:
                {{ sugarCalc.maxChange(data.sgs) }}
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                TIR:&nbsp;&nbsp;
                {{ timeInRange[0] }}%&nbsp;
                <span class="text-rose mx-1">L:&nbsp;{{ timeInRange[1] }}%</span>&nbsp;
                <span class="text-rose">H:&nbsp;{{ timeInRange[2] }}%</span>
              </el-tag>
              <el-tag class="mb-1 mr-1 " size="small" type="primary">
                TTIR:
                {{ tightTimeInRange[0] }}%&nbsp;
                <span class="text-rose mx-1">L:&nbsp;{{ tightTimeInRange[1] }}%</span>&nbsp;
                <span class="text-rose">H:&nbsp;{{ tightTimeInRange[2] }}%</span>
              </el-tag>
              <el-tag class="mb-1 mr-1" size="small" type="warning">泵:
                {{ data.reservoirRemainingUnits }}U&nbsp;
                {{ data.medicalDeviceBatteryLevelPercent }}%&nbsp;
              </el-tag>
              <el-tag class="mb-1 mr-1" size="small" type="warning">
                探头:
                {{
                  data.sensorDurationMinutes ? dayjs.duration(data.sensorDurationMinutes, 'minutes').humanize(true) : '待更换'
                }}&nbsp;
                {{
                  data.gstBatteryLevel || '--'
                }}%
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
          <div class="flex text-xs items-center justify-between align-center mb-1">
            <span :class="{'text-red':lastUpdateTime.sgDiff>=15}" class="mx-2">
              {{
                lastUpdateTime.sg
              }}
            </span>
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

          <div class="flex text-xs items-center justify-between align-center mt-1">
            <span v-if="data.systemStatusMessage===SYSTEM_STATUS_MAP.WARM_UP.key" class="mx-2">预计启动:&nbsp;{{
                toNow(setting.nextStartupTime)
              }}</span>
            <span v-else class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
            <div :style="{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}" class="mr-2">
              {{ SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name }}
            </div>
          </div>
          <div class="flex text-xs items-center justify-between align-center">
            <div class="mr-2">
              <el-checkbox v-model="setting.showAR2" label="AR2" size="small" @change="switchAR2"/>
            </div>
            <div>
              <el-checkbox v-model="setting.showYesterday" label="昨日" size="small" @change="switchYesterday"/>
            </div>
          </div>
        </div>
      </div>
      <div class="flex-1">
        <div ref="myChart" class="border-grey border-grey h-full"></div>
      </div>
      <div class="h-15 px-2 flex items-center justify-around">
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
      </div>
    </div>
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
import {DATE_FORMAT} from "@/model/model-type";
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {
  CARELINK_DICT_KEY,
  COLORS,
  CONST_VAR,
  DIRECTIONS,
  INSULIN_TYPE,
  NOTIFICATION_HASH_KEY,
  NOTIFICATION_MAP,
  PUMP_STATUS,
  REFRESH_INTERVAL,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import defaultSettings from "@/settings";
import {DictService} from "@/service/dict-service";
import CryptoJS from "crypto-js";
import {forEach} from "lodash-es";
import NotificationDialog from "@/views/components/notificationDialog.vue";

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
  showAR2: true,
  showYesterday: true,
  notification: {
    hasNew: false,
    lastKey: null
  },
  nextStartupTime: null
})
const sugarCalc = useSugarCalc()
const setting = lastStatus.value['sugar-setting']

const state: any = reactive({
  tokenData: {},
  status: 200,
  showNotificationDialog: false,
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
    notificationHistory: {
      activeNotifications: [],
      clearedNotifications: []
    },
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

const {data, time, updateDatetime, status, showNotificationDialog} = toRefs(state)

onBeforeMount(() => {
  initSetting()
})

onMounted(async () => {
  await onLoadCarelinkData()
  startInterval()
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
      lastKey: null
    }
  }
}

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

async function onLoadCarelinkData(isMask = true) {
  await loadCarelinkData(isMask)
  // await loadCarelinkMyData(isMask)
  refreshChart()
}

async function reload() {
  forEach(state.interval, (v, k) => {
    clearInterval(v)
  })
  try {
    await onLoadCarelinkData()
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
    await onLoadCarelinkData(false)
    // chart.refresh()
    // chart.setOption(charOption)
  }, REFRESH_INTERVAL.loadData * 60 * 1000)
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
        // state.data.lastSG.datetime = sugarCalc.cleanTime(state.data.lastSG.datetime)
        dealNewNotification()
        // state.data.systemStatusMessage = SYSTEM_STATUS_MAP.WARM_UP.key
        dealWarnUpStatus()
        dealMyData(result.myData)
        /* state.data.therapyAlgorithmState = {
           "autoModeShieldState": "SAFE_BASAL",
           "autoModeReadinessState": "NO_ACTION_REQUIRED",
           "plgmLgsState": "FEATURE_OFF",
           "safeBasalDuration": 213,
           "waitToCalibrateDuration": 0
         }*/
        document.title = `${defaultSettings.title} ${sugarCalc.calcSG(state.data.lastSG.sg)}, ${lastOffset.value > 0 ? '+' + lastOffset.value : lastOffset.value}`
        /* state.data.notificationHistory.activeNotifications = [
           {
             "dateTime": "2024-09-27T19:26:58.000-00:00",
             "GUID": "6F1B0000-3003-0000-823E-A72C00000000",
             "type": "ALERT",
             "faultId": 816,
             "instanceId": 7023,
             "messageId": "BC_SID_HIGH_SG_CHECK_BG",
             "sg": 171,
             "pumpDeliverySuspendState": false,
             "relativeOffset": -307,
             "alertSilenced": false
           }
         ]*/
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

function dealMyData(myData) {
  state.myData = myData
  if (state.myData.yesterdaySG) {
    state.myData.yesterdaySG.sgs.forEach(item => {
      item.datetime = dayjs(sugarCalc.cleanTime(item.datetime)).add(1, 'day').valueOf()
    })
    state.myData.yesterdaySG.sgs = state.myData.yesterdaySG.sgs.filter(item => {
      return item.sensorState === 'NO_ERROR_MESSAGE' && item.datetime >= dayjs().add(-1, 'day').valueOf()
    })
  }
}

function dealWarnUpStatus() {
  const {systemStatusMessage} = state.data
  if (systemStatusMessage === SYSTEM_STATUS_MAP.WARM_UP.key && !setting.nextStartupTime) {
    setting.nextStartupTime = dayjs().add(2, 'hour')
  } else if (systemStatusMessage !== SYSTEM_STATUS_MAP.WARM_UP.key && setting.nextStartupTime) {
    setting.nextStartupTime = null
  }
}

function dealNewNotification() {
  const {notification} = setting;
  const len = state.data.notificationHistory.clearedNotifications.length
  if (state.data.notificationHistory.activeNotifications.length > 0) {
    notification.hasNew = true;
  }
  const notificationKey = CryptoJS.HmacSHA1(JSON.stringify(
      state.data.notificationHistory.clearedNotifications.map(item => {
        return {
          referenceGUID: item.referenceGUID,
          triggeredDateTime: item.triggeredDateTime
        }
      }).sort((a: any, b: any) => {
        return sugarCalc.cleanTime(b.triggeredDateTime) - sugarCalc.cleanTime(a.triggeredDateTime)
      }).slice(0, len > 4 ? 4 : len)
  ), NOTIFICATION_HASH_KEY).toString()
  // console.log(notificationKey);
  if (notification && !notification.hasNew && notificationKey !== notification.lastKey) {
    notification.hasNew = true;
  }
  setting.notification.lastKey = notificationKey
  // console.log(setting.notification);
}

function handleMenu(command) {
  if (command === 'login') {
    window.open("https://carelink.minimed.eu/patient/sso/login?country=hk&lang=zh")
  } else if (command === 'notification') {
    state.showNotificationDialog = true
    setting.notification.hasNew = false
  } else {
    location.href = `/${command}`
  }
}

function switchAR2() {
  if (!sugarCalc.shouldHaveAR2(state.data) && setting.showAR2) {
    setting.showAR2 = false
    Msg.warnMsg(`当前系统状态为:${SYSTEM_STATUS_MAP[state.data.systemStatusMessage]?.name},无法预测`)
  } else {
    refreshChart()
  }
}

function switchYesterday() {
  refreshChart()
}

function changeTimeRange() {
  // setting.startPercent = state.setting.startPercent
  refreshChart()
}

function refreshChart() {
  if (!chart) drawLine()
  chart.setOption(charOption.value, true);
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
  return {
    sg: toNow(lastSgUpdateTime),
    sgDiff: dayjs().diff(lastSgUpdateTime, 'minute'),
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
    legend: {
      icon: 'rect',
      data: [
        {name: '血糖', itemStyle: {color: COLORS[0]}},
        {name: '昨日血糖', itemStyle: {color: COLORS[9]}},
        {name: '基础率', itemStyle: {color: COLORS[1]}},
        {name: '大剂量', itemStyle: {color: COLORS[2]}}
      ]
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
        data: sugarCalc.loadSgData(state.data, state.forecast.ar2, setting),
        smooth: true,
        connectNulls: false,
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
          data: sugarCalc.getSGMarkArea(state.data, setting)
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
        name: '基础率',
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
              return percent === 1 ? COLORS[2] : percent === 0 ? 'white' : new echarts.graphic.LinearGradient(0, 1, 0, 0, [
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
      {
        name: '昨日血糖',
        type: 'line',
        data: sugarCalc.loadYesterdaySgData(state.myData.yesterdaySG, setting),
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
    ]
  }
})

function drawLine() {
  // 基于准备好的dom，初始化echarts实例
  chart = echarts.getInstanceByDom(<HTMLElement>myChart.value);
  if (!chart) { // 如果不存在，就进行初始化。
    chart = echarts.init(<HTMLElement>myChart.value)
  }
  resizeObj = useChartResize(chart)
  resizeObj.mounted()
}
</script>
<style lang="scss" scoped>
.menu-panel {
  position: absolute;
  right: 20px;

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
</style>
