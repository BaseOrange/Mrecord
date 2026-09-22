<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listTempItems } from '@/api/modules/tempItem'
import type { FinTemplateItem } from '@/api/modules/tempItem'
import { queryMonthItem, updateMonthItem } from '@/api/modules/monthItem'
import type { FinMonthItemRecord } from '@/api/modules/monthItem'
import { getYearRecordList } from '@/api/modules/monthRecord'
import type { FinMonthRecord } from '@/api/modules/monthRecord'
import { formatMoney, getChangeText, getChangeColor, roundMoney } from '@/utils/format'
import PageHeader from '@/components/PageHeader.vue'
// 图标雪碧图以模块方式引入，Vite 会自动拼上 BASE_URL 并加内容哈希，
// 保证飞牛网关模式（--base=/app/mrecord-fnos/）下路径正确（D2）
import iconsUrl from '@/../public/icons.svg'

const route = useRoute()

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

// 本月备注（为什么多赚了 / 多花了）
const note = ref('')

// 该账簿全部月度汇总（用于对比上月 / 去年同月，以及回填本月已存备注）
const monthRecords = ref<FinMonthRecord[]>([])

// 按类型分组模板项（防御性过滤 null：后端 LEFT JOIN 在无模板时可能返回 [null]）
const assetItems = computed(() => templateItems.value.filter(i => i && i.itemType === 1).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))
const liabilityItems = computed(() => templateItems.value.filter(i => i && i.itemType === -1).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))
const ignoreItems = computed(() => templateItems.value.filter(i => i && i.itemType === 0).sort((a, b) => Number(a.sort || 0) - Number(b.sort || 0)))

// 解析输入框金额：空/非法 → 0，并按后端 round_money 舍入到分（B7）。
// 负号与科学计数法不在此拦截，由保存时的 isValidAmount 统一校验（B8）。
const parseAmount = (raw: string | undefined): number => {
  const val = parseFloat(raw || '0')
  if (isNaN(val)) return 0
  return roundMoney(val)
}

// 金额输入校验：空值合法（视为 0）；非空时只接受非负的纯小数（拒绝 -5、1e3 等，B8）
const isValidAmount = (raw: string | undefined): boolean => {
  if (raw === undefined || raw === '') return true
  return /^\d*\.?\d*$/.test(raw.trim())
}

// 实时汇总（累加的是与后端一致的「分」级值，避免浮点误差）
const totalAsset = computed(() => {
  return assetItems.value.reduce((sum, item) => sum + parseAmount(itemValues.value[item.id!]), 0)
})
const totalLiability = computed(() => {
  return liabilityItems.value.reduce((sum, item) => sum + parseAmount(itemValues.value[item.id!]), 0)
})
const netAsset = computed(() => totalAsset.value - totalLiability.value)

// ---- 环比 / 同比对比 ----

// 按 `${year}-${month}` 索引月度汇总，便于查找上月与去年同月
const recordIndex = computed(() => {
  const map = new Map<string, FinMonthRecord>()
  for (const r of monthRecords.value) {
    if (r.year && r.month) map.set(`${r.year}-${r.month}`, r)
  }
  return map
})

// 上月汇总 key（跨年时落到去年 12 月）
const prevMonthKey = computed(() => {
  const y = currentYear.value
  const m = currentMonth.value
  return m === 1 ? `${y - 1}-12` : `${y}-${m - 1}`
})

// 去年同月汇总 key
const lastYearKey = computed(() => `${currentYear.value - 1}-${currentMonth.value}`)

// 本月净资产（实时，随输入变动）与上月净资产的差额，null 表示上月无数据
const momAmount = computed(() => {
  const prev = recordIndex.value.get(prevMonthKey.value)
  if (!prev || prev.netAsset === undefined || prev.netAsset === null) return null
  return netAsset.value - prev.netAsset
})

// 本月净资产与去年同月净资产的差额
const yoyAmount = computed(() => {
  const prev = recordIndex.value.get(lastYearKey.value)
  if (!prev || prev.netAsset === undefined || prev.netAsset === null) return null
  return netAsset.value - prev.netAsset
})

