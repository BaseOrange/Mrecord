<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { getBookDetailedStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'
import TrendChart from '@/components/TrendChart.vue'
import PageHeader from '@/components/PageHeader.vue'
import MoneyText from '@/components/MoneyText.vue'
import ChangeText from '@/components/ChangeText.vue'
import StateView from '@/components/StateView.vue'

const route = useRoute()
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

const viewState = computed<'loading' | 'empty' | 'idle'>(() => {
  if (loading.value && records.value.length === 0) return 'loading'
  if (!loading.value && records.value.length === 0) return 'empty'
  return 'idle'
})

// 月份标签，如 "2024-01"
const labels = computed(() =>
  records.value.map(r => `${r.year}-${String(r.month).padStart(2, '0')}`)
)

// 资产趋势图数据（语义色用令牌，深浅色模式自动切换）
const assetDatasets = computed(() => [
  {
    label: '总资产',
    data: records.value.map(r => r.totalAsset || 0),
    color: 'var(--semantic-down)',
  },
  {
    label: '总负债',
    data: records.value.map(r => r.totalLiability || 0),
    color: 'var(--semantic-up)',
  },
  {
    label: '净资产',
    data: records.value.map(r => r.netAsset || 0),
    color: 'var(--brand)',
  },
])

// 环比同比趋势图数据
const changeDatasets = computed(() => [
  {
    label: '环比',
    data: records.value.map(r => r.monthOnMonth || 0),
    color: 'var(--brand)',
  },
  {
    label: '同比',
    data: records.value.map(r => r.yearOnYear || 0),
    color: 'var(--text-tertiary)',
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
    <PageHeader :title="bookName" show-back />

    <div class="page-body">
      <!-- 加载 / 空状态 -->
      <StateView
        v-if="viewState !== 'idle'"
        :state="viewState"
        empty-text="暂无统计数据"
        empty-sub="记录本月数据后可查看趋势"
        empty-icon="trending-up"
      />

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
              <MoneyText :value="latest.totalAsset" size="sm" />
            </div>
            <div class="summary-item">
              <span class="summary-label">总负债</span>
              <MoneyText :value="latest.totalLiability" size="sm" />
            </div>
            <div class="summary-item">
              <span class="summary-label">净资产</span>
              <MoneyText :value="latest.netAsset" size="sm" />
            </div>
            <div class="summary-item">
              <span class="summary-label">环比</span>
              <ChangeText :value="latest.monthOnMonth" :icon="false" />
            </div>
            <div class="summary-item">
              <span class="summary-label">同比</span>
              <ChangeText :value="latest.yearOnYear" :icon="false" />
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
          <div class="chart-subtitle">环比：与上月相比的变化&nbsp;&nbsp;|&nbsp;&nbsp;同比：与去年同月相比的变化</div>
          <TrendChart :labels="labels" :datasets="changeDatasets" />
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.book-stats-detail {
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
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

/* 汇总卡片 */
.summary-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  box-shadow: var(--shadow-sm);
}
.summary-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}
.summary-period {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.summary-tag {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--brand);
  background: var(--brand-soft);
  padding: 2px 8px;
  border-radius: var(--radius-pill);
}
.summary-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-3);
}
.summary-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.summary-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

/* 图表卡片 */
.chart-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-3) var(--space-4);
  box-shadow: var(--shadow-sm);
}
.chart-title {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  padding: var(--space-1) var(--space-1) var(--space-2);
}

.chart-subtitle {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  padding: 0 var(--space-1) var(--space-3);
  line-height: 1.5;
}
</style>
