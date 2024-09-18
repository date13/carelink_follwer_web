<template>
  <el-config-provider :locale="locale">
    <router-view/>
  </el-config-provider>
</template>
<script lang="ts" setup>
import useStore from "@/store";

// 导入 Element Plus 语言包
import zhCn from "element-plus/es/locale/lang/zh-cn";
import en from "element-plus/es/locale/lang/en";

const {global} = useStore();

const language = computed(() => global.language);
const locale = ref();

const isRouterAlive = ref(true)
watch(
    language,
    (value) => {
      if (value == "en") {
        locale.value = en;
      } else {
        locale.value = zhCn;
      }
    },
    {
      // 初始化立即执行
      immediate: true
    }
);


function reload() {
  isRouterAlive.value = false
  nextTick(() => {
    isRouterAlive.value = true
  })
}

provide('reload', reload)
</script>
<style>
</style>
