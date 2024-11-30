<template>
    <MainPanel no - pad = "1" >
<div class = "flex h-full bg-black overflow-x-hidden text-white p-4" >
    <el-dropdown

class

= "menu-panel"
placement = "bottom-start"
trigger = "click"
@command
= "handleMenu" >
    <ep-Menu

class

= "text-white" > </ep-Menu>
    < template
#dropdown >
<el-dropdown - menu >
<el-dropdown - item
command = "" > Home < /el-dropdown-item>
    < el - dropdown - item
command = "info" > Info < /el-dropdown-item>
    < el - dropdown - item
command = "dict" > Dict < /el-dropdown-item>
    < el - dropdown - item
command = "food" > Food < /el-dropdown-item>
    < el - dropdown - item
command = "login" > Login < /el-dropdown-item>
    < /el-dropdown-menu>
    < /template>
    < /el-dropdown>
    < div

class

= "flex flex-col w-3/5  big-panel mr-4" >
<div class = "h-10 flex w-full justify-between items-center px-4" >
    <div
:

class

= "{'text-red':status!==200}"

class

= "text-base hand"
@click
= "refreshCarelinkToken" > 状态
:
{
  {
    status
  }
}
</div>
< div
v -
if= "playing" >
    <ep-AlarmClock class
= "h-10 w-10 hand"
@click
= "stopPlayer" > </ep-AlarmClock>
    < /div>
    < div

class

= "text-4xl font-bold hand"
@click
= "reloadCarelinkData" > {
{
  time.format('HH:mm')
}
}
</div>
< /div>
< div

class

= "flex flex-col w-full h-full flex-1" >
<div class = "flex w-full items-center justify-center flex-1" >
    <div
:
style = "{color:sugarCalc.sgColor(sugarCalc.getLastSg(data.lastSG))}"

class

= "sg font-bold " > {
{
  sugarCalc.getLastSg(data.lastSG)
}
}
</div>
< div

class

= "text-6xl flex font-thin arrow" >
    <template v -
if= "trendObj?.direction" >
    <template v -if= "trendObj.direction === 1" >
    <ep-Top v -
for= "i in trendObj.num" > </ep-Top>
    < /template>
    < template v -
if= "trendObj.direction === 2" >
    <ep-Top - Right > </ep-Top-Right>
    < /template>
    < template v -
if= "trendObj.direction === 3" >
    <ep-Right > </ep-Right>
    < /template>
    < template v -
if= "trendObj.direction === 4" >
    <ep-Bottom - Right > </ep-Bottom-Right>
    < /template>
    < template v -
if= "trendObj.direction === 5" >
    <ep-Bottom v -
for= "i in trendObj.num" > </ep-Bottom>
    < /template>
    < /template>
    < /div>
    < /div>
    < div class
= "h-10 flex items-start justify-center text-base" >
    <span
:

class

= "{'text-red':lastUpdateTime.sgDiff>=15}"

class

= "mx-2"
@click
= "playAlarm(3)" >
    {
{
  lastUpdateTime.sg
}
}
</span>
< span

class

= "mx-2" > {
{
  lastOffset
}
}
</span>
< div

class

= "flex items-center justify-between align-center" >
    <span v -
if= "data.systemStatusMessage===SYSTEM_STATUS_MAP.WARM_UP.key" class
= "mx-2" > 预计启动
:&
nbsp;
{
  {
    Tools.toNow(nextStartTime)
  }
}
</span>
< span
v -
else

class

= "mx-2" > 更新
:&
nbsp;
{
  {
    updateDatetime
  }
}
</span>
< div
:
style = "{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}"

class

= "mr-2" >
    {
{
  SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name
}
}
</div>
< /div>
< /div>
< /div>
< div

class

= "flex items-center justify-around h-20" >
<div class = "flex flex-col justify-center items-center" >
<div class = "text-base" > IOB < /div>
    < div

class

= "text-sm" > {
{
  data.activeInsulin.amount
}
}
</div>
< /div>
< div

class

= "flex flex-col justify-center items-center" >
<div class = "text-base" > AVG < /div>
    < div

class

= "text-sm" > {
{
  sugarCalc.calcSG(data.averageSG)
}
}
</div>
< /div>
< div

