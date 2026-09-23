<script setup lang="ts">
import {ref, computed} from 'vue'
import {useRouter, useRoute} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {resetPassword} from '@/api'
import {md5} from 'js-md5'
import AuthLayout from '@/components/AuthLayout.vue'
import AppIcon from '@/components/AppIcon.vue'

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
  // 重入守卫：键盘 Enter 在 loading 期间可连续触发，需在这里挡住（I1）
  if (loading.value) return
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
    <!-- Token 无效 -->
    <div v-if="invalidToken" class="auth-card auth-status-card">
      <div class="auth-status-icon">
        <div class="status-ring status-ring--danger"><AppIcon name="x" :size="30" :stroke-width="2.4" /></div>
      </div>
      <h3 class="auth-status-title">链接无效</h3>
      <p class="auth-status-desc">密码重置链接无效或已过期，请重新申请。</p>
      <button class="auth-submit-btn" @click="goLogin">返回登录</button>
    </div>

    <!-- 重置密码表单 / 成功 -->
    <Transition v-else name="auth-fade" mode="out-in">
      <div v-if="!success" key="form" class="auth-card">
        <h3 class="auth-card-title">重置密码</h3>
        <p class="auth-card-desc">请设置您的新密码。</p>

        <!-- 新密码 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
                        <AppIcon name="lock" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="新密码（至少6位）"
              class="auth-input"
              autocomplete="new-password"
            />
            <button class="auth-eye-btn" @click="showPassword = !showPassword" type="button" :aria-label="showPassword ? '隐藏密码' : '显示密码'">
              <AppIcon :name="showPassword ? 'eye-off' : 'eye'" :size="20" :stroke-width="1.5" />
            </button>
          </div>
        </div>

        <!-- 确认密码 -->
        <div class="auth-input-group">
          <div class="auth-input-wrapper">
                        <AppIcon name="lock-keyhole" :size="20" :stroke-width="1.5" class="auth-input-icon" />
            <input
              v-model="confirmPassword"
              :type="showConfirm ? 'text' : 'password'"
              placeholder="确认新密码"
              class="auth-input"
              autocomplete="new-password"
              @keydown.enter="onSubmit"
            />
            <button class="auth-eye-btn" @click="showConfirm = !showConfirm" type="button" :aria-label="showConfirm ? '隐藏密码' : '显示密码'">
              <AppIcon :name="showConfirm ? 'eye-off' : 'eye'" :size="20" :stroke-width="1.5" />
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
          <div class="status-ring"><AppIcon name="check" :size="30" :stroke-width="2.6" /></div>
        </div>
        <h3 class="auth-success-title">密码已重置</h3>
        <p class="auth-success-desc">您的密码已成功修改，请使用新密码登录。</p>
        <button class="auth-submit-btn" @click="goLogin">前往登录</button>
      </div>
    </Transition>
  </AuthLayout>
</template>
