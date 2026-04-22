<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'

const router = useRouter()

const username = ref('')
const password = ref('')

const onLogin = () => {
  if (!username.value || !password.value) {
    // @ts-ignore
    Snackbar.warning('请输入用户名和密码')
    return
  }
  // @ts-ignore
  Snackbar.success('登录成功')
  router.replace('/')
}

const goBack = () => {
  router.back()
}
</script>

<template>
  <div class="login-page">
    <var-app-bar title="登录" @click-left="goBack">
      <template #left>
        <var-button round text color="transparent" text-color="#fff">
          <var-icon name="chevron-left" :size="24"/>
        </var-button>
      </template>
    </var-app-bar>

    <div class="content">
      <h2 class="title">账号登录</h2>

      <var-space direction="column" :size="[20, 20]" class="form">
        <var-input
            v-model="username"
            placeholder="请输入用户名"
            variant="outlined"
            :rules="[(v: string) => !!v || '用户名不能为空']"
        >
          <template #prepend-icon>
            <var-icon name="account-circle"/>
          </template>
        </var-input>

        <var-input
            v-model="password"
            type="password"
            placeholder="请输入密码"
            variant="outlined"
            :rules="[(v: string) => !!v || '密码不能为空']"
        >
          <template #prepend-icon>
            <var-icon name="lock"/>
          </template>
        </var-input>

        <var-button type="primary" block size="large" @click="onLogin">
          登录
        </var-button>
      </var-space>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  min-height: 100vh;
  background: #fff;
}

.content {
  padding: 40px 24px;
}

.title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 32px;
  color: #333;
}

.form {
  width: 100%;
}
</style>
