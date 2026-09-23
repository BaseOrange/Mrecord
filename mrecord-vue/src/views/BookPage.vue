<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { createBook, updateBook, deleteBook, listBooks, getMyDataStatistics } from '@/api/modules/book'
import type { BookInfo, BookStatistics } from '@/api/modules/book'
import { formatMoney, getChangeText } from '@/utils/format'
import PageHeader from '@/components/PageHeader.vue'
import ListGroup from '@/components/ListGroup.vue'
import ListCell from '@/components/ListCell.vue'
import StateView from '@/components/StateView.vue'
import AppIcon from '@/components/AppIcon.vue'

const router = useRouter()

// ---- 账簿统计（用于在列表中显示净资产与环比） ----
// BookInfo 只含账簿本身信息，净资产需由统计接口反查；两个请求并行发出。
const statsMap = ref<Map<string, BookStatistics>>(new Map())

const fetchStats = async () => {
  try {
    const res = await getMyDataStatistics()
    const list: BookStatistics[] = res?.recordList ?? []
    statsMap.value = new Map(list.filter((i) => i?.bookId).map((i) => [i.bookId as string, i]))
  } catch {
    // 拦截器已处理；统计缺失时列表只显示名称
  }
}

// ---- 账簿列表（懒加载） ----
const PAGE_SIZE = 10
const books = ref<BookInfo[]>([])
const loading = ref(false)
const refreshing = ref(false)
const pageNum = ref(1)
const hasMore = ref(true)

const fetchBooks = async (reset = false) => {
  if (loading.value) return
  if (!reset && !hasMore.value) return

  if (reset) {
    pageNum.value = 1
    hasMore.value = true
  }

  loading.value = true
  try {
    const res = await listBooks({ pageNum: pageNum.value, pageSize: PAGE_SIZE })
    const records = res.records || []
    if (reset) {
      books.value = records
    } else {
      books.value.push(...records)
    }
    // 没有更多数据了
    if (records.length < PAGE_SIZE || pageNum.value >= res.totalPage) {
      hasMore.value = false
    } else {
      pageNum.value++
    }
  } catch {
    // 拦截器已处理
  } finally {
    loading.value = false
  }
}

const viewState = computed<'loading' | 'empty' | 'idle'>(() => {
  if (loading.value && books.value.length === 0) return 'loading'
  if (!loading.value && books.value.length === 0) return 'empty'
  return 'idle'
})

// ---- 列表行展示：从统计反查净资产与环比 ----
const bookStats = (book: BookInfo) => statsMap.value.get(book.id || '')

const bookValue = (book: BookInfo) => {
  const s = bookStats(book)
  return s ? '¥' + formatMoney(s.netAsset || 0) : ''
}

const bookMomText = (book: BookInfo) => {
  const s = bookStats(book)
  return s?.monthOnMonth !== undefined ? getChangeText(s.monthOnMonth) : ''
}

const bookMomColor = (book: BookInfo) => {
  const v = bookStats(book)?.monthOnMonth
  if (v === undefined) return undefined
  if (v > 0) return 'var(--semantic-up)'
  if (v < 0) return 'var(--semantic-down)'
  return 'var(--text-tertiary)'
}

// ---- 下拉刷新 ----
const pullDistance = ref(0)
const pulling = ref(false)
let touchStartY = 0
const PULL_THRESHOLD = 70

const onTouchStart = (e: TouchEvent) => {
  const scrollTop = document.documentElement.scrollTop || document.body.scrollTop
  if (scrollTop > 0 || loading.value) return
  touchStartY = e.touches[0]?.clientY || 0
  pulling.value = true
}

const onTouchMove = (e: TouchEvent) => {
  if (!pulling.value || refreshing.value) return
  const currentY = e.touches[0]?.clientY || 0
  const delta = currentY - touchStartY

  if (delta <= 0) {
    pullDistance.value = 0
    return
  }

  pullDistance.value = Math.min(delta * 0.5, 100)
}

const onTouchEnd = async () => {
  if (!pulling.value) return

  if (pullDistance.value >= PULL_THRESHOLD && !refreshing.value) {
    refreshing.value = true
    await Promise.all([fetchBooks(true), fetchStats()])
    refreshing.value = false
  }

  pullDistance.value = 0
  pulling.value = false
}

