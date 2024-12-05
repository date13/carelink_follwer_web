<template>
  <el-dropdown class="menu-panel" placement="bottom-start" trigger="click" @command="handleMenu">
    <ep-Menu :class="{'text-white':!isHome}"></ep-Menu>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item v-if="isHome" command="big">Big</el-dropdown-item>
        <el-dropdown-item v-else command="">Home</el-dropdown-item>
        <el-dropdown-item command="info">Info</el-dropdown-item>
        <el-dropdown-item command="dict">Dict</el-dropdown-item>
        <el-dropdown-item command="food">Food</el-dropdown-item>
        <el-dropdown-item command="login">Login</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>
<script lang="ts" name="Menus" setup>
const emit = defineEmits(['handler'])
const props = defineProps({
  isHome: {
    default: true,
    type: Boolean
  }
})

function handleMenu(command) {
  emit('handler', command)
}
</script>

<style lang="scss" scoped>
.menu-panel {
  position: absolute;
  right: 5px;
  z-index: 999;

  svg {
    width: 30px;
    height: 30px;
  }
}
</style>
