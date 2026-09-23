<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getMyDataStatistics } from '@/api/modules/book'
import type { BookStatistics } from '@/api/modules/book'
import { formatMoney } from '@/utils/format'
import appIcon from '@/../public/app-icon.svg'
import PageHeader from '@/components/PageHeader.vue'
import SectionHeader from '@/components/SectionHeader.vue'
import ListGroup from '@/components/ListGroup.vue'
import ListCell from '@/components/ListCell.vue'
import MoneyText from '@/components/MoneyText.vue'
import StateView from '@/components/StateView.vue'
import AppIcon from '@/components/AppIcon.vue'

const router = useRouter()

// 大标题副标题：当前年月
const monthSubtitle = computed(() => {
  const d = new Date()
  return `${d.getFullYear()} 年 ${d.getMonth() + 1} 月`
})

// ==================== 快捷入口 ====================
// 修掉原「记账」「模板」重复指向 /book 的问题（Q11）：模板编辑从账簿页操作菜单进入，
// 快捷入口只留不重复的四项；图标统一走 AppIcon/Lucide，不再用跑偏的 Material 色。
type QuickAction = 'record' | 'stats' | 'export' | 'profile'
const quickEntries: { icon: string; label: string; action: QuickAction }[] = [
  { icon: 'circle-plus', label: '记一笔', action: 'record' },
  { icon: 'calendar-days', label: '月报', action: 'stats' },
  { icon: 'download', label: '导出数据', action: 'export' },
  { icon: 'settings', label: '设置', action: 'profile' },
]

const onQuickEntry = (action: QuickAction) => {
  switch (action) {
    case 'record':
      router.push('/book')
      break
    case 'stats':
      router.push('/stats')
      break
    case 'export':
      router.push('/export')
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
})
const bookSnapshots = ref<BookStatistics[]>([])

/**
 * 由「本月净资产 + 环比增长率」反推单账簿的上月净资产。
 *
 * 后端 monthOnMonth = (本月 - 上月) / |上月| * 100（百分比，见 Java
 * FinMonthRecordServiceImpl#getMonthOnMonthVal / Rust calculate_growth_rate），
 * 故 上月 = 本月 / (1 + 环比/100)。
 *
 * 环比为 0 时无法区分「与上月持平」和「上月净资产为 0」（后端对分母为 0
 * 一律返回 0），保守按「持平」处理，即将上月置为本月。
 */
const derivePrevNetAsset = (cur: number, rate?: number): number => {
  if (!rate || !isFinite(rate)) return cur
  const divisor = 1 + rate / 100
  // 环比 -100（本月净资产归零）时除数为 0，无法反推，按持平处理
  if (divisor === 0) return cur
  const prev = cur / divisor
  return isFinite(prev) ? prev : cur
}

/**
 * 汇总环比：先汇总本月与上月净资产，再按后端同一公式计算整体增长率。
 *
 * 百分比不能跨账簿直接相加（各账簿分母不同），必须用汇总后的金额计算：
 * 总环比 = (Σ本月净资产 - Σ上月净资产) / |Σ上月净资产| * 100。
 */
const totalMonthOnMonth = computed(() => {
  const prevTotal = bookSnapshots.value.reduce(
    (s, r) => s + derivePrevNetAsset(r.netAsset || 0, r.monthOnMonth),
    0,
  )
  // 上月净资产合计为 0 时无法计算增长率，与后端「分母为 0 返回 0」的约定保持一致
  if (prevTotal === 0) return 0
  return ((overview.value.netAsset - prevTotal) / Math.abs(prevTotal)) * 100
})

// Hero 卡上的环比展示：趋势图标 + 文案（渐变底色上用图标表达方向，不依赖红绿）
const momTrendIcon = computed(() => {
  const v = totalMonthOnMonth.value
  if (v > 0) return 'trending-up'
  if (v < 0) return 'trending-down'
  return 'minus'
})
const momTrendText = computed(() => {
  const v = totalMonthOnMonth.value
  if (v === 0) return '与上月持平'
  return (v > 0 ? '+' : '') + v.toFixed(2) + '% 环比'
})