class

= "flex flex-col justify-center items-center" >
<div class = "text-base" > CV < /div>
    < div

class

= "text-sm" > {
{
  sugarCalc.calcCV(data.sgs, data.averageSG)
}
}%
</div>
< /div>
< div

class

= "flex flex-col justify-center items-center" >
<div class = "text-base" > GMI < /div>
    < div

class

= "text-sm" > {
{
  GMI
}
}
</div>
< /div>
< /div>
< /div>
< div

class

= "flex chart-panel flex-col w-2/5 h-full" >
<div class = "flex-1 flex flex-col" >
<div class = "flex w-full h-3/5" >
<div ref = "todayTIRChart"

class

= "w-1/2 h-full w-full" > </div>
    < div
ref = "todayTTIRChart"

class

= "w-1/2 h-full w-full" > </div>
    < /div>
    < div

class

= "flex justify-around items-center h-20" >
    <el-tag

class

= "mb-1 mr-1"
size = "small"
type = "primary" >
    泵
:
{
  {
    data.reservoirRemainingUnits
  }
}
U
{
  {
    data.medicalDeviceBatteryLevelPercent
  }
}
%&
nbsp;
探头:
{
  {
    sugarCalc.sensorState(data)
  }
}
{
  {
    data.gstBatteryLevel || '--'
  }
}
%
</el-tag>
< el - tag

class

= "hand"
size = "small"
type = "warning"
@click
= "updateConduitTime" > 管路
:
{
  {
    lastUpdateTime.conduit || '--'
  }
}
</el-tag>
< /div>
< div

class

= "flex w-full h-2/5 border border-solid border-zinc-300 p-2" >
    <div v -
for= "(item,i) in statistics"
  :

class

= "{'border-r-zinc border-r border-r-solid':i === 'day30'}"

class

= "flex w-1/2 flex-col h-full justify-around items-center" >
<div class = "text-lg font-bold h-10 flex items-center" > {
{
  i === 'day30' ? 30 : 90
}
}
日 < /div>
< div

class

= "w-full flex justify-around flex-1" >
<div class = "flex flex-col items-center justify-center w-1/3" >
    <div>TIR < /div>
    < div

class

= "text-sm" > {
{
  item.avg
}
}
</div>
< /div>
< div

class

= "flex flex-col items-center justify-center w-1/3" >
    <div>BELOW < /div>
    < div

class

= "text-sm" > {
{
  item.below
}
}
</div>
< /div>
< div

class

= "flex flex-col items-center justify-center w-1/3" >
    <div>ABOVE < /div>
    < div

class

= "text-sm" > {
{
  item.above
}
}
</div>
< /div>
< /div>
< div

class

= "w-full flex justify-around flex-1" >
<div class = "flex flex-col items-center justify-center w-1/3" >
    <div>大剂量 < /div>
    < div

class

= "text-sm" > {
{
  item.recommended
}
}
</div>
< /div>
< div

class

= "flex flex-col items-center justify-center w-1/3" >
    <div>基础 < /div>
    < div

class

= "text-sm" > {
{
  item.basal
}
}
</div>
< /div>
< div

class

= "flex flex-col items-center justify-center w-1/3" >
    <div>修正 < /div>
    < div

class

= "text-sm" > {
{
  item.autoCorrection
}
}
</div>
< /div>
< /div>
< /div>
< /div>
< /div>
< div

class

= "h-20 px-2 flex items-center justify-around" >
    <el-tag
:
type = "modeObj.mode.type"

class

= ""
size = "small" >
    {
{
  modeObj.mode.name
}
}
<span v -
if= "modeObj.mode.key===PUMP_STATUS.safe.key" >
    , 闭环退出:
{
  {
    modeObj.timeRemaining
  }
}
</span>
< span
v -
if= "modeObj.mode.key===PUMP_STATUS.sport.key" >
    剩余:
{
  {
    modeObj.timeRemaining
  }
}
</span>
< span
v -
if= "modeObj.mode.key===PUMP_STATUS.manuel.key" >
    , 基础:
{
  {
    modeObj.basalRate
  }
}
<span v -
if= "modeObj.isTemp" >
    , 剩余:
{
  {
    modeObj.timeRemaining
  }
}
</span>
< /span>
< /el-tag>

