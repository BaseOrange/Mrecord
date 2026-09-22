<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import AppIcon from '@/components/AppIcon.vue'

const router = useRouter()
const route = useRoute()

const tabs = [
  { key: 'home', label: '首页', icon: 'home', path: '/home' },
  { key: 'stats', label: '统计', icon: 'arrow-up-bold-box-outline', path: '/stats' },
  { key: 'book', label: '账簿', icon: 'notebook', path: '/book' },
  { key: 'profile', label: '我的', icon: 'account-circle', path: '/profile' },
]

const activeTab = computed(() => {
  const path = route.path
  // 管理员页面、导出、修改密码、个人资料编辑等属于"我的"入口
  if (path.startsWith('/admin') || path === '/export' || path === '/change-password' || path === '/profile-edit') return 'profile'
  const tab = tabs.find(t => path.startsWith(t.path))
  return tab ? tab.key : 'home'
})

const switchTab = (tab: typeof tabs[number]) => {
  if (route.path !== tab.path) {
    router.replace(tab.path)
  }
}
</script>

<template>
  <div class="layout">
    <div class="layout-content">
      <router-view />
    </div>

    <!-- 底部导航栏 -->
    <nav class="tab-bar-wrapper" aria-label="主导航">
      <div class="tab-bar">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          type="button"
          class="tab-item"
          :class="{ active: activeTab === tab.key }"
          :aria-current="activeTab === tab.key ? 'page' : undefined"
          @click="switchTab(tab)"
        >
          <AppIcon :name="tab.icon" :size="21" />
          <span class="tab-label">{{ tab.label }}</span>
        </button>
      </div>
    </nav>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background: var(--bg-canvas);
}

.layout-content {
  flex: 1;
  overflow-y: auto;
  padding-bottom: calc(72px + env(safe-area-inset-bottom, 0px));
}

/* 底部导航 - 外层定位容器 */
.tab-bar-wrapper {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: var(--z-tab-bar);
  display: flex;
  justify-content: center;
  padding: 0 16px;
  padding-bottom: calc(8px + env(safe-area-inset-bottom, 0px));
  pointer-events: none;
}

/* 胶囊形毛玻璃容器 */
.tab-bar {
  pointer-events: auto;
  width: 100%;
  max-width: 300px;
  height: 52px;
  background: var(--header-bg);
  backdrop-filter: blur(20px) saturate(1.4);
  -webkit-backdrop-filter: blur(20px) saturate(1.4);
  border-radius: var(--radius-pill);
  border: 1px solid var(--separator);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2px;
  box-shadow: var(--shadow-md);
  padding: 0 4px;
}

/* 每个导航项 */
.tab-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 72px;
  height: 42px;
  border-radius: var(--radius-pill);
  cursor: pointer;
  color: var(--text-tertiary);
  position: relative;
  transition: color var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out);
  -webkit-tap-highlight-color: transparent;
}

/* 选中态：橙色胶囊高亮 */
.tab-item.active {
  color: var(--brand);
  background-color: var(--brand-soft);
}

.tab-label {
  font-size: var(--text-xs);
  margin-top: 2px;
  line-height: 1;
  font-weight: var(--weight-medium);
}

.tab-item.active .tab-label {
  font-weight: var(--weight-semibold);
}
</style>
