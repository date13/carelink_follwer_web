<template>
  <MainPanel :no-top="1">
    <template v-slot:header>
      <Title :back="true" title="食物碳水"></Title>
    </template>
    <el-card class="food-card text-center w-full h-full">
      <template #header>
        <div class="flex justify-between">
          <el-input
              v-model="searchText"
              :prefix-icon="Search"
              class="w-full"
              filterable
              samll
              placeholder="输入搜索关键字"
          >
          </el-input>
          <div class="flex justify-between items-center px-2">
            <el-button :icon="CirclePlus" type="primary" @click="add">添加</el-button>
            <ep-Refresh class="ml-4 hand" @click="load"></ep-Refresh>
          </div>
        </div>
      </template>
      <el-scrollbar class="flex flex-col h-full">
        <div v-for="item in foodList" class="flex justify-between food-item mb-3 pb-2 text-sm">
          <div class="flex-1 flex justify-start pl-4">{{ item.key }}</div>
          <div class="flex-1">{{ item.val }}</div>
          <div class="flex flex-1 justify-end pr-4">
            <ep-Edit class="hand mr-2 text-blue" @click="edit(item)"></ep-Edit>
            <ep-Close class="hand text-red" @click="del(item)"></ep-Close>
          </div>
        </div>
      </el-scrollbar>
    </el-card>
    <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
               :show-close="false" title="更新食物" width="80%">
      <el-form ref="formRef" :model="curFood" :rules="rules" class="pa-2"
               label-position="left" label-width="50px">
        <el-form-item label="名称" prop="key">
          <el-input v-model="curFood.key" placeholder="请输入食物名称"></el-input>
        </el-form-item>
        <el-form-item label="碳水" prop="val">
          <el-input v-model="curFood.val" placeholder="请输入每百克食物碳水"></el-input>
        </el-form-item>
      </el-form>
      <div class="text-center">
        <el-button :icon="Check" type="warning" @click="update">
          保存
        </el-button>
        <el-button :icon="Close" type="primary" @click="close">
          关闭
        </el-button>
      </div>
    </el-dialog>
  </MainPanel>
</template>
<script lang="ts" name="food" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import Title from "@/components/Title.vue"
import {Check, CirclePlus, Close, Search} from "@element-plus/icons-vue";
import {SugarService} from "@/service/sugar-service";
import {Msg, Tools} from "@/utils/tools";
import {ElForm} from "element-plus";
import {RegFunc} from "@/utils/validator";

const service = new SugarService()
const formRef = ref(ElForm);
const state: any = reactive({
  searchText: '',
  loading: false,
  list: [],
  show: false,
  curFood: {}
})

const {searchText, show, list, curFood} = toRefs(state)

const rules = {
  key: RegFunc.require('食物名称'),
  val: RegFunc.require('食物碳水'),
}
onMounted(async () => {
  load()
})

const foodList = computed(() => {
  return state.list.filter(item => item.key.indexOf(state.searchText) !== -1)
})

function edit(item) {
  Object.assign(state.curFood, item)
  state.show = true
}

function add() {
  state.show = true
}

async function load() {
  const result = await service.loadFood()
  if (result) {
    state.list = Tools.objToArrFlat(result)
  }
}

function update() {
  formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      const result = await service.updateFood(state.curFood)
      if (result) {
        Msg.successMsg('食物碳水保存成功')
        close()
        load()
      }
    }
  })
}

function del(item) {
  Msg.confirm(`是否确认删除食物:${item.key}?`, async () => {
    await service.delFood(item.key).then(() => {
      Msg.successMsg('删除成功')
      load()
    })
  })
}

function close() {
  state.show = false
  state.curFood = {}
}
</script>
<style lang="scss" scoped>
.food-card {
  .food-item {
    border-bottom: 1px solid #e3e2e2;
  }

  :deep(.el-card__header) {
    padding: 4px;
  }

  :deep(.el-card__body) {
    padding: 8px;
    height: calc(100% - 30px);
  }
}

</style>
