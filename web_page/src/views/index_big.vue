<template>
  <MainPanel no-pad="1">
    <div class="flex h-full bg-black overflow-x-hidden text-white p-4">
      <el-dropdown class="menu-panel" placement="bottom-start" trigger="click" @command="handleMenu">
        <ep-Menu class="text-white"></ep-Menu>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="">Home</el-dropdown-item>
            <el-dropdown-item command="info">Info</el-dropdown-item>
            <el-dropdown-item command="dict">Dict</el-dropdown-item>
            <el-dropdown-item command="food">Food</el-dropdown-item>
            <el-dropdown-item command="login">Login</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <div class="flex flex-col w-3/5  big-panel mr-4">
        <div class="h-10 flex w-full justify-between items-center px-4">
          <div :class="{'text-red':status!==200}" class="text-base hand"
               @click="refreshCarelinkToken">状态:{{ status }}
          </div>
          <div class="text-4xl font-bold hand" @click="reloadCarelinkData">{{ time.format('HH:mm') }}</div>
        </div>
        <div class="flex flex-col w-full h-full flex-1">
          <div class="flex w-full items-center justify-center flex-1">
            <div :style="{color:sugarCalc.sgColor(sugarCalc.getLastSg(data.lastSG))}" class="sg font-bold ">{{
                sugarCalc.getLastSg(data.lastSG)
              }}
            </div>
            <div class="text-6xl flex font-thin arrow">
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
          <div class="h-10 flex items-start justify-center text-base">
            <span :class="{'text-red':lastUpdateTime.sgDiff>=15}" class="mx-2">
              {{
                lastUpdateTime.sg
              }}
            </span>
            <span class="mx-2">{{ lastOffset }}</span>
            <div class="flex items-center justify-between align-center">
              <span v-if="data.systemStatusMessage===SYSTEM_STATUS_MAP.WARM_UP.key" class="mx-2">预计启动:&nbsp;{{
                  toNow(nextStartTime)
                }}</span>
              <span v-else class="mx-2">更新:&nbsp;{{ updateDatetime }}</span>
              <div :style="{color: SYSTEM_STATUS_MAP[data.systemStatusMessage]?.color}" class="mr-2">
                {{ SYSTEM_STATUS_MAP[data.systemStatusMessage]?.name }}
              </div>
            </div>
          </div>
        </div>
        <div class="flex items-center justify-around h-20">
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
          <div class="flex w-full h-1/2">
            <div ref="todayTIRChart" class="w-1/2 h-full w-full"></div>
            <div ref="todayTTIRChart" class="w-1/2 h-full w-full"></div>
          </div>
          <div class="flex w-full h-1/2 border border-solid border-zinc-300 p-2">
            <div class="flex w-1/2 flex-col h-full justify-around items-center border-r-zinc border-r border-r-solid">
              <div class="text-lg font-bold h-10 flex items-center">30日</div>
              <div class="w-full flex justify-around">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>TIR</div>
                  <div class="text-sm">{{ statistics.day30?.avg }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>BELOW</div>
                  <div class="text-sm">{{ statistics.day30?.below }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>ABOVE</div>
                  <div class="text-sm">{{ statistics.day30?.above }}</div>
                </div>
              </div>
              <div class="w-full flex justify-around">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>大剂量</div>
                  <div class="text-sm">{{ statistics.day30?.recommended }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>基础</div>
                  <div class="text-sm">{{ statistics.day30?.basal }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>修正</div>
                  <div class="text-sm">{{ statistics.day30?.autoCorrection }}</div>
                </div>
              </div>
            </div>
            <div class="flex w-1/2 flex-col h-full justify-around items-center">
              <div class="text-lg font-bold  h-10 flex items-center">90日</div>
              <div class="w-full flex justify-around">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>TIR</div>
                  <div class="text-sm">{{ statistics.day90?.avg }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>BELOW</div>
                  <div class="text-sm">{{ statistics.day90?.below }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>ABOVE</div>
                  <div class="text-sm">{{ statistics.day90?.above }}</div>
                </div>
              </div>
              <div class="w-full flex justify-around">
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>大剂量</div>
                  <div class="text-sm">{{ statistics.day90?.recommended }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>基础</div>
                  <div class="text-sm">{{ statistics.day90?.basal }}</div>
                </div>
                <div class="flex flex-col items-center justify-center w-1/3">
                  <div>修正</div>
                  <div class="text-sm">{{ statistics.day90?.autoCorrection }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="h-20 px-2 flex items-center justify-around">
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
        <div :class="{'no-top':showSetting}" class="item flex items-center justify-center border-solid border-1 hand"
             @click="reload">
          <ep-Refresh></ep-Refresh>
        </div>
      </div>
      <NotificationDialog v-if="showNotificationDialog" v-model:show="showNotificationDialog"
                          :notificationHistory="data.notificationHistory"></NotificationDialog>
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
import {DATE_FORMAT} from "@/model/model-type";
import {Msg, Tools} from '@/utils/tools'
import {SugarService} from "@/service/sugar-service";
import {
  CARELINK_DICT_KEY,
  COLORS,
  DIRECTIONS,
  NOTIFICATION_HASH_KEY,
  PUMP_STATUS,
  REFRESH_INTERVAL,
  SG_STATUS,
  SYSTEM_STATUS_MAP,
  TIME_RANGE_CONFIG
} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import defaultSettings from "@/settings";
import {DictService} from "@/service/dict-service";
import CryptoJS from "crypto-js";
import {cloneDeep, flatten, forEach} from "lodash-es";
import NotificationDialog from "@/views/components/notificationDialog.vue";
import CarelinkData from "@/model/classes/CarelinkData";

dayjs.locale('zh-cn')
dayjs.extend(relativeTime)
dayjs.extend(duration)

const sugarService = new SugarService()
const dictService = new DictService()

const todayTIRChart = ref<HTMLElement>();
const todayTTIRChart = ref<HTMLElement>();
let chartObj: any = {
  todayTIRChart: null,
  todayTTIRChart: null,
}
const lastStatus: any = Tools.getLastStatus('sugar-setting', {
  startPercent: TIME_RANGE_CONFIG[1].value,
  showAR2: true,
  showYesterday: true,
  notification: {
    hasNew: false,
    lastKey: null
  },
  realWave: true,
  showPeak: true
})
const sugarCalc = useSugarCalc()
const setting = lastStatus.value['sugar-setting']
const state: any = reactive({
  status: 200,
  prepare: false,
  showNotificationDialog: false,
  updateDatetime: '--',//数据更新时间
  interval: {
    time: null,
    data: null,
    myData: null,
  },
  orgMyData: {},
  GMI: 0,
  nextStartTime: -1,
  data: new CarelinkData(),
  statistics: {},
  time: dayjs(),//当前系统时间
})

const {
  data,
  time,
  updateDatetime,
  status,
  showNotificationDialog,
  nextStartTime,
  showSetting,
  GMI,
  statistics
} = toRefs(state)

onBeforeMount(() => {
})

onMounted(async () => {
  // await onLoadCarelinkData()
  startInterval()
  await sugarService.initSugarSSE((res) => {
    dealCarelinkData(res)
    refreshChart()
  })
})

onBeforeUnmount(() => {
})

function triggerSetting() {
  state.showSetting = !state.showSetting
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
  Msg.confirm("是否确认更新管路更换时间", async () => {
    state.orgMyData.lastConduitTime = dayjs().format(DATE_FORMAT.datetime)
    const result = await dictService.updateDict({
      key: CARELINK_DICT_KEY.carelinkMyData,
      val: JSON.stringify(state.orgMyData)
    }, {user: true})
    if (result) {
      Msg.successMsg('更新管路更换时间成功')
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
  // startDataLoadInterval()
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
    const result = await sugarService.loadData(mask)
    dealCarelinkData(result)
  } catch (e) {
    console.log(e);
  }
}

function dealCarelinkData(result) {
  console.log(result);
  if (result) {
    state.data = result.data
    state.status = result.status
    state.forecast = result.forecast || {ar2: []}
    state.GMI = result.GMI
    state.nextStartTime = result.nextStartTime
    state.updateDatetime = dayjs(state.data.update_time).format("MM-DD HH:mm")
    state.statistics = result.statistics
    // state.data.lastSG.datetime = sugarCalc.cleanTime(state.data.lastSG.datetime)
    // setting.notification.hasNew = true
    dealNewNotification()
    dealMyData(result.myData)

    state.prepare = true
    document.title = `${defaultSettings.title} ${sugarCalc.calcSG(state.data.lastSG.sg)}, ${lastOffset.value > 0 ? '+' + lastOffset.value : lastOffset.value}`
  } else {
    state.prepare = false
  }
}

function dealMyData(myData) {
  // console.log(myData);
  state.orgMyData = cloneDeep(myData)
  state.myData = myData
  if (state.myData.yesterday) {
    flattenYesterdayData('sgs', 'datetime', () => {
      state.myData.yesterday.sgs = state.myData.yesterday.sgs.filter(item => {
        return item.sensorState === SG_STATUS.NO_ERROR_MESSAGE.key && item.datetime >= dayjs().add(-1, 'day').valueOf()
      })
    })
    flattenYesterdayData('markers', 'dateTime', () => {
      state.myData.yesterday.markers = state.myData.yesterday.markers.filter(item => {
        return (item.type === 'INSULIN' || item.type === 'MEAL') && item.dateTime >= dayjs().add(-1, 'day').valueOf()
      })
    })
  }
}

function flattenYesterdayData(key, timeKey = 'datetime', suffixFunc = () => {
}) {
  state.myData.yesterday[key] = flatten(state.myData.yesterday[key])
  state.myData.yesterday[key].forEach(item => {
    item[timeKey] = dayjs(sugarCalc.cleanTime(item[timeKey])).add(1, 'day').valueOf()
  })
  suffixFunc()
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

function refreshChart() {
  if (!state.prepare) return
  if (!chartObj.todayTIRChart) {
    drawLine()
  }

  chartObj.todayTIRChart.setOption(charOption.value.todayTIRChart, true);
  chartObj.todayTTIRChart.setOption(charOption.value.todayTTIRChart, true);
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
  let sumInsulin = 0
  let sumBaseDelivery = 0
  if (state.orgMyData.yesterday?.markers) {
    const len = state.orgMyData.yesterday?.markers.length
    state.orgMyData.yesterday?.markers[len === 2 ? 1 : 0].forEach(item => {
      if (item.type === 'INSULIN') {
        sumInsulin += item.deliveredFastAmount
      }
      if (item.type === 'AUTO_BASAL_DELIVERY') {
        sumBaseDelivery += item.bolusAmount
      }
    })
  }

  return {
    sg: toNow(lastSgUpdateTime),
    sgDiff: dayjs().diff(lastSgUpdateTime, 'minute'),
    conduit: toNow(state.orgMyData.lastConduitTime),
    sumInsulin: sumInsulin.toFixed(2),
    sumBaseDelivery: sumBaseDelivery.toFixed(2)
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
}
</script>
<style lang="scss" scoped>
.menu-panel {
  position: absolute;
  right: 5px;
  z-index: 999;

  svg {
    width: 30px;
    height: 30px;
  }
}

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
