<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

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
    <div class="tab-bar-wrapper">
      <div class="tab-bar">
        <div
          v-for="tab in tabs"
          :key="tab.key"
          class="tab-item"
          :class="{ active: activeTab === tab.key }"
          @click="switchTab(tab)"
        >
          <var-icon :name="tab.icon" :size="21" />
          <span class="tab-label">{{ tab.label }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background: #f5f5f5;
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
  z-index: 100;
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
  background: rgba(255, 255, 255, 0.78);
  backdrop-filter: blur(20px) saturate(1.4);
  -webkit-backdrop-filter: blur(20px) saturate(1.4);
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2px;
  box-shadow:
    0 4px 24px rgba(0, 0, 0, 0.07),
    0 1px 4px rgba(0, 0, 0, 0.03);
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
  border-radius: 21px;
  cursor: pointer;
  color: #b0b0b0;
  position: relative;
  transition: color 0.3s ease, background-color 0.3s ease;
  -webkit-tap-highlight-color: transparent;
}

/* 选中态：橙色胶囊高亮 */
.tab-item.active {
  color: #FF6500;
  background-color: rgba(255, 101, 0, 0.1);
}

.tab-label {
  font-size: 10px;
  margin-top: 2px;
  line-height: 1;
  font-weight: 500;
}

.tab-item.active .tab-label {
  font-weight: 600;
}
</style>
