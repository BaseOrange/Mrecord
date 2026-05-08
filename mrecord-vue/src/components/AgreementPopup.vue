<script setup lang="ts">
import { computed } from 'vue'
import agreementText from '@/assets/agreement.md?raw'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ 'update:show': [value: boolean] }>()

const showAgreement = computed({
  get: () => props.show,
  set: (val) => emit('update:show', val)
})

/** 简易 Markdown → HTML 转换 */
function parseMarkdown(text: string): string {
  const lines = text.split('\n')
  let html = ''
  let inList = false

  for (const line of lines) {
    if (!line.trim()) {
      if (inList) { html += '</ul>'; inList = false }
      continue
    }

    if (line.startsWith('# ')) {
      if (inList) { html += '</ul>'; inList = false }
      html += `<h3>${escapeHtml(line.slice(2))}</h3>`
      continue
    }

    if (line.startsWith('## ')) {
      if (inList) { html += '</ul>'; inList = false }
      html += `<h4>${escapeHtml(line.slice(3))}</h4>`
      continue
    }

    let processed = line.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    processed = escapeHtml(processed).replace(/&lt;strong&gt;(.+?)&lt;\/strong&gt;/g, '<strong>$1</strong>')

    const numMatch = processed.match(/^(\d+)\\\.\s(.+)/)
    if (numMatch) {
      if (!inList) { html += '<ul>'; inList = true }
      html += `<li>${numMatch[2]}</li>`
      continue
    }

    if (inList) { html += '</ul>'; inList = false }
    html += `<p>${processed}</p>`
  }

  if (inList) html += '</ul>'
  return html
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
}
</script>

<template>
  <var-popup v-model:show="showAgreement" position="bottom" :overlay-style="{ background: 'rgba(0,0,0,0.5)' }" round>
    <div class="agreement-popup">
      <div class="agreement-header">
        <span class="agreement-title">用户协议及隐私政策</span>
        <button class="agreement-close" @click="showAgreement = false">
          <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
      <div class="agreement-body" v-html="parseMarkdown(agreementText)"></div>
    </div>
  </var-popup>
</template>

<style scoped>
.agreement-popup {
  max-height: 75vh;
  display: flex;
  flex-direction: column;
  background: #fff;
  border-radius: 20px 20px 0 0;
}

.agreement-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 14px;
  border-bottom: 1px solid #f0e8e0;
  flex-shrink: 0;
}

.agreement-title {
  font-size: 17px;
  font-weight: 700;
  color: #333;
  letter-spacing: 1px;
}

.agreement-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: #f5f0ec;
  color: #999;
  cursor: pointer;
  transition: all 0.2s;
  padding: 0;
}

.agreement-close:active {
  background: #e8ddd4;
  color: #FF6500;
}

.agreement-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px 32px;
  font-size: 14px;
  line-height: 1.8;
  color: #444;
}

.agreement-body :deep(h3) {
  font-size: 16px;
  font-weight: 700;
  color: #FF6500;
  margin: 20px 0 10px;
  padding-bottom: 8px;
  border-bottom: 1px solid #f5ede6;
}

.agreement-body :deep(h3:first-child) {
  margin-top: 0;
}

.agreement-body :deep(h4) {
  font-size: 15px;
  font-weight: 600;
  color: #FF8C42;
  margin: 14px 0 8px;
}

.agreement-body :deep(p) {
  margin: 0 0 8px;
  text-align: justify;
}

.agreement-body :deep(ul) {
  margin: 4px 0 8px;
  padding-left: 18px;
  list-style: none;
}

.agreement-body :deep(li) {
  position: relative;
  padding-left: 6px;
  margin-bottom: 4px;
}

.agreement-body :deep(strong) {
  color: #333;
  font-weight: 600;
}
</style>
