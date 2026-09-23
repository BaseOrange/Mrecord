<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {forgotPassword} from '@/api'
import {isValidEmail} from '@/utils/security'
import {useCountdown} from '@/composables/useCountdown'
import AuthLayout from '@/components/AuthLayout.vue'
import AppIcon from '@/components/AppIcon.vue'

const router = useRouter()

const email = ref('')
const loading = ref(false)
const submitted = ref(false)

// I13：发送成功后冷却 60 秒，防止连点刷出大量重置邮件
const {remaining: resendRemaining, start: startResendCountdown} = useCountdown(60)

const onSubmit = async () => {
  // 重入守卫：键盘 Enter 在 loading 期间可连续触发，需在这里挡住（I1）
  if (loading.value) return
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

  loading.value = true
  try {
    await forgotPassword({email: trimmedEmail})
    submitted.value = true
    startResendCountdown()
  } catch {
    // 拦截器已统一弹出后端报文，页面不再重复提示（I8）
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
      <!-- 表单卡片 -->
      <div v-if="!submitted" key="form" class="auth-card">
        <h3 class="card-title">找回密码</h3>
        <p class="card-desc">请输入注册时使用的邮箱，我们将发送密码重置链接。</p>

        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <AppIcon name="mail" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="email"
              type="email"
              placeholder="注册邮箱"
              class="auth-input"
              autocomplete="email"
              @keydown.enter="onSubmit"
            />
          </div>
        </div>

        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': loading }"
          :disabled="loading || resendRemaining > 0"
          @click="onSubmit"
        >
          <span v-if="loading" class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
          <span v-else-if="resendRemaining > 0">{{ resendRemaining }} 秒后可重新发送</span>
          <span v-else>发送重置链接</span>
        </button>

        <div class="auth-links">
          <a class="auth-link" @click="goLogin">返回登录</a>
        </div>
      </div>

      <!-- 发送成功提示 -->
      <div v-else key="success" class="auth-card auth-success-card">
        <div class="auth-success-icon">
          <div class="status-ring"><AppIcon name="mail-check" :size="30" :stroke-width="2" /></div>
        </div>
        <h3 class="auth-success-title">邮件已发送</h3>
        <p class="auth-success-desc">
          重置密码链接已发送至 <strong>{{ email }}</strong>，请前往邮箱点击链接完成密码修改。
        </p>
        <p class="auth-success-hint">如未收到邮件，请检查垃圾邮件箱</p>
        <button class="auth-submit-btn" @click="goLogin">
          返回登录
        </button>
      </div>
    </Transition>
  </AuthLayout>
</template>

<style scoped>
.card-title {
  font-size: var(--text-title-2);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
  margin-bottom: 8px;
}

.card-desc {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  line-height: 1.5;
  margin-bottom: 24px;
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
</style>
