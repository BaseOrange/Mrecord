<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { getMyDataStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'

const router = useRouter()

const quickEntries = [
  { icon: 'plus-circle', label: '记账', color: '#FF6500', action: 'record' },
  { icon: 'calendar-month', label: '月报', color: '#4CAF50', action: 'stats' },
  { icon: 'file-document-outline', label: '模板', color: '#2196F3', action: 'template' },
  { icon: 'share', label: '导出', color: '#9C27B0', action: 'export' },
  { icon: 'cog-outline', label: '设置', color: '#607D8B', action: 'profile' },
]

const onQuickEntry = (action: string) => {
  switch (action) {
    case 'record':
      router.push('/book')
      break
    case 'stats':
      router.push('/stats')
      break
    case 'template':
      router.push('/book')
      break
    case 'export':
      Snackbar.info('即将上线')
      break
    case 'profile':
      router.push('/profile')
      break
  }
}

// ==================== 资产总览 ====================
const loading = ref(false)
const overview = ref({
  totalAsset: 0,
  totalLiability: 0,
  netAsset: 0,
  monthOnMonth: 0,
})
const bookSnapshots = ref<BookStatistics[]>([])

const fetchOverview = async () => {
  loading.value = true
  try {
    const res = await getMyDataStatistics()
    const data: BookStatistics[] = Array.isArray(res) ? res : (res as any)?.recordList || []
    bookSnapshots.value = data.filter((item: any) => item != null)
    overview.value = {
      totalAsset: bookSnapshots.value.reduce((s, r) => s + (r.totalAsset || 0), 0),
      totalLiability: bookSnapshots.value.reduce((s, r) => s + (r.totalLiability || 0), 0),
      netAsset: bookSnapshots.value.reduce((s, r) => s + (r.netAsset || 0), 0),
      monthOnMonth: bookSnapshots.value.reduce((s, r) => s + (r.monthOnMonth || 0), 0),
    }
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchOverview()
})

const formatMoney = (val: number) => {
  return val.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

const getChangeColor = (val: number) => {
  if (val === 0) return '#8e8e93'
  return val > 0 ? '#34c759' : '#ff3b30'
}

const getChangePrefix = (val: number) => {
  if (val === 0) return ''
  return val > 0 ? '+' : ''
}

const onBookCardClick = (item: BookStatistics) => {
  router.push({
    name: 'BookStatsDetail',
    params: { bookId: item.bookId },
    query: { name: item.bookName },
  })
}
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
          @click="onQuickEntry(entry.action)"
        >
          <div class="quick-icon-wrap" :style="{ backgroundColor: entry.color + '15' }">
            <var-icon :name="entry.icon" :size="28" :color="entry.color" />
          </div>
          <span class="quick-label">{{ entry.label }}</span>
        </div>
      </div>
    </div>

    <!-- 资产总览卡片 -->
    <div class="overview-card">
      <div class="overview-header">
        <span class="overview-title">资产总览</span>
      </div>

      <!-- 加载 / 无数据 -->
      <div v-if="loading" class="overview-loading">
        <div class="mini-spinner"></div>
      </div>
      <div v-else-if="bookSnapshots.length === 0" class="overview-empty">
        <p>暂无数据，去记一笔吧</p>
      </div>

      <template v-else>
        <div class="overview-main">
          <span class="overview-label">净资产</span>
          <div class="overview-net-row">
            <span
              class="overview-net-value"
              :style="{ color: overview.netAsset >= 0 ? '#34c759' : '#ff3b30' }"
            >
              {{ formatMoney(overview.netAsset) }}
            </span>
            <span
              class="overview-mom"
              :style="{ color: getChangeColor(overview.monthOnMonth) }"
            >
              {{ getChangePrefix(overview.monthOnMonth) }}{{ formatMoney(overview.monthOnMonth) }}
            </span>
          </div>
        </div>
        <div class="overview-sub">
          <div class="overview-sub-item">
            <span class="overview-sub-label">总资产</span>
            <span class="overview-sub-num">{{ formatMoney(overview.totalAsset) }}</span>
          </div>
          <div class="overview-sub-divider"></div>
          <div class="overview-sub-item">
            <span class="overview-sub-label">总负债</span>
            <span class="overview-sub-num" style="color: #ff3b30;">{{ formatMoney(overview.totalLiability) }}</span>
          </div>
        </div>
      </template>
    </div>

    <!-- 各账簿横向滚动 -->
    <div v-if="bookSnapshots.length > 0" class="books-section">
      <div class="section-title">账簿一览</div>
      <div class="books-scroll">
        <div
          v-for="item in bookSnapshots"
          :key="item.bookId"
          class="book-mini-card"
          @click="onBookCardClick(item)"
        >
          <span class="book-mini-name">{{ item.bookName }}</span>
          <span
            class="book-mini-net"
            :style="{ color: (item.netAsset || 0) >= 0 ? '#34c759' : '#ff3b30' }"
          >
            {{ formatMoney(item.netAsset || 0) }}
          </span>
          <span
            class="book-mini-mom"
            :style="{ color: getChangeColor(item.monthOnMonth || 0) }"
          >
            {{ getChangePrefix(item.monthOnMonth || 0) }}{{ formatMoney(item.monthOnMonth || 0) }}
          </span>
        </div>
      </div>
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
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: transform 0.15s;
}
.quick-card:active {
  transform: scale(0.92);
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

/* ==================== 资产总览卡片 ==================== */
.overview-card {
  margin: 12px 16px;
  background: #fff;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.05);
}

.overview-header {
  margin-bottom: 14px;
}

.overview-title {
  font-size: 15px;
  font-weight: 600;
  color: #1d1d1f;
}

.overview-loading {
  display: flex;
  justify-content: center;
  padding: 16px 0;
}

.mini-spinner {
  width: 20px;
  height: 20px;
  border: 2.5px solid #e0e0e0;
  border-top-color: #FF6500;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.overview-empty {
  text-align: center;
  padding: 16px 0;
  color: #aeaeb2;
  font-size: 14px;
}

.overview-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 14px;
}

.overview-label {
  font-size: 13px;
  color: #8e8e93;
}

.overview-net-row {
  display: flex;
  align-items: baseline;
  gap: 10px;
}

.overview-net-value {
  font-size: 32px;
  font-weight: 700;
  letter-spacing: -1px;
}

.overview-mom {
  font-size: 14px;
  font-weight: 600;
}

.overview-sub {
  display: flex;
  align-items: center;
  background: #f9f9f9;
  border-radius: 12px;
  padding: 12px 0;
}

.overview-sub-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.overview-sub-label {
  font-size: 11px;
  color: #aeaeb2;
}

.overview-sub-num {
  font-size: 15px;
  font-weight: 600;
  color: #333;
}

.overview-sub-divider {
  width: 1px;
  height: 28px;
  background: #e0e0e0;
}

/* ==================== 账簿一览 ==================== */
.books-section {
  padding: 0 16px 24px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #8e8e93;
  margin-bottom: 10px;
}

.books-scroll {
  display: flex;
  gap: 10px;
  overflow-x: auto;
  scroll-snap-type: x mandatory;
  -webkit-overflow-scrolling: touch;
  scrollbar-width: none;
  padding-bottom: 4px;
}

.books-scroll::-webkit-scrollbar {
  display: none;
}

.book-mini-card {
  min-width: 140px;
  background: #fff;
  border-radius: 14px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  scroll-snap-align: start;
  cursor: pointer;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  transition: all 0.15s;
  -webkit-tap-highlight-color: transparent;
}

.book-mini-card:active {
  transform: scale(0.97);
  background: #fafafa;
}

.book-mini-name {
  font-size: 13px;
  font-weight: 600;
  color: #1d1d1f;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.book-mini-net {
  font-size: 18px;
  font-weight: 700;
}

.book-mini-mom {
  font-size: 12px;
  font-weight: 600;
}
</style>
