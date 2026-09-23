<script setup lang="ts">
import {computed, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {register} from '@/api'
import {md5} from 'js-md5'
import {checkPasswordStrength, isValidEmail} from '@/utils/security'
import AuthLayout from '@/components/AuthLayout.vue'
import AppIcon from '@/components/AppIcon.vue'

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
  const colors = ['', 'var(--semantic-up)', 'var(--orange)', 'var(--amber)', 'var(--semantic-down)', 'var(--semantic-down)']
  return colors[passwordStrength.value] || ''
})

const onRegister = async () => {
  // 重入守卫：键盘 Enter 在 loading 期间可连续触发，需在这里挡住（I1）
  if (loading.value) return
  if (!email.value) {
    Snackbar.warning('请输入邮箱')
    return
  }
  const trimmedEmail = email.value.trim()
  // I13：统一用 security.ts 的 isValidEmail 校验格式，尾空格 trim 后再提交
  if (!isValidEmail(trimmedEmail)) {
    Snackbar.warning('邮箱格式不正确')
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
      email: trimmedEmail,
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
      <div v-if="!registered" key="form" class="auth-card">
        <!-- 邮箱 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <AppIcon name="mail" :size="20" :stroke-width="1.5" class="auth-input-icon" />
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
            <AppIcon name="circle-user-round" :size="20" :stroke-width="1.5" class="auth-input-icon" />
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
            <AppIcon name="lock" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="密码（至少6位）"
              class="auth-input"
              autocomplete="new-password"
            />
            <button class="auth-eye-btn" @click="showPassword = !showPassword" type="button" :aria-label="showPassword ? '隐藏密码' : '显示密码'">
              <AppIcon :name="showPassword ? 'eye-off' : 'eye'" :size="20" :stroke-width="1.5" />
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
            <AppIcon name="lock-keyhole" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="confirmPassword"
              :type="showConfirm ? 'text' : 'password'"
              placeholder="确认密码"
              class="auth-input"
              autocomplete="new-password"
              @keydown.enter="onRegister"
            />
            <button class="auth-eye-btn" @click="showConfirm = !showConfirm" type="button" :aria-label="showConfirm ? '隐藏密码' : '显示密码'">
              <AppIcon :name="showConfirm ? 'eye-off' : 'eye'" :size="20" :stroke-width="1.5" />
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
          <div class="success-ring"><AppIcon name="check" :size="32" :stroke-width="2.6" /></div>
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
  background: var(--separator);
  transition: background var(--duration-base);
}

.strength-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  flex-shrink: 0;
  transition: color 0.3s;
}

/* 注册成功环形图标 */
.success-ring {
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