// 增长率（百分比），分母为 0 或无数据时返回 null
const growthRate = (cur: number, base: number | undefined | null): number | null => {
  if (base === undefined || base === null || base === 0) return null
  return ((cur - base) / Math.abs(base)) * 100
}

const momRate = computed(() => {
  const prev = recordIndex.value.get(prevMonthKey.value)
  return growthRate(netAsset.value, prev?.netAsset)
})

const yoyRate = computed(() => {
  const prev = recordIndex.value.get(lastYearKey.value)
  return growthRate(netAsset.value, prev?.netAsset)
})

// 差额描述：多攒了 / 多花了 / 持平 / 无数据
const deltaText = (val: number | null, emptyText: string, flatText: string) => {
  if (val === null) return emptyText
  if (val === 0) return flatText
  return (val > 0 ? '多攒了 ' : '多花了 ') + formatMoney(Math.abs(val))
}

const momText = computed(() => deltaText(momAmount.value, '上月暂无数据', '与上月持平'))
const yoyText = computed(() => deltaText(yoyAmount.value, '去年同月暂无数据', '与去年同月持平'))

// 备注输入框的动态提示：根据本月与上月的对比引导用户记录原因
const noteHint = computed(() => {
  const d = momAmount.value
  if (d !== null && d > 0) return `本月比上月多攒了 ${formatMoney(Math.abs(d))} 元，写下原因吧～`
  if (d !== null && d < 0) return `本月比上月多花了 ${formatMoney(Math.abs(d))} 元，写下原因吧～`
  return '记录本月财务小结，比如大额收支的原因'
})

// ---- 数据拉取 ----
const fetchData = async () => {
  loading.value = true
  // 三个请求独立处理：模板项为空时后端会抛 14301 异常，不应阻塞其他请求，也不应让 loading 卡住
  const [tplRes, recRes, yearRes] = await Promise.allSettled([
    listTempItems({ bookId }),
    queryMonthItem({ bookId, year: currentYear.value, month: currentMonth.value }),
    getYearRecordList({ bookId })
  ])
  templateItems.value = tplRes.status === 'fulfilled' && Array.isArray(tplRes.value) ? tplRes.value.filter((x): x is FinTemplateItem => x != null) : []
  existingRecords.value = recRes.status === 'fulfilled' && Array.isArray(recRes.value) ? recRes.value.filter((x): x is FinMonthItemRecord => x != null) : []
  monthRecords.value = yearRes.status === 'fulfilled' && Array.isArray(yearRes.value) ? yearRes.value.filter((x): x is FinMonthRecord => x != null) : []

  // 填充已有金额
  const values: Record<string, string> = {}
  for (const r of existingRecords.value) {
    if (r.templateItemId && r.itemValue !== undefined && r.itemValue !== null) {
      values[r.templateItemId] = String(r.itemValue)
    }
  }
  itemValues.value = values

  // 回填本月已保存的备注
  const curRecord = recordIndex.value.get(`${currentYear.value}-${currentMonth.value}`)
  note.value = curRecord?.note ?? ''
  loading.value = false
}

onMounted(() => {
  fetchData()
})

// 选择器内的草稿：确定时才写入 currentYear/currentMonth，
// 否则点了别的月份再按「取消」也会让头部标签错位（I7）
const pickerYear = ref(initYear)
const pickerMonth = ref(initMonth)

const openMonthPicker = () => {
  pickerYear.value = currentYear.value
  pickerMonth.value = currentMonth.value
  showMonthPicker.value = true
}

// ---- I6：脏数据检测（未保存的金额或备注） ----
const isDirty = computed(() => {
  // 构建服务端值 Map
  const serverMap = new Map<string, string>()
  for (const r of existingRecords.value) {
    if (r.templateItemId) serverMap.set(r.templateItemId, String(r.itemValue ?? ''))
  }
  // 对比每个模板项的输入值
  for (const item of templateItems.value) {
    if (!item?.id) continue
    if ((itemValues.value[item.id] ?? '') !== (serverMap.get(item.id) ?? '')) return true
  }
  // 对比备注
  const curRecord = recordIndex.value.get(`${currentYear.value}-${currentMonth.value}`)
  return note.value !== (curRecord?.note ?? '')
})

