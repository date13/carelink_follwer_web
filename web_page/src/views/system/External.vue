<template>
  <MainPanel>
    <iframe :src="path" frameborder="0" height="100%" width="100%"></iframe>
  </MainPanel>
</template>
<script lang="ts" name="External" setup>
import {useRoute} from "vue-router";
import {Base64} from "@/utils/tools";
import MainPanel from "@/layout/components/MainPanel.vue";

const path = ref('')
onBeforeMount(() => {
  const route = useRoute()
  const pathQuery = route.query.path
  if (pathQuery) {
    path.value = Base64.decode(pathQuery)
  }
})

</script>

<style lang="scss" scoped>
.iframe-div {
  height: 100%;
}
</style>
