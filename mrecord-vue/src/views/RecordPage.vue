<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listTempItems } from '@/api/modules/tempItem'
import type { FinTemplateItem } from '@/api/modules/tempItem'
import { queryMonthItem, insertMonthItem, updateMonthItem } from '@/api/modules/monthItem'
import type { FinMonthItemRecord } from '@/api/modules/monthItem'

const route = useRoute()
const router = useRouter()

const bookId = route.params.bookId as string
const bookName = (route.query.name as string) || ''
const initYear = route.query.year ? Number(route.query.year) : new Date().getFullYear()
const initMonth = route.query.month ? Number(route.query.month) : new Date().getMonth() + 1

// ---- 年月选择 ----
const currentYear = ref(initYear)
const currentMonth = ref(initMonth)
const showMonthPicker = ref(false)

// 生成年月选项（当前年前后3年）
const yearOptions = computed(() => {
  const now = new Date().getFullYear()
  const years: number[] = []
  for (let y = now + 1; y >= now - 5; y--) years.push(y)
  return years
})

const monthOptions = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]

const monthLabel = computed(() => `${currentYear.value}年${currentMonth.value}月`)

// ---- 数据加载 ----
const loading = ref(false)
const saving = ref(false)
const templateItems = ref<FinTemplateItem[]>([])
const existingRecords = ref<FinMonthItemRecord[]>([])

// 每个模板项对应的输入金额
const itemValues = ref<Record<string, string>>({})

// 按类型分组模板项
const assetItems = computed(() => templateItems.value.filter(i => i.itemType === 1).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))
const liabilityItems = computed(() => templateItems.value.filter(i => i.itemType === -1).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))
const ignoreItems = computed(() => templateItems.value.filter(i => i.itemType === 0).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))

// 实时汇总
const totalAsset = computed(() => {
  return assetItems.value.reduce((sum, item) => {
    const val = parseFloat(itemValues.value[item.id!] || '0')
    return sum + (isNaN(val) ? 0 : val)
  }, 0)
})
const totalLiability = computed(() => {
  return liabilityItems.value.reduce((sum, item) => {
    const val = parseFloat(itemValues.value[item.id!] || '0')
    return sum + (isNaN(val) ? 0 : val)
  }, 0)
})
const netAsset = computed(() => totalAsset.value - totalLiability.value)

