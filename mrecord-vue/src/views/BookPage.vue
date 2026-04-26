<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { createBook, updateBook, deleteBook, listBooks } from '@/api/modules/book'
import type { BookInfo } from '@/api/modules/book'
import BookCard from '@/components/BookCard.vue'

const router = useRouter()

// ---- 账簿列表（懒加载） ----
const PAGE_SIZE = 10
const books = ref<BookInfo[]>([])
const loading = ref(false)
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

// 滚动触底检测
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
  <div class="book-page">
    <!-- 顶部标题 -->
    <div class="page-header">
      <h2>账簿</h2>
      <button class="header-add-btn" @click="openCreateDialog">+</button>
    </div>

    <div class="page-body">
      <!-- 首次加载 thinking 动画 -->
      <div v-if="loading && books.length === 0" class="thinking-state">
        <div class="thinking-face">🤔</div>
        <div class="thinking-text">
          thinking
          <span class="thinking-dots">
            <span class="dot">.</span><span class="dot">.</span><span class="dot">.</span>
          </span>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!loading && books.length === 0" class="empty-state">
        <svg class="empty-icon" viewBox="0 0 64 64" width="64" height="64">
          <rect x="12" y="8" width="40" height="48" rx="4" fill="none" stroke="#ccc" stroke-width="2"/>
          <line x1="22" y1="20" x2="42" y2="20" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="28" x2="38" y2="28" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
          <line x1="22" y1="36" x2="34" y2="36" stroke="#ddd" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <p class="empty-text">还没有账簿</p>
        <p class="empty-sub">点击右上角 + 创建第一个账簿</p>
      </div>

      <!-- 账簿网格 -->
      <div v-else class="books-grid">
        <BookCard
          v-for="book in books"
          :key="book.id"
          :book="book"
          @click="onBookClick"
          @more="openActionMenu"
        />
      </div>

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
      confirm-button-color="#FF6500"
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

    <!-- 操作菜单 -->
    <var-action-sheet
      v-model:show="showActionMenu"
      :actions="[
        { name: '编辑账目模板', icon: 'notebook', key: 'editTemplate' } as any,
        { name: '修改账簿名称', icon: 'wrench', key: 'rename' } as any,
        { name: '删除当前账簿', icon: 'delete', color: '#e74c3c', key: 'delete' } as any
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
      confirm-button-color="#FF6500"
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
      confirm-button-color="#e74c3c"
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
  background: #f5f5f5;
  padding-bottom: calc(80px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: 16px;
  padding-top: calc(16px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}
.header-add-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 50%;
  background: #FF6500;
  color: #fff;
  font-size: 22px;
  font-weight: 400;
  line-height: 1;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(255, 101, 0, 0.3);
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.header-add-btn:active {
  background: #e05800;
  transform: scale(0.95);
}

/* 页面主体 */
.page-body {
  padding: 16px;
}

/* thinking 加载状态 */
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

/* 账簿网格 */
.books-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

/* 底部加载提示 */
.load-more {
  text-align: center;
  padding: 20px 0 8px;
}
.load-more-text {
  font-size: 13px;
  color: #aeaeb2;
}
</style>
