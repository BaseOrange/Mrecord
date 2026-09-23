<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { getYearRecordList } from '@/api/modules/monthRecord'
import type { FinMonthRecord } from '@/api/modules/monthRecord'
import PageHeader from '@/components/PageHeader.vue'
import MoneyText from '@/components/MoneyText.vue'
import ChangeText from '@/components/ChangeText.vue'
import StateView from '@/components/StateView.vue'
import AppIcon from '@/components/AppIcon.vue'

const route = useRoute()
const router = useRouter()
const bookId = route.params.bookId as string
const bookName = (route.query.name as string) || ''

const loading = ref(false)
const records = ref<FinMonthRecord[]>([])

const viewState = computed<'loading' | 'empty' | 'idle'>(() => {
  if (loading.value && records.value.length === 0) return 'loading'
  if (!loading.value && records.value.length === 0) return 'empty'
  return 'idle'
})

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
      <!-- 加载 / 空状态 -->
      <StateView
        v-if="viewState !== 'idle'"
        :state="viewState"
        empty-text="还没有记录"
        empty-sub="点击下方按钮开始记账"
        empty-icon="notebook-text"
      />

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
            <button
              v-for="record in items"
              :key="record.id"
              type="button"
              class="month-card"
              @click="onMonthClick(record)"
            >
              <div class="month-top">
                <div class="month-main">
                  <div class="month-label">{{ record.month }}月</div>
                  <div class="month-net">
                    <span class="net-label">净资产</span>
                    <MoneyText :value="record.netAsset" size="md" />
                  </div>
                </div>
                <div class="month-meta">
                  <div class="meta-row">
                    <span class="meta-label">总资产</span>
                    <MoneyText :value="record.totalAsset" size="sm" />
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">总负债</span>
                    <MoneyText :value="record.totalLiability" size="sm" />
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">环比</span>
                    <ChangeText :value="record.monthOnMonth" :icon="false" />
                  </div>
                  <div class="meta-row">
                    <span class="meta-label">同比</span>
                    <ChangeText :value="record.yearOnYear" :icon="false" />
                  </div>
                </div>
              </div>
              <div v-if="record.note" class="month-note">
                <AppIcon name="file-text" :size="13" class="note-icon" />
                <span class="note-text">{{ record.note }}</span>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部记账按钮 -->
    <div class="bottom-action">
      <button class="record-btn" type="button" @click="startRecord">
        <AppIcon name="plus" :size="20" :stroke-width="2.4" />
        开始记账
      </button>
    </div>
  </div>
</template>

<style scoped>
.book-record-page {
  min-height: 100vh;
  background: var(--bg-canvas);
  display: flex;
  flex-direction: column;
}

/* 页面主体 */
.page-body {
  flex: 1;
  padding: var(--space-3) var(--page-padding) 0;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

/* 年度分组 */
.year-group {
  margin-bottom: var(--space-6);
}
.year-group:last-child {
  margin-bottom: 0;
}
.year-header {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
  padding-left: var(--space-1);
}
.year-label {
  font-size: var(--text-title-2);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
}
.year-count {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  font-weight: var(--weight-regular);
}

/* 月份列表 */
.month-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

/* 月份卡片 */
.month-card {
  width: 100%;
  text-align: left;
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  transition: transform var(--duration-fast) var(--ease-out), background-color var(--duration-fast);
  -webkit-tap-highlight-color: transparent;
}
.month-card:active {
  transform: scale(0.985);
  background: var(--bg-surface-2);
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
  font-size: var(--text-title-3);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.month-net {
  display: flex;
  align-items: baseline;
  gap: var(--space-1);
}
.net-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
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
  gap: var(--space-2);
}
.meta-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

/* 备注行 */
.month-note {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding-top: var(--space-2);
  border-top: 1px dashed var(--separator);
}
.note-icon {
  color: var(--brand);
  flex-shrink: 0;
  margin-top: 2px;
}
.note-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
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
  padding: var(--space-3) var(--page-padding);
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
  background: linear-gradient(to top, var(--bg-canvas) 60%, transparent);
  z-index: 10;
}
.record-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--brand) 0%, var(--brand-accent) 100%);
  color: #fff;
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  letter-spacing: 0.5px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(255, 85, 0, 0.35);
  transition: transform var(--duration-fast) var(--ease-out), box-shadow var(--duration-fast);
  -webkit-tap-highlight-color: transparent;
}
.record-btn:active {
  transform: scale(0.97);
  box-shadow: 0 2px 8px rgba(255, 85, 0, 0.25);
}
</style>
