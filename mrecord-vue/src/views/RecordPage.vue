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
import MoneyText from '@/components/MoneyText.vue'
import StateView from '@/components/StateView.vue'
import AppIcon from '@/components/AppIcon.vue'
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
// 注意：输入框 type="number" 时 v-model 会自动把值转成数字（如 1 而非 "1"），
// 因此参数可能是 number，必须先 String() 转换再 trim，否则数字调 .trim() 抛 TypeError
const isValidAmount = (raw: string | number | undefined | null): boolean => {
  if (raw === undefined || raw === null || raw === '') return true
  return /^\d*\.?\d*$/.test(String(raw).trim())
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

  // [MR-SAVE] 诊断日志：记录数据加载结果（tplRes 若 rejected 说明模板项加载失败，保存按钮不会渲染）
  console.log('[MR-SAVE] fetchData 结果:',
    'templateItems=', templateItems.value.length,
    'existingRecords=', existingRecords.value.length,
    'tplRes=', tplRes.status, tplRes.status === 'rejected' ? String(tplRes.reason) : '',
    'recRes=', recRes.status,
    'bookId=', JSON.stringify(bookId))

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
    // 输入框 type="number" 产出的是数字，服务端回填的是字符串，统一 String() 后比较
    if (String(itemValues.value[item.id] ?? '') !== (serverMap.get(item.id) ?? '')) return true
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
  // [MR-SAVE] 入口日志：确认按钮点击事件生效，记录模板项数量与当前输入值
  console.log('[MR-SAVE] handleSave 被调用, templateItems.length=', templateItems.value.length, 'itemValues=', JSON.stringify(itemValues.value))
  try {
    // 后端 updateMonthItem 会在同一事务内按 id 有无自动执行「更新已有项 / 插入新增项」
    // （Rust: fin_month_item_record.rs 的 update 分支；Java: insertOrUpdateSelective），
    // 因此整月保存合并为单次请求，同事务原子提交，不存在「部分成功」的中间态。
    const existingMap = new Map(existingRecords.value.map(r => [r.templateItemId, r]))
    const itemList: FinMonthItemRecord[] = []
    for (const item of templateItems.value) {
      if (!item || !item.id) continue
      const raw = itemValues.value[item.id]
      // [MR-SAVE] 逐项日志：记录每个模板项的输入原始值与校验结果
      console.log('[MR-SAVE] 遍历项 id=', item.id, 'itemName=', item.itemName, 'raw=', JSON.stringify(raw), 'isValid=', isValidAmount(raw))
      // B8：拦截负数 / 科学计数法等非法输入，不静默钳制，明确提示用户修正
      if (!isValidAmount(raw)) {
        console.warn('[MR-SAVE] 金额校验未通过，中止本次保存')
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

    // [MR-SAVE] itemList 构建结果：若为 0 说明所有项被跳过（无 id 或校验未过）
    console.log('[MR-SAVE] itemList 构建完成 length=', itemList.length, JSON.stringify(itemList))

    if (itemList.length === 0) {
      Snackbar.warning('没有可保存的记账项')
      return
    }

    // 单次请求完成整月保存，note 只随本次请求提交一次
    console.log('[MR-SAVE] 即将调用 updateMonthItem, bookId=', JSON.stringify(bookId), 'year=', currentYear.value, 'month=', currentMonth.value)
    const saveRes = await updateMonthItem({
      bookId,
      year: currentYear.value,
      month: currentMonth.value,
      itemList,
      note: note.value,
    })
    console.log('[MR-SAVE] updateMonthItem 返回:', JSON.stringify(saveRes))

    Snackbar.success('保存成功')
    // 刷新数据
    await fetchData()
  } catch (e) {
    // [MR-SAVE] 关键：此前这里把所有异常静默吞掉（「拦截器已处理」），现在打到控制台定位问题
    console.error('[MR-SAVE] 保存流程抛出异常（此前被静默吞掉）:', e)
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
      <AppIcon name="chevron-down" :size="16" :stroke-width="2.2" />
    </div>

    <div class="page-body">
      <!-- 加载 / 空状态 -->
      <StateView
        v-if="loading || templateItems.length === 0"
        :state="loading ? 'loading' : 'empty'"
        empty-text="还没有记账项"
        empty-sub="请先编辑账目模板"
        empty-icon="notebook-text"
      />

      <!-- 记账表单 -->
      <template v-else>
        <!-- 实时汇总卡片 -->
        <div class="summary-card">
          <div class="summary-item">
            <span class="summary-label">总资产</span>
            <MoneyText :value="totalAsset" size="md" />
          </div>
          <div class="summary-divider"></div>
          <div class="summary-item">
            <span class="summary-label">总负债</span>
            <MoneyText :value="totalLiability" size="md" />
          </div>
          <div class="summary-divider"></div>
          <div class="summary-item">
            <span class="summary-label">净资产</span>
            <MoneyText :value="netAsset" size="md" />
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
            <AppIcon name="file-text" :size="16" class="note-icon" />
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
      confirm-button-color="var(--semantic-danger)"
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
  background: var(--bg-canvas);
  display: flex;
  flex-direction: column;
}

/* I6：切换月份确认弹窗 */
.switch-confirm-tips {
  font-size: var(--text-body);
  color: var(--text-secondary);
  line-height: 1.8;
}

/* 年月选择 */
.month-selector {
  background: var(--bg-surface);
  padding: var(--space-2) var(--space-4);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  border-bottom: 1px solid var(--separator);
}
.month-text {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--brand);
}
.month-selector :deep(svg) {
  color: var(--brand);
  transition: transform var(--duration-fast) var(--ease-out);
}

/* 页面主体 */
.page-body {
  flex: 1;
  padding: var(--space-3) var(--page-padding) 0;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

/* 汇总卡片 */
.summary-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-4) var(--space-5);
  display: flex;
  align-items: center;
  box-shadow: var(--shadow-sm);
  margin-bottom: var(--space-5);
}
.summary-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}
.summary-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}
.summary-divider {
  width: 1px;
  height: 28px;
  background: var(--separator);
}

