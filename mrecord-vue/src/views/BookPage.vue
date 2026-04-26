<script setup lang="ts">
import { ref } from 'vue'
import { Snackbar } from '@varlet/ui'
import { createBook } from '@/api/modules/book'

// ---- 创建账簿弹窗 ----
const showCreateDialog = ref(false)
const newBookName = ref('')
const creating = ref(false)

const openCreateDialog = () => {
  newBookName.value = ''
  showCreateDialog.value = true
}

const handleCreate = async () => {
  const name = newBookName.value.trim()
  if (!name) {
    Snackbar.warning('请输入账簿名称')
    return
  }
  creating.value = true
  try {
    await createBook({ bookName: name })
    Snackbar.success('创建成功')
    showCreateDialog.value = false
    // TODO: 刷新账簿列表
  } catch {
    // 拦截器已处理错误提示
  } finally {
    creating.value = false
  }
}
</script>

<template>
  <div class="book-page">
    <!-- 顶部标题 -->
    <div class="page-header">
      <h2>账簿</h2>
    </div>

    <!-- 空状态 -->
    <div class="page-body">
      <div class="empty-state">
        <var-icon name="notebook" :size="56" color="#ddd" />
        <p class="empty-text">还没有账簿</p>
        <p class="empty-sub">点击右下角按钮创建第一个账簿</p>
      </div>
    </div>

    <!-- 悬浮创建按钮 -->
    <div class="fab" @click="openCreateDialog">
      <var-icon name="plus" :size="26" color="#fff" />
    </div>

    <!-- 创建账簿弹窗 -->
    <var-dialog
      v-model:show="showCreateDialog"
      title="创建账簿"
      confirm-button-text="创建"
      cancel-button-text="取消"
      :confirm-button-loading="creating"
      @confirm="handleCreate"
    >
      <var-input
        v-model="newBookName"
        placeholder="请输入账簿名称"
        :maxlength="20"
        clearable
        autofocus
        @keyup.enter="handleCreate"
      />
    </var-dialog>
  </div>
</template>

<style scoped>
.book-page {
  min-height: 100vh;
  background: #f5f5f5;
}

/* 顶部标题 */
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

/* 页面主体 */
.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 0 40px;
}
.empty-text {
  margin-top: 16px;
  font-size: 16px;
  font-weight: 500;
  color: #999;
}
.empty-sub {
  margin-top: 6px;
  font-size: 13px;
  color: #bbb;
}

/* 悬浮按钮 */
.fab {
  position: fixed;
  right: 20px;
  bottom: calc(88px + env(safe-area-inset-bottom, 0px));
  width: 52px;
  height: 52px;
  border-radius: 50%;
  background: linear-gradient(135deg, #FF8A3A, #FF6500);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 16px rgba(255, 101, 0, 0.35);
  cursor: pointer;
  z-index: 90;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  -webkit-tap-highlight-color: transparent;
}
.fab:active {
  transform: scale(0.92);
  box-shadow: 0 2px 8px rgba(255, 101, 0, 0.3);
}
</style>
