<template>
  <MainPanel no-pad="1">
    <div class="flex h-full bg-black overflow-x-hidden text-white p-4">
      <Menus :is-home="false" @handler="handleMenu"></Menus>
      <div class="flex flex-col w-3/5  big-panel mr-4">
        <div class="h-10 flex w-full justify-between items-center px-4">
          <div :class="{'text-red':status!==200}" class="text-base hand"
               @click="refreshCarelinkToken">状态:{{ status }}
          </div>
          <!--          <div v-if="playAlarmObj.playing">
                      <ep-AlarmClock class="h-10 w-10 hand" @click="stopPlayer"></ep-AlarmClock>
                    </div>-->
          <div class="text-4xl font-bold hand" @click="reloadCarelinkData">{{ time.format('HH:mm') }}</div>
        </div>
        <div class="flex flex-col w-full h-full flex-1">
          <div class="flex w-full items-center justify-center flex-1">
            <div :style="{color:sugarCalc.sgColor(sugarCalc.getLastSg(data.lastSG))}"
                 class="sg font-bold ">{{
                sugarCalc.getLastSg(data.lastSG)
              }}
            </div>
            <Trend :is-home="false" :trend-obj="trendObj"></Trend>
          </div>
          <div class="h-20 flex items-center justify-center text-base">
            <div ref="myChart" class="h-full w-full"></div>
          </div>
          <div class="h-20 flex items-center justify-center text-base">
            <span :class="{'text-red':lastUpdateTime.sgDiff>=15}" class="mx-2" @click="testPlay()">
              {{
                lastUpdateTime.sg
              }}
            </span>
            <span class="mx-2">{{ lastOffset }}</span>
            <div class="flex items-center justify-between align-center">
              <span v-if="data.systemStatusMessage===SYSTEM_STATUS_MAP.WARM_UP.key" class="mx-2">
                 <el-popover
                     :content="`启动:${nextStartTime}`"
                     placement="bottom"
                     trigger="click"
                 >
                  <template #reference>
                    预计启动:&nbsp;{{
                      nextStartTimeToNow
                    }}
                  </template>
                </el-popover>
              </span>
              <span v-else class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
              <div :style="{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}" class="mr-2">
                {{ SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name }}
              </div>
            </div>
          </div>
        </div>
        <div class="flex items-center justify-around h-10">
          <div class="flex flex-col justify-center items-center">
            <div class="text-base">IOB</div>
            <div class="text-sm">{{ data.activeInsulin.amount }}</div>
          </div>
          <div class="flex flex-col justify-center items-center">
            <div class="text-base">AVG</div>
            <div class="text-sm">{{ sugarCalc.calcSG(data.averageSG) }}</div>
          </div>
          <div class="flex flex-col justify-center items-center">
            <div class="text-base">CV</div>
            <div class="text-sm">{{ sugarCalc.calcCV(data.sgs, data.averageSG) }}%</div>
          </div>
          <div class="flex flex-col justify-center items-center">
            <div class="text-base">GMI</div>
            <div class="text-sm">{{ GMI }}</div>
          </div>
        </div>
      </div>
      <div class="flex chart-panel flex-col w-2/5 h-full">
        <div class="flex-1 flex flex-col">
          <div class="flex w-full h-3/5">
            <div ref="todayTIRChart" class="w-1/2 h-full w-full"></div>
            <div ref="todayTTIRChart" class="w-1/2 h-full w-full"></div>
          </div>
          <div class="flex justify-around items-center h-20">
            <Device :data="{
              reservoirRemainingUnits: data.reservoirRemainingUnits,
              medicalDeviceBatteryLevelPercent: data.medicalDeviceBatteryLevelPercent,
              sensorLastDatetime: sugarCalc.sensorState(data,false),
              sensorLastDatetimeHumanize:  sugarCalc.sensorState(data),
              gstBatteryLevel:data.gstBatteryLevel || '--'
            }"></Device>
            <Conduit :last-update-time="lastUpdateTime" @updateConduitDatetime="updateConduitTime"></Conduit>
          </div>
          <div class="flex w-full h-2/5 border border-solid border-zinc-300 p-2">
            <div v-for="(item,i) in statistics"
                 :class="{'border-r-zinc border-r border-r-solid':i === 'day30'}"
                 class="flex w-1/2 flex-col h-full justify-around items-center">
              <div class="text-lg font-bold h-10 flex items-center">{{ i === 'day30' ? 30 : 90 }}日</div>
              <div class="w-full flex justify-around flex-1">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>TIR</div>
                  <div class="text-sm">{{ item.avg }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>BELOW</div>
                  <div class="text-sm">{{ item.below }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>ABOVE</div>
                  <div class="text-sm">{{ item.above }}</div>
                </div>
              </div>
              <div class="w-full flex justify-around flex-1">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>大剂量</div>
                  <div class="text-sm">{{ item.recommended }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>基础</div>
                  <div class="text-sm">{{ item.basal }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>修正</div>
                  <div class="text-sm">{{ item.autoCorrection }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="h-15 px-2 flex items-end justify-around">
          <Modes :mode-obj="modeObj"></Modes>
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
             @click="openLogsDialog">
          <ep-Tickets></ep-Tickets>
        </div>
        <div :class="{'only-right':showSetting}"
             class="item flex items-center justify-center border-solid border-1 hand no-bottom">
          <div v-show="showSetting"
               class="flex float-item-panel items-center justify-between border-solid border-1 no-right bg-transparent">
            <div class="float-item ">
              <el-checkbox v-model="setting.notification.alarmEnable" label="报警" size="small"/>
            </div>
          </div>
          <ep-Setting class="hand" @click="triggerSetting"></ep-Setting>
        </div>
        <div :class="{'no-top':showSetting}" class="item flex items-center justify-center border-solid border-1 hand"
             @click="reload">
          <ep-Refresh></ep-Refresh>
        </div>
      </div>
      <!--      <audio ref="alarmAudio" autoplay class="hide" preload="auto" src="/alarm.mp3"></audio>-->
      <NotificationDialog v-if="showNotificationDialog" v-model:show="showNotificationDialog"
                          :NOTIFICATION_MAP="NOTIFICATION_MAP"
                          :notificationHistory="data.notificationHistory"></NotificationDialog>
      <LogsDialog v-if="showLogsDialog" v-model:show="showLogsDialog" :logs="setting.logs"></LogsDialog>
    </div>
  </MainPanel>
</template>
<script lang="ts" name="mySugarBig" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'
import relativeTime from 'dayjs/plugin/relativeTime'
import duration from 'dayjs/plugin/duration'
import echarts from "@/plugins/echart"
import {Msg} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {INSULIN_TYPE, SYSTEM_STATUS_MAP,} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import NotificationDialog from "@/views/components/notificationDialog.vue";
import useSugarCommon from "@/composition/useSugarCommon";
import LogsDialog from "@/views/components/logsDialog.vue";
import {InTimeBarChartData, Log} from "@/model/classes/Carelink";
import useChartResize from "@/composition/useChartResize";
import Menus from "@/views/components/menus.vue";
import Trend from "@/views/components/trend.vue";
import Device from "@/views/components/device.vue";
import Conduit from "@/views/components/conduit.vue";
import Modes from "@/views/components/modes.vue";
import {AlarmClock} from "@element-plus/icons-vue";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)

const sugarService = new SugarService()
const todayTIRChart = ref<HTMLElement>();
const todayTTIRChart = ref<HTMLElement>();
const myChart = ref<HTMLElement>();
let chartObj: any = {
  todayTIRChart: null,
  todayTTIRChart: null,
  myChart: null
}
let resizeObj: any = {
  todayTIRChart: null,
  todayTTIRChart: null,
  myChart: null
}
const logMaxLen = 30
const sugarCalc = useSugarCalc()
let NOTIFICATION_MAP: any = {}
const sugarCommon = useSugarCommon({
  refreshChart,
  dealSelfData: (result) => {
    state.statistics = result.statistics
  },
  alarmNotification
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
} = sugarCommon

const state: any = reactive({
  showLogsDialog: false,
  statistics: {},
  showSetting: false,
  playAlarmObj: {
    alarmAudio: new Audio('/alarm.mp3'),
    playing: false,
    count: 1,
    totalPlayCount: 1,
    content: ''
  }
})

const {
  statistics,
  showLogsDialog,
  showSetting
} = toRefs(state)

const {
  data,
  updateDatetime,
  status,
  showNotificationDialog,
  nextStartTime,
  GMI,
  time
} = toRefs(sugarCommon.state)

onBeforeMount(() => {
  initSetting()
})

onMounted(async () => {
  // await onLoadCarelinkData()
  startTimeInterval()
  Msg.alert('开启大屏模式', async () => {
        await sugarService.initSugarSSE((res) => {
          dealCarelinkData(res)
          refreshChart()
        })
      },
  )
})

onBeforeUnmount(() => {
  for (const item in resizeObj) {
    resizeObj[item].beforeDestroy()
  }
})

function initSetting() {
  loadSettings().then(settingJSON => {
    NOTIFICATION_MAP = settingJSON.NOTIFICATION_MAP
  })

  const {playAlarmObj} = state
  playAlarmObj.alarmAudio.muted = false
  playAlarmObj.autoplay = true
  playAlarmObj.alarmAudio.addEventListener("ended", playEnd)
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
  if (!setting.logs) {
    setting.logs = []
  }
  if (!setting.notification.lastAlarm) {
    setting.notification.lastAlarm = {
      key: '',
      isClear: false,
      isActive: false
    }
  }
  if (!setting.notification.alarmEnable) {
    setting.notification.alarmEnable = true
  }
}

function openLogsDialog() {
  state.showLogsDialog = true
}

function triggerSetting() {
  state.showSetting = !state.showSetting
}

function alarmNotification(item, notification, isActive) {
  if (!item || !setting.notification.alarmEnable) return
  const notifyObj = NOTIFICATION_MAP[item.messageId]
  console.log(item, state.playAlarmObj.playing);
  if (notifyObj && notifyObj.alarm && !state.playAlarmObj.playing) {
    console.log("play");
    // console.log(isActive, item.referenceGUID, notification.lastAlarm.key);
    const instanceId = item.instanceId
    if (!notification.lastAlarm.key || instanceId !== notification.lastAlarm.key || (instanceId === notification.lastAlarm.key && !notification.lastAlarm.isClear)) {
      notification.lastAlarm.key = instanceId
      // setting.logs.push(new Log({content: `警告源数据:${JSON.stringify(item)},isActive:${isActive}`}))
      // stopPlayer()
      const {playAlarmObj} = state
      playAlarmObj.totalPlayCount = notifyObj.alarm.repeat
      playAlarmObj.content = `${notifyObj.text} instanceId:${instanceId}`
      playAlarm(notifyObj.type)
    }
  }
}

function testPlay() {
  const {notification} = setting;
  const obj1 = {
    "referenceGUID": "66230000-3503-0000-D6C0-422D00000000",
    "dateTime": "2025-01-24T02:28:57.000-00:00",
    "type": "ALERT",
    "faultId": 821,
    "instanceId": 9062,
    "messageId": "BC_SID_SMART_GUARD_MINIMUM_DELIVERY",
    "pumpDeliverySuspendState": false,
    "pnpId": "1.1",
    "relativeOffset": -23430,
    "alertSilenced": false,
    "triggeredDateTime": "2025-01-24T02:24:21.000-00:00"
  }
  const obj2 = {
    "referenceGUID": "67230000-3F03-0000-EAC1-422D00000000",
    "dateTime": "2025-01-24T02:29:00.000-00:00",
    "type": "ALERT",
    "faultId": 831,
    "instanceId": 9063,
    "messageId": "BC_SID_ENTER_BG_TO_CONTINUE_IN_SMART_GUARD",
    "pumpDeliverySuspendState": false,
    "pnpId": "1.1",
    "relativeOffset": -23427,
    "alertSilenced": false,
    "triggeredDateTime": "2025-01-24T02:24:21.000-00:00"
  }
  alarmNotification(obj2, notification, false)
  // alarmNotification(obj1, notification, false)
}

function playAlarm(type = 'error') {
  if (!setting.notification.alarmEnable) return
  const {playAlarmObj} = state
  // console.log(playAlarmObj);
  if (playAlarmObj.count <= playAlarmObj.totalPlayCount && !playAlarmObj.playing) {
    playAlarmObj.playing = true
    playAlarmObj.alarmAudio.play().then(res => {
      // playAlarmObj.alarmAudio.addEventListener("ended", playEnd)
      // console.log(`第${playAlarmObj.count}次警告播放:${playAlarmObj.content}`);
      Msg.showMsg({
        center: true,
        duration: 0,
        showClose: false,
        offset: window.innerHeight / 2 - 150,
        icon: AlarmClock,
        customClass: `alarm-msg ${type}`,
        onClick() {
          stopPlayer()
        }
      })
      setting.logs.unshift(new Log({content: `第${playAlarmObj.count}次警告播放:${playAlarmObj.content}`,}))
    }).catch(error => {
      console.log(error);
      playAlarmObj.playing = false
      Msg.closeMsg()
      setting.logs.unshift(new Log({content: `播放错误:${JSON.stringify(error)}`,}))
      if (error.name === 'NotAllowedError') {
        Msg.alert('播放失败,请允许播放音频', () => {
          playAlarm()
        })
      }
    });
  }
}


async function playEnd() {
  const {playAlarmObj} = state
  // console.log("play end");
  playAlarmObj.count++
  // setting.logs.push(new Log({content: `in ended event:${playAlarmObj.count}, playerCount:${playAlarmObj.totalPlayCount},lastAlarm:${JSON.stringify(setting.notification.lastAlarm)}`,}))
  if (playAlarmObj.count <= playAlarmObj.totalPlayCount) {
    playAlarmObj.playing = false
    Msg.closeMsg()
    setTimeout(playAlarm, 500); // 每次播放间隔1秒
  } else {
    if (setting.logs.length > logMaxLen) {
      setting.logs.splice(logMaxLen)
    }
    stopPlayer()
    // playAlarmObj.alarmAudio.removeEventListener('ended', playEnd)
  }
}

function stopPlayer() {
  state.playAlarmObj.alarmAudio.pause();
  state.playAlarmObj.alarmAudio.currentTime = 0;
  state.playAlarmObj.playing = false
  state.playAlarmObj.count = 1
  state.playAlarmObj.totalPlayCount = 1
  setting.notification.lastAlarm.isClear = true
  setting.logs.unshift(new Log({content: `播放结束`,}))
  Msg.closeMsg()
}

function refreshChart() {
  if (!sugarCommon.state.prepare) return
  if (!chartObj.todayTIRChart) {
    drawLine()
  }

  chartObj.todayTIRChart.setOption(charOption.value.todayTIRChart, true);
  chartObj.todayTTIRChart.setOption(charOption.value.todayTTIRChart, true);
  chartObj.myChart.setOption(charOption.value.myChart, true);
}

const nextStartTimeToNow = computed(() => {
  return time.value.to(sugarCommon.state.nextStartTime)
})
//画图的参数
const charOption = computed(() => {
  return {
    todayTIRChart: new InTimeBarChartData('TIR', timeInRange.value),
    todayTTIRChart: new InTimeBarChartData('TTIR', tightTimeInRange.value),
    myChart: {
      grid: {
        left: 20,
        top: 0,
        right: 20,
        bottom: 0,
      },
      xAxis: {
        type: 'time',
        show: false,
      },
      yAxis: [
        {
          name: '基础',
          nameLocation: 'start',
          show: false,
          type: 'value',
          min: 0,
          max: sugarCalc.loadBaselData(sugarCommon.state.data.markers).max + 0.3,
        }],
      tooltip: {
        trigger: 'axis',
        confine: true,
        formatter: params => {
          // 获取xAxis data中的数据
          const param = params[0]
          let dataStr = `<div class="text-xs font-bold flex justify-between">
          <span>${dayjs(param.data[0]).format("HH:mm")}</span>
        </div>`
          params.forEach((item, i) => {
            const type = item.data[2]
            dataStr += `
            <div class="flex text-xs items-center justify-between my-1 w-15">
              <span style="width:10px;height:10px;background-color:${type.key === INSULIN_TYPE.SG.key ? sugarCalc.sgColor(item.data[1]) : type.color};"></span>
              <span>${item.data[1]}</span>
            </div>`
          })
          return dataStr
        }
      },
      dataZoom: [
        {
          type: 'inside',
          id: 'sliderX',
          start: 50,
          end: 100,
          bottom: 0
        }],
      series:
          [
            {
              name: '基础',
              type: "bar",
              markArea: {
                silent: true,
              },
              label: {
                show: true,
                color: 'inherit',
                formatter: (item) => {
                  return item.data[2].key === INSULIN_TYPE.AUTOCORRECTION.key ? item.data[1] : ''
                },
                position: 'top'
              },
              itemStyle: {
                color: item => {
                  if (item.data) {
                    return item.data[2].color
                  }
                }
              },
              data: sugarCalc.loadBaselData(sugarCommon.state.data.markers).list
            },
          ]
    }
  }
})

function drawLine() {
  // 基于准备好的dom，初始化echarts实例
  if (!chartObj.todayTIRChart) { // 如果不存在，就进行初始化。
    chartObj.todayTIRChart = echarts.init(<HTMLElement>todayTIRChart.value, 'dark')
  }
  if (!chartObj.todayTTIRChart) { // 如果不存在，就进行初始化。
    chartObj.todayTTIRChart = echarts.init(<HTMLElement>todayTTIRChart.value, 'dark')
  }
  if (!chartObj.myChart) { // 如果不存在，就进行初始化。
    chartObj.myChart = echarts.init(<HTMLElement>myChart.value, 'dark')
  }

  for (const item in resizeObj) {
    resizeObj[item] = useChartResize(chartObj[item])
    resizeObj[item].mounted()
  }
}
</script>
<style lang="scss">
.alarm-msg {
  border: none;
  background-color: transparent;

  &.warning {
    .el-icon.el-message-icon--success {
      color: #ffd539;
    }
  }

  &.error {
    .el-icon.el-message-icon--success {
      color: #fd3c7d;
    }
  }

  .el-icon.el-message-icon--success {
    font-size: 150px;
  }

  .el-message__content {
    display: none;
  }
}
</style>
<style lang="scss" scoped>
@import "../styles/float-panel.scss";

.big-panel {
  .sg {
    font-size: 16em;
  }
}

.float-panel {
  background: black;

  .float-item-panel {
    background: black;

    :deep(.el-checkbox__label) {
      color: white;
    }
  }
}
</style>
