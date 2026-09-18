<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {useUserStore} from '@/stores/user'
import {login, queryMyInfo, getRegisterEnabled, revokeCancel} from '@/api'
import type {BusinessError} from '@/utils/request'
import {md5} from 'js-md5'
import AuthLayout from '@/components/AuthLayout.vue'
import AgreementPopup from '@/components/AgreementPopup.vue'

const router = useRouter()
const userStore = useUserStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const showPassword = ref(false)
const registerEnabled = ref(true)
// 注销冷静期：登录被拒绝（11007）时弹出撤销注销入口
const showCancelRevoke = ref(false)
const revoking = ref(false)

onMounted(async () => {
  try {
    registerEnabled.value = await getRegisterEnabled()
  } catch {
    // 获取失败时默认显示注册按钮
  }
})

const onLogin = async () => {
  if (!email.value || !password.value) {
    Snackbar.warning('请输入邮箱和密码')
    return
  }
  loading.value = true
  try {
    const token = await login({email: email.value, password: md5(password.value)})
    userStore.setToken(token)
    const userInfo = await queryMyInfo()
    userStore.setUserInfo(userInfo)
    Snackbar.success('登录成功')
    router.replace('/home')
  } catch (e) {
    // 账号处于注销冷静期：引导用户撤销注销（表单里已填好邮箱密码，可直接复用）
    if ((e as BusinessError).code === '11007') {
      showCancelRevoke.value = true
    }
    // 其余错误由拦截器统一提示
  } finally {
    loading.value = false
  }
}

// 撤销注销：冷静期内凭邮箱+密码恢复账号，成功后自动重新登录
const onRevokeCancel = async () => {
  revoking.value = true
  try {
    await revokeCancel({email: email.value, password: md5(password.value)})
    Snackbar.success('已撤销注销，正在登录')
    // 撤销成功后账号恢复正常，直接走登录流程
    await onLogin()
  } catch {
    // 拦截器已处理错误提示
  } finally {
    revoking.value = false
    showCancelRevoke.value = false
  }
}

const onForgotPassword = () => {
  router.push('/forgot-password')
}

const onRegister = () => {
  router.push('/register')
}

const showAgreement = ref(false)
</script>

<template>
  <AuthLayout>
    <div class="auth-card" @keydown.enter="onLogin">
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
            placeholder="邮箱"
            class="auth-input"
            autocomplete="email"
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
            placeholder="密码"
            class="auth-input"
            autocomplete="current-password"
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

      <!-- 登录按钮 -->
      <button
        class="auth-submit-btn"
        :class="{ 'auth-submit-btn--loading': loading }"
        :disabled="loading"
        @click="onLogin"
      >
        <span v-if="!loading">登录</span>
        <span v-else class="auth-btn-loading">
          <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
            <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
          </svg>
        </span>
      </button>

      <!-- 底部链接 -->
      <div class="auth-links">
        <a class="auth-link" @click="onForgotPassword">忘记密码？</a>
        <template v-if="registerEnabled">
          <span class="auth-link-divider"></span>
          <a class="auth-link" @click="onRegister">注册账户</a>
        </template>
      </div>

      <!-- 用户协议入口 -->
      <p class="agreement-entry">
        注册或登录即表示同意
        <a class="auth-link agreement-link" @click="showAgreement = true">《用户协议及隐私政策》</a>
      </p>
    </div>

    <AgreementPopup v-model:show="showAgreement" />

    <!-- 注销冷静期：撤销注销弹窗 -->
    <var-dialog
      v-model:show="showCancelRevoke"
      title="账号注销中"
      confirm-button-text="撤销注销并登录"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      :confirm-button-disabled="revoking"
      @confirm="onRevokeCancel"
      @cancel="showCancelRevoke = false"
    >
      <div class="cancel-revoke-tips">
        该账号正在注销中，处于 <b>15 天冷静期</b>。<br />
        撤销注销后账号立即恢复正常，可继续使用。
      </div>
    </var-dialog>
  </AuthLayout>
</template>

<style scoped>
.agreement-entry {
  text-align: center;
  font-size: 11px;
  color: #ccc;
  margin-top: 18px;
  letter-spacing: 0.3px;
}

.agreement-link {
  font-size: 11px !important;
}

.cancel-revoke-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
}
.cancel-revoke-tips b {
  color: #FF6500;
  font-weight: 600;
}
</style>
