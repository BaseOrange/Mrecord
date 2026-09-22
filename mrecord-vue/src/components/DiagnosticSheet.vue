<script setup lang="ts">
import { ref, watch } from 'vue'
import { Snackbar } from '@varlet/ui'
import { collectDiagnosticText } from '@/utils/diagnostic'

/**
 * 诊断信息弹层
 *
 * 触发方式：「我的」页面顶部标题 1.5 秒内点击 5 次（见 ProfilePage）。
 * 展示环境信息与最近错误日志，支持一键复制并跳转 GitHub 提 issue。
 *
 * 【隐私】展示内容由 utils/diagnostic 采集，不含金额、账号与账簿内容。
 */

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ 'update:show': [value: boolean] }>()

// 诊断信息 markdown 文本
const diagnosticText = ref('')
// 是否正在采集（调用后端接口期间）
const loading = ref(false)
// 是否正在复制
const copying = ref(false)

// GitHub 仓库地址（与根目录 manifest 的 distributor_url 一致）
const GITHUB_REPO = 'https://github.com/BaseOrange/Mrecord'

/** 弹层打开时采集诊断信息 */
watch(
    () => props.show,
    async (visible) => {
        if (!visible) return
        loading.value = true
        try {
            diagnosticText.value = await collectDiagnosticText()
        } finally {
            loading.value = false
        }
    },
)

/** 复制诊断信息到剪贴板 */
const handleCopy = async () => {
    if (!diagnosticText.value) return
    copying.value = true
    try {
        // 优先用现代剪贴板 API；HTTP / 旧 WebView 降级到 execCommand
        if (navigator.clipboard && window.isSecureContext) {
            await navigator.clipboard.writeText(diagnosticText.value)
        } else {
            const textarea = document.createElement('textarea')
            textarea.value = diagnosticText.value
            textarea.style.position = 'fixed'
            textarea.style.opacity = '0'
            document.body.appendChild(textarea)
            textarea.select()
            document.execCommand('copy')
            document.body.removeChild(textarea)
        }
        Snackbar.success('诊断信息已复制')
    } catch {
        Snackbar.error('复制失败，请长按文本手动选择复制')
    } finally {
        copying.value = false
    }
}

/** 跳转 GitHub 提 issue */
const handleGoGithub = () => {
    window.open(`${GITHUB_REPO}/issues/new`, '_blank')
}

/** 关闭弹层 */
const handleClose = () => {
    emit('update:show', false)
}
</script>

<template>
  <var-popup
    :show="props.show"
    position="bottom"
    round
    :overlay-style="{ background: 'rgba(0,0,0,0.5)' }"
    @update:show="handleClose"
  >
    <div class="diagnostic-sheet">
      <div class="sheet-header">
        <span class="sheet-title">诊断信息</span>
        <var-icon name="window-close" :size="20" class="sheet-close" @click="handleClose" />
      </div>

      <div class="sheet-tip">
        遇到问题？复制以下信息并前往
        <span class="tip-link" @click="handleGoGithub">GitHub</span>
        提交 issue。本信息不含任何金额、账号与账簿内容，可放心提供。
      </div>

      <div class="sheet-body">
        <pre v-if="!loading">{{ diagnosticText }}</pre>
        <div v-else class="loading-tip">采集中…</div>
      </div>

      <div class="sheet-actions">
        <button
          class="action-btn copy-btn"
          :disabled="loading || copying || !diagnosticText"
          @click="handleCopy"
        >
          {{ copying ? '复制中…' : '复制诊断信息' }}
        </button>
        <button class="action-btn github-btn" @click="handleGoGithub">
          前往 GitHub 提 issue
        </button>
      </div>
    </div>
  </var-popup>
</template>

<style scoped>
.diagnostic-sheet {
    max-height: 82vh;
    display: flex;
    flex-direction: column;
    background: #fff;
    border-radius: 16px 16px 0 0;
}

.sheet-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 16px 8px;
}

.sheet-title {
    font-size: 17px;
    font-weight: 600;
    color: #1d1d1f;
}

.sheet-close {
    color: #999;
    cursor: pointer;
    padding: 4px;
    border-radius: 50%;
}

.sheet-close:active {
    background: #f0f0f0;
}

.sheet-tip {
    margin: 0 16px 10px;
    font-size: 12px;
    color: #888;
    line-height: 1.6;
}

.tip-link {
    color: #FF6500;
    font-weight: 500;
    text-decoration: underline;
}

.sheet-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    margin: 0 16px;
    background: #f7f7f8;
    border-radius: 10px;
    border: 1px solid #f0f0f0;
}

.sheet-body pre {
    margin: 0;
    padding: 12px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12px;
    line-height: 1.7;
    color: #333;
    white-space: pre-wrap;
    word-break: break-all;
}

.loading-tip {
    padding: 24px;
    text-align: center;
    font-size: 13px;
    color: #999;
}

.sheet-actions {
    display: flex;
    gap: 10px;
    padding: 14px 16px calc(16px + env(safe-area-inset-bottom, 0px));
}

.action-btn {
    flex: 1;
    height: 46px;
    border: none;
    border-radius: 12px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.15s;
}

.action-btn:active:not(:disabled) {
    opacity: 0.85;
}

.copy-btn {
    background: #FF6500;
    color: #fff;
}

.copy-btn:disabled {
    background: #ffb98e;
    cursor: not-allowed;
}

.github-btn {
    background: #fff;
    color: #333;
    border: 1px solid #e0e0e0;
}
</style>
