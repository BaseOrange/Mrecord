<script setup lang="ts">
const quickEntries = [
  { icon: 'plus-circle', label: '记账', color: '#FF6500' },
  { icon: 'calendar-month', label: '月报', color: '#4CAF50' },
  { icon: 'file-document-outline', label: '模板', color: '#2196F3' },
  { icon: 'share', label: '导出', color: '#9C27B0' },
  { icon: 'cog-outline', label: '设置', color: '#607D8B' },
]
</script>

<template>
  <div class="home-page">
    <!-- 顶部区域 -->
    <div class="page-header">
      <h2>月衡 <span class="header-en">Mrecord</span></h2>
    </div>

    <!-- 快捷入口 -->
    <div class="quick-section">
      <div class="quick-scroll">
        <div
          v-for="(entry, idx) in quickEntries"
          :key="idx"
          class="quick-card"
        >
          <div class="quick-icon-wrap" :style="{ backgroundColor: entry.color + '15' }">
            <var-icon :name="entry.icon" :size="28" :color="entry.color" />
          </div>
          <span class="quick-label">{{ entry.label }}</span>
        </div>
      </div>
    </div>

    <!-- Banner 卡片 -->
    <div class="banner-card">
      <div class="banner-content">
        <h3 class="banner-title">月衡 Mrecord</h3>
        <p class="banner-desc">轻松管理每月收支，让财务一目了然</p>
        <div class="banner-btn">立即记账 ›</div>
      </div>
      <div class="banner-decoration">
        <var-icon name="finance" :size="64" color="rgba(255,255,255,0.3)" />
      </div>
    </div>

    <!-- 内容占位区 -->
    <div class="content-placeholder">
      <div class="placeholder-block" style="height: 120px;"></div>
      <div class="placeholder-block" style="height: 160px;"></div>
      <div class="placeholder-block" style="height: 100px;"></div>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  min-height: 100vh;
  background: #f5f5f5;
}

/* 顶部区域 */
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

.header-en {
  font-size: 14px;
  font-weight: 400;
  color: #999;
  margin-left: 4px;
}

/* 快捷入口 */
.quick-section {
  padding: 16px 0 8px;
}

.quick-scroll {
  display: flex;
  gap: 12px;
  padding: 0 16px;
  overflow-x: auto;
  scroll-snap-type: x mandatory;
  -webkit-overflow-scrolling: touch;
  scrollbar-width: none;
}

.quick-scroll::-webkit-scrollbar {
  display: none;
}

.quick-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  min-width: 72px;
  scroll-snap-align: start;
}

.quick-icon-wrap {
  width: 56px;
  height: 56px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.quick-label {
  font-size: 12px;
  color: #666;
  white-space: nowrap;
}

/* Banner 卡片 */
.banner-card {
  margin: 12px 16px;
  padding: 20px;
  border-radius: 16px;
  background: linear-gradient(135deg, #FF6500 0%, #FF8A3D 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  overflow: hidden;
  position: relative;
}

.banner-content {
  flex: 1;
  z-index: 1;
}

.banner-title {
  font-size: 18px;
  font-weight: 700;
  color: #fff;
  margin-bottom: 6px;
}

.banner-desc {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.85);
  margin-bottom: 14px;
  line-height: 1.4;
}

.banner-btn {
  display: inline-block;
  font-size: 13px;
  font-weight: 600;
  color: #FF6500;
  background: #fff;
  padding: 6px 16px;
  border-radius: 20px;
}

.banner-decoration {
  position: absolute;
  right: 16px;
  bottom: -4px;
  opacity: 0.6;
}

/* 内容占位区 */
.content-placeholder {
  padding: 0 16px 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.placeholder-block {
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.03);
}
</style>
