<script setup lang="ts">
import { computed } from 'vue'
import agreementText from '@/assets/agreement.md?raw'
import { parseMarkdown } from '@/utils/markdown'
import AppIcon from './AppIcon.vue'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ 'update:show': [value: boolean] }>()

const showAgreement = computed({
  get: () => props.show,
  set: (val) => emit('update:show', val)
})
</script>

<template>
  <var-popup v-model:show="showAgreement" position="bottom" :overlay-style="{ background: 'rgba(0,0,0,0.5)' }" round>
    <div class="agreement-popup">
      <div class="agreement-header">
        <span class="agreement-title">用户协议及隐私政策</span>
        <button class="agreement-close" @click="showAgreement = false">
          <AppIcon name="x" :size="20" />
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
  background: var(--bg-surface);
  border-radius: var(--radius-xl) var(--radius-xl) 0 0;
}

.agreement-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-5) var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--separator);
  flex-shrink: 0;
}

.agreement-title {
  font-size: var(--text-title-3);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
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
  background: var(--bg-surface-2);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast);
  padding: 0;
}

.agreement-close:active {
  opacity: 0.7;
  color: var(--brand);
}

.agreement-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5) var(--space-8);
  font-size: var(--text-body);
  line-height: 1.8;
  color: var(--text-secondary);
}

.agreement-body :deep(h3) {
  font-size: var(--text-title-3);
  font-weight: var(--weight-bold);
  color: var(--brand);
  margin: var(--space-5) 0 var(--space-3);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--separator);
}

.agreement-body :deep(h3:first-child) {
  margin-top: 0;
}

.agreement-body :deep(h4) {
  font-size: var(--text-body);
  font-weight: var(--weight-semibold);
  color: var(--brand-accent);
  margin: var(--space-4) 0 var(--space-2);
}

.agreement-body :deep(p) {
  margin: 0 0 var(--space-2);
  text-align: justify;
}

.agreement-body :deep(ul) {
  margin: var(--space-1) 0 var(--space-2);
  padding-left: 18px;
  list-style: none;
}

.agreement-body :deep(li) {
  position: relative;
  padding-left: 6px;
  margin-bottom: 4px;
}

.agreement-body :deep(strong) {
  color: var(--text-primary);
  font-weight: var(--weight-semibold);
}
</style>