// 切换月份前确认
const showSwitchConfirm = ref(false)
const pendingYear = ref(0)
const pendingMonth = ref(0)

// 切换年月
const confirmMonthPick = () => {
  showMonthPicker.value = false
  if (isDirty.value) {
    pendingYear.value = pickerYear.value
    pendingMonth.value = pickerMonth.value
    showSwitchConfirm.value = true
    return
  }
  currentYear.value = pickerYear.value
  currentMonth.value = pickerMonth.value
  fetchData()
}

const confirmSwitchMonth = () => {
  showSwitchConfirm.value = false
  currentYear.value = pendingYear.value
  currentMonth.value = pendingMonth.value
  fetchData()
}

// ---- 保存 ----
const handleSave = async () => {
  saving.value = true
  try {
    // 后端 updateMonthItem 会在同一事务内按 id 有无自动执行「更新已有项 / 插入新增项」
    // （Rust: fin_month_item_record.rs 的 update 分支；Java: insertOrUpdateSelective），
    // 因此整月保存合并为单次请求，同事务原子提交，不存在「部分成功」的中间态。
    const existingMap = new Map(existingRecords.value.map(r => [r.templateItemId, r]))
    const itemList: FinMonthItemRecord[] = []
    for (const item of templateItems.value) {
      if (!item || !item.id) continue
      const raw = itemValues.value[item.id]
      // B8：拦截负数 / 科学计数法等非法输入，不静默钳制，明确提示用户修正
      if (!isValidAmount(raw)) {
        Snackbar.warning(`「${item.itemName || '记账项'}」的金额格式不正确，请输入非负数`)
        return
      }
      // B7：提交值按后端 round_money 舍入到分，与落库值一致
      const record: FinMonthItemRecord = {
        bookId,
        year: currentYear.value,
        month: currentMonth.value,
        templateItemId: item.id,
        itemValue: raw === '' || raw === undefined ? 0 : roundMoney(parseFloat(raw)),
      }
      // 已有记录补上 id → 走更新；无 id → 走插入
      const existing = existingMap.get(item.id)
      if (existing?.id) record.id = existing.id
      itemList.push(record)
    }

    if (itemList.length === 0) {
      Snackbar.warning('没有可保存的记账项')
      return
    }

    // 单次请求完成整月保存，note 只随本次请求提交一次
    await updateMonthItem({
      bookId,
      year: currentYear.value,
      month: currentMonth.value,
      itemList,
      note: note.value,
    })

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
    <!-- 顶部导航（I11 + Q1：统一用 PageHeader，含历史栈兜底） -->
    <PageHeader :title="bookName" show-back />

    <!-- 年月选择 -->
    <div class="month-selector" @click="openMonthPicker">
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
                <svg v-if="item.icon" class="item-icon"><use :href="`${iconsUrl}#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  min="0"
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
                <svg v-if="item.icon" class="item-icon"><use :href="`${iconsUrl}#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  min="0"
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
                <svg v-if="item.icon" class="item-icon"><use :href="`${iconsUrl}#icon-${item.icon}`"/></svg>
                <span v-else class="item-icon-placeholder">¥</span>
                <span class="item-name">{{ item.itemName }}</span>
              </div>
              <div class="item-input-wrapper">
                <input
                  type="number"
                  inputmode="decimal"
                  min="0"
                  class="item-input"
                  placeholder="0.00"
                  v-model="itemValues[item.id!]"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- 环比 / 同比对比卡片 -->
        <div class="compare-card">
          <div class="compare-row">
            <div class="compare-label">
              <span class="compare-name">环比</span>
              <span class="compare-sub">较上月</span>
            </div>
            <div class="compare-values">
              <span class="compare-amount" :style="{ color: getChangeColor(momAmount) }">{{ momText }}</span>
              <span class="compare-rate" :style="{ color: getChangeColor(momAmount) }">{{ getChangeText(momRate) }}</span>
            </div>
          </div>
          <div class="compare-divider"></div>
          <div class="compare-row">
            <div class="compare-label">
              <span class="compare-name">同比</span>
              <span class="compare-sub">较去年同月</span>
            </div>
            <div class="compare-values">
              <span class="compare-amount" :style="{ color: getChangeColor(yoyAmount) }">{{ yoyText }}</span>
              <span class="compare-rate" :style="{ color: getChangeColor(yoyAmount) }">{{ getChangeText(yoyRate) }}</span>
            </div>
          </div>
        </div>

        <!-- 本月备注 -->
        <div class="note-card">
          <div class="note-header">
            <svg class="note-icon" viewBox="0 0 24 24" width="16" height="16">
              <path d="M12 20h9M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span class="note-title">本月备注</span>
          </div>
          <textarea
            class="note-textarea"
            v-model="note"
            rows="3"
            maxlength="200"
            :placeholder="noteHint"
          ></textarea>
          <div class="note-foot">
            <span class="note-count">{{ note.length }}/200</span>
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
              :class="{ active: pickerYear === y }"
              @click="pickerYear = y"
            >
              {{ y }}年
            </div>
          </div>
          <div class="picker-column">
            <div
              v-for="m in monthOptions"
              :key="m"
              class="picker-option"
              :class="{ active: pickerMonth === m }"
              @click="pickerMonth = m"
            >
              {{ m }}月
            </div>
          </div>
        </div>
      </div>
    </var-popup>

    <!-- I6：切换月份前确认（有未保存修改时） -->
    <var-dialog
      v-model:show="showSwitchConfirm"
      title="切换月份"
      confirm-button-text="放弃修改并切换"
      cancel-button-text="继续编辑"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      @confirm="confirmSwitchMonth"
    >
      <div class="switch-confirm-tips">
        当前月份有未保存的修改，切换后将丢失。确定要放弃修改并切换到 {{ pendingYear }} 年 {{ pendingMonth }} 月吗？
      </div>
    </var-dialog>
  </div>
</template>

<style scoped>
.record-page {
  min-height: 100vh;
  background: #f5f5f5;
  display: flex;
  flex-direction: column;
}

/* 顶部导航样式已迁移至 PageHeader.vue（Q1） */

/* I6：切换月份确认弹窗 */
.switch-confirm-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
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

/* 环比 / 同比对比卡片 */
.compare-card {
  background: #fff;
  border-radius: 14px;
  padding: 14px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  margin-bottom: 10px;
}
.compare-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 0;
}
.compare-divider {
  height: 1px;
  background: #f0f0f0;
  margin: 2px 0;
}
.compare-label {
  display: flex;
  align-items: baseline;
  gap: 6px;
}
.compare-name {
  font-size: 14px;
  font-weight: 600;
  color: #1d1d1f;
}
.compare-sub {
  font-size: 11px;
  color: #aeaeb2;
}
.compare-values {
  display: flex;
  align-items: baseline;
  gap: 10px;
}
.compare-amount {
  font-size: 14px;
  font-weight: 600;
}
.compare-rate {
  font-size: 12px;
  font-weight: 500;
  min-width: 56px;
  text-align: right;
}

/* 本月备注 */
.note-card {
  background: #fff;
  border-radius: 14px;
  padding: 14px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  margin-bottom: 8px;
}
.note-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 10px;
}
.note-icon {
  color: #FF6500;
  flex-shrink: 0;
}
.note-title {
  font-size: 14px;
  font-weight: 600;
  color: #1d1d1f;
}
.note-textarea {
  width: 100%;
  border: 1.5px solid #e8e8e8;
  border-radius: 10px;
  padding: 10px 12px;
  font-size: 14px;
  line-height: 1.6;
  color: #1d1d1f;
  background: #fafafa;
  resize: none;
  font-family: inherit;
  transition: border-color 0.2s;
  box-sizing: border-box;
}
.note-textarea:focus {
  outline: none;
  border-color: #FF6500;
  background: #fff;
}
.note-textarea::placeholder {
  color: #c2c2c7;
  font-size: 13px;
}
.note-foot {
  display: flex;
  justify-content: flex-end;
  margin-top: 6px;
}
.note-count {
  font-size: 11px;
  color: #c2c2c7;
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
  color: #FF6500;
}
.item-icon-placeholder {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  color: #FF6500;
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
