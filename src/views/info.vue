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
      </el-card>
      <el-card class="info-card">
        <template #header>
          ICR
        </template>
        <el-tag
            class="m-1"
            effect="dark"
            type="warning"
        >
          早,1U={{ ICR.morning }}碳水
        </el-tag>
        <el-tag
            class="m-1"
            effect="dark"
            type="warning"
        >
          中,1U={{ ICR.afternoon }}碳水
        </el-tag>
        <el-tag
            class="m-1"
            effect="dark"
            type="warning"
        >
          晚,1U={{ ICR.evening }}碳水
        </el-tag>
      </el-card>
    </div>
    <div class="flex">
      <el-card class="info-card">
        <template #header>
          ISF
        </template>
        <el-tag
            class="m-1"
            effect="dark"
            type="info"
        >
          早,1U={{ toISF(ICR.morning) }}mmo/l
        </el-tag>
        <el-tag
            class="m-1"
            effect="dark"
            type="info"
        >
          中,1U={{ toISF(ICR.afternoon) }}mmo/l
        </el-tag>
        <el-tag
            class="m-1"
            effect="dark"
            type="info"
        >
          晚,1U={{ toISF(ICR.evening) }}mmo/l
        </el-tag>
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
            <div class="text-sm mt-2">{{ luck.noRate }}%</div>
          </div>
          <div class="flex flex-col items-center content-center w-1/2">
            <el-button :icon="Sugar" circle color="green" size="large" @click="push(true)">
              {{ luck.yes }}
            </el-button>
            <div class="text-sm mt-2">{{ luck.yesRate }}%</div>
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

const retireDays = dayjs('2030-10-29').diff(dayjs(), 'days')
const illDays = dayjs().diff(dayjs('2023-2-27'), 'days')
const luckDays = dayjs().diff(dayjs('2023-10-16'), 'days')

const service = new DictService()

const state = reactive({
  ICR: {
    morning: 6.5,
    afternoon: 9,
    evening: 11.5
  },
  luck: {
    id: '',
    yes: 0,
    no: 0,
    yesRate: '0',
    noRate: '0'
  }
})

const {ICR, luck} = toRefs(state)

function toISF(c) {
  return (c / 4).toFixed(2)
}

async function push(isLuck) {
  const params = {
    yes: state.luck.yes,
    no: state.luck.no
  }
  isLuck ? params.yes++ : params.no++
  const result = await service.updateDict({
    key: 'luck',
    val: JSON.stringify(params)
  })
  if (result) {
    isLuck ? state.luck.yes++ : state.luck.no++
    refreshLuck()
  }
}

onMounted(async () => {
  const result = await service.getDict("luck")
  if (result) {
    state.luck.id = result.id
    Object.assign(state.luck, JSON.parse(result.val))
    refreshLuck()
  }
})

function refreshLuck() {
  let day = luckDays
  state.luck.yesRate = (state.luck.yes / day * 100).toFixed(2)
  state.luck.noRate = (state.luck.no / day * 100).toFixed(2)
}
</script>
<style lang="scss" scoped>
.info-card {
  width: 350px;
  margin: 10px;
}
</style>
