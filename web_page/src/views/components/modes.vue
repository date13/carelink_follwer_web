<template>
  <el-tag :type="modeObj.mode.type" class="" size="small">
    {{ modeObj.mode.name }}
    <span v-if="modeObj.mode.key===PUMP_STATUS.safe.key">
      ,闭环退出:{{ modeObj.timeRemaining }}
    </span>
    <span v-if="modeObj.mode.key===PUMP_STATUS.sport.key">
    剩余:{{ modeObj.timeRemaining }}
    </span>
    <span v-if="modeObj.mode.key===PUMP_STATUS.manuel.key">
      ,基础:{{ modeObj.basalRate }}
      <span v-if="modeObj.isTemp">
        ,剩余:{{ modeObj.timeRemaining }}
        </span>
    </span>
  </el-tag>
</template>
<script lang="ts" name="Device" setup>

import {PUMP_STATUS} from "@/views/const";

const props = defineProps({
  modeObj: {},
  type: Object
})

</script>

<style lang="scss" scoped>
</style>
