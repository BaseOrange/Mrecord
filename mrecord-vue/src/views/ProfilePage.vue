<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { useUserStore } from '@/stores/user'
import { logout, canceledMyUser } from '@/api'
import AgreementPopup from '@/components/AgreementPopup.vue'
import PageHeader from '@/components/PageHeader.vue'
import DiagnosticSheet from '@/components/DiagnosticSheet.vue'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import ListGroup from '@/components/ListGroup.vue'
import ListCell from '@/components/ListCell.vue'

const router = useRouter()
const userStore = useUserStore()

// 首字母头像（取代 emoji，Q1：Apple HIG 用字字符号/字母 monogram）
const avatarText = computed(() => {
  const name = userStore.userInfo?.nickname?.trim() || ''
  if (name) return name[0].toUpperCase()
  const email = userStore.userInfo?.email?.trim() || ''
  return email ? email[0].toUpperCase() : '?'
})
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
/**
 * 连点冷却：面板打开（以及关闭后）的一小段时间内不再计数。
 * 连点第 5 次弹出面板时，手指往往还有余下的点击，不加冷却会在关闭后立刻又弹出来。
 */
const TAP_COOLDOWN_MS = 1500
let titleTapCooldownUntil = 0

const handleTitleTap = () => {
    // 面板已打开时不再计数；冷却期内的点击一律忽略
    if (showDiagnostic.value) return
    if (Date.now() < titleTapCooldownUntil) {
        titleTapCount.value = 0
        return
    }
    titleTapCount.value += 1
    if (titleTapTimer) clearTimeout(titleTapTimer)
    // 1.5 秒内未点满 5 次则重置计数，防止缓慢连点误触发
    titleTapTimer = setTimeout(() => {
        titleTapCount.value = 0
    }, 1500)
    if (titleTapCount.value >= 5) {
        titleTapCount.value = 0
        if (titleTapTimer) clearTimeout(titleTapTimer)
        titleTapCooldownUntil = Date.now() + TAP_COOLDOWN_MS
        showDiagnostic.value = true
    }
}

/** 面板关闭后同样进入冷却，避免连点余波立刻重新弹出 */
watch(showDiagnostic, (visible) => {
    if (!visible) titleTapCooldownUntil = Date.now() + TAP_COOLDOWN_MS
})

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
    <PageHeader title="我的" large @title-click="handleTitleTap" />

    <div class="page-body">
      <!-- 用户信息卡片 -->
      <div class="user-card">
        <div class="avatar">
          <span class="avatar-text">{{ avatarText }}</span>
        </div>
        <div class="user-info">
          <div class="welcome">欢迎回来</div>
          <div class="nickname">{{ userStore.userInfo?.nickname || '未设置昵称' }}</div>
          <div class="email">{{ userStore.userInfo?.email || '' }}</div>
        </div>
      </div>

      <!-- 外观切换（跟随系统 / 浅色 / 深色） -->
      <div class="theme-card">
        <span class="theme-label">外观</span>
        <ThemeSwitcher />
      </div>

      <!-- 管理员入口 -->
      <div v-if="userStore.userInfo?.admin === 1" class="admin-card" @click="router.push('/admin')">
        <span class="admin-badge">管理员</span>
        <span class="admin-label">管理中心</span>
        <span class="admin-arrow" aria-hidden="true">›</span>
      </div>

      <!-- 功能列表 -->
      <ListGroup>
        <ListCell
          icon="file-text"
          label="用户协议及隐私政策"
          @click="showAgreement = true"
        />
        <ListCell icon="download" label="导出数据" @click="router.push('/export')" />
        <ListCell
          icon="circle-user-round"
          label="个人资料"
          @click="router.push('/profile-edit')"
        />
        <ListCell icon="lock" label="修改密码" @click="router.push('/change-password')" />
        <ListCell icon="circle-alert" label="注销账户" danger @click="showCancelStep1 = true" />
      </ListGroup>

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
      confirm-button-color="var(--brand)"
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
      confirm-button-color="var(--semantic-danger)"
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
      confirm-button-color="var(--semantic-danger)"
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
  background: var(--bg-canvas);
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.page-body {
  padding: var(--space-3) var(--page-padding) 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

/* 用户信息卡片 */
.user-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-5) var(--space-4);
  display: flex;
  align-items: center;
  gap: var(--space-4);
  box-shadow: var(--shadow-sm);
}
.avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--brand), var(--brand-accent));
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  box-shadow: 0 4px 12px rgba(255, 101, 0, 0.28);
}
.avatar-text {
  font-size: 24px;
  font-weight: var(--weight-bold);
  color: #fff;
  line-height: 1;
}
.user-info {
  flex: 1;
  min-width: 0;
}
.welcome {
  font-size: var(--text-sm);
  color: var(--brand);
  font-weight: var(--weight-medium);
  margin-bottom: 2px;
}
.nickname {
  font-size: var(--text-title-3);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.email {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin-top: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 外观切换卡片 */
.theme-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  padding: var(--space-3) var(--space-4);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
}
.theme-label {
  font-size: var(--text-body);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  flex-shrink: 0;
}

/* 管理员入口卡片 */
.admin-card {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  background: linear-gradient(135deg, var(--brand-soft), rgba(255, 122, 31, 0.16));
  border: 1px solid rgba(255, 101, 0, 0.18);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: transform var(--duration-fast) var(--ease-out), opacity var(--duration-fast);
}
.admin-card:active {
  transform: scale(0.98);
  opacity: 0.85;
}
.admin-label {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--brand);
}
.admin-badge {
  display: inline-flex;
  align-items: center;
  font-size: 10px;
  font-weight: var(--weight-semibold);
  color: #fff;
  background: linear-gradient(135deg, var(--brand-accent), var(--brand));
  padding: 2px 7px;
  border-radius: var(--radius-pill);
  line-height: 1.4;
}
.admin-arrow {
  margin-left: auto;
  font-size: 18px;
  color: var(--brand);
  font-weight: 300;
}

/* 注销确认弹窗提示文案 */
.cancel-tips {
  font-size: var(--text-body);
  color: var(--text-secondary);
  line-height: 1.8;
}
.cancel-tips b {
  color: var(--semantic-danger);
  font-weight: var(--weight-semibold);
}

/* 退出登录按钮 */
.logout-btn {
  margin-top: var(--space-3);
  width: 100%;
  height: 48px;
  border: none;
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  color: var(--semantic-danger);
  font-size: var(--text-body);
  font-weight: var(--weight-medium);
  cursor: pointer;
  box-shadow: var(--shadow-sm);
  -webkit-tap-highlight-color: transparent;
  transition: transform var(--duration-fast) var(--ease-out), background-color var(--duration-fast);
}
.logout-btn:active:not(:disabled) {
  background: var(--bg-surface-2);
  transform: scale(0.98);
}
.logout-btn--loading {
  color: var(--text-tertiary);
}
</style>
