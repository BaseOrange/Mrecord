<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {useUserStore} from '@/stores/user'
import {changePassword, logout} from '@/api'
import {md5} from 'js-md5'

const router = useRouter()
const userStore = useUserStore()

const oldPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const showOld = ref(false)
const showNew = ref(false)
const showConfirm = ref(false)

const onSubmit = async () => {
  if (!oldPassword.value) {
    Snackbar.warning('请输入旧密码')
    return
  }
  if (!newPassword.value) {
    Snackbar.warning('请输入新密码')
    return
  }
  if (newPassword.value.length < 6) {
    Snackbar.warning('新密码至少6位')
    return
  }
  if (newPassword.value !== confirmPassword.value) {
    Snackbar.warning('两次新密码不一致')
    return
  }
  if (oldPassword.value === newPassword.value) {
    Snackbar.warning('新密码不能与旧密码相同')
    return
  }

  loading.value = true
  try {
    await changePassword({
      oldPassword: md5(oldPassword.value),
      newPassword: md5(newPassword.value),
    })
    Snackbar.success('密码修改成功，请重新登录')
    try {
      await logout()
    } catch {
      // 忽略接口错误
    }
    userStore.logout()
    router.replace('/login')
  } catch (e: any) {
    Snackbar.error(e?.message || '修改失败，请检查旧密码是否正确')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="change-password-page">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="header-back" @click="router.back()">
        <var-icon name="chevron-left" :size="24" color="#333" />
      </button>
      <h2>修改密码</h2>
      <div class="header-placeholder"></div>
    </div>

    <div class="page-body" @keydown.enter="onSubmit">
      <div class="form-card">
        <div class="form-item">
          <label class="form-label">旧密码</label>
          <div class="input-wrapper">
            <input
              v-model="oldPassword"
              :type="showOld ? 'text' : 'password'"
              placeholder="请输入当前密码"
              class="form-input"
              autocomplete="current-password"
            />
            <button class="eye-btn" @click="showOld = !showOld" type="button">
              <svg v-if="!showOld" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8S1 12 1 12z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94" />
                <path d="M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19" />
                <line x1="1" y1="1" x2="23" y2="23" />
              </svg>
            </button>
          </div>
        </div>

        <div class="form-item">
          <label class="form-label">新密码</label>
          <div class="input-wrapper">
            <input
              v-model="newPassword"
              :type="showNew ? 'text' : 'password'"
              placeholder="新密码（至少6位）"
              class="form-input"
              autocomplete="new-password"
            />
            <button class="eye-btn" @click="showNew = !showNew" type="button">
              <svg v-if="!showNew" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8S1 12 1 12z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94" />
                <path d="M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19" />
                <line x1="1" y1="1" x2="23" y2="23" />
              </svg>
            </button>
          </div>
        </div>

        <div class="form-item">
          <label class="form-label">确认新密码</label>
          <div class="input-wrapper">
            <input
              v-model="confirmPassword"
              :type="showConfirm ? 'text' : 'password'"
              placeholder="再次输入新密码"
              class="form-input"
              autocomplete="new-password"
            />
            <button class="eye-btn" @click="showConfirm = !showConfirm" type="button">
              <svg v-if="!showConfirm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8S1 12 1 12z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
              <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
                <path d="M17.94 17.94A10.07 10.07 0 0112 20c-7 0-11-8-11-8a18.45 18.45 0 015.06-5.94" />
                <path d="M9.9 4.24A9.12 9.12 0 0112 4c7 0 11 8 11 8a18.5 18.5 0 01-2.16 3.19" />
                <line x1="1" y1="1" x2="23" y2="23" />
              </svg>
            </button>
          </div>
        </div>

        <button
          class="submit-btn"
          :class="{ 'submit-btn--loading': loading }"
          :disabled="loading"
          @click="onSubmit"
        >
          {{ loading ? '保存中...' : '确认修改' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.change-password-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: calc(16px + env(safe-area-inset-top, 0px)) 16px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.page-header h2 {
  font-size: 17px;
  font-weight: 600;
  color: #333;
}

.header-back {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 50%;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

.header-back:active {
  background: #f5f5f5;
}

.header-placeholder {
  width: 32px;
}

.page-body {
  padding: 16px;
}

.form-card {
  background: #fff;
  border-radius: 16px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.form-item {
  margin-bottom: 18px;
}

.form-item:last-of-type {
  margin-bottom: 24px;
}

.form-label {
  display: block;
  font-size: 13px;
  color: #666;
  font-weight: 500;
  margin-bottom: 8px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 12px;
  padding: 0 12px;
  height: 48px;
  transition: all 0.2s;
}

.input-wrapper:focus-within {
  border-color: #FF6500;
  background: #fff;
}

.form-input {
  flex: 1;
  height: 100%;
  font-size: 15px;
  color: #333;
  background: transparent;
  border: none;
  outline: none;
  letter-spacing: 0.5px;
}

.form-input::placeholder {
  color: #bbb;
}

.eye-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  color: #c0a080;
  cursor: pointer;
  transition: color 0.2s;
  background: none;
  border: none;
  padding: 0;
  flex-shrink: 0;
}

.eye-btn:active {
  color: #FF6500;
}

.submit-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: #FF6500;
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}

.submit-btn:active:not(:disabled) {
  background: #e05800;
  transform: scale(0.98);
}

.submit-btn--loading {
  opacity: 0.7;
}
</style>
