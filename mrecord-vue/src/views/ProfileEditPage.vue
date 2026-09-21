<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { useUserStore } from '@/stores/user'
import { updateMyInfo } from '@/api'
import PageHeader from '@/components/PageHeader.vue'

const router = useRouter()
const userStore = useUserStore()

const nickname = ref(userStore.userInfo?.nickname || '')
const remindEnabled = ref(userStore.userInfo?.remindEnabled === 1)
const remindDay = ref(userStore.userInfo?.remindDay || 1)
const loading = ref(false)

// I6：记录初始值，用于脏数据检测
const initialNickname = ref(nickname.value)
const initialRemindEnabled = ref(remindEnabled.value)
const initialRemindDay = ref(remindDay.value)

// I6：是否有未保存的修改
const isDirty = computed(() =>
  nickname.value.trim() !== initialNickname.value ||
  remindEnabled.value !== initialRemindEnabled.value ||
  remindDay.value !== initialRemindDay.value
)

// 提醒日期选项 1-31
const dayOptions = Array.from({ length: 31 }, (_, i) => i + 1)

// 当关闭提醒时，重置提醒日期
watch(remindEnabled, (val) => {
  if (!val) {
    remindDay.value = 1
  }
})

const onSubmit = async () => {
  if (!nickname.value.trim()) {
    Snackbar.warning('请输入昵称')
    return
  }
  if (nickname.value.trim().length > 20) {
    Snackbar.warning('昵称不能超过20个字符')
    return
  }

  loading.value = true
  try {
    const res = await updateMyInfo({
      nickname: nickname.value.trim(),
      remindEnabled: remindEnabled.value ? 1 : 0,
      remindDay: remindEnabled.value ? remindDay.value : undefined,
    })
    userStore.setUserInfo(res)
    Snackbar.success('保存成功')
    // I11：历史栈兜底
    if (window.history.length <= 1) {
      router.replace('/profile')
    } else {
      router.back()
    }
  } catch {
    // 拦截器已统一弹出后端报文，页面不再重复提示（I8）
  } finally {
    loading.value = false
  }
}

// I6：返回前确认（有未保存修改时）
const showBackConfirm = ref(false)

const goBack = () => {
  if (isDirty.value) {
    showBackConfirm.value = true
    return
  }
  if (window.history.length <= 1) {
    router.replace('/profile')
  } else {
    router.back()
  }
}

const confirmBack = () => {
  showBackConfirm.value = false
  if (window.history.length <= 1) {
    router.replace('/profile')
  } else {
    router.back()
  }
}
</script>

<template>
  <div class="profile-edit-page">
    <!-- 顶部导航（I11 + Q1：统一用 PageHeader，含历史栈兜底） -->
    <PageHeader title="个人资料" show-back>
      <template #right>
        <button class="header-save-btn" :disabled="loading" @click="goBack">
          完成
        </button>
      </template>
    </PageHeader>

    <div class="page-body">
      <!-- 昵称卡片 -->
      <div class="form-card">
        <div class="form-item">
          <label class="form-label">昵称</label>
          <div class="input-wrapper">
            <input
              v-model="nickname"
              placeholder="请输入昵称"
              class="form-input"
              maxlength="20"
            />
          </div>
        </div>
      </div>

      <!-- 提醒设置卡片 -->
      <div class="form-card">
        <div class="section-title">提醒设置</div>
        <div class="form-item">
          <div class="switch-row">
            <div class="switch-info">
              <div class="switch-label">邮件提醒</div>
              <div class="switch-desc">每月定期邮件提醒记账</div>
            </div>
            <var-switch
              v-model="remindEnabled"
              :color="'#FF6500'"
              :close-color="'#e0e0e0'"
              size="22"
            />
          </div>
        </div>

        <div v-if="remindEnabled" class="form-item remind-day-item">
          <label class="form-label">每月提醒日期</label>
          <div class="day-picker">
            <button
              v-for="day in dayOptions"
              :key="day"
              class="day-btn"
              :class="{ 'day-btn--active': remindDay === day }"
              @click="remindDay = day"
              type="button"
            >
              {{ day }}
            </button>
          </div>
          <p class="remind-day-hint">若当月无该日期（如2月无30/31号），将自动取月末最后一天</p>
        </div>
      </div>

      <!-- 保存按钮 -->
      <button
        class="submit-btn"
        :class="{ 'submit-btn--loading': loading }"
        :disabled="loading"
        @click="onSubmit"
      >
        {{ loading ? '保存中...' : '保存' }}
      </button>
    </div>

    <!-- I6：返回前确认（有未保存修改时） -->
    <var-dialog
      v-model:show="showBackConfirm"
      title="放弃修改"
      confirm-button-text="放弃并返回"
      cancel-button-text="继续编辑"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      @confirm="confirmBack"
    >
      <div class="back-confirm-tips">
        当前有未保存的资料修改，返回后将丢失。确定要放弃修改并返回吗？
      </div>
    </var-dialog>
  </div>
</template>

<style scoped>
.profile-edit-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

/* 顶部导航样式已迁移至 PageHeader.vue（Q1）；完成按钮保留本页样式 */
.header-save-btn {
  font-size: 15px;
  font-weight: 500;
  color: #FF6500;
  background: none;
  border: none;
  padding: 6px 12px;
  border-radius: 8px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.header-save-btn:disabled {
  opacity: 0.5;
}

/* I6：返回确认弹窗 */
.back-confirm-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
}

/* 页面主体 */
.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* 表单卡片 */
.form-card {
  background: #fff;
  border-radius: 16px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
}

.form-item {
  margin-bottom: 16px;
}
.form-item:last-child {
  margin-bottom: 0;
}
.form-label {
  display: block;
  font-size: 13px;
  color: #666;
  font-weight: 500;
  margin-bottom: 8px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 12px;
  padding: 0 12px;
  height: 48px;
  transition: all 0.2s;
}
.input-wrapper:focus-within {
  border-color: #FF6500;
  background: #fff;
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
  color: #bbb;
}

/* 开关行 */
.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.switch-info {
  flex: 1;
  min-width: 0;
}
.switch-label {
  font-size: 15px;
  color: #333;
  font-weight: 500;
}
.switch-desc {
  font-size: 12px;
  color: #999;
  margin-top: 2px;
}

/* 日期选择器 */
.remind-day-item {
  margin-top: 12px;
  padding-top: 16px;
  border-top: 1px solid #f0f0f0;
}
.day-picker {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
}
.day-btn {
  height: 36px;
  border: 1px solid #e8e8e8;
  border-radius: 8px;
  background: #fafafa;
  color: #333;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  -webkit-tap-highlight-color: transparent;
}
.day-btn:active {
  transform: scale(0.95);
}
.day-btn--active {
  background: #FF6500;
  border-color: #FF6500;
  color: #fff;
}

.remind-day-hint {
  margin: 10px 0 0;
  font-size: 12px;
  color: #999;
  line-height: 1.5;
}

/* 提交按钮 */
.submit-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: #FF6500;
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.submit-btn:active:not(:disabled) {
  background: #e05800;
  transform: scale(0.98);
}
.submit-btn--loading {
  opacity: 0.7;
}
</style>
