<script setup lang="ts">
import loginBg from '@/assets/login_bg.png'

const appIcon = '/app-icon.svg'
</script>

<template>
  <div class="auth-page">
    <div class="bg-layer">
      <div class="bg-glow bg-glow--top"></div>
      <div class="bg-glow bg-glow--bottom"></div>
      <div class="bg-icon-wrapper">
        <img :src="loginBg" alt="月衡" class="bg-icon" />
      </div>
    </div>

    <div class="auth-container">
      <div class="brand">
        <div class="brand-title">
          <img :src="appIcon" alt="月衡 Logo" class="brand-logo" />
          <h1 class="brand-name">月衡</h1>
        </div>
        <p class="brand-en">Mrecord</p>
        <p class="brand-slogan">逐月记账，衡知资产</p>
      </div>

      <slot />
    </div>
  </div>
</template>

<style scoped>
.auth-page {
  position: relative;
  min-height: 100vh;
  min-height: 100dvh;
  background: #ffffff;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

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

.auth-container {
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

.brand {
  text-align: center;
  margin-bottom: 36px;
}

.brand-title {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 2px;
}

.brand-logo {
  width: 42px;
  height: 42px;
  border-radius: 12px;
  box-shadow: 0 6px 16px rgba(249, 114, 22, 0.18);
  flex-shrink: 0;
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
</style>

<style>
/* Shared auth styles (unscoped so child components can use them) */

.auth-card {
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
  animation: authCardAppear 0.7s cubic-bezier(0.16, 1, 0.3, 1) 0.15s both;
}

.auth-input-group {
  margin-bottom: 16px;
}

.auth-input-wrapper {
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

.auth-input-wrapper:focus-within {
  border-color: #FF8C42;
  background: rgba(255, 250, 246, 0.85);
  box-shadow:
    0 0 0 3px rgba(255, 140, 66, 0.1),
    0 2px 8px rgba(255, 101, 0, 0.08);
}

.auth-input-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  color: #FFa060;
  margin-right: 12px;
  opacity: 0.7;
  transition: opacity 0.3s;
}

.auth-input-wrapper:focus-within .auth-input-icon {
  opacity: 1;
  color: #FF6500;
}

.auth-input {
  flex: 1;
  height: 100%;
  font-size: 15px;
  color: #333;
  background: transparent;
  border: none;
  outline: none;
  letter-spacing: 0.5px;
}

.auth-input::placeholder {
  color: #cca88a;
  font-weight: 400;
}

.auth-eye-btn {
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

.auth-eye-btn:active {
  color: #FF6500;
}

.auth-submit-btn {
  width: 100%;
  height: 52px;
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

.auth-submit-btn::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(255,255,255,0.18) 0%, transparent 60%);
  pointer-events: none;
}

.auth-submit-btn:active:not(:disabled) {
  transform: scale(0.97);
  box-shadow:
    0 3px 12px rgba(255, 101, 0, 0.25),
    0 1px 3px rgba(255, 101, 0, 0.12);
}

.auth-submit-btn--loading {
  opacity: 0.8;
  cursor: not-allowed;
}

.auth-btn-loading {
  display: flex;
  align-items: center;
  justify-content: center;
}

.auth-spinner {
  animation: authSpin 0.8s linear infinite;
}

@keyframes authSpin {
  to { transform: rotate(360deg); }
}

.auth-links {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 22px;
  gap: 4px;
}

.auth-link {
  font-size: 13px;
  color: #FF8C42;
  cursor: pointer;
  padding: 4px 2px;
  letter-spacing: 0.5px;
  transition: color 0.2s;
  text-decoration: none;
  font-weight: 500;
}

.auth-link:active {
  color: #E85500;
}

.auth-link-divider {
  width: 1px;
  height: 12px;
  background: #e0cfc0;
  margin: 0 12px;
  opacity: 0.6;
}

.auth-link-hint {
  font-size: 13px;
  color: #bbb;
}

/* Success card styles */
.auth-success-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 36px 24px 28px;
}

.auth-success-icon {
  margin-bottom: 16px;
}

.auth-success-title {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  margin-bottom: 12px;
}

.auth-success-desc {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  margin-bottom: 8px;
}

.auth-success-desc strong {
  color: #FF6500;
  font-weight: 600;
}

.auth-success-hint {
  font-size: 12px;
  color: #bbb;
  margin-bottom: 24px;
}

/* Status card (shared by success + error) */
.auth-status-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 36px 24px 28px;
}

.auth-status-icon {
  margin-bottom: 16px;
}

.auth-status-title {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  margin-bottom: 12px;
}

.auth-status-desc {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  margin-bottom: 24px;
}

/* Secondary button (outlined) */
.auth-secondary-btn {
  width: 100%;
  height: 48px;
  margin-top: 12px;
  border: 1.5px solid rgba(255, 160, 100, 0.35);
  border-radius: 16px;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 2px;
  color: #FF8C42;
  cursor: pointer;
  background: transparent;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.auth-secondary-btn:active {
  background: rgba(255, 140, 66, 0.06);
  border-color: #FF8C42;
}

/* Card title/desc (for form cards with headers) */
.auth-card-title {
  font-size: 18px;
  font-weight: 700;
  color: #333;
  margin-bottom: 8px;
}

.auth-card-desc {
  font-size: 13px;
  color: #999;
  line-height: 1.5;
  margin-bottom: 24px;
}

/* Auth loading spinner (for status cards) */
.auth-loading-spinner {
  animation: authSpin 1s linear infinite;
}

/* Card transition */
.auth-fade-enter-active,
.auth-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.auth-fade-enter-from {
  opacity: 0;
  transform: translateY(16px) scale(0.97);
}

.auth-fade-leave-to {
  opacity: 0;
  transform: translateY(-12px) scale(0.97);
}

@keyframes authCardAppear {
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
