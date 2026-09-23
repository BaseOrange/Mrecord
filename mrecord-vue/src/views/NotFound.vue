<script setup lang="ts">
import {useRouter} from 'vue-router'

const router = useRouter()

const goHome = () => {
  router.replace('/')
}
</script>

<template>
  <div class="not-found">
    <AppIcon name="triangle-alert" :size="56" :stroke-width="1.4" class="not-found-icon" />
    <h2>页面不存在</h2>
    <p>您访问的页面已丢失</p>
    <var-button type="primary" block @click="goHome" class="back-btn">
      返回首页
    </var-button>
  </div>
</template>

<style scoped>
.not-found {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 0 32px;
  background: var(--bg-canvas);
}

.not-found h2 {
  margin: 16px 0 8px;
  font-size: 20px;
  color: var(--text-primary);
}

.not-found p {
  color: var(--text-tertiary);
  font-size: 14px;
  margin-bottom: 32px;
}

.back-btn {
  width: 100%;
  max-width: 280px;
}

.not-found-icon {
  color: var(--text-tertiary);
  margin-bottom: 24px;
}
</style>
