<template>
  <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
             :show-close="false" width="80%" @close="close">
    <div class="flex flex-col h-100">
      <div v-if="notificationHistory.activeNotifications.length>0" class="mx-3 mb-2">
        <div class="mb-1">当前警报:</div>
        <el-alert v-for="{messageId,sg,triggeredDateTime} in notificationHistory.activeNotifications.sort((a:any,b:any)=>{
          return sugarCalc.cleanTime(b.triggeredDateTime) - sugarCalc.cleanTime(a.triggeredDateTime)
        })" :closable="false"
                  :description="dayjs(sugarCalc.cleanTime(triggeredDateTime)).format(DATE_FORMAT.datetime2)"
                  :title="NOTIFICATION_MAP[messageId]?sugarCalc.showNotificationMsg(messageId,sg):messageId"
                  :type="NOTIFICATION_MAP[messageId]?NOTIFICATION_MAP[messageId].type:'warning'"
                  class="item" show-icon/>
      </div>

      <div class="ml-3 mb-1">历史警报:</div>
      <div class="w-full h-full flex-1 notification-panel">
        <el-alert v-for="{messageId,sg,triggeredDateTime} in notificationHistory.clearedNotifications.sort((a:any,b:any)=>{
          return sugarCalc.cleanTime(b.triggeredDateTime) - sugarCalc.cleanTime(a.triggeredDateTime)
        })" :closable="false"
                  :description="dayjs(sugarCalc.cleanTime(triggeredDateTime)).format(DATE_FORMAT.datetime2)"
                  :title="NOTIFICATION_MAP[messageId]?sugarCalc.showNotificationMsg(messageId,sg):messageId"
                  :type="NOTIFICATION_MAP[messageId]?NOTIFICATION_MAP[messageId].type:'warning'"
                  class="item" show-icon/>
      </div>
      <div class="text-center w-full mt-4">
        <el-button :icon="Close" type="primary" @click="close">
          关闭
        </el-button>
      </div>
    </div>
  </el-dialog>
</template>
<script lang="ts" name="notificationDialog" setup>
import {Close} from "@element-plus/icons-vue";
import {NOTIFICATION_MAP} from "@/views/const";
import useSugarCalc from "@/composition/useSugarCalc";
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";

const props = defineProps({
  show: {
    default: false,
    type: Boolean
  },
  notificationHistory: {
    default: {
      clearedNotifications: [],
      activeNotifications: []
    },
    type: Object
  }
})

const emit = defineEmits(["update:show"])
const sugarCalc = useSugarCalc()
onMounted(() => {
})

function close() {
  emit("update:show", false);
}
</script>

<style lang="scss" scoped>
.notification-panel {
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
