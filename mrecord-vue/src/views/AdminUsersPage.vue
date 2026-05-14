<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Snackbar } from '@varlet/ui'
import { listUsers, enableOrDisableUser, deleteUser, adminResetPassword, type SysUser, type ListUsersParams } from '@/api'
import type { PageResult } from '@/api/types'

const router = useRouter()

// ==================== 搜索与分页 ====================
const searchParams = reactive<ListUsersParams>({
  nickname: '',
  email: '',
  status: undefined as number | undefined,
  isAdmin: undefined as number | undefined,
  pageNum: 1,
  pageSize: 10,
})
const loading = ref(false)
const pageResult = ref<PageResult<SysUser>>({
  records: [],
  pageNumber: 1,
  pageSize: 10,
  totalPage: 0,
  totalRow: 0,
})

async function loadUsers() {
  loading.value = true
  try {
    pageResult.value = await listUsers(searchParams)
  } catch {
    // 拦截器处理
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  searchParams.pageNum = 1
  loadUsers()
}

function resetSearch() {
  searchParams.nickname = ''
  searchParams.email = ''
  searchParams.status = undefined
  searchParams.isAdmin = undefined
  searchParams.pageNum = 1
  loadUsers()
}

function changePage(page: number) {
  searchParams.pageNum = page
  loadUsers()
}

// ==================== 用户操作 ====================
const showResetDialog = ref(false)
const resetTarget = ref<SysUser | null>(null)
const resetPassword = ref('')
const resetting = ref(false)

function openResetDialog(user: SysUser) {
  resetTarget.value = user
  resetPassword.value = ''
  showResetDialog.value = true
}

async function confirmResetPassword() {
  if (!resetTarget.value || !resetPassword.value) {
    Snackbar.warning('请输入新密码')
    return
  }
  if (resetPassword.value.length < 6) {
    Snackbar.warning('密码至少6位')
    return
  }
  resetting.value = true
  try {
    await adminResetPassword({ email: resetTarget.value.email, password: resetPassword.value })
    Snackbar.success('密码重置成功')
    showResetDialog.value = false
  } catch {
    // 拦截器处理
  } finally {
    resetting.value = false
  }
}

const showToggleDialog = ref(false)
const toggleTarget = ref<SysUser | null>(null)
const toggling = ref(false)

function openToggleDialog(user: SysUser) {
  toggleTarget.value = user
  showToggleDialog.value = true
}

async function confirmToggle() {
  if (!toggleTarget.value) return
  toggling.value = true
  try {
    await enableOrDisableUser([toggleTarget.value.id!])
    Snackbar.success(toggleTarget.value.status === 0 ? '已停用' : '已启用')
    showToggleDialog.value = false
    loadUsers()
  } catch {
    // 拦截器处理
  } finally {
    toggling.value = false
  }
}

const showDeleteDialog = ref(false)
const deleteTarget = ref<SysUser | null>(null)
const deleting = ref(false)

function openDeleteDialog(user: SysUser) {
  deleteTarget.value = user
  showDeleteDialog.value = true
}

async function confirmDelete() {
  if (!deleteTarget.value) return
  deleting.value = true
  try {
    await deleteUser([deleteTarget.value.id!])
    Snackbar.success('已删除')
    showDeleteDialog.value = false
    loadUsers()
  } catch {
    // 拦截器处理
  } finally {
    deleting.value = false
  }
}

// ==================== 状态显示辅助 ====================
const statusMap: Record<number, { label: string; color: string }> = {
  0: { label: '正常', color: '#52c41a' },
  1: { label: '停用', color: '#ff4d4f' },
  2: { label: '注销中', color: '#faad14' },
  3: { label: '已注销', color: '#999' },
}

function getStatusInfo(status?: number) {
  return statusMap[status ?? 0] || statusMap[0]
}

// ==================== 操作菜单 ====================
const showActionSheet = ref(false)
const actionItems = ref<{ name: string; icon: string }[]>([])
const actionCallbacks = ref<Record<string, () => void>>({})

function openActions(user: SysUser) {
  const toggleLabel = user.status === 0 ? '停用用户' : '启用用户'
  actionItems.value = [
    { name: '重置密码', icon: '🔑' },
    { name: toggleLabel, icon: user.status === 0 ? '🚫' : '✅' },
    { name: '删除用户', icon: '🗑️' },
  ]
  actionCallbacks.value = {
    '重置密码': () => openResetDialog(user),
    [toggleLabel]: () => openToggleDialog(user),
    '删除用户': () => openDeleteDialog(user),
  }
  showActionSheet.value = true
}

function handleAction(action: { name: string }) {
  showActionSheet.value = false
  const name = action.name
  setTimeout(() => {
    actionCallbacks.value[name]?.()
  }, 200)
}

onMounted(() => {
  loadUsers()
})
</script>

<template>
  <div class="admin-users-page">
    <!-- 顶部导航 -->
    <div class="page-header">
      <button class="back-btn" @click="router.push('/admin')">
        <svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>
      <h2>用户管理</h2>
      <span class="header-spacer"></span>
    </div>

    <!-- 搜索栏 -->
    <div class="search-bar">
      <div class="search-inputs">
        <div class="input-wrapper">
          <input v-model="searchParams.nickname" placeholder="昵称" class="search-input" />
        </div>
        <div class="input-wrapper">
          <input v-model="searchParams.email" placeholder="邮箱" class="search-input" />
        </div>
        <div class="input-wrapper select-wrapper">
          <select v-model="searchParams.status" class="search-select">
            <option :value="undefined">全部状态</option>
            <option :value="0">正常</option>
            <option :value="1">停用</option>
            <option :value="2">注销中</option>
            <option :value="3">已注销</option>
          </select>
        </div>
      </div>
      <div class="search-actions">
        <button class="search-btn" @click="handleSearch">搜索</button>
        <button class="reset-btn" @click="resetSearch">重置</button>
      </div>
    </div>

    <!-- 统计 -->
    <div class="stats-bar">
      共 <strong>{{ pageResult.totalRow }}</strong> 位用户
    </div>

    <!-- 用户列表 -->
    <div class="user-list">
      <div v-if="loading" class="loading-state">
        <var-loading type="circle" />
      </div>
      <div v-else-if="pageResult.records.length === 0" class="empty-state">
        暂无用户数据
      </div>
      <div v-else>
        <div v-for="user in pageResult.records" :key="user.id" class="user-card">
          <div class="user-main">
            <div class="user-avatar">
              <span class="avatar-emoji">😊</span>
            </div>
            <div class="user-info">
              <div class="user-name-row">
                <span class="user-nickname">{{ user.nickname || '未设置昵称' }}</span>
                <span v-if="user.admin === 1" class="admin-tag">管理员</span>
                <span class="status-tag" :style="{ color: getStatusInfo(user.status).color, borderColor: getStatusInfo(user.status).color }">
                  {{ getStatusInfo(user.status).label }}
                </span>
              </div>
              <div class="user-email">{{ user.email }}</div>
              <div class="user-meta">
                <span>创建于 {{ user.createTime?.slice(0, 10) || '-' }}</span>
              </div>
            </div>
          </div>
          <button class="action-btn" @click="openActions(user)">
            <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
              <circle cx="12" cy="5" r="2" />
              <circle cx="12" cy="12" r="2" />
              <circle cx="12" cy="19" r="2" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- 分页 -->
    <div v-if="pageResult.totalPage > 1" class="pagination">
      <button class="page-btn" :disabled="searchParams.pageNum <= 1" @click="changePage(searchParams.pageNum - 1)">上一页</button>
      <span class="page-info">{{ searchParams.pageNum }} / {{ pageResult.totalPage }}</span>
      <button class="page-btn" :disabled="searchParams.pageNum >= pageResult.totalPage" @click="changePage(searchParams.pageNum + 1)">下一页</button>
    </div>

    <!-- 操作菜单 ActionSheet -->
    <var-action-sheet
      :actions="actionItems"
      v-model:show="showActionSheet"
      @select="handleAction"
    />

    <!-- 重置密码弹窗 -->
    <var-popup v-model:show="showResetDialog" position="center" :overlay-style="{ background: 'rgba(0,0,0,0.5)' }" round>
      <div class="custom-dialog">
        <div class="custom-dialog-title">重置密码</div>
        <div class="custom-dialog-body">
          <p class="dialog-hint">为用户 <strong>{{ resetTarget?.nickname || resetTarget?.email }}</strong> 设置新密码：</p>
          <div class="dialog-input-wrapper">
            <input v-model="resetPassword" type="password" placeholder="请输入新密码（至少6位）" class="dialog-input" />
          </div>
        </div>
        <div class="custom-dialog-footer">
          <button class="dialog-btn dialog-btn--cancel" @click="showResetDialog = false">取消</button>
          <button class="dialog-btn dialog-btn--confirm dialog-btn--orange" :disabled="resetting" @click="confirmResetPassword">
            <span v-if="!resetting">确认重置</span>
            <span v-else class="btn-loading"><svg class="spinner" viewBox="0 0 24 24" width="18" height="18"><circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4"/></svg></span>
          </button>
        </div>
      </div>
    </var-popup>

    <!-- 启用/停用弹窗 -->
    <var-popup v-model:show="showToggleDialog" position="center" :overlay-style="{ background: 'rgba(0,0,0,0.5)' }" round>
      <div class="custom-dialog">
        <div class="custom-dialog-title">{{ toggleTarget?.status === 0 ? '停用用户' : '启用用户' }}</div>
        <div class="custom-dialog-body">
          <p class="dialog-hint">确定要{{ toggleTarget?.status === 0 ? '停用' : '启用' }}用户 <strong>{{ toggleTarget?.nickname || toggleTarget?.email }}</strong> 吗？</p>
        </div>
        <div class="custom-dialog-footer">
          <button class="dialog-btn dialog-btn--cancel" @click="showToggleDialog = false">取消</button>
          <button
            class="dialog-btn dialog-btn--confirm"
            :class="toggleTarget?.status === 0 ? 'dialog-btn--red' : 'dialog-btn--green'"
            :disabled="toggling"
            @click="confirmToggle"
          >
            <span v-if="!toggling">{{ toggleTarget?.status === 0 ? '确认停用' : '确认启用' }}</span>
            <span v-else class="btn-loading"><svg class="spinner" viewBox="0 0 24 24" width="18" height="18"><circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4"/></svg></span>
          </button>
        </div>
      </div>
    </var-popup>

    <!-- 删除弹窗 -->
    <var-popup v-model:show="showDeleteDialog" position="center" :overlay-style="{ background: 'rgba(0,0,0,0.5)' }" round>
      <div class="custom-dialog">
        <div class="custom-dialog-title">删除用户</div>
        <div class="custom-dialog-body">
          <p class="dialog-hint">确定要删除用户 <strong>{{ deleteTarget?.nickname || deleteTarget?.email }}</strong> 吗？此操作不可恢复！</p>
        </div>
        <div class="custom-dialog-footer">
          <button class="dialog-btn dialog-btn--cancel" @click="showDeleteDialog = false">取消</button>
          <button class="dialog-btn dialog-btn--confirm dialog-btn--red" :disabled="deleting" @click="confirmDelete">
            <span v-if="!deleting">确认删除</span>
            <span v-else class="btn-loading"><svg class="spinner" viewBox="0 0 24 24" width="18" height="18"><circle cx="12" cy="12" r="10" stroke="white" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4"/></svg></span>
          </button>
        </div>
      </div>
    </var-popup>
  </div>
</template>

<style scoped>
.admin-users-page {
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

/* 搜索栏 */
.search-bar {
  background: #fff;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
}
.search-inputs {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}
.input-wrapper {
  flex: 1;
}
.search-input {
  width: 100%;
  height: 38px;
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  padding: 0 12px;
  font-size: 13px;
  color: #333;
  background: #fafafa;
  outline: none;
  transition: border-color 0.2s;
}
.search-input:focus {
  border-color: #FF8C42;
  background: #fff;
}
.select-wrapper {
  flex: 0 0 auto;
}
.search-select {
  height: 38px;
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  padding: 0 10px;
  font-size: 13px;
  color: #333;
  background: #fafafa;
  outline: none;
  cursor: pointer;
  appearance: none;
  -webkit-appearance: none;
}
.search-actions {
  display: flex;
  gap: 8px;
}
.search-btn {
  flex: 1;
  height: 36px;
  border: none;
  border-radius: 10px;
  background: linear-gradient(135deg, #FF8C42, #FF6500);
  color: #fff;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}
.search-btn:active {
  opacity: 0.85;
}
.reset-btn {
  width: 80px;
  height: 36px;
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  background: #fff;
  color: #666;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}
.reset-btn:active {
  background: #f5f5f5;
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

/* 用户列表 */
.user-list {
  padding: 0 16px;
}
.loading-state,
.empty-state {
  text-align: center;
  padding: 40px 0;
  color: #bbb;
  font-size: 14px;
}
.user-card {
  background: #fff;
  border-radius: 14px;
  padding: 14px 16px;
  margin-bottom: 10px;
  display: flex;
  align-items: center;
  gap: 12px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}
.user-main {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}
.user-avatar {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: linear-gradient(135deg, #FFF3E0, #FFE0B2);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.avatar-emoji {
  font-size: 22px;
  line-height: 1;
}
.user-info {
  flex: 1;
  min-width: 0;
}
.user-name-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 2px;
}
.user-nickname {
  font-size: 15px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.admin-tag {
  font-size: 10px;
  font-weight: 600;
  color: #fff;
  background: linear-gradient(135deg, #FF8C42, #FF6500);
  padding: 1px 6px;
  border-radius: 8px;
  line-height: 1.5;
}
.status-tag {
  font-size: 11px;
  font-weight: 500;
  padding: 1px 6px;
  border-radius: 6px;
  border: 1px solid;
  line-height: 1.5;
}
.user-email {
  font-size: 12px;
  color: #999;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.user-meta {
  font-size: 11px;
  color: #bbb;
  margin-top: 2px;
}
.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: #f5f5f5;
  color: #999;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;
  flex-shrink: 0;
}
.action-btn:active {
  background: #eee;
  color: #FF6500;
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

/* 弹窗内容 */
.dialog-content {
  padding: 8px 0;
}
.dialog-hint {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  margin: 0 0 12px;
}
.dialog-hint strong {
  color: #FF6500;
}
.dialog-input-wrapper {
  margin-top: 4px;
}
.dialog-input {
  width: 100%;
  height: 42px;
  border: 1px solid #e8e8e8;
  border-radius: 10px;
  padding: 0 12px;
  font-size: 14px;
  color: #333;
  background: #fafafa;
  outline: none;
  transition: border-color 0.2s;
}
.dialog-input:focus {
  border-color: #FF8C42;
  background: #fff;
}

/* 自定义弹窗 */
.custom-dialog {
  width: 300px;
  background: #fff;
  border-radius: 16px;
  overflow: hidden;
}
.custom-dialog-title {
  font-size: 17px;
  font-weight: 700;
  color: #333;
  text-align: center;
  padding: 22px 20px 8px;
}
.custom-dialog-body {
  padding: 8px 20px 18px;
}
.custom-dialog-footer {
  display: flex;
  border-top: 1px solid #f0f0f0;
}
.dialog-btn {
  flex: 1;
  height: 48px;
  border: none;
  background: none;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  padding: 0;
}
.dialog-btn:active:not(:disabled) {
  opacity: 0.7;
}
.dialog-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.dialog-btn--cancel {
  color: #999;
  border-right: 1px solid #f0f0f0;
}
.dialog-btn--confirm {
  font-weight: 600;
}
.dialog-btn--orange {
  color: #FF6500;
}
.dialog-btn--red {
  color: #ff4d4f;
}
.dialog-btn--green {
  color: #52c41a;
}
.btn-loading {
  display: flex;
  align-items: center;
  justify-content: center;
}
.spinner {
  animation: spin 0.8s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>