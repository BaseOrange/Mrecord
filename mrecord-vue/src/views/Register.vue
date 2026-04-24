<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {register} from '@/api'
import loginBg from '@/assets/login_bg.png'

const router = useRouter()

const email = ref('')
const nickname = ref('')
const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const showPassword = ref(false)
const showConfirm = ref(false)
const registered = ref(false)

const onRegister = async () => {
  if (!email.value) {
    // @ts-ignore
    Snackbar.warning('请输入邮箱')
    return
  }
  if (!nickname.value) {
    // @ts-ignore
    Snackbar.warning('请输入昵称')
    return
  }
  if (!password.value) {
    // @ts-ignore
    Snackbar.warning('请输入密码')
    return
  }
  if (password.value.length < 6) {
    // @ts-ignore
    Snackbar.warning('密码至少6位')
    return
  }
  if (password.value !== confirmPassword.value) {
    // @ts-ignore
    Snackbar.warning('两次密码不一致')
    return
  }

  loading.value = true
  try {
    await register({
      username: email.value,
      password: password.value,
      nickname: nickname.value,
    })
    registered.value = true
  } catch (e: any) {
    // @ts-ignore
    Snackbar.error(e?.message || '注册失败')
  } finally {
    loading.value = false
  }
}

const goLogin = () => {
  router.replace('/login')
}
</script>

