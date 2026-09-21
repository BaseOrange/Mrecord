<script setup lang="ts">
import { useRouter } from 'vue-router'
import PageHeader from '@/components/PageHeader.vue'

const router = useRouter()

const menus = [
  { icon: '👥', label: '用户管理', desc: '查看、搜索、启用/停用、重置密码、删除用户', path: '/admin/users' },
  { icon: '📋', label: '操作日志', desc: '查看用户操作审计日志记录', path: '/admin/logs' },
  { icon: '⚙️', label: '系统配置', desc: '刷新配置项缓存', path: '/admin/config' },
  { icon: '📧', label: '邮箱服务配置', desc: '配置SMTP邮件发送服务参数', path: '/admin/email-config' },
  { icon: '🌐', label: '站点信息配置', desc: '配置站点地址、管理员邮箱、注册开关', path: '/admin/site-config' },
]
</script>

<template>
  <div class="admin-page">
    <PageHeader title="管理中心" show-back back-path="/profile" />

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
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
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