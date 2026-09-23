<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getMyDataStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'
import PageHeader from '@/components/PageHeader.vue'
import MoneyText from '@/components/MoneyText.vue'
import ChangeText from '@/components/ChangeText.vue'
import StateView from '@/components/StateView.vue'
import AppIcon from '@/components/AppIcon.vue'

const router = useRouter()

const loading = ref(false)
const list = ref<BookStatistics[]>([])

const fetchList = async () => {
  loading.value = true
  try {
    const res = await getMyDataStatistics()
    const data: BookStatistics[] = res?.recordList ?? []
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

const viewState = computed<'loading' | 'empty' | 'idle'>(() => {
  if (loading.value && list.value.length === 0) return 'loading'
  if (!loading.value && list.value.length === 0) return 'empty'
  return 'idle'
})

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
    <PageHeader title="统计" large />

    <div class="page-body">
      <!-- 加载 / 空状态 -->
      <StateView
        v-if="viewState !== 'idle'"
        :state="viewState"
        empty-text="暂无统计数据"
        empty-sub="记录本月数据后，这里会显示统计概览"
        empty-icon="trending-up"
      />

      <!-- 账簿卡片列表 -->
      <div v-else class="stats-list">
        <button
          v-for="(item, index) in list"
          :key="item.bookId || index"
          type="button"
          class="stats-card"
          @click="onCardClick(item)"
        >
          <div class="card-header">
            <div class="card-header-left">
              <span class="book-name">{{ item.bookName }}</span>
              <span class="period">{{ item.year }}年{{ item.month }}月</span>
            </div>
            <span class="card-arrow" aria-hidden="true">
              <AppIcon name="chevron-right" :size="16" />
            </span>
          </div>

          <div class="card-body">
            <div class="main-value">
              <span class="main-label">净资产</span>
              <MoneyText :value="item.netAsset" size="lg" />
            </div>

            <div class="sub-grid">
              <div class="sub-item">
                <span class="sub-label">总资产</span>
                <MoneyText :value="item.totalAsset" size="sm" />
              </div>
              <div class="sub-item">
                <span class="sub-label">总负债</span>
                <MoneyText :value="item.totalLiability" size="sm" />
              </div>
              <div class="sub-item">
                <span class="sub-label">环比</span>
                <ChangeText :value="item.monthOnMonth" />
              </div>
              <div class="sub-item">
                <span class="sub-label">同比</span>
                <ChangeText :value="item.yearOnYear" />
              </div>
            </div>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-page {
  min-height: 100vh;
  background: var(--bg-canvas);
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.page-body {
  padding: var(--space-3) var(--page-padding) 0;
}

/* 卡片列表 */
.stats-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.stats-card {
  width: 100%;
  text-align: left;
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  box-shadow: var(--shadow-sm);
  position: relative;
  cursor: pointer;
  transition: transform var(--duration-fast) var(--ease-out), background-color var(--duration-fast);
  -webkit-tap-highlight-color: transparent;
}
.stats-card:active {
  transform: scale(0.985);
  background: var(--bg-surface-2);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-3);
}
.card-header-left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.book-name {
  font-size: var(--text-title-3);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.period {
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.main-value {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
}
.main-label {
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.sub-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--space-2);
}
.sub-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: var(--bg-surface-2);
  border-radius: var(--radius-md);
  padding: var(--space-2) var(--space-3);
}
.sub-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.card-arrow {
  color: var(--text-tertiary);
  display: flex;
  align-items: center;
  flex-shrink: 0;
}
</style>
