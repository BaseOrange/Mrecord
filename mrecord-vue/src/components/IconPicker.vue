<script setup lang="ts">
import { ref, computed } from 'vue'
import AppIcon from './AppIcon.vue'
// 图标雪碧图以模块方式引入，Vite 会自动拼上 BASE_URL 并加内容哈希，
// 保证飞牛网关模式（--base=/app/mrecord-fnos/）下路径正确（D2）
import iconsUrl from '@/../public/icons.svg'

const props = defineProps<{
  show: boolean
  modelValue?: string
}>()

const emit = defineEmits<{
  'update:show': [val: boolean]
  'update:modelValue': [val: string]
  select: [icon: string]
}>()

// ---- 图标分类 ----
const categories = [
  {
    name: '金融',
    icons: [
      { key: 'bank-card', label: '银行卡' },
      { key: 'credit-card', label: '信用卡' },
      { key: 'wallet', label: '钱包' },
      { key: 'cash', label: '现金' },
      { key: 'coins', label: '硬币' },
      { key: 'piggy-bank', label: '存钱罐' },
      { key: 'chart-up', label: '投资' },
      { key: 'stock', label: '股票' },
      { key: 'fund', label: '基金' },
      { key: 'gold', label: '黄金' },
      { key: 'diamond', label: '珠宝' },
      { key: 'safe', label: '保险箱' },
    ]
  },
  {
    name: '生活',
    icons: [
      { key: 'home', label: '房产' },
      { key: 'car', label: '汽车' },
      { key: 'phone', label: '手机' },
      { key: 'laptop', label: '电脑' },
      { key: 'shopping', label: '购物' },
      { key: 'food', label: '餐饮' },
      { key: 'coffee', label: '咖啡' },
      { key: 'medical', label: '医疗' },
      { key: 'education', label: '教育' },
      { key: 'travel', label: '旅行' },
      { key: 'pet', label: '宠物' },
      { key: 'gift', label: '礼物' },
    ]
  },
  {
    name: '其他',
    icons: [
      { key: 'debt', label: '借贷' },
      { key: 'transfer', label: '转账' },
      { key: 'receipt', label: '账单' },
      { key: 'tag', label: '标签' },
      { key: 'star', label: '收藏' },
      { key: 'heart', label: '爱心' },
      { key: 'umbrella', label: '保险' },
      { key: 'briefcase', label: '工作' },
      { key: 'tool', label: '维修' },
      { key: 'music', label: '娱乐' },
      { key: 'game', label: '游戏' },
      { key: 'more', label: '其他' },
    ]
  }
]

const activeTab = ref(0)
const currentIcons = computed(() => categories[activeTab.value].icons)

const onSelect = (key: string) => {
  emit('update:modelValue', key)
  emit('select', key)
  emit('update:show', false)
  activeTab.value = 0
}

const onClose = () => {
  emit('update:show', false)
  activeTab.value = 0
}
</script>

<template>
  <var-popup
    :show="show"
    position="bottom"
    @update:show="$emit('update:show', $event)"
  >
    <div class="icon-picker">
      <!-- 标题栏 -->
      <div class="picker-header">
        <span class="picker-title">选择图标</span>
        <button class="picker-close" @click="onClose" type="button" aria-label="关闭"><AppIcon name="x" :size="14" :stroke-width="2.4" /></button>
      </div>

      <!-- 分类 tab -->
      <div class="picker-tabs">
        <button
          v-for="(cat, i) in categories"
          :key="cat.name"
          class="picker-tab"
          :class="{ 'picker-tab--active': activeTab === i }"
          @click="activeTab = i"
          type="button"
        >
          {{ cat.name }}
        </button>
      </div>

      <!-- 图标网格 -->
      <div class="picker-grid">
        <div
          v-for="icon in currentIcons"
          :key="icon.key"
          class="picker-item"
          :class="{ 'picker-item--selected': modelValue === icon.key }"
          @click="onSelect(icon.key)"
        >
          <div class="picker-icon-circle">
            <svg class="picker-svg" viewBox="0 0 24 24" width="24" height="24">
              <use :href="`${iconsUrl}#icon-${icon.key}`" />
            </svg>
          </div>
          <span class="picker-label">{{ icon.label }}</span>
        </div>
      </div>
    </div>
  </var-popup>
</template>

<style scoped>
.icon-picker {
  background: var(--bg-surface);
  border-radius: var(--radius-xl) var(--radius-xl) 0 0;
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
  max-height: 65vh;
  display: flex;
  flex-direction: column;
}

.picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 10px;
}
.picker-title {
  font-size: 17px;
  font-weight: 600;
  color: var(--text-primary);
}
.picker-close {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: var(--bg-surface-2);
  border-radius: 50%;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

/* 分类 tab */
.picker-tabs {
  display: flex;
  gap: 8px;
  padding: 0 20px 12px;
}
.picker-tab {
  padding: 6px 16px;
  border-radius: var(--radius-pill);
  border: 1px solid var(--separator);
  background: var(--bg-surface);
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.picker-tab--active {
  background: var(--brand);
  color: #fff;
  border-color: var(--brand);
}

/* 图标网格 */
.picker-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  padding: 4px 20px 16px;
  overflow-y: auto;
}

.picker-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  padding: 8px 0;
  border-radius: var(--radius-md);
  transition: background var(--duration-fast);
}
.picker-item:active {
  background: var(--bg-surface-2);
}
.picker-item--selected .picker-icon-circle {
  background: var(--brand);
  color: #fff;
}

.picker-icon-circle {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  background: var(--bg-surface-2);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.picker-svg {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.picker-label {
  font-size: 11px;
  color: var(--text-tertiary);
  line-height: 1;
}
</style>
