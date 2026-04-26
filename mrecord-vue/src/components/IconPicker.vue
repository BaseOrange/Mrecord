<script setup lang="ts">
import { ref, computed } from 'vue'

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
}

const onClose = () => {
  emit('update:show', false)
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
        <button class="picker-close" @click="onClose" type="button">✕</button>
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
              <use :href="`/icons.svg#icon-${icon.key}`" />
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
  background: #fff;
  border-radius: 20px 20px 0 0;
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
  color: #1d1d1f;
}
.picker-close {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: #f0f0f0;
  border-radius: 50%;
  font-size: 13px;
  color: #666;
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
  border-radius: 20px;
  border: 1px solid #e0e0e0;
  background: #fff;
  font-size: 13px;
  font-weight: 500;
  color: #666;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.picker-tab--active {
  background: #FF6500;
  color: #fff;
  border-color: #FF6500;
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
  border-radius: 12px;
  transition: background 0.15s;
}
.picker-item:active {
  background: #f5f5f5;
}
.picker-item--selected .picker-icon-circle {
  background: #FF6500;
  color: #fff;
}

.picker-icon-circle {
  width: 44px;
  height: 44px;
  border-radius: 14px;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
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
  color: #8e8e93;
  line-height: 1;
}
</style>
