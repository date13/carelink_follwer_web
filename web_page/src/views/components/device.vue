<template>
  <el-tag class="mb-1 mr-1" size="small" type="primary">
    <div class="inline-block">
      泵:
      <div
        :class="{ 'text-red': !isNumber(data.reservoirRemainingUnits) || data.reservoirRemainingUnits <= lowerWarn.reservoirRemainingUnits }"
        class="inline-block">
        {{ data.reservoirRemainingUnits }}U
      </div>
      <div
        :class="{ 'text-red': !isNumber(data.medicalDeviceBatteryLevelPercent) || data.medicalDeviceBatteryLevelPercent <= lowerWarn.medicalDeviceBatteryLevelPercent }"
        class="inline-block ml-1">
        {{ data.medicalDeviceBatteryLevelPercent }}%&nbsp;
      </div>
    </div>
    <el-popover v-if="isNumber(data.sensorLastDatetime)" :content="`剩余:${data.sensorLastDatetime}小时`" placement="bottom"
      trigger="click">
      <template #reference>
        <div class="inline-block">
          探头:
          <div
            :class="{ 'text-red': !isNumber(data.sensorLastDatetime) || parseInt(data.sensorLastDatetime) <= lowerWarn.sensorLastDatetime }"
            class="inline-block">
            {{
              data.sensorLastDatetimeHumanize
            }}
          </div>
        </div>
      </template>
    </el-popover>
    <div class="inline-block" v-else>
      探头:
      <div class="inline-block text-red">
        {{
          data.sensorLastDatetimeHumanize
        }}
      </div>
    </div>
    <div
      :class="{ 'text-red': !isNumber(data.gstBatteryLevel) || parseInt(data.gstBatteryLevel) <= lowerWarn.gstBatteryLevel }"
      class="inline-block ml-1">
      {{
        data.gstBatteryLevel
      }}%
    </div>
  </el-tag>
</template>
<script lang="ts" name="Device" setup>

import { isNumber } from "mathjs";

const lowerWarn = {
  reservoirRemainingUnits: 25,
  medicalDeviceBatteryLevelPercent: 25,
  sensorLastDatetime: 5,
  gstBatteryLevel: 25
}
const props = defineProps({
  data: {
    default: {
      reservoirRemainingUnits: 0,
      medicalDeviceBatteryLevelPercent: 0,
      sensorLastDatetime: '--',
      sensorLastDatetimeHumanize: '--',
      gstBatteryLevel: '--'
    },
  },
  type: Object
})

</script>

<style lang="scss" scoped></style>
<script lang="ts" setup>
</script>