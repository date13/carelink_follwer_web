<template>
  <el-popover
      :content="`管路启用:${dayjs(lastUpdateTime.conduitDatetime).format(DATE_FORMAT.datetime2)}`"
      placement="bottom"
  >
    <template #reference>
      <div class="flex">
        <el-tag class="hand mr-2" size="small" type="warning" @click="updateConduitDatetime">管路:
          {{ lastUpdateTime.conduit }}
        </el-tag>

        <ep-Setting class="hand" @click="openConduitDate"></ep-Setting>
        <el-date-picker
            ref="datetimeRef"
            v-model="dateTime"
            :default-time="defaultTime"
            :default-value="defaultTime"
            :show-now="false"
            class="conduit-datetime "
            placeholder="选择日期时间"
            type="datetime"
            @change="submitConduitTime"
        >
        </el-date-picker>
      </div>
    </template>
  </el-popover>
</template>
<script lang="ts" name="Conduit" setup>
import dayjs from "dayjs";
import {DATE_FORMAT} from "@/model/model-type";
import {ElDatePicker} from "element-plus";

const emit = defineEmits(['updateConduitDatetime'])
const props = defineProps({
  lastUpdateTime: {
    default: {},
  },
  type: Object
})
const dateTime = ref('')
const datetimeRef = ref(ElDatePicker)
const defaultTime = ref(new Date())
const getParams = () => {
  return {
    startTime: dayjs(props.lastUpdateTime.conduitDatetime).format(DATE_FORMAT.datetime2)
  }
}


function updateConduitDatetime() {
  emit('updateConduitDatetime', getParams())
}

function openConduitDate() {
  datetimeRef.value.focus()
  defaultTime.value = new Date()
}

function submitConduitTime() {
  emit('updateConduitDatetime', {
    datetime: dateTime.value,
    ...getParams()
  })
  datetimeRef.value.blur()
}
</script>

<style lang="scss">
.conduit-datetime {
  width: 0;
  height: 0;
  position: fixed;
  bottom: 0;
  z-index: -1;
}
</style>
