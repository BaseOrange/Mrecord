<script setup lang="ts">
import {useRouter} from 'vue-router'

const router = useRouter()

const goLogin = () => {
  router.push('/login')
}
</script>

<template>
  <div class="home-page">
    <var-app-bar title="月衡 Mrecord"/>

    <div class="content">
      <var-icon name="home" :size="48" color="var(--color-primary)"/>
      <h2>欢迎使用 月衡 Mrecord</h2>
      <p class="desc">这是一个移动端应用首页</p>

      <var-space direction="column" :size="[16, 16]" class="actions">
        <var-button type="primary" block @click="goLogin">
          去登录页
        </var-button>
      </var-space>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  min-height: 100vh;
  background: #f5f5f5;
}

.content {
  padding: 32px 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.content h2 {
  margin: 16px 0 8px;
  font-size: 20px;
  color: #333;
}

.desc {
  color: #999;
  font-size: 14px;
  margin-bottom: 32px;
}

.actions {
  width: 100%;
}
</style>
