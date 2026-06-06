<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listBooks } from '@/api/modules/book'
import type { BookInfo } from '@/api/modules/book'
import { exportBookData, listExportTasks } from '@/api/modules/exportTask'
import type { ExportTaskInfo } from '@/api/modules/exportTask'

const router = useRouter()

// ==================== 账簿选择 ====================
const books = ref<BookInfo[]>([])
const loadingBooks = ref(false)

const fetchBooks = async () => {
  loadingBooks.value = true
  try {
    const res = await listBooks({ pageNum: 1, pageSize: 100 })
    books.value = res.records || []
  } catch {
    // 拦截器已处理
  } finally {
    loadingBooks.value = false
  }
}

// ==================== 导出表单 ====================
const selectedBookId = ref('')
const startMonth = ref('')
const endMonth = ref('')
const exporting = ref(false)

const handleExport = async () => {
  const params: { bookId?: string; startYearMonth?: string; endYearMonth?: string } = {}

  if (selectedBookId.value) {
    params.bookId = selectedBookId.value
  }
  if (startMonth.value) {
    params.startYearMonth = startMonth.value.replace('-', '')
  }
  if (endMonth.value) {
    params.endYearMonth = endMonth.value.replace('-', '')
  }

  exporting.value = true
  try {
    await exportBookData(params)
    Snackbar.success('导出任务已创建')
    // 刷新任务列表
    await fetchTasks(true)
  } catch {
    // 拦截器已处理
  } finally {
    exporting.value = false
  }
}

// ==================== 任务列表 ====================
const tasks = ref<ExportTaskInfo[]>([])
const loadingTasks = ref(false)
const pageNum = ref(1)
const hasMore = ref(true)
const PAGE_SIZE = 10

const fetchTasks = async (reset = false) => {
  if (loadingTasks.value) return
  if (!reset && !hasMore.value) return

  if (reset) {
    pageNum.value = 1
    hasMore.value = true
  }

  loadingTasks.value = true
  try {
    const res = await listExportTasks({ pageNum: pageNum.value, pageSize: PAGE_SIZE })
    const records = res.records || []
    if (reset) {
      tasks.value = records
    } else {
      tasks.value.push(...records)
    }
    if (records.length < PAGE_SIZE || pageNum.value >= res.totalPage) {
      hasMore.value = false
    } else {
      pageNum.value++
    }
  } catch {
    // 拦截器已处理
  } finally {
    loadingTasks.value = false
  }
}

// ==================== 状态轮询 ====================
let pollTimer: ReturnType<typeof setInterval> | null = null

const startPolling = () => {
  stopPolling()
  pollTimer = setInterval(() => {
    // 有进行中的任务时才刷新
    const hasRunning = tasks.value.some(
      (t) => t.status === 'WAIT' || t.status === 'RUN'
    )
    if (hasRunning) {
      fetchTasks(true)
    }
  }, 3000)
}

const stopPolling = () => {
  if (pollTimer) {
    clearInterval(pollTimer)
    pollTimer = null
  }
}

onMounted(() => {
  fetchBooks()
  fetchTasks(true).then(() => startPolling())
})

onUnmounted(() => {
  stopPolling()
})

// ==================== 工具函数 ====================
const statusMap: Record<string, { label: string; color: string }> = {
  WAIT: { label: '等待中', color: '#FF9500' },
  RUN: { label: '执行中', color: '#007AFF' },
  SUCCESS: { label: '成功', color: '#34C759' },
  FAIL: { label: '失败', color: '#FF3B30' },
}

const formatStatus = (status?: string) => {
  return statusMap[status || ''] || { label: status || '-', color: '#8e8e93' }
}