// ---- 滚动触底检测 ----
const onScroll = () => {
  if (!hasMore.value || loading.value) return
  const scrollTop = document.documentElement.scrollTop || document.body.scrollTop
  const scrollHeight = document.documentElement.scrollHeight
  const clientHeight = document.documentElement.clientHeight
  // 距离底部 100px 时触发加载
  if (scrollTop + clientHeight >= scrollHeight - 100) {
    fetchBooks()
  }
}

onMounted(() => {
  fetchBooks()
  fetchStats()
  window.addEventListener('scroll', onScroll, { passive: true })
})

onUnmounted(() => {
  window.removeEventListener('scroll', onScroll)
})

// ---- 创建账簿弹窗 ----
const showCreateDialog = ref(false)
const newBookName = ref('')
const creating = ref(false)

const openCreateDialog = () => {
  newBookName.value = ''
  showCreateDialog.value = true
}

const handleCreate = async () => {
  const name = newBookName.value.trim()
  if (!name) {
    Snackbar.warning('请输入账簿名称')
    return
  }
  creating.value = true
  try {
    await createBook({ bookName: name })
    Snackbar.success('创建成功')
    showCreateDialog.value = false
    fetchBooks(true)
  } catch {
    // 拦截器已处理
  } finally {
    creating.value = false
  }
}

// ---- 卡片点击跳转 ----
const onBookClick = (book: BookInfo) => {
  router.push({
    name: 'BookRecord',
    params: { bookId: book.id },
    query: { name: book.bookName }
  })
}

// ---- 操作菜单 ----
const activeBook = ref<BookInfo | null>(null)
const showActionMenu = ref(false)

const openActionMenu = (book: BookInfo) => {
  activeBook.value = book
  showActionMenu.value = true
}

const onActionSelect = (action: any) => {
  if (action.key === 'rename') {
    openRenameDialog()
  } else if (action.key === 'editTemplate') {
    showActionMenu.value = false
    router.push({
      path: `/book/${activeBook.value?.id}/template`,
      query: { name: activeBook.value?.bookName }
    })
  } else if (action.key === 'delete') {
    openDeleteConfirm()
  }
}

// ---- 重命名弹窗 ----
const showRenameDialog = ref(false)
const renameValue = ref('')
const renaming = ref(false)

const openRenameDialog = () => {
  showActionMenu.value = false
  renameValue.value = activeBook.value?.bookName || ''
  showRenameDialog.value = true
}

const handleRename = async () => {
  const name = renameValue.value.trim()
  if (!name) {
    Snackbar.warning('请输入账簿名称')
    return
  }
  if (!activeBook.value?.id) return
  renaming.value = true
  try {
    await updateBook({ id: activeBook.value.id, bookName: name })
    Snackbar.success('修改成功')
    showRenameDialog.value = false
    fetchBooks(true)
  } catch {
    // 拦截器已处理
  } finally {
    renaming.value = false
  }
}

// ---- 删除确认 ----
const showDeleteConfirm = ref(false)
const deleting = ref(false)

const openDeleteConfirm = () => {
  showActionMenu.value = false
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (!activeBook.value?.id) return
  deleting.value = true
  try {
    await deleteBook({ id: activeBook.value.id })
    Snackbar.success('删除成功')
    showDeleteConfirm.value = false
    fetchBooks(true)
  } catch {
    // 拦截器已处理
  } finally {
    deleting.value = false
  }
}
</script>

