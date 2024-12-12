<template>
  <MainPanel no-pad="1">
    <div class="flex h-full bg-black overflow-x-hidden text-white p-4">
      <Menus :is-home="false" @handler="handleMenu"></Menus>
      <div class="flex flex-col w-3/5  big-panel mr-4">
        <div class="h-10 flex w-full justify-between items-center px-4">
          <div :class="{'text-red':status!==200}" class="text-base hand"
               @click="refreshCarelinkToken">状态:{{ status }}
          </div>
          <div v-if="playAlarmObj.playing">
            <ep-AlarmClock class="h-10 w-10 hand" @click="stopPlayer"></ep-AlarmClock>
          </div>
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
            <span :class="{'text-red':lastUpdateTime.sgDiff>=15}" class="mx-2" @click="playAlarm">
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
                      Tools.toNow(nextStartTime)
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
        <div class="item flex items-center justify-center border-solid border-1 hand"
             @click="reload">
          <ep-Refresh></ep-Refresh>
        </div>
      </div>
      <!--      <audio ref="alarmAudio" autoplay class="hide" preload="auto" src="/alarm.mp3"></audio>-->
      <NotificationDialog v-if="showNotificationDialog" v-model:show="showNotificationDialog"
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
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {INSULIN_TYPE, NOTIFICATION_MAP, SYSTEM_STATUS_MAP,} from "@/views/const";
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
const sugarCalc = useSugarCalc()
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
  playAlarmObj
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
}

function openLogsDialog() {
  state.showLogsDialog = true
}

function alarmNotification(item, notification, isActive) {
  if (!item) return
  const notifyObj = NOTIFICATION_MAP[item.messageId]
  if (notifyObj && notifyObj.alarm && !state.playing) {
    // console.log(isActive, item.referenceGUID, notification.lastAlarm.key);
    const notificationKey = isActive ? item.GUID : item.referenceGUID
    if (!notification.lastAlarm.key || notificationKey !== notification.lastAlarm.key || (notificationKey === notification.lastAlarm.key && !notification.lastAlarm.isClear)) {
      notification.lastAlarm.key = notificationKey
      // setting.logs.push(new Log({content: `警告源数据:${JSON.stringify(item)},isActive:${isActive}`}))
      // stopPlayer()
      const {playAlarmObj} = state
      playAlarmObj.totalPlayCount = notifyObj.alarm.repeat
      playAlarmObj.content = `${notifyObj.text} key:${notificationKey}`
      playAlarm()
    }
  }
}

function playAlarm() {
  const {playAlarmObj} = state
  // console.log(playAlarmObj);
  if (playAlarmObj.count <= playAlarmObj.totalPlayCount && !playAlarmObj.playing) {
    playAlarmObj.alarmAudio.play().then(res => {
      playAlarmObj.playing = true
      // playAlarmObj.alarmAudio.addEventListener("ended", playEnd)
      console.log(`第${playAlarmObj.count}次警告播放:${playAlarmObj.content}`);
      setting.logs.push(new Log({content: `第${playAlarmObj.count}次警告播放:${playAlarmObj.content}`,}))
    }).catch(error => {
      console.log(error);
      playAlarmObj.playing = false
      setting.logs.push(new Log({content: `播放错误:${JSON.stringify(error)}`,}))
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
    setTimeout(playAlarm, 500); // 每次播放间隔1秒
  } else {
    stopPlayer()
    console.log("警告播放结束");
    setting.logs.push(new Log({content: `警告播放结束`,}))
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
          max: sugarCalc.loadBaselData(sugarCommon.state.data.markers).max + 0.1,
        }],
      series: [
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
<style lang="scss" scoped>
.big-panel {
  .sg {
    font-size: 16em;
  }
}

.float-panel {
  right: 5px;
  position: absolute;
  bottom: 20px;
  background: black;

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

  .no-top {
    border-top: none;
  }

  .no-bottom {
    border-bottom: none;
  }
}
</style>
