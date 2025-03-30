<template>
  <MainPanel :no-top="1">
    <template v-slot:header>
      <Title :back="true" title="字典"></Title>
    </template>
    <div class="flex h-full">
      <el-card class="w-full dict-panel h-full">
        <template #header>
          <div class="flex justify-between">
            <span>字典</span>
            <el-button :icon="Refresh" type="primary" @click="loadDict">刷新</el-button>
          </div>
        </template>
        <el-form ref="formRef" :model="params" :rules="rules" class="dict-form"
                 label-position="left" label-width="80px">
          <el-form-item label="Key" prop="key">
            <div class="flex w-full justify-center items-center">
              <el-select v-model="params.key" class="flex-1" size="small" @change="loadDict">
                <el-option v-for="(item,i) in keys" :key="i"
                           :label="item.key" :value="item.key">
                </el-option>
              </el-select>
              <div v-if="curKeyObj.type" class="flex ml-2 flex-1  justify-center items-center">
                <span class="mr-2">SubKeys</span>
                <el-select v-model="params.subKey" size="small" @change="loadSubDict">
                  <el-option v-for="(item,i) in hashObj.keys" :key="i"
                             :label="item" :value="item">
                  </el-option>
                </el-select>
              </div>
            </div>
          </el-form-item>
          <el-form-item v-if="!curKeyObj.type" label="更新时间" prop="update_time">
            {{ params.value.update_time }}
          </el-form-item>
          <el-form-item class="json-editor-item" label="Value" prop="value">
            <vue-jsoneditor v-if="load"
                            v-model:json="params.value"
                            class="json-editor"
                            mode="tree"
                            @change="handleJsonChange"
            />
          </el-form-item>
        </el-form>
        <template #footer>
          <div class="w-full flex justify-center">
            <el-button type="primary" @click="update">更新</el-button>
          </div>
        </template>
      </el-card>
    </div>
  </MainPanel>
</template>
<script lang="ts" name="dict" setup>
import MainPanel from '@/layout/components/MainPanel.vue'
import Title from "@/components/Title.vue"
import {DictService} from "@/service/dict-service";
import {ElForm} from "element-plus";
import {RegFunc} from "@/utils/validator";
import {Msg, Tools} from "@/utils/tools";
import VueJsoneditor from 'vue3-ts-jsoneditor';
import {Refresh} from "@element-plus/icons-vue";
import dayjs from "dayjs";

const service = new DictService()
const keys = [
  {
    key: 'carelinkAuth',
    user: true
  }, {
    key: 'carelinkData',
    user: true
  },
  {
    key: "setting",
    isJson: true,
    user: true,
  },
  {
    key: 'carelinkMyData',
    user: true
  }, {
    key: "history",
    type: 'hash',
    isJson: true,
    user: true,
    sort: (a: any, b: any) => dayjs(b).diff(dayjs(a))
  }, {
    key: 'food',
    type: 'hash',
    isJson: false,
    user: true
  }, {
    key: "luck",
    user: true
  }, {
    key: "user",
    type: 'hash',
    isJson: true,
    user: false
  }]
const keysMap = Tools.arrToObj(keys, 'key')
const state: any = reactive({
  params: {
    key: keys[0].key,
    subKey: '',
    value: {}
  },
  hashObj: {
    keys: [],
    result: {}
  },
  curKeyObj: {},
  isHash: false,
  load: false
})

const formRef = ref(ElForm);
const {params, load, curKeyObj, hashObj} = toRefs(state)
const rules = {
  key: RegFunc.require('字典Key'),
  value: RegFunc.require('字典值'),
}


onMounted(async () => {
  loadDict()
})

async function loadDict() {
  state.curKeyObj = keysMap[state.params.key]
  const result = await service.getDict(state.params.key, true, state.curKeyObj)
  state.load = true
  if (result && result !== true) {
    if (!state.curKeyObj.type) {
      try {
        state.params.subKey = null
        state.params.value = JSON.parse(result)
      } catch (e) {
        console.log(e);
      }
    } else {
      state.hashObj.keys = Object.keys(result).sort(state.curKeyObj.sort ? state.curKeyObj.sort : (a, b) => a.localeCompare(b))
      state.hashObj.result = result
      state.params.subKey = state.hashObj.keys[0]
      loadSubDict(state.curKeyObj.isJson)
    }
  } else {
    state.params.value = {}
  }
}

function handleJsonChange(content: any) {
  if (content.text) {
    try {
      state.params.value = JSON.parse(content.text)
    } catch (e) {
      console.error('JSON 解析错误:', e)
    }
  } else {
    state.params.value = content.json
  }
}

function loadSubDict(isJson) {
  state.params.value = isJson ? JSON.parse(state.hashObj.result[state.params.subKey]) : state.hashObj.result[state.params.subKey]
}

function update() {
  formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      const result = await service.updateDict({
        key: state.params.key,
        subKey: state.params.subKey,
        val: (!state.curKeyObj.type || state.curKeyObj.isJson) ? JSON.stringify(state.params.value) : state.params.value
      }, state.curKeyObj)
      if (result) {
        Msg.success('字典保存成功')
      }
    }
  })
}
</script>
<style lang="scss" scoped>
.dict-panel {
  display: flex;
  flex-flow: column;

  :deep(.el-card__body) {
    overflow: auto;
    height: 100%;
  }

  .dict-form {
    height: 100%;
    display: flex;
    flex-flow: column;

    .json-editor-item {
      flex: 1;
      overflow: auto;

      :deep(.el-form-item__content) {
        height: 100%;
      }
    }

    :deep(.el-form-item) {
      margin-bottom: 8px;
    }

    .json-editor {
      height: calc(100%)
    }
  }

  :deep(.el-card__header) {
    padding: 8px;
  }

  :deep(.el-card__footer) {
    padding: 8px;
  }
}
</style>
