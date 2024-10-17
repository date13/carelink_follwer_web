<template>
  <MainPanel :no-top="1">
    <template v-slot:header>
      <Title :back="true" title="信息"></Title>
    </template>
    <div class="flex">
      <el-card class="info-card">
        <template #header>
          Note
        </template>
        <div class="flex ma-auto w-max items-center">
          <el-tag
              class="m-1"
              effect="dark"
              type="success"
          >
            {{ retireDays }}
          </el-tag>
          <el-tag
              class="m-1"
              effect="dark"
              type="success"
          >
            {{ illDays }}
          </el-tag>
        </div>
      </el-card>
      <el-card class="info-card">
        <template #header>
          翻车记录({{ luckDays }} - {{ luck.yes + luck.no }})
        </template>
        <div class="flex justify-around">
          <div class="flex flex-col items-center content-center w-1/2">
            <el-button :icon="Smoking" circle class="text-sm" color="red" size="large" @click="push(false)">
              {{ luck.no }}
            </el-button>
            <div class="text-sm mt-2">{{ showRate(false) }}%</div>
          </div>
          <div class="flex flex-col items-center content-center w-1/2">
            <el-button :icon="Sugar" circle color="green" size="large" @click="push(true)">
              {{ luck.yes }}
            </el-button>
            <div class="text-sm mt-2">{{ showRate(true) }}%</div>
          </div>
        </div>
      </el-card>
    </div>
    <div class="flex">
      <el-card class="info-card text-center">
        <template #header>
          <div class="flex justify-between items-center">
            ICR
            <el-button size="small" type="primary" @click="updateICR">保存</el-button>
          </div>
        </template>
        <div class="flex flex-col ma-auto w-max">
          <el-tag
              class="m-1"
              effect="dark"
              type="warning"
          >
            早
            <el-input-number v-model="luck.ICR.morning" :max="20" :min="1" :precision="1" :step="0.1" size="small"/>
            碳
          </el-tag>
          <el-tag
              class="m-1"
              effect="dark"
              type="warning"
          >
            中
            <el-input-number v-model="luck.ICR.afternoon" :max="20" :min="1" :precision="1" :step="0.1" size="small"/>
            碳
          </el-tag>
          <el-tag
              class="m-1"
              effect="dark"
              type="warning"
          >
            晚
            <el-input-number v-model="luck.ICR.evening" :max="20" :min="1" :precision="1" :step="0.1" size="small"/>
            碳
          </el-tag>
        </div>
      </el-card>
      <el-card class="info-card">
        <template #header>
          ISF
        </template>
        <div class="flex flex-col ma-auto w-max">
          <el-tag
              class="m-1"
              effect="dark"
              type="info"
          >
            早,1U={{ toISF(luck.ICR.morning) }}mmo/l
          </el-tag>
          <el-tag
              class="m-1"
              effect="dark"
              type="info"
          >
            中,1U={{ toISF(luck.ICR.afternoon) }}mmo/l
          </el-tag>
          <el-tag
              class="m-1"
              effect="dark"
              type="info"
          >
            晚,1U={{ toISF(luck.ICR.evening) }}mmo/l
          </el-tag>
        </div>
      </el-card>
    </div>
    <div class="flex">
      <el-card class="info-card lg">
        <template #header>
          <div class="flex justify-between items-center">
            <span>计算</span>
            <el-radio-group v-model="curICR" size="small">
              <el-radio-button :value="luck.ICR.morning" label="早"/>
              <el-radio-button :value="luck.ICR.afternoon" label="中"/>
              <el-radio-button :value="luck.ICR.evening" label="晚"/>
            </el-radio-group>
            <el-button size="small" type="primary" @click="addFood">选择</el-button>
          </div>
        </template>
        <div class="flex flex-col calc-panel">
          <div class="flex justify-between mb-1 pb-1 food-item text-sm font-semibold">
            <div class="w-10"></div>
            <div class="flex-1 flex justify-start items-center">名称</div>
            <div class="flex-1 flex justify-center items-center">碳水</div>
            <div class="flex-1 flex justify-center items-center">重量(克)</div>
          </div>
          <div v-for="(item,i) in foods" class="flex justify-between mb-1 pb-1 food-item text-sm">
            <div class="w-10 flex justify-start items-center text-red hand" @click="delFood(i)">
              <ep-Close></ep-Close>
            </div>
            <div class="flex-1 flex justify-start items-center">{{ item.key }}</div>
            <div class="flex-1 flex justify-center items-center">{{ item.val }}</div>
            <div class="flex-1 flex justify-center items-center">
              <el-input-number v-model="item.weight" :min="1" clearable size="small"></el-input-number>
            </div>
          </div>
        </div>
        <template #footer>
          <div class="flex justify-between text-sm font-semibold">
            <div>
              <el-button size="small" type="warning" @click="resetCalc">清除</el-button>
            </div>
            <div class="flex items-center">总碳水:{{ totalCalc.carbon }} 剂量:{{ totalCalc.insulin }}</div>
          </div>
        </template>
      </el-card>
    </div>
    <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
               :show-close="false" style="height:70%;" width="80%">
      <food :callback="foodCallback" :closeDialog="close" :is-dialog="true" :selected="foods"></food>
    </el-dialog>
  </MainPanel>
