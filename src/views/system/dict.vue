<template>
  <MainPanel :no-top="1">
    <template v-slot:header>
      <Title title="字典"></Title>
    </template>
    <div class="flex">
      <el-card class="w-full ma-6">
        <template #header>
          <div class="flex justify-between">
            <span>字典</span>
            <el-button :icon="Refresh" type="primary" @click="loadDict">刷新</el-button>
          </div>
        </template>
        <el-form ref="formRef" :model="params" :rules="rules" label-position="left"
                 label-width="80px">
          <el-form-item label="KEY" prop="key">
            <el-select v-model="params.key" size="small" @change="loadDict">
              <el-option v-for="(item,i) in keys" :key="i"
                         :value="item">
              </el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="value" prop="value">
            <vue-jsoneditor v-if="load"
                            v-model:json="params.value"
                            height="400"
                            mode="tree"
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
import {Msg} from "@/utils/tools";
import VueJsoneditor from 'vue3-ts-jsoneditor';
import {Refresh} from "@element-plus/icons-vue";

const service = new DictService()
const keys = ['carelinkAuth', 'carelinkData']
const state = reactive({
  params: {
    key: keys[0],
    value: {}
  },
  load: false
})

const formRef = ref(ElForm);
const {params, load} = toRefs(state)
const rules = {
  key: RegFunc.require('字典Key'),
  value: RegFunc.require('字典值'),
}


onMounted(async () => {
  loadDict()
})

async function loadDict() {
  const result = await service.getDict(state.params.key)
  state.load = true
  if (result) {
    try {
      state.params.value = JSON.parse(result.val)
    } catch (e) {
      console.log(e);
    }
  } else {
    state.params.value = {}
  }
}

function update() {
  formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      const result = await service.updateDict({
        key: state.params.key,
        val: JSON.stringify(state.params.value)
      })
      if (result) {
        Msg.success('字典保存成功')
      }
    }
  })
}
</script>
<style lang="scss" scoped>
</style>
