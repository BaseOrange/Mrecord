<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { refreshCache } from '@/api'

const router = useRouter()
const refreshing = ref(false)

async function handleRefreshCache() {
  refreshing.value = true
  try {
    await refreshCache()
    Snackbar.success('配置缓存刷新成功')
  } catch {
    // 拦截器处理
  } finally {
    refreshing.value = false
  }
}
</script>

<template>
  <div class="admin-config-page">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="back-btn" @click="router.back()">
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>系统配置</h2>
      <span class="header-spacer"></span>
    </div>

    <div class="page-body">
      <!-- 刷新缓存 -->
      <div class="config-card">
        <div class="config-info">
          <div class="config-icon">🔄</div>
          <div class="config-text">
            <div class="config-label">刷新配置缓存</div>
            <div class="config-desc">清除服务端配置项缓存，重新从数据库加载最新配置</div>
          </div>
        </div>
        <button
          class="config-btn"
          :class="{ 'config-btn--loading': refreshing }"
          :disabled="refreshing"
          @click="handleRefreshCache"
        >
          <span v-if="!refreshing">刷新</span>
          <span v-else class="btn-loading">
            <svg class="spinner" viewBox="0 0 24 24" width="18" height="18">
              <circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
            </svg>
          </span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.admin-config-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(80px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: 12px 16px;
  padding-top: calc(12px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #f0f0f0;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}
.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: #f5f5f5;
  color: #666;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;
}
.back-btn:active {
  background: #eee;
  color: #FF6500;
}
.header-spacer {
  width: 36px;
}

.page-body {
  padding: 16px;
}

.config-card {
  background: #fff;
  border-radius: 14px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.config-info {
  display: flex;
  align-items: flex-start;
  gap: 14px;
  margin-bottom: 16px;
}

.config-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: linear-gradient(135deg, #FFF3E0, #FFE0B2);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  flex-shrink: 0;
}

.config-text {
  flex: 1;
}

.config-label {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
}

.config-desc {
  font-size: 12px;
  color: #999;
  line-height: 1.5;
}

.config-btn {
  width: 100%;
  height: 44px;
  border: none;
  border-radius: 12px;
  background: linear-gradient(135deg, #FF8C42, #FF6500);
  color: #fff;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 2px;
  cursor: pointer;
  transition: all 0.2s;
}
.config-btn:active:not(:disabled) {
  opacity: 0.85;
  transform: scale(0.98);
}
.config-btn--loading {
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
</style>