/* 环比 / 同比对比卡片 */
.compare-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  box-shadow: var(--shadow-sm);
  margin-bottom: var(--space-2);
}
.compare-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-1) 0;
}
.compare-divider {
  height: 1px;
  background: var(--separator);
  margin: 2px 0;
}
.compare-label {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
}
.compare-name {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.compare-sub {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}
.compare-values {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
}
.compare-amount {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
}
.compare-rate {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  min-width: 56px;
  text-align: right;
}

/* 本月备注 */
.note-card {
  background: var(--bg-surface);
  border-radius: var(--radius-lg);
  padding: var(--space-3) var(--space-4);
  box-shadow: var(--shadow-sm);
  margin-bottom: var(--space-2);
}
.note-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}
.note-icon {
  color: var(--brand);
  flex-shrink: 0;
}
.note-title {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.note-textarea {
  width: 100%;
  border: 1.5px solid var(--separator);
  border-radius: var(--radius-md);
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-body);
  line-height: 1.6;
  color: var(--text-primary);
  background: var(--bg-surface-2);
  resize: none;
  font-family: inherit;
  transition: border-color var(--duration-fast), background-color var(--duration-fast);
  box-sizing: border-box;
}
.note-textarea:focus {
  outline: none;
  border-color: var(--brand);
  background: var(--bg-surface);
}
.note-textarea::placeholder {
  color: var(--text-quaternary);
  font-size: var(--text-sm);
}
.note-foot {
  display: flex;
  justify-content: flex-end;
  margin-top: var(--space-1);
}
.note-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

/* 分组 */
.item-group {
  margin-bottom: var(--space-5);
}
.group-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
  padding-left: var(--space-1);
}
.group-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.asset-dot { background: var(--semantic-down); }
.liability-dot { background: var(--semantic-up); }
.ignore-dot { background: var(--text-tertiary); }
.group-label {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

/* 记账项列表 */
.item-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.item-row {
  background: var(--bg-surface);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: var(--shadow-xs);
}
.item-info {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
}
.item-icon {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  color: var(--brand);
}
.item-icon-placeholder {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--text-body);
  color: var(--brand);
  flex-shrink: 0;
}
.item-name {
  font-size: var(--text-body);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item-input-wrapper {
  flex-shrink: 0;
  margin-left: var(--space-3);
}
.item-input {
  width: 100px;
  height: 36px;
  border: 1.5px solid var(--separator);
  border-radius: var(--radius-sm);
  padding: 0 var(--space-2);
  font-size: var(--text-body);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  text-align: right;
  background: var(--bg-surface-2);
  transition: border-color var(--duration-fast), background-color var(--duration-fast);
  -webkit-appearance: none;
}
.item-input:focus {
  outline: none;
  border-color: var(--brand);
  background: var(--bg-surface);
}
.item-input::placeholder {
  color: var(--text-quaternary);
  font-weight: var(--weight-regular);
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
  padding: var(--space-3) var(--page-padding);
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
  background: linear-gradient(to top, var(--bg-canvas) 60%, transparent);
  z-index: 10;
}
.save-btn {
  width: 100%;
  height: 48px;
  border: none;
  border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--brand) 0%, var(--brand-accent) 100%);
  color: #fff;
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  letter-spacing: 0.5px;
  cursor: pointer;
  box-shadow: 0 4px 16px rgba(255, 85, 0, 0.35);
  transition: transform var(--duration-fast) var(--ease-out), box-shadow var(--duration-fast);
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
  background: var(--bg-surface);
  border-radius: var(--radius-lg) var(--radius-lg) 0 0;
  padding-bottom: calc(16px + env(safe-area-inset-bottom, 0px));
}
.picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-4);
  border-bottom: 1px solid var(--separator);
}
.picker-cancel {
  font-size: var(--text-body);
  color: var(--text-secondary);
  background: none;
  border: none;
  cursor: pointer;
}
.picker-title {
  font-size: var(--text-title-3);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}
.picker-confirm {
  font-size: var(--text-body);
  color: var(--brand);
  font-weight: var(--weight-semibold);
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
  padding: var(--space-2) 0;
  text-align: center;
  font-size: var(--text-body);
  color: var(--text-secondary);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: all var(--duration-fast);
}
.picker-option.active {
  color: var(--brand);
  font-weight: var(--weight-bold);
  background: var(--brand-soft);
}
</style>
