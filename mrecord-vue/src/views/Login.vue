<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {useUserStore} from '@/stores/user'
import {login, queryMyInfo, getRegisterEnabled, revokeCancel} from '@/api'
import type {BusinessError} from '@/utils/request'
import {isValidEmail} from '@/utils/security'
import {md5} from 'js-md5'
import AuthLayout from '@/components/AuthLayout.vue'
import AgreementPopup from '@/components/AgreementPopup.vue'
import AppIcon from '@/components/AppIcon.vue'

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
  // 重入守卫：按钮的 :disabled 只挡点击，键盘 Enter 绕过得靠这里挡（I1）
  if (loading.value) return
  if (!email.value || !password.value) {
    Snackbar.warning('请输入邮箱和密码')
    return
  }
  const trimmedEmail = email.value.trim()
  if (!isValidEmail(trimmedEmail)) {
    Snackbar.warning('邮箱格式不正确')
    return
  }
  loading.value = true
  try {
    // I13：trim 后再提交，尾空格会导致「注册成功但登录失败」
    const token = await login({email: trimmedEmail, password: md5(password.value)})
    // token 先只入内存（请求拦截器需要它来调用 queryMyInfo），暂不落盘；
    // 拿到用户信息后才算登录完成，届时 token 与 userInfo 一起持久化（B5）
    userStore.setToken(token, false)
    const userInfo = await queryMyInfo()
    userStore.setToken(token)
    userStore.setUserInfo(userInfo)
    Snackbar.success('登录成功')
    router.replace('/home')
  } catch (e) {
    // 账号处于注销冷静期：引导用户撤销注销（表单里已填好邮箱密码，可直接复用）
    if ((e as BusinessError).code === '11007') {
      showCancelRevoke.value = true
    }
    // login 已成功但 queryMyInfo 失败 → 登录未完成，回滚到未登录态，
    // 不留「有 token 无 userInfo」的半登录态（其余错误由拦截器统一提示）
    if (userStore.token) {
      userStore.clearToken()
    }
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
    <div class="auth-card">
      <!-- 邮箱 -->
      <div class="auth-input-group">
        <div class="auth-input-wrapper">
          <AppIcon name="mail" :size="20" :stroke-width="1.5" class="auth-input-icon" />
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
          <AppIcon name="lock" :size="20" :stroke-width="1.5" class="auth-input-icon" />
          <input
            v-model="password"
            :type="showPassword ? 'text' : 'password'"
            placeholder="密码"
            class="auth-input"
            autocomplete="current-password"
            @keydown.enter="onLogin"
          />
          <button class="auth-eye-btn" @click="showPassword = !showPassword" type="button" :aria-label="showPassword ? '隐藏密码' : '显示密码'">
            <AppIcon :name="showPassword ? 'eye-off' : 'eye'" :size="20" :stroke-width="1.5" />
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
      confirm-button-color="var(--brand)"
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
  font-size: var(--text-xs);
  color: var(--text-quaternary);
  margin-top: 18px;
  letter-spacing: 0.3px;
}

.agreement-link {
  font-size: 11px !important;
}

.cancel-revoke-tips {
  font-size: var(--text-body);
  color: var(--text-secondary);
  line-height: 1.8;
}
.cancel-revoke-tips b {
  color: var(--brand);
  font-weight: var(--weight-semibold);
}
</style>