< el - tag
size = "large"
type = "primary" >
<div class = "flex flex-col " >
    <span>剂量(昨)
:
{
  {
    lastUpdateTime.sumInsulin || '--'
  }
}
U
< /span>
< span > 基础(昨)
:
{
  {
    lastUpdateTime.sumBaseDelivery || '--'
  }
}
U
< /span>
< /div>
< /el-tag>
< /div>
< /div>
< div

class

= "float-panel flex flex-col items-center justify-center" >
<div class = "item flex items-center justify-center border-solid border-1 hand no-bottom"
@click
= "handleMenu('notification')" >
    <el-badge
:
is - dot = "setting.notification.hasNew" >
    <ep-Bell > </ep-Bell>
    < /el-badge>
    < /div>
    < div

class

= "item flex items-center justify-center border-solid border-1 hand no-bottom"
@click
= "openLogsDialog" >
    <ep-Tickets > </ep-Tickets>
    < /div>
    < div

class

= "item flex items-center justify-center border-solid border-1 hand"
@click
= "reload" >
    <ep-Refresh > </ep-Refresh>
    < /div>
    < /div>
      <!--      <audio ref = "alarmAudio"
autoplay

class

= "hide"
preload = "auto"
src = "/alarm.mp3" > </audio>-->
    < NotificationDialog
v -
if= "showNotificationDialog" v - model
:
show = "showNotificationDialog"
:
notificationHistory = "data.notificationHistory" > </NotificationDialog>
    < LogsDialog
v -
if= "showLogsDialog" v - model
:
show = "showLogsDialog"
:
logs = "setting.logs" > </LogsDialog>
    < /div>
    < /MainPanel>
    < /template>
    < script
lang = "ts"
name = "mySugarBig"
setup >
import MainPanel from '@/layout/components/MainPanel.vue'
import dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'
import relativeTime from 'dayjs/plugin/relativeTime'
import duration from 'dayjs/plugin/duration'
import echarts from "@/plugins/echart"
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {COLORS, NOTIFICATION_MAP, SYSTEM_STATUS_MAP,} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import NotificationDialog from "@/views/components/notificationDialog.vue";
import useSugarCommon from "@/composition/useSugarCommon";
import LogsDialog from "@/views/components/logsDialog.vue";
import {Log} from "@/model/classes/Carelink";
import useChartResize from "@/composition/useChartResize";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)

const sugarService = new SugarService()
const todayTIRChart = ref<HTMLElement>();
const todayTTIRChart = ref<HTMLElement>();
let chartObj: any = {
  todayTIRChart: null,
  todayTTIRChart: null,
}
let resizeObj: any = {}
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

const alarmAudio = new Audio('/alarm.mp3')
const state: any = reactive({
  playing: false,
  showLogsDialog: false,
  statistics: {},
})

const {
  playing,
  statistics,
  showLogsDialog
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
  if (resizeObj) {
    resizeObj.chart1.beforeDestroy()
    resizeObj.chart2.beforeDestroy()
  }
})

function initSetting() {
  alarmAudio.muted = false
  // alarmAudio.autoplay = true
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
      isClear: false
    }
  }
}

function openLogsDialog() {
  state.showLogsDialog = true
}

function alarmNotification(item, notification) {
  if (!item) return
  const notifyObj = NOTIFICATION_MAP[item.messageId]
  if (notifyObj && notifyObj.alarm) {
    // console.log(item.referenceGUID, notification.lastAlarm.key);
    if (!notification.lastAlarm.key || item.referenceGUID !== notification.lastAlarm.key || (item.referenceGUID === notification.lastAlarm.key && !notification.lastAlarm.isClear)) {
      notification.lastAlarm.key = item.referenceGUID
      playAlarm(notifyObj.alarm.repeat, notifyObj.text)
    }
  }
}

