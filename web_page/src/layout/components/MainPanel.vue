<template>
  <div class="container-main-body">
    <div :style="{position:relative?'relative':'initial'}" class="container-component">
      <div :class="[noTop ? 'no-top' : '', noPad?'no-pad':'', noHeader?'no-header':'']"
           :style="{marginLeft:!left?0:`${left}px`,backgroundColor:isLight?'white!important':''}"
           class="container-full">
        <div class="container-full__header">
          <slot name="header">
          </slot>
        </div>
        <div class="container-full__body">
          <slot/>
        </div>
        <div class="container-full__footer">
          <slot name="footer">
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  name: 'MainPanel',
  props: ['noTop', 'left', 'noPad', 'noHeader', 'isLight', 'relative'],
  data() {
    return {}
  },
  components: {},
  mounted() {
  },
  methods: {}
}
</script>
<style lang="scss" scoped>
@import "../../styles/layout";
</style>
