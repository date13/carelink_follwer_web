<template>
  <MainPanel :no-top="1">
    <template v-if="!isDialog" v-slot:header>
      <Title :back="true" title="食物碳水"></Title>
    </template>
    <el-card class="food-card text-center w-full h-full">
      <template #header>
        <div class="flex justify-between">
          <el-input
              v-model="searchText"
              :prefix-icon="Search"
              class="w-full"
              clearable
              size="small"
              placeholder="输入搜索关键字"
          >
          </el-input>
          <div class="flex justify-between items-center px-2">
            <el-button :icon="CirclePlus" size="small" type="primary" @click="add">添加</el-button>
            <ep-Refresh class="ml-4 hand" @click="load"></ep-Refresh>
          </div>
        </div>
      </template>
      <el-scrollbar class="flex flex-col h-full">
        <el-checkbox-group v-model="selectItems">
          <div v-for="(item,i) in foodList" :key="i" class="flex justify-between food-item mb-3 pb-2 text-sm">
            <div v-if="isDialog" class="w-10">
              <el-checkbox :value="item"/>
            </div>
            <div class="flex-1 flex justify-start pl-4 items-center hand"
                 @click="checkFood(item)">{{ item.key }}
            </div>
            <div class="flex-1 flex justify-center items-center hand"
                 @click="checkFood(item)">{{ item.val }}
            </div>
            <div class="flex flex-1 justify-end pr-4 items-center">
              <ep-Edit class="hand mr-2 text-blue" @click="edit(item,i)"></ep-Edit>
              <ep-Close class="hand text-red" @click="del(item)"></ep-Close>
            </div>
          </div>
        </el-checkbox-group>
      </el-scrollbar>
    </el-card>
    <template v-if="isDialog" v-slot:footer>
      <div class="flex justify-center items-center pa-2">
        <el-button :icon="Close" size="small" type="warning" @click="closeDialog">关闭</el-button>
        <el-button :icon="Check" size="small" type="primary" @click="selectFoods">确定</el-button>
      </div>
    </template>
    <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
               :show-close="false" title="更新食物" width="80%">
      <el-form ref="formRef" :model="curFood" :rules="rules" class="pa-2"
               label-position="left" label-width="50px">
        <el-form-item label="名称" prop="key">
          <el-input v-model="curFood.key" :disabled="curIndex!==-1" clearable placeholder="请输入食物名称"></el-input>
        </el-form-item>
        <el-form-item label="碳水" prop="val">
          <el-input-number v-model="curFood.val" :min="1" placeholder="请输入每百克食物碳水"
                           style="width: 100%;"></el-input-number>
        </el-form-item>
      </el-form>
      <div class="text-center">
        <el-button :icon="Close" size="small" type="warning" @click="close">
          关闭
        </el-button>
        <el-button :icon="Check" size="small" type="primary" @click="update">
          保存
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
import {cloneDeep} from "lodash-es";

const service = new SugarService()
const formRef = ref(ElForm);
const props = defineProps({
  isDialog: {
    default: false,
    type: Boolean
  },
  callback: {
    default: (foods) => {
    }
  },
  selected: {
    default: [],
    type: Array
  },
  closeDialog: {
    default: () => {
    }
  }
})

const state: any = reactive({
  searchText: '',
  loading: false,
  list: [],
  show: false,
  selectItems: [],
  curFood: {
    key: '',
    val: 10
  },
  curIndex: -1
})

const {searchText, show, selectItems, curFood, curIndex} = toRefs(state)

const rules = {
  key: RegFunc.require('食物名称'),
  val: RegFunc.require('食物碳水'),
}
onMounted(async () => {
  load()
  const selected = cloneDeep(props.selected)
  selected.forEach(item => delete item.weight)
  Object.assign(state.selectItems, selected)
})

const foodList = computed(() => {
  return state.list.filter(item => item.key.indexOf(state.searchText) !== -1)
})

function edit(item, i) {
  item.val = parseFloat(item.val)
  Object.assign(state.curFood, item)
  state.show = true
  state.curIndex = i
}

function add() {
  state.show = true
  state.curIndex = -1
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
      let list = []
      if (state.curIndex !== -1) {
        list = state.list.filter((item, i) => i !== state.curIndex)
      } else {
        list = state.list
      }
      state.curFood.key = state.curFood.key.trim()
      if (list.findIndex(item => item.key === state.curFood.key) !== -1) {
        Msg.warnMsg('已存在该名称，请重新输入')
        return
      }
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
  state.curFood = {
    key: '',
    val: 10
  }
}

function selectFoods() {
  state.selectItems.forEach(item => {
    const orgFood = props.selected.find(food => food.key === item.key)
    if (orgFood) {
      item.weight = orgFood.weight
    }
  })
  props.callback(state.selectItems)
  props.closeDialog()
}

function checkFood(food) {
  const index = state.selectItems.findIndex(item => item.key === food.key)
  if (index !== -1) {
    state.selectItems.splice(index, 1)
  } else {
    state.selectItems.push(food)
  }
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
