<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const transitionName = computed(() => {
  return (route.meta.transition as string) || 'fade'
})
</script>

<template>
  <router-view v-slot="{ Component }">
    <transition :name="transitionName" mode="out-in">
      <component :is="Component"/>
    </transition>
  </router-view>
</template>

<style>
/* 移动端基础过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 全局移动端样式 */
html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  -webkit-tap-highlight-color: transparent;
  -webkit-font-smoothing: antialiased;
}

#app {
  width: 100%;
  height: 100%;
  max-width: 100%;
  margin: 0 auto;
}
</style>
