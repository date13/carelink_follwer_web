<template>
  <MainPanel>
    <el-button type="primary" @click="testApi">test api limit</el-button>
  </MainPanel>
</template>
<script lang="ts" name="Text" setup>
import MainPanel from "@/layout/components/MainPanel.vue";
import {TestService} from "@/service/test-service";

const service = new TestService()

function testApi() {
  const result = service.testHelloWorld()
  console.log(result);
}

</script>

<style lang="scss" scoped>

</style>