<template>
  <div class="register-page">
    <!-- 背景层 -->
    <div class="bg-layer">
      <div class="bg-glow bg-glow--top"></div>
      <div class="bg-glow bg-glow--bottom"></div>
      <div class="bg-icon-wrapper">
        <img :src="loginBg" alt="月衡" class="bg-icon" />
      </div>
    </div>

    <!-- 前景内容 -->
    <div class="register-container">
      <!-- 品牌区域 -->
      <div class="brand">
        <h1 class="brand-name">月衡</h1>
        <p class="brand-en">Mrecord</p>
        <p class="brand-slogan">逐月记账，衡知资产</p>
      </div>

      <!-- 注册表单卡片 -->
      <div v-if="!registered" class="form-card">
        <!-- 邮箱 -->
        <div class="input-group">
          <div class="input-wrapper">
            <svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="4" width="20" height="16" rx="3" />
              <path d="M2 7l10 6 10-6" />
            </svg>
            <input
              v-model="email"
              type="email"
              placeholder="邮箱（用于登录和激活）"
              class="form-input"
              autocomplete="email"
            />
          </div>
        </div>

        <!-- 昵称 -->
        <div class="input-group">
          <div class="input-wrapper">
            <svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="8" r="4" />
              <path d="M20 21a8 8 0 10-16 0" />
            </svg>
            <input
              v-model="nickname"
              type="text"
              placeholder="昵称"
              class="form-input"
              autocomplete="nickname"
            />
          </div>
        </div>

        <!-- 密码 -->
        <div class="input-group">
          <div class="input-wrapper">
            <svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="11" width="18" height="11" rx="2" />
              <path d="M7 11V7a5 5 0 0110 0v4" />
              <circle cx="12" cy="16.5" r="1.5" fill="currentColor" stroke="none" />
            </svg>
            <input
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              placeholder="密码（至少6位）"
              class="form-input"
              autocomplete="new-password"
            />
            <button class="eye-btn" @click="showPassword = !showPassword" type="button">
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
        <div class="input-group">
          <div class="input-wrapper">
            <svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M9 12l2 2 4-4" />
              <rect x="3" y="11" width="18" height="11" rx="2" />
              <path d="M7 11V7a5 5 0 0110 0v4" />
            </svg>
            <input
              v-model="confirmPassword"
              :type="showConfirm ? 'text' : 'password'"
              placeholder="确认密码"
              class="form-input"
              autocomplete="new-password"
            />
            <button class="eye-btn" @click="showConfirm = !showConfirm" type="button">
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
          class="submit-btn"
          :class="{ 'submit-btn--loading': loading }"
          :disabled="loading"
          @click="onRegister"
        >
          <span v-if="!loading">注册</span>
          <span v-else class="btn-loading">
            <svg class="spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>

        <!-- 底部链接 -->
        <div class="form-links">
          <span class="link-hint">已有账户？</span>
          <a class="link" @click="goLogin">去登录</a>
        </div>
      </div>

      <!-- 注册成功提示卡片 -->
      <div v-else class="form-card success-card">
        <div class="success-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
            <path d="M22 33l7 7 13-14" stroke="#FF6500" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <h3 class="success-title">注册成功！</h3>
        <p class="success-desc">
          我们已向 <strong>{{ email }}</strong> 发送了一封激活邮件，请前往邮箱完成账户激活。
        </p>
        <p class="success-hint">如未收到邮件，请检查垃圾邮件箱</p>
        <button class="submit-btn" @click="goLogin">
          前往登录
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ======= 页面整体 ======= */
.register-page {
  position: relative;
  min-height: 100vh;
  min-height: 100dvh;
  background: #ffffff;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* ======= 背景层 ======= */
.bg-layer {
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
}

.bg-glow {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.35;
}

.bg-glow--top {
  width: 340px;
  height: 340px;
  top: -80px;
  right: -60px;
  background: radial-gradient(circle, #FF8C42 0%, #FFB380 50%, transparent 70%);
}

.bg-glow--bottom {
  width: 280px;
  height: 280px;
  bottom: -40px;
  left: -40px;
  background: radial-gradient(circle, #FFAB76 0%, #FFD4B8 50%, transparent 70%);
}

.bg-icon-wrapper {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 420px;
  height: 420px;
  opacity: 0.08;
}

.bg-icon {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

/* ======= 前景容器 ======= */
.register-container {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 380px;
  padding: 0 28px;
  display: flex;
  flex-direction: column;
  align-items: center;
  animation: fadeSlideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) both;
}

/* ======= 品牌区域 ======= */
.brand {
  text-align: center;
  margin-bottom: 32px;
}

.brand-name {
  font-size: 42px;
  font-weight: 800;
  letter-spacing: 6px;
  color: #FF6500;
  text-shadow: 0 2px 20px rgba(255, 101, 0, 0.18);
  margin-bottom: 2px;
  line-height: 1.2;
}

.brand-en {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 4px;
  color: #FF8C42;
  text-transform: uppercase;
  margin-bottom: 12px;
}

.brand-slogan {
  font-size: 13px;
  letter-spacing: 3px;
  color: #bbb;
  font-weight: 400;
}

/* ======= 表单卡片 ======= */
.form-card {
  width: 100%;
  background: rgba(255, 255, 255, 0.72);
  backdrop-filter: blur(24px) saturate(1.6);
  -webkit-backdrop-filter: blur(24px) saturate(1.6);
  border-radius: 24px;
  border: 1px solid rgba(255, 160, 80, 0.15);
  padding: 28px 24px 24px;
  box-shadow:
    0 8px 32px rgba(255, 101, 0, 0.06),
    0 2px 8px rgba(0, 0, 0, 0.03),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
  animation: cardAppear 0.7s cubic-bezier(0.16, 1, 0.3, 1) 0.15s both;
}

/* ======= 输入框 ======= */
.input-group {
  margin-bottom: 14px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: rgba(255, 245, 238, 0.6);
  border: 1.5px solid rgba(255, 160, 100, 0.3);
  border-radius: 14px;
  padding: 0 14px;
  height: 50px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 4px rgba(255, 140, 66, 0.04);
}

.input-wrapper:focus-within {
  border-color: #FF8C42;
  background: rgba(255, 250, 246, 0.85);
  box-shadow:
    0 0 0 3px rgba(255, 140, 66, 0.1),
    0 2px 8px rgba(255, 101, 0, 0.08);
}

.input-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  color: #FFa060;
  margin-right: 12px;
  opacity: 0.7;
  transition: opacity 0.3s;
}

.input-wrapper:focus-within .input-icon {
  opacity: 1;
  color: #FF6500;
}

.form-input {
  flex: 1;
  height: 100%;
  font-size: 15px;
  color: #333;
  background: transparent;
  border: none;
  outline: none;
  letter-spacing: 0.5px;
}

.form-input::placeholder {
  color: #cca88a;
  font-weight: 400;
}

.eye-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  color: #c0a080;
  cursor: pointer;
  transition: color 0.2s;
  background: none;
  border: none;
  padding: 0;
  flex-shrink: 0;
}

.eye-btn:active {
  color: #FF6500;
}

/* ======= 提交按钮 ======= */
.submit-btn {
  width: 100%;
  height: 52px;
  margin-top: 6px;
  border: none;
  border-radius: 16px;
  font-size: 17px;
  font-weight: 700;
  letter-spacing: 4px;
  color: #fff;
  cursor: pointer;
  background: linear-gradient(135deg, #FF8C42 0%, #FF6500 50%, #E85500 100%);
  box-shadow:
    0 6px 20px rgba(255, 101, 0, 0.3),
    0 2px 6px rgba(255, 101, 0, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.submit-btn::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(255,255,255,0.18) 0%, transparent 60%);
  pointer-events: none;
}

.submit-btn:active:not(:disabled) {
  transform: scale(0.97);
  box-shadow:
    0 3px 12px rgba(255, 101, 0, 0.25),
    0 1px 3px rgba(255, 101, 0, 0.12);
}

.submit-btn--loading {
  opacity: 0.8;
  cursor: not-allowed;
}

.btn-loading {
  display: flex;
  align-items: center;
  justify-content: center;
}

.spinner {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* ======= 底部链接 ======= */
.form-links {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 20px;
  gap: 4px;
}

.link-hint {
  font-size: 13px;
  color: #bbb;
}

.link {
  font-size: 13px;
  color: #FF8C42;
  cursor: pointer;
  padding: 4px 2px;
  letter-spacing: 0.5px;
  transition: color 0.2s;
  text-decoration: none;
  font-weight: 500;
}

.link:active {
  color: #E85500;
}

/* ======= 注册成功卡片 ======= */
.success-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 36px 24px 28px;
}

.success-icon {
  margin-bottom: 16px;
}

.success-title {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  margin-bottom: 12px;
}

.success-desc {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  margin-bottom: 8px;
}

.success-desc strong {
  color: #FF6500;
  font-weight: 600;
}

.success-hint {
  font-size: 12px;
  color: #bbb;
  margin-bottom: 24px;
}

/* ======= 动画 ======= */
@keyframes fadeSlideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes cardAppear {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.97);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
</style>
