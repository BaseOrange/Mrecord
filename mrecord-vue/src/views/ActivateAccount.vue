<script setup lang="ts">
import {ref, computed, onMounted} from 'vue'
import {useRouter, useRoute} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {activateAccount, resendActivateEmail} from '@/api'
import AuthLayout from '@/components/AuthLayout.vue'

const router = useRouter()
const route = useRoute()

const token = computed(() => (route.query.token as string) || '')
const invalidToken = computed(() => !token.value)

const loading = ref(false)
const success = ref(false)
const errorMsg = ref('')
const email = ref('')
const resendLoading = ref(false)
const resendSuccess = ref(false)

const doActivate = async () => {
  if (invalidToken.value) return

  loading.value = true
  errorMsg.value = ''
  try {
    await activateAccount(token.value)
    success.value = true
  } catch (e: any) {
    errorMsg.value = e?.message || '激活失败，链接可能已过期或已被使用'
  } finally {
    loading.value = false
  }
}

const onResendEmail = async () => {
  if (!email.value) {
    Snackbar.warning('请输入邮箱')
    return
  }
  resendLoading.value = true
  try {
    await resendActivateEmail(email.value)
    resendSuccess.value = true
    Snackbar.success('激活邮件已发送')
  } catch {
    // 拦截器已处理错误提示
  } finally {
    resendLoading.value = false
  }
}

const goLogin = () => {
  router.replace('/login')
}

onMounted(() => {
  if (!invalidToken.value) {
    doActivate()
  }
})
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
      <p class="auth-status-desc">账户激活链接无效或已过期，请重新注册或联系管理员。</p>
      <button class="auth-submit-btn" @click="goLogin">返回登录</button>
    </div>

    <!-- 激活中 -->
    <div v-else-if="loading" class="auth-card auth-status-card">
      <div class="auth-status-icon">
        <svg class="auth-loading-spinner" viewBox="0 0 64 64" width="48" height="48">
          <circle cx="32" cy="32" r="28" stroke="#FF6500" stroke-width="3" fill="none" stroke-dasharray="60 60" opacity="0.3" />
          <circle cx="32" cy="32" r="28" stroke="#FF6500" stroke-width="3" fill="none" stroke-dasharray="40 80" stroke-linecap="round" />
        </svg>
      </div>
      <h3 class="auth-status-title">正在激活</h3>
      <p class="auth-status-desc">请稍候，正在完成账户激活...</p>
    </div>

    <!-- 激活失败 -->
    <div v-else-if="errorMsg && !success" class="auth-card auth-status-card" @keydown.enter="onResendEmail">
      <div class="auth-status-icon">
        <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
          <circle cx="32" cy="32" r="30" stroke="#e74c3c" stroke-width="2.5" opacity="0.15" />
          <circle cx="32" cy="32" r="24" fill="#e74c3c" opacity="0.08" />
          <line x1="22" y1="22" x2="42" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
          <line x1="42" y1="22" x2="22" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
        </svg>
      </div>
      <h3 class="auth-status-title">激活失败</h3>
      <p class="auth-status-desc">{{ errorMsg }}</p>

      <!-- 重新发送激活邮件 -->
      <div v-if="!resendSuccess" class="resend-section">
        <p class="resend-hint">输入邮箱重新发送激活邮件</p>
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="4" width="20" height="16" rx="3" />
              <path d="M2 7l10 6 10-6" />
            </svg>
            <input
              v-model="email"
              type="email"
              placeholder="请输入邮箱"
              class="auth-input"
              autocomplete="email"
            />
          </div>
        </div>
        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': resendLoading }"
          :disabled="resendLoading"
          @click="onResendEmail"
        >
          <span v-if="!resendLoading">重新发送激活邮件</span>
          <span v-else class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>
      </div>

      <div v-else class="resend-success">
        <p class="auth-status-desc">激活邮件已重新发送，请前往邮箱查收。</p>
      </div>

      <button class="auth-secondary-btn" @click="goLogin">返回登录</button>
    </div>

    <!-- 激活成功 -->
    <div v-else-if="success" class="auth-card auth-success-card">
      <div class="auth-success-icon">
        <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
          <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
          <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
          <path d="M22 33l7 7 13-14" stroke="#FF6500" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <h3 class="auth-success-title">激活成功</h3>
      <p class="auth-success-desc">您的账户已成功激活，现在可以使用邮箱和密码登录。</p>
      <button class="auth-submit-btn" @click="goLogin">前往登录</button>
    </div>
  </AuthLayout>
</template>

<style scoped>
.resend-section {
  width: 100%;
  margin-bottom: 4px;
}

.resend-hint {
  font-size: 13px;
  color: #999;
  margin-bottom: 12px;
}

.resend-success {
  margin-bottom: 4px;
}
</style>
