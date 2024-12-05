<template>
  <el-tag class="mb-1 mr-1" size="small" type="primary">
    泵:
    {{ data.reservoirRemainingUnits }}U
    {{ data.medicalDeviceBatteryLevelPercent }}%&nbsp;
    <el-popover
        :content="`剩余:${data.sensorLastDatetime}`"
        placement="bottom"
        trigger="click"
    >
      <template #reference>
        探头:
        {{
          data.sensorLastDatetimeHumanize
        }}
        {{
          data.gstBatteryLevel
        }}%
      </template>
    </el-popover>
  </el-tag>
</template>
<script lang="ts" name="Device" setup>

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

<style lang="scss" scoped>
</style>
