<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import {useUserStore} from '@/stores/user'
import {login} from '@/api'
import {md5} from 'js-md5'
import loginBg from '@/assets/login_bg.png'

const router = useRouter()
const userStore = useUserStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const showPassword = ref(false)

const onLoginReal = async () => {
  if (!email.value || !password.value) {
    // @ts-ignore
    Snackbar.warning('请输入邮箱和密码')
    return
  }
  loading.value = true
  try {
    const res = await login({email: email.value, password: md5(password.value)})
    userStore.setToken(res)
    // @ts-ignore
    Snackbar.success('登录成功')
    router.replace('/home')
  } catch (e: any) {
    // @ts-ignore
    Snackbar.error(e?.message || '登录失败')
  } finally {
    loading.value = false
  }
}

// 临时：跳过登录直接进入主页
const onLogin = () => {
  router.replace('/home')
}

const onForgotPassword = () => {
  router.push('/forgot-password')
}

const onRegister = () => {
  router.push('/register')
}
</script>

<template>
  <div class="login-page">
    <!-- 背景层：暖橙柔光氛围 -->
    <div class="bg-layer">
      <div class="bg-glow bg-glow--top"></div>
      <div class="bg-glow bg-glow--bottom"></div>
      <div class="bg-icon-wrapper">
        <img :src="loginBg" alt="月衡" class="bg-icon" />
      </div>
    </div>

    <!-- 前景：磨砂玻璃登录卡片 -->
    <div class="login-container">
      <!-- 品牌区域 -->
      <div class="brand">
        <h1 class="brand-name">月衡</h1>
        <p class="brand-en">Mrecord</p>
        <p class="brand-slogan">逐月记账，衡知资产</p>
      </div>

      <!-- 登录卡片 -->
      <div class="login-card">
        <!-- 邮箱输入框 -->
        <div class="input-group">
          <div class="input-wrapper">
            <svg class="input-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="4" width="20" height="16" rx="3" />
              <path d="M2 7l10 6 10-6" />
            </svg>
            <input
              v-model="email"
              type="email"
              placeholder="邮箱"
              class="login-input"
              autocomplete="email"
            />
          </div>
        </div>

        <!-- 密码输入框 -->
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
              placeholder="密码"
              class="login-input"
              autocomplete="current-password"
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

        <!-- 登录按钮 -->
        <button
          class="login-btn"
          :class="{ 'login-btn--loading': loading }"
          :disabled="loading"
          @click="onLogin"
        >
          <span v-if="!loading">登录</span>
          <span v-else class="btn-loading">
            <svg class="spinner" viewBox="0 0 24 24" width="22" height="22">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>

        <!-- 底部链接 -->
        <div class="login-links">
          <a class="link" @click="onForgotPassword">忘记密码？</a>
          <span class="link-divider"></span>
          <a class="link" @click="onRegister">注册账户</a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ======= 页面整体 ======= */
.login-page {
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
.login-container {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 380px;
  padding: 0 28px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

/* ======= 品牌区域 ======= */
.brand {
  text-align: center;
  margin-bottom: 44px;
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

/* ======= 登录卡片 ======= */
.login-card {
  width: 100%;
  background: rgba(255, 255, 255, 0.72);
  backdrop-filter: blur(24px) saturate(1.6);
  -webkit-backdrop-filter: blur(24px) saturate(1.6);
  border-radius: 24px;
  border: 1px solid rgba(255, 160, 80, 0.15);
  padding: 32px 24px 28px;
  box-shadow:
    0 8px 32px rgba(255, 101, 0, 0.06),
    0 2px 8px rgba(0, 0, 0, 0.03),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

/* ======= 输入框 ======= */
.input-group {
  margin-bottom: 18px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: rgba(255, 245, 238, 0.6);
  border: 1.5px solid rgba(255, 160, 100, 0.3);
  border-radius: 14px;
  padding: 0 14px;
  height: 52px;
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

.login-input {
  flex: 1;
  height: 100%;
  font-size: 15px;
  color: #333;
  background: transparent;
  border: none;
  outline: none;
  letter-spacing: 0.5px;
}

.login-input::placeholder {
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

/* ======= 登录按钮 ======= */
.login-btn {
  width: 100%;
  height: 54px;
  margin-top: 8px;
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

.login-btn::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(255,255,255,0.18) 0%, transparent 60%);
  pointer-events: none;
}

.login-btn:active:not(:disabled) {
  transform: scale(0.97);
  box-shadow:
    0 3px 12px rgba(255, 101, 0, 0.25),
    0 1px 3px rgba(255, 101, 0, 0.12);
}

.login-btn--loading {
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
.login-links {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 24px;
  gap: 0;
}

.link {
  font-size: 13px;
  color: #FF8C42;
  cursor: pointer;
  padding: 4px 2px;
  letter-spacing: 0.5px;
  transition: color 0.2s;
  text-decoration: none;
}

.link:active {
  color: #E85500;
}

.link-divider {
  width: 1px;
  height: 12px;
  background: #e0cfc0;
  margin: 0 16px;
  opacity: 0.6;
}

/* ======= 动画 ======= */
.login-container {
  animation: fadeSlideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) both;
}

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

.login-card {
  animation: cardAppear 0.7s cubic-bezier(0.16, 1, 0.3, 1) 0.15s both;
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
