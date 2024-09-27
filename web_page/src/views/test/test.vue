<template>
  <MainPanel>
    <el-button type="primary" @click="testApi">test api limit</el-button>
    <el-button type="primary" @click="test">test</el-button>
  </MainPanel>
</template>
<script lang="ts" name="Text" setup>
import MainPanel from "@/layout/components/MainPanel.vue";
import {TestService} from "@/service/test-service";
import {Msg} from "@/utils/tools";
import CryptoJS from "crypto-js";

const service = new TestService()

function testApi() {
  const result = service.testHelloWorld()
  console.log(result);
}

function test() {
  Msg.successMsg("ok")
  console.log(CryptoJS.HmacSHA1(JSON.stringify({a: 1}), 'key').toString());
}
</script>

<style lang="scss" scoped>

</style>