<template>
  <div
    class="book-page"
    @touchstart.passive="onTouchStart"
    @touchmove="onTouchMove"
    @touchend="onTouchEnd"
  >
    <!-- 顶部标题 -->
    <PageHeader title="账簿" large>
      <template #right>
        <button class="header-add-btn" type="button" aria-label="创建账簿" @click="openCreateDialog">
          <AppIcon name="plus" :size="18" :stroke-width="2.4" />
        </button>
      </template>
    </PageHeader>

    <div class="page-body">
      <!-- 下拉刷新提示 -->
      <div class="pull-refresh-indicator" :style="{ height: `${pullDistance}px` }">
        <span class="pull-refresh-text">
          {{ refreshing ? '刷新中...' : pullDistance >= 70 ? '松开立即刷新' : '下拉刷新' }}
        </span>
      </div>

      <!-- 加载 / 空状态 -->
      <StateView
        v-if="viewState !== 'idle'"
        :state="viewState"
        empty-text="还没有账簿"
        empty-sub="点击右上角 + 创建第一个账簿"
        empty-icon="notebook-text"
      />

      <!-- 账簿列表 -->
      <ListGroup v-else>
        <ListCell
          v-for="book in books"
          :key="book.id"
          icon="notebook-text"
          :label="book.bookName || '未命名账簿'"
          :value="bookValue(book)"
          :sub-value="bookMomText(book)"
          :sub-value-color="bookMomColor(book)"
          @click="onBookClick(book)"
        >
          <template #trailing>
            <button
              class="more-btn"
              type="button"
              aria-label="更多操作"
              @click.stop="openActionMenu(book)"
            >
              <AppIcon name="ellipsis-vertical" :size="16" />
            </button>
          </template>
        </ListCell>
      </ListGroup>

      <!-- 底部加载状态 -->
      <div v-if="books.length > 0" class="load-more">
        <span v-if="loading" class="load-more-text">加载中...</span>
        <span v-else-if="!hasMore" class="load-more-text">没有更多了</span>
      </div>
    </div>

    <!-- 创建账簿弹窗 -->
    <var-dialog
      v-model:show="showCreateDialog"
      title="创建账簿"
      confirm-button-text="创建"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="var(--brand)"
      :confirm-button-loading="creating"
      @confirm="handleCreate"
    >
      <var-input
        v-model="newBookName"
        placeholder="请输入账簿名称"
        :maxlength="20"
        clearable
        autofocus
        @keyup.enter="handleCreate"
      />
    </var-dialog>

    <!-- 操作菜单（Apple 风：纯文本行，危险项红色） -->
    <var-action-sheet
      v-model:show="showActionMenu"
      :actions="[
        { name: '编辑账目模板', key: 'editTemplate' } as any,
        { name: '修改账簿名称', key: 'rename' } as any,
        { name: '删除当前账簿', key: 'delete', color: 'var(--semantic-danger)' } as any
      ]"
      @select="onActionSelect"
    />

    <!-- 重命名弹窗 -->
    <var-dialog
      v-model:show="showRenameDialog"
      title="修改账簿名称"
      confirm-button-text="保存"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="var(--brand)"
      :confirm-button-loading="renaming"
      @confirm="handleRename"
    >
      <var-input
        v-model="renameValue"
        placeholder="请输入新的账簿名称"
        :maxlength="20"
        clearable
        autofocus
        @keyup.enter="handleRename"
      />
    </var-dialog>

    <!-- 删除确认弹窗 -->
    <var-dialog
      v-model:show="showDeleteConfirm"
      title="删除账簿"
      confirm-button-text="删除"
      cancel-button-text="取消"
      confirm-button-text-color="#fff"
      confirm-button-color="var(--semantic-danger)"
      :confirm-button-loading="deleting"
      @confirm="handleDelete"
    >
      确定要删除「{{ activeBook?.bookName }}」吗？删除后不可恢复。
    </var-dialog>
  </div>
</template>

<style scoped>
.book-page {
  min-height: 100vh;
  background: var(--bg-canvas);
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

.header-add-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 50%;
  background: var(--brand);
  color: #fff;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(255, 101, 0, 0.3);
  transition: transform var(--duration-fast) var(--ease-out), background-color var(--duration-fast);
  -webkit-tap-highlight-color: transparent;
}
.header-add-btn:active {
  transform: scale(0.92);
}

/* 页面主体 */
.page-body {
  padding: var(--space-3) var(--page-padding) 0;
}

/* 下拉刷新 */
.pull-refresh-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  transition: height var(--duration-fast) ease;
}
.pull-refresh-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

/* 更多操作按钮 */
.more-btn {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
  border-radius: 50%;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: background-color var(--duration-fast);
}
.more-btn:active {
  background-color: var(--bg-surface-2);
  color: var(--text-primary);
}

/* 底部加载状态 */
.load-more {
  text-align: center;
  padding: var(--space-5) 0 var(--space-2);
}
.load-more-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>
