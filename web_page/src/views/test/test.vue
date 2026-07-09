<template>
  <MainPanel>
    <el-button type="primary" @click="testApi">test api limit</el-button>
    <el-button type="primary" @click="test">test</el-button>
    <el-button type="primary" @click="getUser">getUser</el-button>
    <el-button type="primary" @click="getUserById">getUserById</el-button>
    <el-button type="primary" @click="getUserByIdFromUser">getUserByIdFromUser</el-button>
    <el-button type="primary" @click="getInfoFromReids">getInfoFromReids</el-button>
    <el-button type="primary" @click="getInfoHashFromRedis">getInfoHashFromRedis</el-button>
    <el-button type="primary" @click="set_json_handler">set_json_handler</el-button>
    <el-button type="primary" @click="set_hash_handler">set_hash_handler</el-button>
    <el-button type="primary" @click="del_hash_handler">del_hash_handler</el-button>
    <el-button type="primary" @click="createUser">createUser</el-button>
    <el-button type="primary" @click="login">login</el-button>
    <el-button class="mt-2" type="primary" @click="get_user_setting">get_user_setting</el-button>
    <el-button class="mt-2" type="primary" @click="loadData">loadData</el-button>
    <el-button class="mt-2" type="primary" @click="loadTask">loadTask</el-button>
    <br><br>
    <el-table :data="taskList">
      <el-table-column label="JobId" prop="job_id"></el-table-column>
      <el-table-column label="任务名称" prop="name"></el-table-column>
      <el-table-column label="计划" prop="schedule_type">
        <template #default="{row}">
          {{
            row.schedule_type.Cron ? row.schedule_type.Cron : (row.schedule_type.Repeated ? row.schedule_type.Repeated : '')
          }}
        </template>
      </el-table-column>
      <el-table-column label="状态" prop="status"></el-table-column>

      <el-table-column label="操作" prop="status">
        <template #default="{row}">
          <el-button type="primary" @click="stop(row)">停止</el-button>
          <el-button type="primary" @click="start(row)">启动</el-button>
        </template>
      </el-table-column>
    </el-table>
  </MainPanel>
</template>
<script lang="ts" name="Text" setup>
import MainPanel from "@/layout/components/MainPanel.vue";
import {TestService} from "@/service/test-service";
import {Msg, Tools} from "@/utils/tools";
import CryptoJS from "crypto-js";
import {SugarService} from "@/service/sugar-service";

const service = new TestService()
const sugarService = new SugarService()
const taskList = ref([])

onMounted(() => {
  loadTask()
})
async function testApi() {
  const result = await service.testHelloWorld()
  console.log(result);
}

function test() {
  Msg.successMsg("ok")
  console.log(CryptoJS.HmacSHA1(JSON.stringify({a: 1}), 'key').toString());
}

async function getUser() {
  const result = await service.getUser(0)
  console.log(result);
}

async function getUserById() {
  const result = await service.getUserById({id: 13, name: "test", age: 18});
  console.log(result);
}

async function getUserByIdFromUser() {
  const result = await service.getUserByIdFromUser({id: 13, name: "test", age: 18});
  console.log(result);
}

async function createUser() {
  const result = await service.createUser({id: 13, age: 18});
  console.log(result);
}

async function login() {
  const result = await service.login({name: "alex", password: "205DED2984E1DAF6E3B9373767E8BF6E"});
  if (result) {
    Tools.setUser({
      ...result.user,
      token: `Bearer ${result.token}`,
    })
    console.log(result);
  }
}

async function getInfoFromReids() {
  const result = await service.getInfoFromRedis();
  console.log(result);
}

async function getInfoHashFromRedis() {
  const result = await service.getInfoHashFromRedis();
  console.log(result);
}

async function set_json_handler() {
  const result = await service.set_json_handler({
    "yes": 519,
    "no": 90,
    "ICR": {
      "morning": 5,
      "afternoon": 8.6,
      "evening": 12.5
    },
    "update_time": "2025-10-17 00:00:00"
  });
  console.log(result);
}

async function set_hash_handler() {
  const result = await service.set_hash_handler({
    key: "test1",
    value: "11",
  });
  console.log(result);
}

async function del_hash_handler() {
  const result = await service.del_hash_handler({
    key: "test1",
  });
  console.log(result);
}

async function get_user_setting() {
  const result = await service.getUserSetting();
  console.log(result);
}

async function loadData() {
  const result = await sugarService.loadData();
  console.log(result);
}

async function loadTask() {
  const result = await service.loadTask();
  console.log(result);
  if (result) {
    taskList.value = result
  }
}

async function stop(row) {
  const result = await service.stopTask(row.job_id);
  loadTask()
}

async function start(row) {
  const result = await service.startTask(row.job_id);
  loadTask()
}
</script>

<style lang="scss" scoped>

</style>
