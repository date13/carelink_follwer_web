<template>
  <el-dialog :close-on-press-escape="false" :destroy-on-close="true" :model-value="show" :show-close="false" width="80%"
             @close="close">
    <div class="flex flex-col h-100">
      <div class="ml-3 mb-1">基础率分布:</div>
      <div class="w-full h-full flex-1 basal-panel">
        <el-alert v-for="(value, key) in hourlyTotals" :closable="false" :description="value.toString()"
                  :title="`${key}:00 - ${key}:59`" class="item" show-icon type="info"/>
      </div>
      <div class="text-center w-full mt-4">
        <el-button :icon="Close" type="primary" @click="close">
          关闭
        </el-button>
      </div>
    </div>
  </el-dialog>
</template>
<script lang="ts" name="basalDialog" setup>
import {Close} from "@element-plus/icons-vue";
import useSugarCalc from "@/composition/useSugarCalc";

const props = defineProps({
  show: {
    default: false,
    type: Boolean
  },
  basals: {
    default: [],
    type: Object
  }
})

const emit = defineEmits(["update:show"])
const sugarCalc = useSugarCalc()
const hourlyTotals = ref({});
onMounted(() => {
  for (let i = 0; i < 24; i++) {
    hourlyTotals.value[i] = 0;
  }
  // 处理每个数据项
  props.basals.forEach(item => {
    const date = new Date(item.dateTime);
    const hour = date.getHours(); // 获取小时数 (0-23)

    // 累加bolusAmount到对应的小时
    hourlyTotals.value[hour] += item.bolusAmount;
  });

  // 处理浮点数精度问题（可选）
  for (const hour in hourlyTotals.value) {
    hourlyTotals.value[hour] = parseFloat(hourlyTotals.value[hour].toFixed(3));
  }

  // 输出结果
  console.log(hourlyTotals.value);

  // 如果需要格式化的输出
  const formattedResult = Object.entries(hourlyTotals.value).map(([hour, total]) => ({
    hour: `${hour}:00 - ${hour}:59`,
    totalBolusAmount: total
  }));
  console.log(formattedResult)
})

function close() {
  emit("update:show", false);
}
</script>

<style lang="scss" scoped>
.basal-panel {
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
    font-size: 14px !important;
  }
}
</style>
