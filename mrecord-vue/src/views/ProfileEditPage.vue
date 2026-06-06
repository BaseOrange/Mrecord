<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { useUserStore } from '@/stores/user'
import { updateMyInfo } from '@/api'

const router = useRouter()
const userStore = useUserStore()

const nickname = ref(userStore.userInfo?.nickname || '')
const remindEnabled = ref(userStore.userInfo?.remindEnabled === 1)
const remindDay = ref(userStore.userInfo?.remindDay || 1)
const loading = ref(false)

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
    router.back()
  } catch (e: any) {
    Snackbar.error(e?.message || '保存失败')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="profile-edit-page">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="header-back" @click="router.back()">
        <var-icon name="chevron-left" :size="18" color="#333" />
      </button>
      <h2>个人资料</h2>
      <div class="header-placeholder"></div>
    </div>

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
  </div>
</template>

<style scoped>
.profile-edit-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

/* 顶部导航 */
.page-header {
  background: #fff;
  padding: calc(16px + env(safe-area-inset-top, 0px)) 16px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0;
  line-height: 1;
}
.header-back {
  width: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 50%;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.header-back:active {
  background: #f5f5f5;
}
.header-placeholder {
  width: 32px;
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