const formatTime = (time?: string) => {
  if (!time) return '-'
  const d = new Date(time)
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

const onScroll = () => {
  if (!hasMore.value || loadingTasks.value) return
  const scrollTop = document.documentElement.scrollTop || document.body.scrollTop
  const scrollHeight = document.documentElement.scrollHeight
  const clientHeight = document.documentElement.clientHeight
  if (scrollTop + clientHeight >= scrollHeight - 100) {
    fetchTasks()
  }
}
</script>

<template>
  <div class="export-page" @scroll.passive="onScroll">
    <!-- 顶部标题 -->
    <div class="page-header">
      <button class="header-back" @click="router.back()">
        <var-icon name="chevron-left" :size="18" color="#333" />
      </button>
      <h2>数据导出</h2>
      <div class="header-placeholder"></div>
    </div>

    <div class="page-body">
      <!-- 导出配置卡片 -->
      <div class="config-card">
        <div class="card-title">导出配置</div>

        <div class="form-item">
          <label class="form-label">选择账簿</label>
          <select v-model="selectedBookId" class="form-select">
            <option value="">全部账簿</option>
            <option v-for="book in books" :key="book.id" :value="book.id">
              {{ book.bookName }}
            </option>
          </select>
        </div>

        <div class="form-row">
          <div class="form-item flex-1">
            <label class="form-label">起始年月</label>
            <input v-model="startMonth" type="month" class="form-input" />
          </div>
          <div class="form-item flex-1">
            <label class="form-label">结束年月</label>
            <input v-model="endMonth" type="month" class="form-input" />
          </div>
        </div>

        <button
          class="export-btn"
          :class="{ 'export-btn--loading': exporting }"
          :disabled="exporting"
          @click="handleExport"
        >
          {{ exporting ? '创建中...' : '创建导出任务' }}
        </button>
      </div>

      <!-- 任务列表 -->
      <div class="tasks-card">
        <div class="card-title">
          导出记录
          <span v-if="loadingTasks && tasks.length === 0" class="tasks-loading">
            <div class="mini-spinner"></div>
          </span>
        </div>

        <div v-if="!loadingTasks && tasks.length === 0" class="tasks-empty">
          <span class="tasks-empty-icon">📭</span>
          <span class="tasks-empty-text">暂无导出记录</span>
        </div>

        <div v-else class="task-list">
          <div
            v-for="task in tasks"
            :key="task.id"
            class="task-item"
          >
            <div class="task-main">
              <div class="task-info">
                <span class="task-book">账簿：{{ task.bookId ? task.bookName : '全部账簿' }}</span>
                <span
                  class="task-status"
                  :style="{ color: formatStatus(task.status).color }"
                >
                  {{ formatStatus(task.status).label }}
                </span>
              </div>
              <div class="task-meta">
                <span v-if="task.startYearMonth && task.endYearMonth">
                  {{ task.startYearMonth }} ~ {{ task.endYearMonth }}
                </span>
                <span v-else>全部时间</span>
                <span class="task-time">{{ formatTime(task.createTime) }}</span>
              </div>
            </div>

            <div class="task-actions">
              <span
                v-if="task.status === 'SUCCESS'"
                class="task-mail-tip"
              >
                已发至邮箱，请查收附件
              </span>
              <span
                v-else-if="task.status === 'FAIL'"
                class="task-fail-reason"
                :title="task.failReason || ''"
              >
                {{ task.failReason || '导出失败' }}
              </span>
              <span v-else class="task-action-placeholder"></span>
            </div>
          </div>

          <div v-if="tasks.length > 0" class="load-more">
            <span v-if="loadingTasks" class="load-more-text">加载中...</span>
            <span v-else-if="!hasMore" class="load-more-text">没有更多了</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(24px + env(safe-area-inset-bottom, 0px));
}

/* 顶部标题 */
.page-header {
  background: #fff;
  padding: calc(16px + env(safe-area-inset-top, 0px)) 16px 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0;
  line-height: 1;
}
.header-back {
  width: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: 50%;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.header-back:active {
  background: #f5f5f5;
}
.header-placeholder {
  width: 32px;
}

/* 页面主体 */
.page-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 卡片通用 */
.config-card,
.tasks-card {
  background: #fff;
  border-radius: 16px;
  padding: 20px 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.card-title {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

/* 表单 */
.form-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 14px;
}
.form-item:last-child {
  margin-bottom: 0;
}
.form-row {
  display: flex;
  gap: 12px;
}
.flex-1 {
  flex: 1;
}
.form-label {
  font-size: 13px;
  color: #666;
  font-weight: 500;
}
.form-select,
.form-input {
  height: 44px;
  padding: 0 12px;
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  font-size: 15px;
  color: #333;
  background: #fafafa;
  outline: none;
  -webkit-appearance: none;
  appearance: none;
}
.form-select {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 32px;
}
.form-select:focus,
.form-input:focus {
  border-color: #FF6500;
  background: #fff;
}

/* 导出按钮 */
.export-btn {
  width: 100%;
  height: 46px;
  margin-top: 6px;
  border: none;
  border-radius: 12px;
  background: #FF6500;
  color: #fff;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  -webkit-tap-highlight-color: transparent;
}
.export-btn:active:not(:disabled) {
  background: #e05800;
  transform: scale(0.98);
}
.export-btn--loading {
  opacity: 0.7;
}

/* 任务列表加载 */
.tasks-loading {
  display: flex;
  align-items: center;
}
.mini-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #e0e0e0;
  border-top-color: #FF6500;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 空状态 */
.tasks-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 0;
  gap: 8px;
}
.tasks-empty-icon {
  font-size: 40px;
}
.tasks-empty-text {
  font-size: 14px;
  color: #aeaeb2;
}

/* 任务项 */
.task-list {
  display: flex;
  flex-direction: column;
}
.task-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 0;
  border-bottom: 1px solid #f5f5f5;
}
.task-item:last-child {
  border-bottom: none;
}
.task-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.task-info {
  display: flex;
  align-items: center;
  gap: 8px;
}
.task-book {
  font-size: 15px;
  font-weight: 600;
  color: #333;
}
.task-status {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.04);
}
.task-meta {
  font-size: 12px;
  color: #999;
  display: flex;
  align-items: center;
  gap: 8px;
}
.task-time {
  color: #bbb;
}
.task-actions {
  flex-shrink: 0;
  margin-left: 12px;
  display: flex;
  align-items: center;
}
.task-mail-tip {
  font-size: 12px;
  color: #34C759;
  font-weight: 500;
  white-space: nowrap;
}
.task-fail-reason {
  font-size: 12px;
  color: #FF3B30;
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.task-action-placeholder {
  width: 56px;
}

/* 底部加载 */
.load-more {
  text-align: center;
  padding: 16px 0 4px;
}
.load-more-text {
  font-size: 12px;
  color: #aeaeb2;
}
</style>
