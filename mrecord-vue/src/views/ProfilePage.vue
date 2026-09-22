<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { useUserStore } from '@/stores/user'
import { logout, canceledMyUser } from '@/api'
import AgreementPopup from '@/components/AgreementPopup.vue'
import PageHeader from '@/components/PageHeader.vue'
import DiagnosticSheet from '@/components/DiagnosticSheet.vue'

const router = useRouter()
const userStore = useUserStore()
const loggingOut = ref(false)
const showLogoutConfirm = ref(false)
const showAgreement = ref(false)
// 注销账户：两步确认
const showCancelStep1 = ref(false)
const showCancelStep2 = ref(false)
const cancelling = ref(false)

// ==================== 诊断信息入口（彩蛋式） ====================
// 「我的」页面顶部标题 1.5 秒内点击 5 次打开诊断面板，供用户提 GitHub issue 时
// 复制环境信息。无中间进度提示，避免普通用户误触看到技术信息。
const showDiagnostic = ref(false)
const titleTapCount = ref(0)
let titleTapTimer: ReturnType<typeof setTimeout> | null = null

const handleTitleTap = () => {
    titleTapCount.value += 1
    if (titleTapTimer) clearTimeout(titleTapTimer)
    // 1.5 秒内未点满 5 次则重置计数，防止缓慢连点误触发
    titleTapTimer = setTimeout(() => {
        titleTapCount.value = 0
    }, 1500)
    if (titleTapCount.value >= 5) {
        titleTapCount.value = 0
        if (titleTapTimer) clearTimeout(titleTapTimer)
        showDiagnostic.value = true
    }
}

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

const handleCancelAccount = async () => {
  showCancelStep2.value = false
  cancelling.value = true
  try {
    await canceledMyUser()
    // 注销申请提交成功后，本地登录态立即失效（账号已进入「注销待生效」状态）
    userStore.logout()
    Snackbar.success('已提交注销申请，进入 15 天冷静期')
    router.replace('/login')
  } catch {
    // 拦截器已处理错误提示
  } finally {
    cancelling.value = false
  }
}
</script>

<template>
  <div class="profile-page">
    <!-- 顶部标题（1.5 秒内点击 5 次打开诊断面板） -->
    <PageHeader title="我的" @title-click="handleTitleTap" />

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

      <!-- 管理员入口 -->
      <div v-if="userStore.userInfo?.admin === 1" class="menu-card admin-card" @click="router.push('/admin')">
        <div class="menu-item">
          <var-icon name="shield-outline" :size="22" class="menu-svg-icon" />
          <span class="menu-text">
            <span class="admin-label">管理中心</span>
            <span class="admin-badge">管理员</span>
          </span>
          <span class="menu-arrow">›</span>
        </div>
      </div>

      <!-- 功能列表 -->
      <div class="menu-card">
        <div class="menu-item" @click="showAgreement = true">
          <var-icon name="file-text-outline" :size="22" class="menu-svg-icon" />
          <span class="menu-text">用户协议及隐私政策</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" @click="router.push('/export')">
          <var-icon name="download-outline" :size="22" class="menu-svg-icon" />
          <span class="menu-text">导出数据</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" @click="router.push('/profile-edit')">
          <var-icon name="account-circle-outline" :size="22" class="menu-svg-icon" />
          <span class="menu-text">个人资料</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" @click="router.push('/change-password')">
          <var-icon name="lock-outline" :size="22" class="menu-svg-icon" />
          <span class="menu-text">修改密码</span>
          <span class="menu-arrow">›</span>
        </div>
        <div class="menu-divider"></div>
        <div class="menu-item" @click="showCancelStep1 = true">
          <var-icon name="alert-circle-outline" :size="22" class="menu-svg-icon danger-svg-icon" />
          <span class="menu-text danger-text">注销账户</span>
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

    <!-- 注销确认 第一步：说明后果 -->
    <var-dialog
      v-model:show="showCancelStep1"
      title="注销账户"
      confirm-button-text="继续注销"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      @confirm="showCancelStep1 = false; showCancelStep2 = true"
      @cancel="showCancelStep1 = false"
    >
      <div class="cancel-tips">
        注销后账户将进入 <b>15 天冷静期</b>，期间：<br />
        · 账号将<b>无法登录</b>；<br />
        · 冷静期内可在登录页<b>撤销注销</b>恢复账号；<br />
        · 超过冷静期后，名下账簿及全部数据将被备份并<b>永久删除</b>。
      </div>
    </var-dialog>

    <!-- 注销确认 第二步：最终确认 -->
    <var-dialog
      v-model:show="showCancelStep2"
      title="最终确认"
      confirm-button-text="确认注销"
      cancel-button-text="我再想想"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      :confirm-button-disabled="cancelling"
      @confirm="handleCancelAccount"
      @cancel="showCancelStep2 = false"
    >
      <div class="cancel-tips">
        注销操作不可立即撤回，请确认你了解上述后果。<br />
        点击「确认注销」后立即生效。
      </div>
    </var-dialog>

    <!-- 协议弹窗 -->
    <AgreementPopup v-model:show="showAgreement" />

    <!-- 诊断信息弹层 -->
    <DiagnosticSheet v-model:show="showDiagnostic" />
  </div>
</template>

<style scoped>
.profile-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
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

/* 管理员入口卡片 */
.admin-card {
  background: linear-gradient(135deg, #FFF8F0, #FFF0E0);
  border: 1px solid rgba(255, 101, 0, 0.12);
  cursor: pointer;
  transition: all 0.2s;
}
.admin-card:active {
  background: linear-gradient(135deg, #FFF0E0, #FFE8D0);
  transform: scale(0.98);
}
.admin-label {
  font-size: 15px;
  font-weight: 600;
  color: #FF6500;
}
.admin-badge {
  display: inline-block;
  font-size: 10px;
  font-weight: 600;
  color: #fff;
  background: linear-gradient(135deg, #FF8C42, #FF6500);
  padding: 1px 6px;
  border-radius: 8px;
  margin-left: 6px;
  vertical-align: middle;
  line-height: 1.5;
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
.menu-svg-icon {
  margin-right: 12px;
  color: #888;
  flex-shrink: 0;
}
.menu-svg-icon.danger-svg-icon {
  color: #ff4d4f;
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

/* 注销账户危险项 */
.danger-text {
  color: #e74c3c;
  font-weight: 500;
}

/* 注销确认弹窗提示文案 */
.cancel-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
}
.cancel-tips b {
  color: #e74c3c;
  font-weight: 600;
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
