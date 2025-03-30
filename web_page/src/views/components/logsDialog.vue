<template>
  <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
             :show-close="false" width="80%" @close="close">
    <div class="flex flex-col h-100">
      <div class="w-full h-full flex-1 logs-panel">
        <el-alert v-for="(log) in logs" :closable="false"
                  :description="dayjs(log.time).format('MM-DD HH:mm:ss')"
                  :title="log.content"
                  :type="log.type"
                  class="item" show-icon/>
      </div>
      <div class="text-center w-full mt-4">
        <el-button :icon="Delete" type="warning" @click="clearLog">
          清除
        </el-button>
        <el-button :icon="Close" type="primary" @click="close">
          关闭
        </el-button>
      </div>
    </div>
  </el-dialog>
</template>
<script lang="ts" name="logsDialog" setup>
import {Close, Delete} from "@element-plus/icons-vue";
import dayjs from "dayjs";
import {Log} from "@/model/classes/Carelink";

const props = defineProps({
  show: {
    default: false,
    type: Boolean
  },
  logs: {
    default: [],
    type: Array<Log>
  }
})

const emit = defineEmits(["update:show"])
onMounted(() => {
})

function clearLog() {
  props.logs.splice(0, props.logs.length);
}

function close() {
  emit("update:show", false);
}
</script>

<style lang="scss" scoped>
.logs-panel {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 0 10px;
}

.item {
  margin: 0 10px 10px 0;

  :deep(.el-alert__icon) {
    font-size: 18px;
  }

  :deep(.el-alert__title) {
    font-size: 13px !important;
  }

  :deep(.el-alert__description) {
    font-size: 12px !important;
  }
}
</style>
