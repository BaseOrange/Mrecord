<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listTempItems, createTempItem, updateTempItem } from '@/api/modules/tempItem'
import type { FinTemplateItem } from '@/api/modules/tempItem'
import draggable from 'vuedraggable'

const route = useRoute()
const router = useRouter()
const bookId = computed(() => route.params.bookId as string)
const bookName = computed(() => (route.query.name as string) || '账簿')

// ---- 模板列表 ----
const items = ref<FinTemplateItem[]>([])
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

const typeOptions = [
  { label: '资产', value: 1 },
  { label: '负债', value: -1 },
  { label: '不统计', value: 0 },
]

const openAddDialog = () => {
  newItemName.value = ''
  newItemType.value = 1
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
    sort: String(items.value.length),
    _tempKey: genTempId(),
  } as any)
  showAddDialog.value = false
  hasChanges.value = true
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
      await updateTempItem({ bookId: bookId.value, itemList: existingItems })
    }
    // 新增项创建
    if (newItems.length > 0) {
      await createTempItem({ bookId: bookId.value, itemList: newItems })
    }

    Snackbar.success('保存成功')
    hasChanges.value = false
    // 重新拉取，让新增项也有 id
    await fetchItems()
  } catch {
    // 拦截器已处理
  } finally {
    saving.value = false
  }
}

// ---- 返回 ----
const goBack = () => {
  router.back()
}
</script>

<template>
  <div class="temp-editor">
    <!-- 顶部导航 -->
    <div class="nav-header">
      <button class="nav-back" @click="goBack">
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="15 18 9 12 15 6" />
        </svg>
      </button>
      <div class="nav-title">{{ bookName }} · 账目模板</div>
      <button
        class="nav-save"
        :class="{ 'nav-save--active': hasChanges }"
        :disabled="!hasChanges || saving"
        @click="handleSave"
      >
        {{ saving ? '保存中' : '保存' }}
      </button>
    </div>

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

    <!-- 重命名弹窗（已有项，仅改名） -->
    <var-dialog
      v-model:show="showRenameDialog"
      title="修改名称"
      confirm-button-text="确定"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="#FF6500"
      @confirm="handleRename"
    >
      <var-input
        v-model="renameValue"
        placeholder="请输入新名称"
        :maxlength="20"
        clearable
        autofocus
        @keyup.enter="handleRename"
      />
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
      </div>
    </var-dialog>
  </div>
</template>

<style scoped>
.temp-editor {
  min-height: 100vh;
  background: #f5f5f5;
}

/* 顶部导航 */
.nav-header {
  background: #fff;
  padding: 14px 16px;
  padding-top: calc(14px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid #f0f0f0;
}
.nav-back {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: none;
  color: #333;
  padding: 0;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  flex-shrink: 0;
}
.nav-title {
  flex: 1;
  font-size: 17px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
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
  flex-shrink: 0;
}
.nav-save--active {
  color: #fff;
  background: #FF6500;
  box-shadow: 0 2px 8px rgba(255, 101, 0, 0.3);
}
.nav-save:disabled {
  opacity: 0.5;
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
