<template>
  <el-popover
      :content="`管路启用:${dayjs(lastUpdateTime.conduitDatetime).format(DATE_FORMAT.datetime2)}`"
      placement="bottom"
      trigger="hover"
  >
    <template #reference>
      <el-tag class="hand" size="small" type="warning" @click="updateConduitDatetime">管路:
        {{ lastUpdateTime.conduit }}
      </el-tag>
    </template>
  </el-popover>
</template>
<script lang="ts" name="Conduit" setup>
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";

const emit = defineEmits(['updateConduitDatetime'])
const props = defineProps({
  lastUpdateTime: {
    default: {},
  },
  type: Object
})

function updateConduitDatetime() {
  emit('updateConduitDatetime')
}
</script>

<style lang="scss" scoped>
</style>
