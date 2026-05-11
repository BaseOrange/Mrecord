<script setup lang="ts">
import {ref, computed, onMounted} from 'vue'
import {useRouter, useRoute} from 'vue-router'
import {activateAccount} from '@/api'
import loginBg from '@/assets/login_bg.png'

const router = useRouter()
const route = useRoute()

const token = computed(() => (route.query.token as string) || '')
const invalidToken = computed(() => !token.value)

const loading = ref(false)
const success = ref(false)
const errorMsg = ref('')

const doActivate = async () => {
  if (invalidToken.value) return

  loading.value = true
  errorMsg.value = ''
  try {
    await activateAccount({activateToken: token.value})
    success.value = true
  } catch (e: any) {
    errorMsg.value = e?.message || '激活失败，链接可能已过期或已被使用'
  } finally {
    loading.value = false
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
  <div class="activate-page">
    <!-- 背景层 -->
    <div class="bg-layer">
      <div class="bg-glow bg-glow--top"></div>
      <div class="bg-glow bg-glow--bottom"></div>
      <div class="bg-icon-wrapper">
        <img :src="loginBg" alt="月衡" class="bg-icon" />
      </div>
    </div>

    <!-- 前景内容 -->
    <div class="activate-container">
      <!-- 品牌区域 -->
      <div class="brand">
        <h1 class="brand-name">月衡</h1>
        <p class="brand-en">Mrecord</p>
        <p class="brand-slogan">逐月记账，衡知资产</p>
      </div>

      <!-- Token 无效 -->
      <div v-if="invalidToken" class="form-card error-card">
        <div class="status-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#e74c3c" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#e74c3c" opacity="0.08" />
            <line x1="22" y1="22" x2="42" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
            <line x1="42" y1="22" x2="22" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
          </svg>
        </div>
        <h3 class="status-title">链接无效</h3>
        <p class="status-desc">账户激活链接无效或已过期，请重新注册或联系管理员。</p>
        <button class="submit-btn" @click="goLogin">返回登录</button>
      </div>

      <!-- 激活中 -->
      <div v-else-if="loading" class="form-card status-card">
        <div class="status-icon">
          <svg class="spinner" viewBox="0 0 64 64" width="48" height="48">
            <circle cx="32" cy="32" r="28" stroke="#FF6500" stroke-width="3" fill="none" stroke-dasharray="60 60" opacity="0.3" />
            <circle cx="32" cy="32" r="28" stroke="#FF6500" stroke-width="3" fill="none" stroke-dasharray="40 80" stroke-linecap="round" />
          </svg>
        </div>
        <h3 class="status-title">正在激活</h3>
        <p class="status-desc">请稍候，正在完成账户激活...</p>
      </div>

      <!-- 激活失败 -->
      <div v-else-if="errorMsg && !success" class="form-card error-card">
        <div class="status-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#e74c3c" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#e74c3c" opacity="0.08" />
            <line x1="22" y1="22" x2="42" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
            <line x1="42" y1="22" x2="22" y2="42" stroke="#e74c3c" stroke-width="3" stroke-linecap="round" />
          </svg>
        </div>
        <h3 class="status-title">激活失败</h3>
        <p class="status-desc">{{ errorMsg }}</p>
        <button class="submit-btn" @click="doActivate">重新尝试</button>
        <button class="secondary-btn" @click="goLogin">返回登录</button>
      </div>

      <!-- 激活成功 -->
      <div v-else-if="success" class="form-card success-card">
        <div class="status-icon">
          <svg viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#FF6500" stroke-width="2.5" opacity="0.15" />
            <circle cx="32" cy="32" r="24" fill="#FF6500" opacity="0.08" />
            <path d="M22 33l7 7 13-14" stroke="#FF6500" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <h3 class="status-title">激活成功</h3>
        <p class="status-desc">您的账户已成功激活，现在可以使用邮箱和密码登录。</p>
        <button class="submit-btn" @click="goLogin">前往登录</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.activate-page {
  position: relative;
  min-height: 100vh;
  min-height: 100dvh;
  background: #ffffff;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 背景层 */
.bg-layer { position: absolute; inset: 0; z-index: 0; pointer-events: none; }
.bg-glow { position: absolute; border-radius: 50%; filter: blur(80px); opacity: 0.35; }
.bg-glow--top { width: 340px; height: 340px; top: -80px; right: -60px; background: radial-gradient(circle, #FF8C42 0%, #FFB380 50%, transparent 70%); }
.bg-glow--bottom { width: 280px; height: 280px; bottom: -40px; left: -40px; background: radial-gradient(circle, #FFAB76 0%, #FFD4B8 50%, transparent 70%); }
.bg-icon-wrapper { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 420px; height: 420px; opacity: 0.08; }
.bg-icon { width: 100%; height: 100%; object-fit: contain; }

/* 前景容器 */
.activate-container {
  position: relative; z-index: 1; width: 100%; max-width: 380px;
  padding: 0 28px; display: flex; flex-direction: column; align-items: center;
  animation: fadeSlideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) both;
}

/* 品牌 */
.brand { text-align: center; margin-bottom: 36px; }
.brand-name { font-size: 42px; font-weight: 800; letter-spacing: 6px; color: #FF6500; text-shadow: 0 2px 20px rgba(255,101,0,0.18); margin-bottom: 2px; line-height: 1.2; }
.brand-en { font-size: 15px; font-weight: 600; letter-spacing: 4px; color: #FF8C42; text-transform: uppercase; margin-bottom: 12px; }
.brand-slogan { font-size: 13px; letter-spacing: 3px; color: #bbb; font-weight: 400; }

/* 卡片 */
.form-card {
  width: 100%;
  background: rgba(255,255,255,0.72);
  backdrop-filter: blur(24px) saturate(1.6);
  -webkit-backdrop-filter: blur(24px) saturate(1.6);
  border-radius: 24px;
  border: 1px solid rgba(255,160,80,0.15);
  padding: 32px 24px 28px;
  box-shadow: 0 8px 32px rgba(255,101,0,0.06), 0 2px 8px rgba(0,0,0,0.03), inset 0 1px 0 rgba(255,255,255,0.8);
  animation: cardAppear 0.7s cubic-bezier(0.16, 1, 0.3, 1) 0.15s both;
}

/* 状态卡片 */
.status-card, .success-card, .error-card {
  display: flex; flex-direction: column; align-items: center;
  text-align: center; padding: 36px 24px 28px;
}
.status-icon { margin-bottom: 16px; }
.status-title { font-size: 20px; font-weight: 700; color: #333; margin-bottom: 12px; }
.status-desc { font-size: 14px; color: #666; line-height: 1.6; margin-bottom: 24px; }

/* 提交按钮 */
.submit-btn {
  width: 100%; height: 52px; margin-top: 6px; border: none; border-radius: 16px;
  font-size: 16px; font-weight: 700; letter-spacing: 3px; color: #fff; cursor: pointer;
  background: linear-gradient(135deg, #FF8C42 0%, #FF6500 50%, #E85500 100%);
  box-shadow: 0 6px 20px rgba(255,101,0,0.3), 0 2px 6px rgba(255,101,0,0.15), inset 0 1px 0 rgba(255,255,255,0.2);
  transition: all 0.3s cubic-bezier(0.4,0,0.2,1); position: relative; overflow: hidden;
}
.submit-btn::before {
  content: ''; position: absolute; inset: 0; border-radius: 16px;
  background: linear-gradient(135deg, rgba(255,255,255,0.18) 0%, transparent 60%); pointer-events: none;
}
.submit-btn:active:not(:disabled) {
  transform: scale(0.97);
  box-shadow: 0 3px 12px rgba(255,101,0,0.25), 0 1px 3px rgba(255,101,0,0.12);
}

/* 次要按钮 */
.secondary-btn {
  width: 100%; height: 48px; margin-top: 12px; border: 1.5px solid rgba(255,160,100,0.35); border-radius: 16px;
  font-size: 15px; font-weight: 600; letter-spacing: 2px; color: #FF8C42; cursor: pointer;
  background: transparent;
  transition: all 0.3s cubic-bezier(0.4,0,0.2,1);
}
.secondary-btn:active {
  background: rgba(255,140,66,0.06);
  border-color: #FF8C42;
}

/* 加载动画 */
.spinner { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }

/* 动画 */
@keyframes fadeSlideUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}
@keyframes cardAppear {
  from { opacity: 0; transform: translateY(20px) scale(0.97); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}
</style>
