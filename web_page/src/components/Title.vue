<template>
  <div class="title">
    <div class="title_title flex justify-between">
      <span>{{ title }}</span>
      <ep-Back v-if="back" class="hand text-lg" @click="backPage"></ep-Back>
    </div>
    <div v-if="subTitle" class="title_sub-title">{{ subTitle }}</div>
  </div>
</template>

<script>

export default {
  name: 'Title',
  props: {
    title: {
      type: String,
      default: ''
    },
    subTitle: {
      type: String,
      default: ''
    },
    back: {
      type: Boolean,
      default: false
    }
  },
  methods: {
    backPage() {
      this.$router.back();
    }
  }
}
</script>

<style lang="scss" scoped>
@import "../styles/variables.module";

.title {
  padding-bottom: 15px;
}

.title_title {
  padding: 0 10px;
  color: $titleColor;
  font-size: 14px;
  font-weight: bold;
  line-height: 20px;
  border-left: 4px solid $baseColor;
}

.title_sub-title {
  padding-top: 10px;
  padding-left: 14px;
  color: $placeholderColor;
  font-size: 12px;
}
</style>
