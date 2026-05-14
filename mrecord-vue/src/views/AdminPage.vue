<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const menus = [
  { icon: '👥', label: '用户管理', desc: '查看、搜索、启用/停用、重置密码、删除用户', path: '/admin/users' },
  { icon: '📋', label: '操作日志', desc: '查看用户操作审计日志记录', path: '/admin/logs' },
  { icon: '⚙️', label: '系统配置', desc: '刷新配置项缓存', path: '/admin/config' },
]
</script>

<template>
  <div class="admin-page">
    <div class="page-header">
      <button class="back-btn" @click="router.back()">
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>管理中心</h2>
      <span class="header-spacer"></span>
    </div>

    <div class="page-body">
      <div v-for="menu in menus" :key="menu.path" class="admin-card" @click="router.push(menu.path)">
        <div class="card-icon">{{ menu.icon }}</div>
        <div class="card-info">
          <div class="card-label">{{ menu.label }}</div>
          <div class="card-desc">{{ menu.desc }}</div>
        </div>
        <span class="card-arrow">›</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.admin-page {
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
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.admin-card {
  background: #fff;
  border-radius: 14px;
  padding: 18px 16px;
  display: flex;
  align-items: center;
  gap: 14px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  cursor: pointer;
  transition: all 0.2s;
}
.admin-card:active {
  background: #fafafa;
  transform: scale(0.98);
}
.card-icon {
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
.card-info {
  flex: 1;
  min-width: 0;
}
.card-label {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
}
.card-desc {
  font-size: 12px;
  color: #999;
  line-height: 1.4;
}
.card-arrow {
  font-size: 20px;
  color: #ccc;
  font-weight: 300;
  flex-shrink: 0;
}
</style>