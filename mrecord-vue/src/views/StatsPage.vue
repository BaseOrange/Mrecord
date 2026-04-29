<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getMyDataStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'

const router = useRouter()

const loading = ref(false)
const list = ref<BookStatistics[]>([])

const fetchList = async () => {
  loading.value = true
  try {
    const res = await getMyDataStatistics()
    // 兼容后端返回数组或 { recordList: [] } 对象两种情况
    const data = Array.isArray(res) ? res : (res as any)?.recordList || []
    list.value = data.filter((item: any) => item != null)
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchList()
})

// 格式化金额
const formatMoney = (val?: number) => {
  if (val === undefined || val === null) return '--'
  return val.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

// 环比/同比颜色
const getChangeColor = (val?: number) => {
  if (val === undefined || val === null || val === 0) return '#8e8e93'
  return val > 0 ? '#34c759' : '#ff3b30'
}

// 环比/同比文字
const getChangeText = (val?: number) => {
  if (val === undefined || val === null) return '--'
  if (val === 0) return '持平'
  const prefix = val > 0 ? '+' : ''
  return prefix + formatMoney(val)
}

// 点击卡片 → 详情页
const onCardClick = (item: BookStatistics) => {
  router.push({
    name: 'BookStatsDetail',
    params: { bookId: item.bookId },
    query: { name: item.bookName }
  })
}
</script>

<template>
  <div class="stats-page">
    <div class="page-header">
      <h2>统计</h2>
    </div>

    <div class="page-body">
      <!-- 加载态 -->
      <div v-if="loading && list.length === 0" class="loading-state">
        <div class="loading-spinner"></div>
        <p>加载中...</p>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!loading && list.length === 0" class="empty-state">
        <svg class="empty-icon" viewBox="0 0 64 64" width="64" height="64">
          <rect x="12" y="8" width="40" height="48" rx="4" fill="none" stroke="#ccc" stroke-width="2"/>
          <line x1="22" y1="20" x2="42" y2="20" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="28" x2="38" y2="28" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="36" x2="34" y2="36" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <p class="empty-text">暂无统计数据</p>
      </div>

      <!-- 账簿卡片列表 -->
      <div v-else class="stats-list">
        <div
          v-for="(item, index) in list"
          :key="item.bookId || index"
          class="stats-card"
          @click="onCardClick(item)"
        >
          <div class="card-header">
            <span class="book-name">{{ item.bookName }}</span>
            <span class="period">{{ item.year }}年{{ item.month }}月</span>
          </div>

          <div class="card-body">
            <div class="main-value">
              <span class="main-label">净资产</span>
              <span class="main-num" :style="{ color: (item.netAsset || 0) >= 0 ? '#34c759' : '#ff3b30' }">
                {{ formatMoney(item.netAsset) }}
              </span>
            </div>

            <div class="sub-grid">
              <div class="sub-item">
                <span class="sub-label">总资产</span>
                <span class="sub-num">{{ formatMoney(item.totalAsset) }}</span>
              </div>
              <div class="sub-item">
                <span class="sub-label">总负债</span>
                <span class="sub-num">{{ formatMoney(item.totalLiability) }}</span>
              </div>
              <div class="sub-item">
                <span class="sub-label">环比</span>
                <span class="sub-num" :style="{ color: getChangeColor(item.monthOnMonth) }">
                  {{ getChangeText(item.monthOnMonth) }}
                </span>
              </div>
              <div class="sub-item">
                <span class="sub-label">同比</span>
                <span class="sub-num" :style="{ color: getChangeColor(item.yearOnYear) }">
                  {{ getChangeText(item.yearOnYear) }}
                </span>
              </div>
            </div>
          </div>

          <div class="card-arrow">
            <svg viewBox="0 0 24 24" width="16" height="16">
              <path d="M9 18l6-6-6-6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-page {
  min-height: 100vh;
  background: #f5f5f5;
}

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

.page-body {
  padding: 16px;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
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

/* 卡片列表 */
.stats-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.stats-card {
  background: #fff;
  border-radius: 16px;
  padding: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  position: relative;
  cursor: pointer;
  transition: all 0.15s ease;
  -webkit-tap-highlight-color: transparent;
}
.stats-card:active {
  transform: scale(0.985);
  background: #fafafa;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}
.book-name {
  font-size: 16px;
  font-weight: 600;
  color: #1d1d1f;
}
.period {
  font-size: 13px;
  color: #8e8e93;
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.main-value {
  display: flex;
  align-items: baseline;
  gap: 8px;
}
.main-label {
  font-size: 13px;
  color: #8e8e93;
}
.main-num {
  font-size: 22px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.sub-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}
.sub-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: #f9f9f9;
  border-radius: 10px;
  padding: 10px 12px;
}
.sub-label {
  font-size: 11px;
  color: #aeaeb2;
}
.sub-num {
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.card-arrow {
  position: absolute;
  right: 16px;
  top: 50%;
  transform: translateY(-50%);
  color: #c7c7cc;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
