<script setup lang="ts">
import {useRouter} from 'vue-router'

const router = useRouter()

const goHome = () => {
  router.replace('/')
}
</script>

<template>
  <div class="not-found">
    <var-icon name="warning" :size="64" color="#999"/>
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
  background: #f5f5f5;
}

.not-found h2 {
  margin: 16px 0 8px;
  font-size: 20px;
  color: #333;
}

.not-found p {
  color: #999;
  font-size: 14px;
  margin-bottom: 32px;
}

.back-btn {
  width: 100%;
  max-width: 280px;
}
</style>
