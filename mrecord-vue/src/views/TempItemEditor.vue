<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listTempItems, createTempItem, updateTempItem, deleteTempItem } from '@/api/modules/tempItem'
import type { FinTemplateItem } from '@/api/modules/tempItem'
import draggable from 'vuedraggable'
import IconPicker from '@/components/IconPicker.vue'
import PageHeader from '@/components/PageHeader.vue'
// 图标雪碧图以模块方式引入，Vite 会自动拼上 BASE_URL 并加内容哈希，
// 保证飞牛网关模式（--base=/app/mrecord-fnos/）下路径正确（D2）
import iconsUrl from '@/../public/icons.svg'

// Q2：编辑态类型（新增项用 _tempKey 做拖拽 key，领域类型无此字段）
type EditableItem = FinTemplateItem & { _tempKey?: string }

const route = useRoute()
const router = useRouter()
const bookId = computed(() => route.params.bookId as string)
const bookName = computed(() => (route.query.name as string) || '账簿')

// ---- 模板列表 ----
const items = ref<EditableItem[]>([])
const loading = ref(false)
const saving = ref(false)
const hasChanges = ref(false)

const fetchItems = async () => {
  loading.value = true
  try {
    const res = await listTempItems({ bookId: bookId.value })
    items.value = (res || []).map((item, index) => ({
      ...item,
      sort: item.sort ?? String(index)
    }))
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

onMounted(fetchItems)

// ---- 拖拽排序 ----
const onDragEnd = () => {
  // 拖拽结束后重新编排 sort
  items.value.forEach((item, index) => {
    item.sort = String(index)
  })
  hasChanges.value = true
}

// ---- 重命名（已有项） ----
const showRenameDialog = ref(false)
const renameIndex = ref(-1)
const renameValue = ref('')

const openRename = (index: number) => {
  renameIndex.value = index
  renameValue.value = items.value[index].itemName || ''
  showRenameDialog.value = true
}

const handleRename = () => {
  const name = renameValue.value.trim()
  if (!name) {
    Snackbar.warning('请输入名称')
    return
  }
  items.value[renameIndex.value].itemName = name
  showRenameDialog.value = false
  hasChanges.value = true
}

// ---- 删除已有项 ----
const showDeleteConfirm = ref(false)
const deleting = ref(false)

const openDeleteConfirm = () => {
  const item = items.value[renameIndex.value]
  if (!item?.id) return
  // 关闭编辑弹窗，改由二次确认弹窗承接
  showRenameDialog.value = false
  showDeleteConfirm.value = true
}

const handleDeleteExisting = async () => {
  const item = items.value[renameIndex.value]
  if (!item?.id) return
  deleting.value = true
  try {
    await deleteTempItem({ bookId: bookId.value, templateItemId: item.id })
    // 后端已落库，本地直接移除并重排（不用整表 refetch，避免丢失未保存的新增项）
    items.value.splice(renameIndex.value, 1)
    items.value.forEach((it, i) => { it.sort = String(i) })
    showDeleteConfirm.value = false
    Snackbar.success('已删除')
  } catch {
    // 拦截器已处理（如 14306「已有记账记录，无法删除」）
  } finally {
    deleting.value = false
  }
}

// ---- 编辑新增项（改名+改类型+删除） ----
const showEditNewDialog = ref(false)
const editNewIndex = ref(-1)
const editNewName = ref('')
const editNewType = ref(1)

const openEditNew = (index: number) => {
  editNewIndex.value = index
  editNewName.value = items.value[index].itemName || ''
  editNewType.value = items.value[index].itemType ?? 1
  showEditNewDialog.value = true
}

const handleEditNewSave = () => {
  const name = editNewName.value.trim()
  if (!name) {
    Snackbar.warning('请输入名称')
    return
  }
  items.value[editNewIndex.value].itemName = name
  items.value[editNewIndex.value].itemType = editNewType.value
  showEditNewDialog.value = false
  hasChanges.value = true
}

const handleDeleteNew = () => {
  items.value.splice(editNewIndex.value, 1)
  showEditNewDialog.value = false
  // 重新编排 sort
  items.value.forEach((item, i) => { item.sort = String(i) })
  hasChanges.value = items.value.some(i => !i.id) || true
}

// ---- 点击行：区分已有项 vs 新增项 ----
const onItemClick = (index: number) => {
  const item = items.value[index]
  if (item.id) {
    openRename(index)
  } else {
    openEditNew(index)
  }
}

// ---- 临时ID生成（新增项用） ----
let tempIdCounter = 0
const genTempId = () => `_new_${Date.now()}_${++tempIdCounter}`

// ---- 添加模板项 ----
const showAddDialog = ref(false)
const newItemName = ref('')
const newItemType = ref(1)
const newItemIcon = ref('')

const typeOptions = [
  { label: '资产', value: 1 },
  { label: '负债', value: -1 },
  { label: '不统计', value: 0 },
]

const openAddDialog = () => {
  newItemName.value = ''
  newItemType.value = 1
  newItemIcon.value = ''
  showAddDialog.value = true
}

const handleAdd = () => {
  const name = newItemName.value.trim()
  if (!name) {
    Snackbar.warning('请输入名称')
    return
  }
  items.value.push({
    bookId: bookId.value,
    itemName: name,
    itemType: newItemType.value,
    icon: newItemIcon.value || undefined,
    sort: String(items.value.length),
    _tempKey: genTempId(),
  })
  showAddDialog.value = false
  hasChanges.value = true
}

// ---- 图标选择 ----
const showIconPicker = ref(false)
const iconPickerTarget = ref<'rename' | 'editNew' | 'add'>('add')
const iconPickerValue = ref('')

const openIconPicker = (target: 'rename' | 'editNew' | 'add') => {
  iconPickerTarget.value = target
  if (target === 'rename') {
    iconPickerValue.value = items.value[renameIndex.value]?.icon || ''
  } else if (target === 'editNew') {
    iconPickerValue.value = items.value[editNewIndex.value]?.icon || ''
  } else {
    iconPickerValue.value = newItemIcon.value
  }
  showIconPicker.value = true
}

const onIconSelect = (key: string) => {
  if (iconPickerTarget.value === 'rename') {
    items.value[renameIndex.value].icon = key
    hasChanges.value = true
  } else if (iconPickerTarget.value === 'editNew') {
    items.value[editNewIndex.value].icon = key
    hasChanges.value = true
  } else {
    newItemIcon.value = key
  }
}

// ---- 类型标签 ----
const typeLabel = (type?: number) => {
  switch (type) {
    case 1: return '资产'
    case -1: return '负债'
    case 0: return '不统计'
    default: return '未知'
  }
}
const typeColor = (type?: number) => {
  switch (type) {
    case 1: return '#34c759'
    case -1: return '#ff3b30'
    case 0: return '#aeaeb2'
    default: return '#aeaeb2'
  }
}

// ---- 保存 ----

/**
 * 用接口返回的结果回填本地模板项的 id。
 *
 * 后端按入参顺序逐项处理并**原序**返回：
 * - Rust `fin_template_item::create/update` 的 `result.push(model.into())`；
 * - Java `ceateFinTemplateItemList` / `updateFinTemplateItemList` 直接返回入参列表（已塞入新 id）。
 *
 * 因此按索引一一对应回填即可；数量不一致（异常响应）时放弃回填，交由调用方兜底处理。
 */
const reconcileItems = (local: FinTemplateItem[], remote?: FinTemplateItem[] | null) => {
  if (!remote || remote.length !== local.length) return
  local.forEach((item, index) => {
    item.id = remote[index].id
  })
}

const handleSave = async () => {
  saving.value = true
  try {
    // 按页面顺序重新编排 sort
    items.value.forEach((item, index) => {
      item.sort = String(index)
    })

    const existingItems = items.value.filter(i => i.id)
    const newItems = items.value.filter(i => !i.id)

    // 已有项更新（改名/排序）
    if (existingItems.length > 0) {
      const updated = await updateTempItem({ bookId: bookId.value, itemList: existingItems })
      reconcileItems(existingItems, updated)
    }
    // 新增项创建：响应内即含后端生成的 id，直接回填。
    // 不能依赖随后的 fetchItems() 回补——一旦「保存成功但回拉失败」，hasChanges
    // 已置 false 而新增项仍无 id，下次保存会把已创建的科目再创建一遍（B4）。
    if (newItems.length > 0) {
      const created = await createTempItem({ bookId: bookId.value, itemList: newItems })
      reconcileItems(newItems, created)
    }

    // 回填异常（响应与入参数量不一致）→ 以服务端为准整体回拉兜底
    if (items.value.some(i => !i.id)) {
      await fetchItems()
    }
    // 仍有无 id 的项，说明保存未完整落库：保持脏标记让用户重试，绝不重复创建
    if (items.value.some(i => !i.id)) {
      Snackbar.warning('部分模板项保存失败，请重试')
      hasChanges.value = true
      return
    }

    Snackbar.success('保存成功')
    hasChanges.value = false
  } catch {
    // 拦截器已处理错误提示；保存未完成，保持脏标记便于重试
    hasChanges.value = true
  } finally {
    saving.value = false
  }
}

// ---- I6：返回前确认（有未保存修改时） ----
const showBackConfirm = ref(false)

// ---- 返回 ----
const goBack = () => {
  if (hasChanges.value) {
    showBackConfirm.value = true
    return
  }
  router.back()
}

const confirmBack = () => {
  showBackConfirm.value = false
  router.back()
}
</script>

<template>
  <div class="temp-editor">
    <!-- 顶部导航（I11 + Q1：统一用 PageHeader，含历史栈兜底） -->
    <PageHeader :title="`${bookName} · 账目模板`" show-back>
      <template #right>
        <button
          class="nav-save"
          :class="{ 'nav-save--active': hasChanges }"
          :disabled="!hasChanges || saving"
          @click="handleSave"
        >
          {{ saving ? '保存中' : '保存' }}
        </button>
      </template>
    </PageHeader>

    <div class="editor-body">
      <!-- 加载中 -->
      <div v-if="loading" class="thinking-state">
        <div class="thinking-face">🤔</div>
        <div class="thinking-text">
          thinking
          <span class="thinking-dots">
            <span class="dot">.</span><span class="dot">.</span><span class="dot">.</span>
          </span>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else-if="items.length === 0" class="empty-state">
        <p class="empty-text">暂无账目模板</p>
        <p class="empty-sub">点击下方按钮添加第一个模板项</p>
      </div>

      <!-- 模板列表（可拖拽） -->
      <template v-else>
        <p class="tip-text">长按拖拽调整顺序，点击项目可修改名称</p>

        <draggable
          v-model="items"
          :item-key="(item: any) => item.id || item._tempKey"
          handle=".drag-handle"
          ghost-class="drag-ghost"
          animation="200"
          @end="onDragEnd"
        >
          <template #item="{ element, index }">
            <div class="item-row" @click="onItemClick(index)">
              <!-- 拖拽手柄 -->
              <div class="drag-handle">
                <svg viewBox="0 0 24 24" width="18" height="18" fill="#c7c7cc">
                  <circle cx="9" cy="6" r="1.5" /><circle cx="15" cy="6" r="1.5" />
                  <circle cx="9" cy="12" r="1.5" /><circle cx="15" cy="12" r="1.5" />
                  <circle cx="9" cy="18" r="1.5" /><circle cx="15" cy="18" r="1.5" />
                </svg>
              </div>
              <!-- 新增标记 -->
              <span v-if="!element.id" class="new-badge">新</span>
              <!-- 图标 -->
              <div class="item-icon-wrap" :class="{ 'item-icon-wrap--empty': !element.icon }">
                <svg v-if="element.icon" viewBox="0 0 24 24" width="18" height="18">
                  <use :href="`${iconsUrl}#icon-${element.icon}`" />
                </svg>
                <span v-else class="item-icon-placeholder">?</span>
              </div>
              <!-- 名称 -->
              <div class="item-name">{{ element.itemName }}</div>
              <!-- 类型标签 -->
              <span class="item-type" :style="{ color: typeColor(element.itemType) }">
                {{ typeLabel(element.itemType) }}
              </span>
            </div>
          </template>
        </draggable>
      </template>

      <!-- 添加按钮（非加载中时始终显示） -->
      <div v-if="!loading" class="add-row" @click="openAddDialog">
        <span class="add-icon">+</span>
        <span class="add-text">添加模板项</span>
      </div>
    </div>

    <!-- 重命名弹窗（已有项，改名+改图标） -->
    <var-dialog
      v-model:show="showRenameDialog"
      title="编辑模板项"
      confirm-button-text="确定"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      @confirm="handleRename"
    >
      <div class="add-form">
        <var-input
          v-model="renameValue"
          placeholder="请输入新名称"
          :maxlength="20"
          clearable
          autofocus
          @keyup.enter="handleRename"
        />
        <div class="icon-select-row" @click="openIconPicker('rename')">
          <span class="icon-select-label">图标</span>
          <div class="icon-select-preview" v-if="items[renameIndex]?.icon">
            <svg viewBox="0 0 24 24" width="20" height="20">
              <use :href="`${iconsUrl}#icon-${items[renameIndex].icon}`" />
            </svg>
          </div>
          <span v-else class="icon-select-hint">点击选择</span>
          <span class="icon-select-arrow">›</span>
        </div>
        <button class="delete-new-btn" @click="openDeleteConfirm" type="button">
          删除此项
        </button>
      </div>
    </var-dialog>

    <!-- 删除确认弹窗（已有项，删除需调后端） -->
    <var-dialog
      v-model:show="showDeleteConfirm"
      title="删除模板项"
      confirm-button-text="删除"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      :confirm-button-loading="deleting"
      @confirm="handleDeleteExisting"
    >
      <div class="delete-confirm-tips">
        确定要删除「{{ items[renameIndex]?.itemName }}」吗？
        <br />已有记账记录的科目无法删除（保护历史数据）。
      </div>
    </var-dialog>

    <!-- 编辑新增项弹窗（改名+改类型+删除） -->
    <var-dialog
      v-model:show="showEditNewDialog"
      title="编辑模板项"
      confirm-button-text="确定"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      @confirm="handleEditNewSave"
    >
      <div class="add-form">
        <var-input
          v-model="editNewName"
          placeholder="请输入名称"
          :maxlength="20"
          clearable
          autofocus
        />
        <div class="type-selector">
          <span class="type-label">类型</span>
          <div class="type-chips">
            <button
              v-for="opt in typeOptions"
              :key="opt.value"
              class="type-chip"
              :class="{ 'type-chip--active': editNewType === opt.value }"
              :style="editNewType === opt.value ? { background: typeColor(opt.value), color: '#fff' } : {}"
              @click="editNewType = opt.value"
              type="button"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>
        <div class="icon-select-row" @click="openIconPicker('editNew')">
          <span class="icon-select-label">图标</span>
          <div class="icon-select-preview" v-if="items[editNewIndex]?.icon">
            <svg viewBox="0 0 24 24" width="20" height="20">
              <use :href="`${iconsUrl}#icon-${items[editNewIndex].icon}`" />
            </svg>
          </div>
          <span v-else class="icon-select-hint">点击选择</span>
          <span class="icon-select-arrow">›</span>
        </div>
        <button class="delete-new-btn" @click="handleDeleteNew" type="button">
          删除此项
        </button>
      </div>
    </var-dialog>

    <!-- 添加模板项弹窗 -->
    <var-dialog
      v-model:show="showAddDialog"
      title="添加模板项"
      confirm-button-text="添加"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      @confirm="handleAdd"
    >
      <div class="add-form">
        <var-input
          v-model="newItemName"
          placeholder="请输入模板项名称"
          :maxlength="20"
          clearable
          autofocus
        />
        <div class="type-selector">
          <span class="type-label">类型</span>
          <div class="type-chips">
            <button
              v-for="opt in typeOptions"
              :key="opt.value"
              class="type-chip"
              :class="{ 'type-chip--active': newItemType === opt.value }"
              :style="newItemType === opt.value ? { background: typeColor(opt.value), color: '#fff' } : {}"
              @click="newItemType = opt.value"
              type="button"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>
        <div class="icon-select-row" @click="openIconPicker('add')">
          <span class="icon-select-label">图标</span>
          <div class="icon-select-preview" v-if="newItemIcon">
            <svg viewBox="0 0 24 24" width="20" height="20">
              <use :href="`${iconsUrl}#icon-${newItemIcon}`" />
            </svg>
          </div>
          <span v-else class="icon-select-hint">点击选择</span>
          <span class="icon-select-arrow">›</span>
        </div>
      </div>
    </var-dialog>

    <!-- 图标选择器 -->
    <IconPicker
      v-model:show="showIconPicker"
      v-model="iconPickerValue"
      @select="onIconSelect"
    />

    <!-- I6：返回前确认（有未保存修改时） -->
    <var-dialog
      v-model:show="showBackConfirm"
      title="放弃修改"
      confirm-button-text="放弃并返回"
      cancel-button-text="继续编辑"
      confirm-button-text-color="#fff"
      confirm-button-color="#e74c3c"
      @confirm="confirmBack"
    >
      <div class="back-confirm-tips">
        当前有未保存的模板修改，返回后将丢失。确定要放弃修改并返回吗？
      </div>
    </var-dialog>
  </div>
</template>

<style scoped>
.temp-editor {
  min-height: 100vh;
  background: #f5f5f5;
}

/* 顶部导航样式已迁移至 PageHeader.vue（Q1）；保存按钮保留本页样式（在 right slot 中） */
.nav-save {
  border: none;
  background: none;
  font-size: 15px;
  font-weight: 500;
  color: #c7c7cc;
  padding: 6px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.nav-save--active {
  color: #fff;
  background: #FF6500;
  box-shadow: 0 2px 8px rgba(255, 101, 0, 0.3);
}
.nav-save:disabled {
  opacity: 0.5;
}

/* I6：返回确认弹窗 */
.back-confirm-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
}

/* 编辑区域 */
.editor-body {
  padding: 16px;
}

.tip-text {
  font-size: 13px;
  color: #aeaeb2;
  margin-bottom: 12px;
}

/* 模板项行 */
.item-row {
  display: flex;
  align-items: center;
  background: #fff;
  padding: 14px 16px;
  margin-bottom: 1px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: background 0.15s;
}
.item-row:first-child {
  border-radius: 12px 12px 0 0;
}
.item-row:last-child {
  border-radius: 0 0 12px 12px;
  margin-bottom: 0;
}
.item-row:only-child {
  border-radius: 12px;
}
.item-row:active {
  background: #f8f8f8;
}

.drag-handle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  flex-shrink: 0;
  cursor: grab;
  touch-action: none;
}
.drag-handle:active {
  cursor: grabbing;
}

.item-name {
  flex: 1;
  font-size: 16px;
  font-weight: 500;
  color: #1d1d1f;
  margin-left: 10px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 列表中的图标 */
.item-icon-wrap {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  margin-left: 8px;
  color: #666;
}
.item-icon-wrap svg {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.item-icon-wrap--empty {
  border: 1px dashed #d0d0d0;
  background: transparent;
}
.item-icon-placeholder {
  font-size: 14px;
  color: #c7c7cc;
}

.item-type {
  font-size: 12px;
  font-weight: 500;
  flex-shrink: 0;
  margin-left: 8px;
}

/* 拖拽幽灵样式 */
.drag-ghost {
  opacity: 0.4;
  background: #FFF3E0;
}

/* 新增标记 */
.new-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 6px;
  background: #FF6500;
  color: #fff;
  font-size: 10px;
  font-weight: 600;
  margin-left: 6px;
  flex-shrink: 0;
}

/* 添加按钮行 */
.add-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  margin-top: 16px;
  padding: 14px;
  background: #fff;
  border-radius: 12px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: background 0.15s;
}
.add-row:active {
  background: #f8f8f8;
}
.add-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: #FF6500;
  color: #fff;
  font-size: 18px;
  line-height: 1;
}
.add-text {
  font-size: 15px;
  font-weight: 500;
  color: #FF6500;
}

