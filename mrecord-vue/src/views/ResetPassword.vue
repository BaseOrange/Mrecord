<script setup lang="ts">
import {ref, computed} from 'vue'
import {useRouter, useRoute} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {resetPassword} from '@/api'
import {md5} from 'js-md5'
import AuthLayout from '@/components/AuthLayout.vue'

const router = useRouter()
const route = useRoute()

const token = computed(() => (route.query.token as string) || '')
const invalidToken = computed(() => !token.value)

const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const showPassword = ref(false)
const showConfirm = ref(false)
const success = ref(false)

const onSubmit = async () => {
  if (!password.value) {
    Snackbar.warning('请输入新密码')
    return
  }
  if (password.value.length < 6) {
    Snackbar.warning('密码至少6位')
    return
  }
  if (password.value !== confirmPassword.value) {
    Snackbar.warning('两次密码不一致')
    return
  }

  loading.value = true
  try {
    await resetPassword({password: md5(password.value), rePasswordToken: token.value})
    success.value = true
  } catch (e: any) {
    Snackbar.error(e?.message || '重置失败，链接可能已过期')
  } finally {
    loading.value = false
  }
}

const goLogin = () => {
  router.replace('/login')
}
</script>

<template>
  <AuthLayout>
    <!-- Token 无效 -->
    <div v-if="invalidToken" class="auth-card auth-status-card">
      <div class="auth-status-icon">
        <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
          <circle cx="32" cy="32" r="30" stroke="#e74c3c" stroke-width="2.5" opacity="0.15" />
          <circle cx="32" cy="32" r="24" fill="#e74c3c" opacity="0.08" />
          <line x1="22" y1="22" x2="42" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
          <line x1="42" y1="22" x2="22" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
        </svg>
      </div>
      <h3 class="auth-status-title">链接无效</h3>
      <p class="auth-status-desc">密码重置链接无效或已过期，请重新申请。</p>
      <button class="auth-submit-btn" @click="goLogin">返回登录</button>
    </div>

    <!-- 重置密码表单 / 成功 -->
    <Transition v-else name="auth-fade" mode="out-in">
      <div v-if="!success" key="form" class="auth-card" @keydown.enter="onSubmit">
        <h3 class="auth-card-title">重置密码</h3>
        <p class="auth-card-desc">请设置您的新密码。</p>

        <!-- 新密码 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="11" width="18" height="11" rx="2" />
              <path d="M7 11V7a5 5 0 0110 0v4" />
              <circle cx="12" cy="16.5" r="1.5" fill="currentColor" stroke="none" />
            </svg>
            <input
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="新密码（至少6位）"
              class="auth-input"
              autocomplete="new-password"
            />
            <button class="auth-eye-btn" @click="showPassword = !showPassword" type="button">
              <svg v-if="!showPassword" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="20" height="20">
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

        <!-- 确认密码 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M9 12l2 2 4-4" />
              <rect x="3" y="11" width="18" height="11" rx="2" />
              <path d="M7 11V7a5 5 0 0110 0v4" />
            </svg>
            <input
              v-model="confirmPassword"
              :type="showConfirm ? 'text' : 'password'"
              placeholder="确认新密码"
              class="auth-input"
              autocomplete="new-password"
            />
            <button class="auth-eye-btn" @click="showConfirm = !showConfirm" type="button">
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

        <!-- 提交按钮 -->
        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': loading }"
          :disabled="loading"
          @click="onSubmit"
        >
          <span v-if="!loading">确认重置</span>
          <span v-else class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>
      </div>

      <!-- 重置成功 -->
      <div v-else key="success" class="auth-card auth-success-card">
        <div class="auth-success-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
            <path d="M22 33l7 7 13-14" stroke="#FF6500" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <h3 class="auth-success-title">密码已重置</h3>
        <p class="auth-success-desc">您的密码已成功修改，请使用新密码登录。</p>
        <button class="auth-submit-btn" @click="goLogin">前往登录</button>
      </div>
    </Transition>
  </AuthLayout>
</template>
