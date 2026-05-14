<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { listOperateLogs, type OperateLogInfo } from '@/api'
import type { PageResult, PageParams } from '@/api/types'

const router = useRouter()

const pageParams = reactive<PageParams>({
  pageNum: 1,
  pageSize: 15,
})
const loading = ref(false)
const pageResult = ref<PageResult<OperateLogInfo>>({
  records: [],
  pageNumber: 1,
  pageSize: 15,
  totalPage: 0,
  totalRow: 0,
})

async function loadLogs() {
  loading.value = true
  try {
    pageResult.value = await listOperateLogs(pageParams)
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}

function changePage(page: number) {
  pageParams.pageNum = page
  loadLogs()
}

const operateTypeMap: Record<string, { label: string; color: string }> = {
  LOGIN: { label: '登录', color: '#1890ff' },
  LOGOUT: { label: '登出', color: '#999' },
  UPDATE: { label: '修改', color: '#faad14' },
  EXPORT: { label: '导出', color: '#52c41a' },
  CANCEL: { label: '注销', color: '#ff4d4f' },
  RESET_PWD: { label: '重置密码', color: '#FF6500' },
}

function getTypeInfo(type?: string) {
  return operateTypeMap[type || ''] || { label: type || '未知', color: '#999' }
}

onMounted(() => {
  loadLogs()
})
</script>

<template>
  <div class="admin-logs-page">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="back-btn" @click="router.push('/admin')">
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>操作日志</h2>
      <span class="header-spacer"></span>
    </div>

    <!-- 统计 -->
    <div class="stats-bar">
      共 <strong>{{ pageResult.totalRow }}</strong> 条记录
    </div>

    <!-- 日志列表 -->
    <div class="log-list">
      <div v-if="loading" class="loading-state">
        <var-loading type="circle" />
      </div>
      <div v-else-if="pageResult.records.length === 0" class="empty-state">
        暂无日志记录
      </div>
      <div v-else>
        <div v-for="log in pageResult.records" :key="log.id" class="log-card">
          <div class="log-header">
            <span class="log-type" :style="{ color: getTypeInfo(log.operateType).color, borderColor: getTypeInfo(log.operateType).color }">
              {{ getTypeInfo(log.operateType).label }}
            </span>
            <span class="log-time">{{ log.createTime || '-' }}</span>
          </div>
          <div class="log-content">{{ log.content || '-' }}</div>
          <div class="log-footer">
            <span v-if="log.ip" class="log-ip">IP: {{ log.ip }}</span>
            <span v-if="log.userId" class="log-user">用户ID: {{ log.userId }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div v-if="pageResult.totalPage > 1" class="pagination">
      <button class="page-btn" :disabled="pageParams.pageNum <= 1" @click="changePage(pageParams.pageNum - 1)">上一页</button>
      <span class="page-info">{{ pageParams.pageNum }} / {{ pageResult.totalPage }}</span>
      <button class="page-btn" :disabled="pageParams.pageNum >= pageResult.totalPage" @click="changePage(pageParams.pageNum + 1)">下一页</button>
    </div>
  </div>
</template>

<style scoped>
.admin-logs-page {
  min-height: 100vh;
  background: #f5f5f5;
  padding-bottom: calc(80px + env(safe-area-inset-bottom, 0px));
}

.page-header {
  background: #fff;
  padding: 12px 16px;
  padding-top: calc(12px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #f0f0f0;
}
.page-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}
.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: #f5f5f5;
  color: #666;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;
}
.back-btn:active {
  background: #eee;
  color: #FF6500;
}
.header-spacer {
  width: 36px;
}

/* 统计 */
.stats-bar {
  padding: 10px 16px;
  font-size: 13px;
  color: #999;
}
.stats-bar strong {
  color: #FF6500;
}

/* 日志列表 */
.log-list {
  padding: 0 16px;
}
.loading-state,
.empty-state {
  text-align: center;
  padding: 40px 0;
  color: #bbb;
  font-size: 14px;
}
.log-card {
  background: #fff;
  border-radius: 14px;
  padding: 14px 16px;
  margin-bottom: 10px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.log-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}
.log-type {
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 6px;
  border: 1px solid;
}
.log-time {
  font-size: 11px;
  color: #bbb;
}
.log-content {
  font-size: 14px;
  color: #333;
  line-height: 1.6;
  margin-bottom: 8px;
}
.log-footer {
  display: flex;
  gap: 12px;
  font-size: 11px;
  color: #bbb;
}

/* 分页 */
.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 16px;
}
.page-btn {
  height: 34px;
  padding: 0 14px;
  border: 1px solid #e8e8e8;
  border-radius: 8px;
  background: #fff;
  color: #666;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}
.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.page-btn:active:not(:disabled) {
  border-color: #FF6500;
  color: #FF6500;
}
.page-info {
  font-size: 13px;
  color: #999;
}
</style>