const fetchOverview = async () => {
  loading.value = true
  try {
    const res = await getMyDataStatistics()
    const data: BookStatistics[] = res?.recordList ?? []
    bookSnapshots.value = data.filter((item: any) => item != null)
    overview.value = {
      totalAsset: bookSnapshots.value.reduce((s, r) => s + (r.totalAsset || 0), 0),
      totalLiability: bookSnapshots.value.reduce((s, r) => s + (r.totalLiability || 0), 0),
      netAsset: bookSnapshots.value.reduce((s, r) => s + (r.netAsset || 0), 0),
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
    <PageHeader title="月衡" large :subtitle="monthSubtitle">
      <template #right>
        <img :src="appIcon" alt="月衡 Logo" class="header-logo" />
      </template>
    </PageHeader>

    <div class="home-body">
      <!-- ==================== Hero 资产卡（Q9-④） ==================== -->
      <section class="hero-card">
        <div class="hero-top">
          <span class="hero-label">净资产</span>
          <span class="hero-mom">
            <AppIcon :name="momTrendIcon" :size="13" :stroke-width="2.2" />
            {{ momTrendText }}
          </span>
        </div>

        <!-- 加载中 -->
        <div v-if="loading && bookSnapshots.length === 0" class="hero-loading">
          <div class="hero-spinner"></div>
        </div>
        <template v-else>
          <div class="hero-money">
            <MoneyText :value="overview.netAsset" size="hero" />
          </div>
          <div class="hero-divider"></div>
          <div class="hero-sub">
            <div class="hero-sub-item">
              <span class="hero-sub-label">总资产</span>
              <span class="hero-sub-value">{{ formatMoney(overview.totalAsset) }}</span>
            </div>
            <div class="hero-sub-item">
              <span class="hero-sub-label">总负债</span>
              <span class="hero-sub-value">{{ formatMoney(overview.totalLiability) }}</span>
            </div>
          </div>
        </template>
      </section>

      <!-- ==================== 常用功能 ==================== -->
      <section class="home-section">
        <SectionHeader title="常用功能" />
        <ListGroup>
          <ListCell
            v-for="entry in quickEntries"
            :key="entry.action"
            :icon="entry.icon"
            :label="entry.label"
            @click="onQuickEntry(entry.action)"
          />
        </ListGroup>
      </section>

      <!-- ==================== 我的账簿 ==================== -->
      <section v-if="bookSnapshots.length > 0" class="home-section">
        <SectionHeader title="我的账簿" action-text="全部" @action="router.push('/book')" />
        <ListGroup>
          <ListCell
            v-for="item in bookSnapshots"
            :key="item.bookId"
            icon="notebook-text"
            :label="item.bookName || '未命名账簿'"
            :value="'¥' + formatMoney(item.netAsset || 0)"
            :sub-value="`${item.year}年${item.month}月`"
            @click="onBookCardClick(item)"
          />
        </ListGroup>
      </section>

      <!-- 无账簿时的空状态 -->
      <section v-else-if="!loading" class="home-section">
        <StateView
          state="empty"
          empty-text="还没有账簿"
          empty-sub="创建第一个账簿，开始记录每月的资产与负债"
          empty-icon="notebook-text"
        >
          <template #empty-action>
            <button class="create-btn" type="button" @click="router.push('/book')">去创建账簿</button>
          </template>
        </StateView>
      </section>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  min-height: 100vh;
  background: var(--bg-canvas);
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.header-logo {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  box-shadow: 0 6px 16px rgba(249, 114, 22, 0.18);
}

.home-body {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

/* ==================== Hero 资产卡 ==================== */
.hero-card {
  margin: var(--space-3) var(--page-padding) 0;
  background: linear-gradient(135deg, var(--brand) 0%, var(--brand-accent) 100%);
  border-radius: var(--radius-xl);
  padding: var(--space-5);
  color: #fff;
  box-shadow: 0 8px 24px rgba(255, 101, 0, 0.22);
  min-height: 132px;
}

.hero-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-1);
}

.hero-label {
  font-size: var(--text-sm);
  color: rgba(255, 255, 255, 0.85);
}

.hero-mom {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  background: rgba(255, 255, 255, 0.18);
  border-radius: var(--radius-pill);
  padding: 3px 10px;
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: #fff;
}

.hero-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 28px 0;
}

.hero-spinner {
  width: 24px;
  height: 24px;
  border: 2.5px solid rgba(255, 255, 255, 0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: hero-spin 0.8s linear infinite;
}

@keyframes hero-spin {
  to {
    transform: rotate(360deg);
  }
}

.hero-money {
  color: #fff;
}

.hero-divider {
  height: 1px;
  background: rgba(255, 255, 255, 0.22);
  margin: var(--space-4) 0;
}

.hero-sub {
  display: flex;
  align-items: center;
}

.hero-sub-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.hero-sub-label {
  font-size: var(--text-xs);
  color: rgba(255, 255, 255, 0.75);
}

.hero-sub-value {
  font-size: 15px;
  font-weight: var(--weight-semibold);
}

/* ==================== 分组区块 ==================== */
.home-section {
  display: flex;
  flex-direction: column;
}

.create-btn {
  height: 44px;
  padding: 0 var(--space-6);
  border-radius: var(--radius-pill);
  background: var(--brand);
  color: #fff;
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  box-shadow: 0 4px 14px rgba(255, 101, 0, 0.3);
  transition: transform var(--duration-fast) var(--ease-out);
}

.create-btn:active {
  transform: scale(0.96);
}
</style>
