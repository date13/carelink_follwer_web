<template>
  <MainPanel :no-top="1">
    <template v-slot:header>
      <Title title="信息"></Title>
    </template>
    <div class="flex">
      <el-card class="info-card">
        <template #header>
          Note
        </template>
        <div class="flex ma-auto w-max">
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
      <el-card class="info-card text-center">
        <template #header>
          <div class="flex justify-between items-center">
            ICR
            <el-button type="primary" @click="updateICR">保存</el-button>
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
    </div>
    <div class="flex">
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
      <el-card class="info-card">
        <template #header>
          翻车记录({{ luckDays }} - {{ luck.yes + luck.no }})
        </template>
        <div class="flex justify-around">
          <div class="flex flex-col items-center content-center w-1/2">
            <el-button :icon="Smoking" circle color="red" size="large" @click="push(false)">
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
  </MainPanel>
</template>
<script lang="ts" name="myinfo" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import Title from "@/components/Title.vue"
import dayjs from 'dayjs'
import {Smoking, Sugar} from "@element-plus/icons-vue";
import {DictService} from "@/service/dict-service";
import {Msg} from "@/utils/tools";

const retireDays = dayjs('2030-10-29').diff(dayjs(), 'days')
const illDays = dayjs().diff(dayjs('2023-2-27'), 'days')
const luckDays = dayjs().diff(dayjs('2023-10-16'), 'days')

const service = new DictService()

const state = reactive({
  luck: {
    yes: 0,
    no: 0,
    ICR: {
      morning: 6.5,
      afternoon: 9,
      evening: 11.5
    }
  }
})

const {luck} = toRefs(state)

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
})

async function updateICR() {
  const result = await service.updateDict({
    key: 'luck',
    val: JSON.stringify(state.luck)
  })
  if (result) {
    Msg.successMsg('数据保存成功')
  }
}

function showRate(isLuck) {
  return (state.luck[isLuck ? 'yes' : 'no'] / luckDays * 100).toFixed(2)
}
</script>
<style lang="scss" scoped>
.info-card {
  width: 350px;
  margin: 10px;

  :deep(.el-card__header) {
    padding: 8px;
  }

  :deep(.el-card__body) {
    padding: 8px;
  }

  :deep(.el-input-number--small) {
    width: 90px;
  }
}
</style>