/* 添加弹窗表单 */
.add-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.type-selector {
  display: flex;
  align-items: center;
  gap: 12px;
}
.type-label {
  font-size: 14px;
  color: #666;
  flex-shrink: 0;
}
.type-chips {
  display: flex;
  gap: 8px;
}
.type-chip {
  padding: 6px 14px;
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
.type-chip--active {
  border-color: transparent;
}

/* 图标选择行 */
.icon-select-row {
  display: flex;
  align-items: center;
  padding: 10px 0;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.icon-select-label {
  font-size: 14px;
  color: #666;
  flex-shrink: 0;
}
.icon-select-preview {
  margin-left: auto;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
}
.icon-select-preview svg {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.icon-select-hint {
  margin-left: auto;
  font-size: 14px;
  color: #c7c7cc;
}
.icon-select-arrow {
  margin-left: 6px;
  font-size: 18px;
  color: #c7c7cc;
}

/* 删除新增项按钮 */
.delete-new-btn {
  width: 100%;
  padding: 10px;
  border: none;
  border-radius: 10px;
  background: #fff5f5;
  color: #ff3b30;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.15s;
  -webkit-tap-highlight-color: transparent;
}
.delete-new-btn:active {
  background: #ffe5e5;
}

/* 删除确认提示 */
.delete-confirm-tips {
  font-size: 14px;
  color: #555;
  line-height: 1.8;
}

/* thinking */
.thinking-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100px 0 40px;
}
.thinking-face {
  font-size: 48px;
  animation: thinking-wobble 2s ease-in-out infinite;
}
@keyframes thinking-wobble {
  0%, 100% { transform: rotate(0deg); }
  25% { transform: rotate(-8deg); }
  75% { transform: rotate(8deg); }
}
.thinking-text {
  margin-top: 16px;
  font-size: 16px;
  font-weight: 500;
  color: #8e8e93;
  font-style: italic;
  letter-spacing: 1px;
}
.thinking-dots .dot {
  animation: thinking-blink 1.4s infinite both;
  opacity: 0;
}
.thinking-dots .dot:nth-child(1) { animation-delay: 0s; }
.thinking-dots .dot:nth-child(2) { animation-delay: 0.2s; }
.thinking-dots .dot:nth-child(3) { animation-delay: 0.4s; }
@keyframes thinking-blink {
  0%, 80%, 100% { opacity: 0; }
  40% { opacity: 1; }
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 80px 0 40px;
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
</style>