</template>
<script lang="ts" name="myinfo" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import Title from "@/components/Title.vue"
import dayjs from 'dayjs'
import {Smoking, Sugar} from "@element-plus/icons-vue";
import {DictService} from "@/service/dict-service";
import {Msg} from "@/utils/tools";
import Food from "./food.vue"

const retireDays = dayjs('2030-10-29').diff(dayjs(), 'days')
const illDays = dayjs().diff(dayjs('2023-2-27'), 'days')
const luckDays = dayjs().diff(dayjs('2023-10-16'), 'days')

const service = new DictService()

const state: any = reactive({
  luck: {
    yes: 0,
    no: 0,
    ICR: {
      morning: 6.5,
      afternoon: 9,
      evening: 11.5
    }
  },
  curICR: 0,
  foods: [],
  show: false
})

const {luck, show, foods, curICR} = toRefs(state)

function toISF(c) {
  return (c / 4).toFixed(2)
}

async function push(isLuck) {
  state.luck[isLuck ? 'yes' : 'no']++
  const result = await service.updateDict({
    key: 'luck',
    val: JSON.stringify(state.luck)
  })
  if (!result) {
    Msg.error('更新失败')
    state.luck[isLuck ? 'yes' : 'no']--
  } else {
    Msg.successMsg('更新成功')
  }
}

onMounted(async () => {
  const result = await service.getDict("luck")
  if (result) {
    Object.assign(state.luck, JSON.parse(result))
  }
  calcCurICR()
})

function calcCurICR() {
  const hour = dayjs().hour()
  if (hour >= 4 && hour <= 10) {
    state.curICR = state.luck.ICR.morning
  } else if (hour >= 11 && hour <= 15) {
    state.curICR = state.luck.ICR.afternoon
  } else {
    state.curICR = state.luck.ICR.evening
  }
}

async function updateICR() {
  const result = await service.updateDict({
    key: 'luck',
    val: JSON.stringify(state.luck)
  })
  if (result) {
    Msg.successMsg('数据保存成功')
    calcCurICR()
  }
}

function resetCalc() {
  state.foods = []
}

function showRate(isLuck) {
  return (state.luck[isLuck ? 'yes' : 'no'] / luckDays * 100).toFixed(2)
}

function close() {
  state.show = false
  // state.foods = []
}

function addFood() {
  state.show = true
}

function foodCallback(foods) {
  state.foods = foods
  state.foods.forEach(item => item.weight = 0)
}

function delFood(i) {
  state.foods.splice(i, 1)
}

const totalCalc = computed(() => {
  const carbon = state.foods.reduce((total, item) => {
    return Math.round((total + (item.val * item.weight / 100)) * 100) / 100;
  }, 0)
  const insulin = Math.round(carbon / state.curICR * 100) / 100
  return {
    carbon,
    insulin
  }
})
</script>
<style lang="scss" scoped>
.info-card {
  width: 350px;
  margin: 10px;

  &.lg {
    width: 720px;
  }

  :deep(.el-card__header) {
    padding: 8px;
    font-size: 14px;
  }

  :deep(.el-card__body) {
    padding: 8px;
  }

  :deep(.el-card__footer) {
    padding: 8px;
  }

  :deep(.el-input-number--small) {
    width: 90px;
  }
}

.calc-panel {
  min-height: 100px;
  max-height: 200px;
  overflow: auto;
}

.food-item {
  border-bottom: 1px solid #e3e2e2;
}
</style>