// 格式化显示金额
const formatMoney = (val: number) => {
  return val.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

// ---- 数据拉取 ----
const fetchData = async () => {
  loading.value = true
  try {
    // 并行拉取模板项和月度记录
    const [templates, records] = await Promise.all([
      listTempItems({ bookId }),
      queryMonthItem({ bookId, year: currentYear.value, month: currentMonth.value })
    ])
    templateItems.value = templates || []
    existingRecords.value = records || []

    // 填充已有金额
    const values: Record<string, string> = {}
    for (const r of existingRecords.value) {
      if (r.templateItemId && r.itemValue !== undefined && r.itemValue !== null) {
        values[r.templateItemId] = String(r.itemValue)
      }
    }
    itemValues.value = values
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchData()
})

// 切换年月
const confirmMonthPick = () => {
  showMonthPicker.value = false
  fetchData()
}

// ---- 保存 ----
const handleSave = async () => {
  saving.value = true
  try {
    // 构建明细列表
    const itemList: FinMonthItemRecord[] = []
    for (const item of templateItems.value) {
      if (!item.id) continue
      const raw = itemValues.value[item.id]
      const val = parseFloat(raw || '0')
      if (isNaN(val) && raw !== '' && raw !== undefined) continue
      itemList.push({
        bookId,
        year: currentYear.value,
        month: currentMonth.value,
        templateItemId: item.id,
        itemValue: raw === '' || raw === undefined ? 0 : val
      })
    }

    // 区分已有记录（有id）和新增记录
    const existingMap = new Map(existingRecords.value.map(r => [r.templateItemId, r]))
    const updateList: FinMonthItemRecord[] = []
    const insertList: FinMonthItemRecord[] = []

    for (const i of itemList) {
      const existing = existingMap.get(i.templateItemId)
      if (existing) {
        // 已有记录：补上 id，后端根据 id 更新
        updateList.push({ ...i, id: existing.id })
      } else {
        insertList.push(i)
      }
    }

    if (updateList.length > 0) {
      await updateMonthItem({ bookId, year: currentYear.value, month: currentMonth.value, itemList: updateList })
    }
    if (insertList.length > 0) {
      await insertMonthItem({ bookId, year: currentYear.value, month: currentMonth.value, itemList: insertList })
    }

    Snackbar.success('保存成功')
    // 刷新数据
    await fetchData()
  } catch {
    // 拦截器已处理
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="record-page">
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

    <!-- 年月选择 -->
    <div class="month-selector" @click="showMonthPicker = true">
      <span class="month-text">{{ monthLabel }}</span>
      <svg viewBox="0 0 24 24" width="16" height="16">
        <path d="M6 9l6 6 6-6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>

    <div class="page-body">
      <!-- 加载态 -->
      <div v-if="loading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>加载中...</p>
      </div>

      <!-- 无模板项 -->
      <div v-else-if="templateItems.length === 0" class="empty-state">
        <svg class="empty-icon" viewBox="0 0 64 64" width="64" height="64">
          <rect x="12" y="8" width="40" height="48" rx="4" fill="none" stroke="#ccc" stroke-width="2"/>
          <line x1="22" y1="20" x2="42" y2="20" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="28" x2="38" y2="28" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <p class="empty-text">还没有记账项</p>
        <p class="empty-sub">请先编辑账目模板</p>
      </div>

      <!-- 记账表单 -->
      <template v-else>
        <!-- 实时汇总卡片 -->
        <div class="summary-card">
          <div class="summary-item">
            <span class="summary-label">总资产</span>
            <span class="summary-value asset">{{ formatMoney(totalAsset) }}</span>
          </div>
          <div class="summary-divider"></div>
          <div class="summary-item">
            <span class="summary-label">总负债</span>
            <span class="summary-value liability">{{ formatMoney(totalLiability) }}</span>
          </div>
          <div class="summary-divider"></div>
          <div class="summary-item">
            <span class="summary-label">净资产</span>
            <span class="summary-value net">{{ formatMoney(netAsset) }}</span>
          </div>
        </div>

        <!-- 资产项 -->
        <div v-if="assetItems.length > 0" class="item-group">
          <div class="group-header">
            <span class="group-dot asset-dot"></span>
            <span class="group-label">资产</span>
          </div>
          <div class="item-list">
            <div v-for="item in assetItems" :key="item.id" class="item-row">
              <div class="item-info">
                <svg v-if="item.icon" class="item-icon"><use :href="`/icons.svg#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  class="item-input"
                  placeholder="0.00"
                  v-model="itemValues[item.id!]"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 负债项 -->
        <div v-if="liabilityItems.length > 0" class="item-group">
          <div class="group-header">
            <span class="group-dot liability-dot"></span>
            <span class="group-label">负债</span>
          </div>
          <div class="item-list">
            <div v-for="item in liabilityItems" :key="item.id" class="item-row">
              <div class="item-info">
                <svg v-if="item.icon" class="item-icon"><use :href="`/icons.svg#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  class="item-input"
                  placeholder="0.00"
                  v-model="itemValues[item.id!]"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 不统计项 -->
        <div v-if="ignoreItems.length > 0" class="item-group">
          <div class="group-header">
            <span class="group-dot ignore-dot"></span>
            <span class="group-label">仅记录</span>
          </div>
          <div class="item-list">
            <div v-for="item in ignoreItems" :key="item.id" class="item-row">
              <div class="item-info">
                <svg v-if="item.icon" class="item-icon"><use :href="`/icons.svg#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  class="item-input"
                  placeholder="0.00"
                  v-model="itemValues[item.id!]"
                />
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- 底部保存按钮 -->
    <div v-if="templateItems.length > 0" class="bottom-action">
      <button class="save-btn" :disabled="saving" @click="handleSave">
        {{ saving ? '保存中...' : '保存' }}
      </button>
    </div>

    <!-- 年月选择弹窗 -->
    <var-popup v-model:show="showMonthPicker" position="bottom" round>
      <div class="month-picker">
        <div class="picker-header">
          <button class="picker-cancel" @click="showMonthPicker = false">取消</button>
          <span class="picker-title">选择月份</span>
          <button class="picker-confirm" @click="confirmMonthPick">确定</button>
        </div>
        <div class="picker-body">
          <div class="picker-column">
            <div
              v-for="y in yearOptions"
              :key="y"
              class="picker-option"
              :class="{ active: currentYear === y }"
              @click="currentYear = y"
            >
              {{ y }}年
            </div>
          </div>
          <div class="picker-column">
            <div
              v-for="m in monthOptions"
              :key="m"
              class="picker-option"
              :class="{ active: currentMonth === m }"
              @click="currentMonth = m"
            >
              {{ m }}月
            </div>
          </div>
        </div>
      </div>
    </var-popup>
  </div>
</template>

<style scoped>
.record-page {
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

/* 年月选择 */
.month-selector {
  background: #fff;
  padding: 10px 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}
.month-text {
  font-size: 15px;
  font-weight: 600;
  color: #FF6500;
}
.month-selector svg {
  color: #FF6500;
  transition: transform 0.2s;
}

/* 页面主体 */
.page-body {
  flex: 1;
  padding: 16px;
  padding-bottom: calc(80px + env(safe-area-inset-bottom, 0px));
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

/* 汇总卡片 */
.summary-card {
  background: #fff;
  border-radius: 14px;
  padding: 16px 20px;
  display: flex;
  align-items: center;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  margin-bottom: 20px;
}
.summary-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}
.summary-label {
  font-size: 11px;
  color: #aeaeb2;
}
.summary-value {
  font-size: 16px;
  font-weight: 700;
}
.summary-value.asset { color: #34c759; }
.summary-value.liability { color: #ff3b30; }
.summary-value.net { color: #FF6500; }
.summary-divider {
  width: 1px;
  height: 28px;
  background: #eee;
}

/* 分组 */
.item-group {
  margin-bottom: 20px;
}
.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  padding-left: 4px;
}
.group-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.asset-dot { background: #34c759; }
.liability-dot { background: #ff3b30; }
.ignore-dot { background: #aeaeb2; }
.group-label {
  font-size: 15px;
  font-weight: 600;
  color: #1d1d1f;
}

/* 记账项列表 */
.item-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.item-row {
  background: #fff;
  border-radius: 12px;
  padding: 12px 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.03);
}
.item-info {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}
.item-icon {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  color: #666;
}
.item-icon-placeholder {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  color: #999;
  flex-shrink: 0;
}
.item-name {
  font-size: 15px;
  color: #1d1d1f;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item-input-wrapper {
  flex-shrink: 0;
  margin-left: 12px;
}
.item-input {
  width: 100px;
  height: 36px;
  border: 1.5px solid #e8e8e8;
  border-radius: 8px;
  padding: 0 10px;
  font-size: 15px;
  font-weight: 500;
  color: #1d1d1f;
  text-align: right;
  background: #fafafa;
  transition: border-color 0.2s;
  -webkit-appearance: none;
}
.item-input:focus {
  outline: none;
  border-color: #FF6500;
  background: #fff;
}
.item-input::placeholder {
  color: #ccc;
  font-weight: 400;
}
/* 隐藏数字输入的上下箭头 */
.item-input::-webkit-inner-spin-button,
.item-input::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
.item-input[type=number] {
  -moz-appearance: textfield;
}

/* 底部保存按钮 */
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
.save-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: 14px;
  background: linear-gradient(135deg, #FF7A1A 0%, #FF5500 100%);
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0.5px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(255, 85, 0, 0.35);
  transition: all 0.2s ease;
  -webkit-tap-highlight-color: transparent;
}
.save-btn:active {
  transform: scale(0.97);
  box-shadow: 0 2px 8px rgba(255, 85, 0, 0.25);
}
.save-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 年月选择弹窗 */
.month-picker {
  background: #fff;
  border-radius: 14px 14px 0 0;
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
}
.picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  border-bottom: 1px solid #f0f0f0;
}
.picker-cancel {
  font-size: 15px;
  color: #8e8e93;
  background: none;
  border: none;
  cursor: pointer;
}
.picker-title {
  font-size: 16px;
  font-weight: 600;
  color: #1d1d1f;
}
.picker-confirm {
  font-size: 15px;
  color: #FF6500;
  font-weight: 600;
  background: none;
  border: none;
  cursor: pointer;
}
.picker-body {
  display: flex;
  height: 240px;
}
.picker-column {
  flex: 1;
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
}
.picker-option {
  padding: 10px 0;
  text-align: center;
  font-size: 15px;
  color: #555;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: all 0.15s;
}
.picker-option.active {
  color: #FF6500;
  font-weight: 700;
  background: rgba(255, 101, 0, 0.06);
}
</style>
