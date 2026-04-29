<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getBookDetailedStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'
import TrendChart from '@/components/TrendChart.vue'

const route = useRoute()
const router = useRouter()
const bookId = route.params.bookId as string
const bookName = (route.query.name as string) || '账簿详情'

const loading = ref(false)
const records = ref<BookStatistics[]>([])

const fetchDetail = async () => {
  loading.value = true
  try {
    const res = await getBookDetailedStatistics({ id: bookId })
    // 按时间正序排列（旧 → 新），便于折线图展示
    records.value = (res.recordList || []).sort((a, b) => {
      const ta = (a.year || 0) * 100 + (a.month || 0)
      const tb = (b.year || 0) * 100 + (b.month || 0)
      return ta - tb
    })
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchDetail()
})

// 月份标签，如 "2024-01"
const labels = computed(() =>
  records.value.map(r => `${r.year}-${String(r.month).padStart(2, '0')}`)
)

// 格式化金额
const formatMoney = (val?: number) => {
  if (val === undefined || val === null) return '--'
  return val.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

// 资产趋势图数据
const assetDatasets = computed(() => [
  {
    label: '总资产',
    data: records.value.map(r => r.totalAsset || 0),
    color: '#34c759',
  },
  {
    label: '总负债',
    data: records.value.map(r => r.totalLiability || 0),
    color: '#ff3b30',
  },
  {
    label: '净资产',
    data: records.value.map(r => r.netAsset || 0),
    color: '#FF6500',
  },
])

// 环比同比趋势图数据
const changeDatasets = computed(() => [
  {
    label: '环比',
    data: records.value.map(r => r.monthOnMonth || 0),
    color: '#2196F3',
  },
  {
    label: '同比',
    data: records.value.map(r => r.yearOnYear || 0),
    color: '#9C27B0',
  },
])

// 最新一期汇总
const latest = computed(() => {
  if (records.value.length === 0) return null
  return records.value[records.value.length - 1]
})
</script>

<template>
  <div class="book-stats-detail">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="back-btn" @click="router.back()">
        <svg viewBox="0 0 24 24" width="24" height="24">
          <path d="M15 19l-7-7 7-7" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
      <h2 class="header-title">{{ bookName }}</h2>
      <div class="header-placeholder"></div>
    </div>

    <div class="page-body">
      <!-- 加载态 -->
      <div v-if="loading && records.length === 0" class="loading-state">
        <div class="loading-spinner"></div>
        <p>加载中...</p>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!loading && records.length === 0" class="empty-state">
        <svg class="empty-icon" viewBox="0 0 64 64" width="64" height="64">
          <rect x="12" y="8" width="40" height="48" rx="4" fill="none" stroke="#ccc" stroke-width="2"/>
          <line x1="22" y1="20" x2="42" y2="20" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="28" x2="38" y2="28" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="36" x2="34" y2="36" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <p class="empty-text">暂无统计数据</p>
      </div>

      <template v-else>
        <!-- 最新一期汇总卡片 -->
        <div v-if="latest" class="summary-card">
          <div class="summary-header">
            <span class="summary-period">{{ latest.year }}年{{ latest.month }}月</span>
            <span class="summary-tag">最新</span>
          </div>
          <div class="summary-grid">
            <div class="summary-item">
              <span class="summary-label">总资产</span>
              <span class="summary-value" style="color: #34c759;">{{ formatMoney(latest.totalAsset) }}</span>
            </div>
            <div class="summary-item">
              <span class="summary-label">总负债</span>
              <span class="summary-value" style="color: #ff3b30;">{{ formatMoney(latest.totalLiability) }}</span>
            </div>
            <div class="summary-item">
              <span class="summary-label">净资产</span>
              <span class="summary-value" style="color: #FF6500;">{{ formatMoney(latest.netAsset) }}</span>
            </div>
            <div class="summary-item">
              <span class="summary-label">环比</span>
              <span class="summary-value" style="color: #2196F3;">{{ formatMoney(latest.monthOnMonth) }}</span>
            </div>
            <div class="summary-item">
              <span class="summary-label">同比</span>
              <span class="summary-value" style="color: #9C27B0;">{{ formatMoney(latest.yearOnYear) }}</span>
            </div>
          </div>
        </div>

        <!-- 资产趋势图 -->
        <div class="chart-card">
          <div class="chart-title">资产趋势</div>
          <TrendChart :labels="labels" :datasets="assetDatasets" />
        </div>

        <!-- 环比同比趋势图 -->
        <div class="chart-card">
          <div class="chart-title">环比 / 同比变动</div>
          <TrendChart :labels="labels" :datasets="changeDatasets" />
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.book-stats-detail {
  min-height: 100vh;
  background: #f5f5f5;
  display: flex;
  flex-direction: column;
}

/* 顶部导航 */
.page-header {
  background: #fff;
  padding: 12px 16px;
  padding-top: calc(12px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: sticky;
  top: 0;
  z-index: 10;
  box-shadow: 0 1px 0 rgba(0, 0, 0, 0.05);
}
.back-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  color: #333;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 50%;
  -webkit-tap-highlight-color: transparent;
  transition: background 0.15s;
}
.back-btn:active {
  background: rgba(0, 0, 0, 0.06);
}
.header-title {
  font-size: 17px;
  font-weight: 600;
  color: #1d1d1f;
}
.header-placeholder {
  width: 36px;
}

/* 页面主体 */
.page-body {
  flex: 1;
  padding: 16px;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 加载态 */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100px 0 40px;
  color: #8e8e93;
  font-size: 14px;
}
.loading-spinner {
  width: 28px;
  height: 28px;
  border: 3px solid #e0e0e0;
  border-top-color: #FF6500;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin-bottom: 12px;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 0 40px;
}
.empty-icon {
  margin-bottom: 20px;
  opacity: 0.5;
}
.empty-text {
  font-size: 16px;
  font-weight: 500;
  color: #8e8e93;
}

/* 汇总卡片 */
.summary-card {
  background: #fff;
  border-radius: 16px;
  padding: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.summary-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}
.summary-period {
  font-size: 15px;
  font-weight: 600;
  color: #1d1d1f;
}
.summary-tag {
  font-size: 11px;
  font-weight: 600;
  color: #FF6500;
  background: rgba(255, 101, 0, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
}
.summary-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}
.summary-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.summary-label {
  font-size: 12px;
  color: #aeaeb2;
}
.summary-value {
  font-size: 15px;
  font-weight: 600;
  color: #333;
}

/* 图表卡片 */
.chart-card {
  background: #fff;
  border-radius: 16px;
  padding: 12px 12px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.chart-title {
  font-size: 15px;
  font-weight: 600;
  color: #1d1d1f;
  padding: 4px 4px 8px;
}
</style>
