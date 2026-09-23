<script setup lang="ts">
import {ref, computed, onMounted} from 'vue'
import {useRouter, useRoute} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {activateAccount, resendActivateEmail} from '@/api'
import {isValidEmail} from '@/utils/security'
import {useCountdown} from '@/composables/useCountdown'
import AuthLayout from '@/components/AuthLayout.vue'
import AppIcon from '@/components/AppIcon.vue'

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

// I13：重发激活邮件成功后冷却 60 秒，防止连点刷出大量激活邮件
const {remaining: resendRemaining, start: startResendCountdown} = useCountdown(60)

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
  // 重入守卫：键盘 Enter 在 loading 期间可连续触发，会连发多封激活邮件（I1）
  if (resendLoading.value) return
  if (!email.value) {
    Snackbar.warning('请输入邮箱')
    return
  }
  const trimmedEmail = email.value.trim()
  if (!isValidEmail(trimmedEmail)) {
    Snackbar.warning('邮箱格式不正确')
    return
  }
  // 冷却期内不允许重复发送
  if (resendRemaining.value > 0) return

  resendLoading.value = true
  try {
    await resendActivateEmail(trimmedEmail)
    resendSuccess.value = true
    Snackbar.success('激活邮件已发送')
    startResendCountdown()
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
        <div class="status-ring status-ring--danger"><AppIcon name="x" :size="30" :stroke-width="2.4" /></div>
      </div>
      <h3 class="auth-status-title">链接无效</h3>
      <p class="auth-status-desc">账户激活链接无效或已过期，请重新注册或联系管理员。</p>
      <button class="auth-submit-btn" @click="goLogin">返回登录</button>
    </div>

    <!-- 激活中 -->
    <div v-else-if="loading" class="auth-card auth-status-card">
      <div class="auth-status-icon">
        <div class="status-ring"><AppIcon name="refresh-cw" :size="28" :stroke-width="2.2" class="auth-loading-spinner" /></div>
      </div>
      <h3 class="auth-status-title">正在激活</h3>
      <p class="auth-status-desc">请稍候，正在完成账户激活...</p>
    </div>

    <!-- 激活失败 -->
    <div v-else-if="errorMsg && !success" class="auth-card auth-status-card">
      <div class="auth-status-icon">
        <div class="status-ring status-ring--danger"><AppIcon name="x" :size="30" :stroke-width="2.4" /></div>
      </div>
      <h3 class="auth-status-title">激活失败</h3>
      <p class="auth-status-desc">{{ errorMsg }}</p>

      <!-- 重新发送激活邮件 -->
      <div v-if="!resendSuccess" class="resend-section">
        <p class="resend-hint">输入邮箱重新发送激活邮件</p>
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
                        <AppIcon name="mail" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="email"
              type="email"
              placeholder="请输入邮箱"
              class="auth-input"
              autocomplete="email"
              @keydown.enter="onResendEmail"
            />
          </div>
        </div>
        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': resendLoading }"
          :disabled="resendLoading || resendRemaining > 0"
          @click="onResendEmail"
        >
          <span v-if="resendLoading" class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
          <span v-else-if="resendRemaining > 0">{{ resendRemaining }} 秒后可重新发送</span>
          <span v-else>重新发送激活邮件</span>
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
        <div class="status-ring"><AppIcon name="check" :size="30" :stroke-width="2.6" /></div>
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
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  margin-bottom: var(--space-3);
}

.resend-success {
  margin-bottom: 4px;
}

/* 状态环形图标 */
.status-ring {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--brand-soft);
  color: var(--brand);
  display: flex;
  align-items: center;
  justify-content: center;
}
.status-ring--danger {
  background: var(--danger-soft);
  color: var(--semantic-danger);
}
</style>