function playAlarm(plyCount = 1, alarmContent = '') {
  let count = 1;
  stopPlayer()

  function playNext() {
    if (count <= plyCount && !state.playing) {
      state.playing = true
      alarmAudio.play().then(res => {
        console.log(`第${count}次警告播放:${alarmContent}`);
        setting.logs.push(new Log({content: `第${count}次警告播放:${alarmContent}`,}))
      }).catch(error => {
        console.log(error);
        if (error.name === 'NotAllowedError') {
          Msg.alert('播放失败,请允许播放音频', () => {
            playNext()
          })
        }
      });
    }
  }

  // 监听音频播放结束事件
  alarmAudio.addEventListener('ended', () => {
    count++;
    state.playing = false
    if (count <= plyCount) {
      setTimeout(playNext, 500); // 每次播放间隔1秒
    } else {
      stopPlayer(true)
      console.log("警告播放结束");
      setting.logs.push(new Log({content: `警告播放结束`,}))
      // 清除事件监听器，防止内存泄漏
      alarmAudio.removeEventListener('ended', playNext);
    }
  });

  // 开始第一次播放
  playNext();
}

function stopPlayer(isClear = false) {
  alarmAudio.pause();
  alarmAudio.currentTime = 0;
  state.playing = false
  setting.notification.lastAlarm.isClear = isClear
}

function refreshChart() {
  if (!sugarCommon.state.prepare) return
  if (!chartObj.todayTIRChart) {
    drawLine()
  }

  chartObj.todayTIRChart.setOption(charOption.value.todayTIRChart, true);
  chartObj.todayTTIRChart.setOption(charOption.value.todayTTIRChart, true);
}

//画图的参数
const charOption = computed(() => {
  return {
    todayTIRChart: {
      backgroundColor: '', //设置无背景色
      title: {
        text: '今日TIR',
        left: 'center'
      },
      tooltip: {
        trigger: 'item'
      },
      label: {
        show: true, // 显示标签
        formatter: '{d}%' // 设置标签格式
      },
      series: [
        {
          name: '今日TIR',
          type: 'pie',
          radius: '50%',
          data: [
            {value: timeInRange.value[0], name: '框内', itemStyle: {color: COLORS[0]}},
            {value: timeInRange.value[1], name: '低于', itemStyle: {color: COLORS[1]}},
            {value: timeInRange.value[2], name: '高于', itemStyle: {color: COLORS[5]}}
          ]
        }
      ]
    },
    todayTTIRChart: {
      backgroundColor: '', //设置无背景色
      title: {
        text: '今日TTIR',
        left: 'center'
      },
      tooltip: {
        trigger: 'item'
      },
      label: {
        show: true, // 显示标签
        formatter: '{d}%' // 设置标签格式
      },
      series: [
        {
          name: '今日TTIR',
          type: 'pie',
          radius: '50%',
          data: [
            {value: tightTimeInRange.value[0], name: '框内', itemStyle: {color: COLORS[0]}},
            {value: tightTimeInRange.value[1], name: '低于', itemStyle: {color: COLORS[4]}},
            {value: tightTimeInRange.value[2], name: '高于', itemStyle: {color: COLORS[5]}}
          ]
        }
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
  resizeObj.chart1 = useChartResize(chartObj.todayTIRChart)
  resizeObj.chart2 = useChartResize(chartObj.todayTTIRChart)
  resizeObj.chart1.mounted()
  resizeObj.chart2.mounted()
}

</script>
< style
lang = "scss"
scoped >
.
menu - panel
{
  position: absolute;
  right: 5
  px;
  z - index
:
  999;

  svg
  {
    width: 30
    px;
    height: 30
    px;
  }
}

.
big - panel
{
.
  sg
  {
    font - size
  :
    16
    em;
  }
}

.
float - panel
{
  right: 5
  px;
  position: absolute;
  bottom: 20
  px;
  background: black;

.
  float - item - panel
  {
    background: white;
    position: absolute;
    right: 35
    px;

  .
    float - item
    {
      height: 35
      px;
      padding: 0
      5
      px;
      display: flex;
      align - items
    :
      center;
    }
  }

.
  item
  {
    width: 35
    px;
    height: 35
    px;
  }

.
  no - top
  {
    border - top
  :
    none;
  }

.
  no - bottom
  {
    border - bottom
  :
    none;
  }
}
</style>
