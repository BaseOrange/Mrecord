<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { useUserStore } from '@/stores/user'
import { logout } from '@/api'

const router = useRouter()
const userStore = useUserStore()
const loggingOut = ref(false)
const showLogoutConfirm = ref(false)

const handleLogout = async () => {
  showLogoutConfirm.value = false
  loggingOut.value = true
  try {
    await logout()
    userStore.logout()
    Snackbar.success('已退出登录')
    router.replace('/login')
  } catch {
    // 即使接口失败也清除本地状态
    userStore.logout()
    router.replace('/login')
  } finally {
    loggingOut.value = false
  }
}
</script>

<template>
  <div class="profile-page">
    <!-- 顶部标题 -->
    <div class="page-header">
      <h2>我的</h2>
    </div>

    <div class="page-body">
      <!-- 用户信息卡片 -->
      <div class="user-card">
        <div class="avatar">
          <span class="avatar-emoji">😊</span>
        </div>
        <div class="user-info">
          <div class="welcome">欢迎回来</div>
          <div class="nickname">{{ userStore.userInfo?.nickname || '未设置昵称' }}</div>
          <div class="email">{{ userStore.userInfo?.email || '' }}</div>
        </div>
      </div>

      <!-- 功能列表占位 -->
      <div class="menu-card">
        <div class="menu-item" style="opacity: 0.4;">
          <span class="menu-icon">📊</span>
          <span class="menu-text">导出数据</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" style="opacity: 0.4;">
          <span class="menu-icon">🔔</span>
          <span class="menu-text">提醒设置</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" style="opacity: 0.4;">
          <span class="menu-icon">🔒</span>
          <span class="menu-text">修改密码</span>
          <span class="menu-arrow">›</span>
        </div>
      </div>

      <!-- 退出登录按钮 -->
      <button
        class="logout-btn"
        :class="{ 'logout-btn--loading': loggingOut }"
        :disabled="loggingOut"
        @click="showLogoutConfirm = true"
      >
        {{ loggingOut ? '退出中...' : '退出登录' }}
      </button>
    </div>

    <!-- 退出确认弹窗 -->
    <var-dialog
      v-model:show="showLogoutConfirm"
      title="提示"
      confirm-button-text="退出"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      @confirm="handleLogout"
      @cancel="showLogoutConfirm = false"
    >
      确定要退出登录吗？
    </var-dialog>
  </div>
</template>

<style scoped>
.profile-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(80px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: 16px;
  padding-top: calc(16px + env(safe-area-inset-top, 0px));
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* 用户信息卡片 */
.user-card {
  background: #fff;
  border-radius: 14px;
  padding: 20px 16px;
  display: flex;
  align-items: center;
  gap: 14px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, #FFF3E0, #FFE0B2);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.avatar-emoji {
  font-size: 28px;
  line-height: 1;
}
.user-info {
  flex: 1;
  min-width: 0;
}
.welcome {
  font-size: 12px;
  color: #FF6500;
  font-weight: 500;
  margin-bottom: 2px;
}
.nickname {
  font-size: 17px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.email {
  font-size: 13px;
  color: #999;
  margin-top: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 功能菜单卡片 */
.menu-card {
  background: #fff;
  border-radius: 14px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  overflow: hidden;
}
.menu-item {
  display: flex;
  align-items: center;
  padding: 15px 16px;
  cursor: pointer;
  transition: background 0.15s;
}
.menu-item:active {
  background: #f9f9f9;
}
.menu-icon {
  font-size: 20px;
  line-height: 1;
  margin-right: 12px;
}
.menu-text {
  flex: 1;
  font-size: 15px;
  color: #333;
}
.menu-arrow {
  font-size: 18px;
  color: #ccc;
  font-weight: 300;
}
.menu-divider {
  height: 1px;
  background: #f5f5f5;
  margin: 0 16px;
}

/* 退出登录按钮 */
.logout-btn {
  margin-top: 12px;
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: #fff;
  color: #e74c3c;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  transition: all 0.2s;
}
.logout-btn:active:not(:disabled) {
  background: #fef0ef;
  transform: scale(0.98);
}
.logout-btn--loading {
  color: #ccc;
}
</style>
