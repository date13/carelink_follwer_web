<template>
  <div :class="isHome?'text-3xl':'text-6xl'" class="flex font-thin arrow">
    <template v-if="trendObj?.direction">
      <template v-if="trendObj.direction === 1">
        <ep-Top v-for="i in trendObj.num"></ep-Top>
      </template>
      <template v-if="trendObj.direction === 2">
        <ep-Top-Right></ep-Top-Right>
      </template>
      <template v-if="trendObj.direction === 3">
        <ep-Right></ep-Right>
      </template>
      <template v-if="trendObj.direction === 4">
        <ep-Bottom-Right></ep-Bottom-Right>
      </template>
      <template v-if="trendObj.direction === 5">
        <ep-Bottom v-for="i in trendObj.num"></ep-Bottom>
      </template>
    </template>
  </div>
</template>
<script lang="ts" name="Trend" setup>
const props = defineProps({
  isHome: {
    default: true,
    type: Boolean
  },
  trendObj: {
    default: {},
    type: Object
  }
})
</script>

<style lang="scss" scoped>
</style>
