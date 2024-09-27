<template>
  <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show"
             :show-close="false" title="警告列表" width="60%" @close="close">
    <div class="flex flex-col h-100">
      <div class="w-full h-full flex-1 notification-panel">
        <el-alert v-for="{messageId,sg,triggeredDateTime} in list.sort((a:any,b:any)=>{
          return sugarCalc.cleanTime(b.triggeredDateTime) - sugarCalc.cleanTime(a.triggeredDateTime)
        })" :closable="false"
                  :description="dayjs(sugarCalc.cleanTime(triggeredDateTime)).format(DATE_FORMAT.datetime)"
                  :title="NOTIFICATION_MAP[messageId]?showMsg(messageId,sg):messageId"
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
import {DATE_FORMAT} from "@/model/model-type";
import dayjs from "dayjs";

const props = defineProps({
  show: {
    default: false,
    type: Boolean
  },
  list: {
    default: []
  }
})

const emit = defineEmits(["update:show"])
const sugarCalc = useSugarCalc()
onMounted(() => {
})

function showMsg(messageId, sg) {
  const item = NOTIFICATION_MAP[messageId]
  if (sg) return item.text.replace(item.replace, sugarCalc.calcSG(sg))
  return item.text
}

function close() {
  emit("update:show", false);
}
</script>

<style lang="scss" scoped>
.notification-panel {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 0 10px;

  .item {
    margin: 0 10px 10px 0;
  }
}
</style>
