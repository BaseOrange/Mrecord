<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {Snackbar} from '@varlet/ui'
import {forgotPassword} from '@/api'
import AuthLayout from '@/components/AuthLayout.vue'

const router = useRouter()

const email = ref('')
const loading = ref(false)
const submitted = ref(false)

const onSubmit = async () => {
  if (!email.value) {
    Snackbar.warning('请输入邮箱')
    return
  }

  loading.value = true
  try {
    await forgotPassword({email: email.value})
    submitted.value = true
  } catch (e: any) {
    Snackbar.error(e?.message || '发送失败，请稍后重试')
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
      <div v-if="!submitted" key="form" class="auth-card" @keydown.enter="onSubmit">
        <h3 class="card-title">找回密码</h3>
        <p class="card-desc">请输入注册时使用的邮箱，我们将发送密码重置链接。</p>

        <div class="auth-input-group">
          <div class="auth-input-wrapper">
            <svg class="auth-input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="4" width="20" height="16" rx="3" />
              <path d="M2 7l10 6 10-6" />
            </svg>
            <input
              v-model="email"
              type="email"
              placeholder="注册邮箱"
              class="auth-input"
              autocomplete="email"
            />
          </div>
        </div>

        <button
          class="auth-submit-btn"
          :class="{ 'auth-submit-btn--loading': loading }"
          :disabled="loading"
          @click="onSubmit"
        >
          <span v-if="!loading">发送重置链接</span>
          <span v-else class="auth-btn-loading">
            <svg class="auth-spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>

        <div class="auth-links">
          <a class="auth-link" @click="goLogin">返回登录</a>
        </div>
      </div>

      <!-- 发送成功提示 -->
      <div v-else key="success" class="auth-card auth-success-card">
        <div class="auth-success-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
            <path d="M20 32l4 0 4-8 8 16 4-8 4 0" stroke="#FF6500" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
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
  font-size: 18px;
  font-weight: 700;
  color: #333;
  margin-bottom: 8px;
}

.card-desc {
  font-size: 13px;
  color: #999;
  line-height: 1.5;
  margin-bottom: 24px;
}
</style>
