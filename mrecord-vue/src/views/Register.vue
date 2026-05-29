<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {register} from '@/api'
import {md5} from 'js-md5'
import {checkPasswordStrength} from '@/utils/security'
import AuthLayout from '@/components/AuthLayout.vue'

const router = useRouter()

const email = ref('')
const nickname = ref('')
const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const showPassword = ref(false)
const showConfirm = ref(false)
const registered = ref(false)

const passwordStrength = computed(() => {
  if (!password.value) return 0
  return checkPasswordStrength(password.value)
})

const strengthLabel = computed(() => {
  const labels = ['', '弱', '较弱', '中等', '强', '很强']
  return labels[passwordStrength.value] || ''
})

const strengthColor = computed(() => {
  const colors = ['', '#ff4444', '#ff8800', '#ffaa00', '#44bb44', '#22aa22']
  return colors[passwordStrength.value] || ''
})

const onRegister = async () => {
  if (!email.value) {
    Snackbar.warning('请输入邮箱')
    return
  }
  if (!nickname.value) {
    Snackbar.warning('请输入昵称')
    return
  }
  if (!password.value) {
    Snackbar.warning('请输入密码')
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
    await register({
      email: email.value,
      password: md5(password.value),
      nickname: nickname.value,
    })
    registered.value = true
  } catch {
    // 拦截器已处理错误提示
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
    <Transition name="auth-fade" mode="out-in">
      <!-- 注册表单 -->
      <div v-if="!registered" key="form" class="auth-card" @keydown.enter="onRegister">
        <!-- 邮箱 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="4" width="20" height="16" rx="3" />
              <path d="M2 7l10 6 10-6" />
            </svg>
            <input
              v-model="email"
              type="email"
              placeholder="邮箱（用于登录和激活）"
              class="auth-input"
              autocomplete="email"
            />
          </div>
        </div>

        <!-- 昵称 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="8" r="4" />
              <path d="M20 21a8 8 0 10-16 0" />
            </svg>
            <input
              v-model="nickname"
              type="text"
              placeholder="昵称"
              class="auth-input"
              autocomplete="nickname"
            />
          </div>
        </div>

        <!-- 密码 -->
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
              placeholder="密码（至少6位）"
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
          <!-- 密码强度条 -->
          <div v-if="password" class="strength-bar">
            <div class="strength-track">
              <div
                v-for="i in 5"
                :key="i"
                class="strength-segment"
                :class="{ active: i <= passwordStrength }"
                :style="{ background: i <= passwordStrength ? strengthColor : '' }"
              />
            </div>
            <span class="strength-label" :style="{ color: strengthColor }">{{ strengthLabel }}</span>
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
              placeholder="确认密码"
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

        <!-- 注册按钮 -->
        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': loading }"
          :disabled="loading"
          @click="onRegister"
        >
          <span v-if="!loading">注册</span>
          <span v-else class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>

        <!-- 底部链接 -->
        <div class="auth-links">
          <span class="auth-link-hint">已有账户？</span>
          <a class="auth-link" @click="goLogin">去登录</a>
        </div>
      </div>

      <!-- 注册成功提示 -->
      <div v-else key="success" class="auth-card auth-success-card">
        <div class="auth-success-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
            <path d="M22 33l7 7 13-14" stroke="#FF6500" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <h3 class="auth-success-title">注册成功！</h3>
        <p class="auth-success-desc">
          我们已向 <strong>{{ email }}</strong> 发送了一封激活邮件，请前往邮箱完成账户激活。
        </p>
        <p class="auth-success-hint">如未收到邮件，请检查垃圾邮件箱</p>
        <button class="auth-submit-btn" @click="goLogin">
          前往登录
        </button>
      </div>
    </Transition>
  </AuthLayout>
</template>

<style scoped>
.strength-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
  padding: 0 2px;
}

.strength-track {
  display: flex;
  gap: 4px;
  flex: 1;
}

.strength-segment {
  height: 4px;
  flex: 1;
  border-radius: 2px;
  background: #eee;
  transition: background 0.3s;
}

.strength-label {
  font-size: 11px;
  font-weight: 500;
  flex-shrink: 0;
  transition: color 0.3s;
}
</style>
