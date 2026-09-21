<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { getYearRecordList } from '@/api/modules/monthRecord'
import type { FinMonthRecord } from '@/api/modules/monthRecord'
import { formatMoney, getChangeText, getChangeColor } from '@/utils/format'
import PageHeader from '@/components/PageHeader.vue'

const route = useRoute()
const router = useRouter()
const bookId = route.params.bookId as string
const bookName = (route.query.name as string) || ''

const loading = ref(false)
const records = ref<FinMonthRecord[]>([])

const fetchRecords = async () => {
  loading.value = true
  try {
    const res = await getYearRecordList({ bookId })
    records.value = res || []
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchRecords()
})

// 按年度分组，倒序
const groupedByYear = computed(() => {
  const map = new Map<number, FinMonthRecord[]>()
  for (const r of records.value) {
    const year = r.year || 0
    if (!map.has(year)) map.set(year, [])
    map.get(year)!.push(r)
  }
  // 每年内按月份倒序
  for (const [, items] of map) {
    items.sort((a, b) => (b.month || 0) - (a.month || 0))
  }
  // 年份倒序
  const sorted = [...map.entries()].sort((a, b) => b[0] - a[0])
  return sorted
})

// 开始记账
const startRecord = () => {
  router.push({
    name: 'Record',
    params: { bookId },
    query: { name: bookName }
  })
}

// 点击月份记录 → 进入该月记账/查看
const onMonthClick = (record: FinMonthRecord) => {
  router.push({
    name: 'Record',
    params: { bookId },
    query: { name: bookName, year: record.year, month: record.month }
  })
}
</script>

<template>
  <div class="book-record-page">
    <!-- 顶部导航 -->
    <PageHeader :title="bookName" show-back />

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
        <p class="empty-text">还没有记录</p>
        <p class="empty-sub">点击下方按钮开始记账</p>
      </div>

      <!-- 按年度分组显示 -->
      <div v-else class="year-groups">
        <div v-for="[year, items] in groupedByYear" :key="year" class="year-group">
          <!-- 年度小标题 -->
          <div class="year-header">
            <span class="year-label">{{ year }}年</span>
            <span class="year-count">共{{ items.length }}个月</span>
          </div>

          <!-- 月份卡片列表 -->
          <div class="month-list">
            <div
              v-for="record in items"
              :key="record.id"
              class="month-card"
              @click="onMonthClick(record)"
            >
              <div class="month-top">
                <div class="month-main">
                  <div class="month-label">{{ record.month }}月</div>
                  <div class="month-net">
                    <span class="net-label">净资产</span>
                    <span class="net-value">{{ formatMoney(record.netAsset) }}</span>
                  </div>
                </div>
                <div class="month-meta">
                  <div class="meta-row">
                    <span class="meta-label">总资产</span>
                    <span class="meta-value asset">{{ formatMoney(record.totalAsset) }}</span>
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">总负债</span>
                    <span class="meta-value liability">{{ formatMoney(record.totalLiability) }}</span>
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">环比</span>
                    <span class="meta-value" :style="{ color: getChangeColor(record.monthOnMonth) }">
                      {{ getChangeText(record.monthOnMonth) }}
                    </span>
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">同比</span>
                    <span class="meta-value" :style="{ color: getChangeColor(record.yearOnYear) }">
                      {{ getChangeText(record.yearOnYear) }}
                    </span>
                  </div>
                </div>
              </div>
              <div v-if="record.note" class="month-note">
                <svg class="note-icon" viewBox="0 0 24 24" width="13" height="13">
                  <path d="M12 20h9M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <span class="note-text">{{ record.note }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部记账按钮 -->
    <div class="bottom-action">
      <button class="record-btn" @click="startRecord">
        <svg viewBox="0 0 24 24" width="22" height="22">
          <line x1="12" y1="5" x2="12" y2="19" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          <line x1="5" y1="12" x2="19" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        开始记账
      </button>
    </div>
  </div>
</template>

<style scoped>
.book-record-page {
  min-height: 100vh;
  background: #f5f5f5;
  display: flex;
  flex-direction: column;
}

/* 页面主体 */
.page-body {
  flex: 1;
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
.empty-sub {
  margin-top: 6px;
  font-size: 13px;
  color: #aeaeb2;
}

/* 年度分组 */
.year-group {
  margin-bottom: 24px;
}
.year-group:last-child {
  margin-bottom: 0;
}
.year-header {
  display: flex;
  align-items: baseline;
  gap: 8px;
  margin-bottom: 12px;
  padding-left: 4px;
}
.year-label {
  font-size: 20px;
  font-weight: 700;
  color: #1d1d1f;
}
.year-count {
  font-size: 12px;
  color: #aeaeb2;
  font-weight: 400;
}

/* 月份列表 */
.month-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* 月份卡片 */
.month-card {
  background: #fff;
  border-radius: 14px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  cursor: pointer;
  transition: all 0.15s ease;
  -webkit-tap-highlight-color: transparent;
}
.month-card:active {
  transform: scale(0.985);
  background: #fafafa;
}
.month-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.month-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.month-label {
  font-size: 17px;
  font-weight: 600;
  color: #1d1d1f;
}
.month-net {
  display: flex;
  align-items: baseline;
  gap: 4px;
}
.net-label {
  font-size: 11px;
  color: #aeaeb2;
}
.net-value {
  font-size: 14px;
  font-weight: 600;
  color: #FF6500;
}

.month-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}
.meta-row {
  display: flex;
  align-items: baseline;
  gap: 6px;
}
.meta-label {
  font-size: 11px;
  color: #aeaeb2;
}
.meta-value {
  font-size: 13px;
  font-weight: 500;
  color: #555;
}
.meta-value.asset {
  color: #34c759;
}
.meta-value.liability {
  color: #ff3b30;
}

/* 备注行 */
.month-note {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  padding-top: 8px;
  border-top: 1px dashed #eee;
}
.note-icon {
  color: #FF6500;
  flex-shrink: 0;
  margin-top: 2px;
}
.note-text {
  font-size: 12px;
  color: #8e8e93;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 底部记账按钮 */
.bottom-action {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 12px 24px;
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
  background: linear-gradient(to top, #f5f5f5 60%, transparent);
  z-index: 10;
}
.record-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: linear-gradient(135deg, #FF7A1A 0%, #FF5500 100%);
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0.5px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(255, 85, 0, 0.35);
  transition: all 0.2s ease;
  -webkit-tap-highlight-color: transparent;
}
.record-btn:active {
  transform: scale(0.97);
  box-shadow: 0 2px 8px rgba(255, 85, 0, 0.25);
}
</style>
