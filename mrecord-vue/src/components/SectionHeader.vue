<script setup lang="ts">
/**
 * 分组列表的 section 头（Apple grouped list 风格）。
 *
 * 左侧标题（大写小字次级色），右侧可选操作入口（如「全部 ›」）。
 */
defineProps<{
    title: string
    /** 右侧操作文案，如「全部」 */
    actionText?: string
}>()

defineEmits<{
    action: []
}>()
</script>

<template>
    <div class="section-header">
        <span class="section-title">{{ title }}</span>
        <button
            v-if="actionText"
            type="button"
            class="section-action"
            @click="$emit('action')"
        >
            {{ actionText }} <span class="section-action-arrow">›</span>
        </button>
    </div>
</template>

<style scoped>
.section-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    /* 横向内边距由父级 section 提供（页面统一 --page-padding），此处不再自带，避免双重缩进 */
    margin-bottom: var(--space-2);
}

.section-title {
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
    color: var(--text-secondary);
    letter-spacing: 0.02em;
}

.section-action {
    display: flex;
    align-items: center;
    font-size: var(--text-sm);
    color: var(--brand);
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
    padding: 2px 4px;
    margin: -2px -4px;
    border-radius: var(--radius-sm);
}

.section-action:active {
    opacity: 0.6;
}

.section-action-arrow {
    font-size: 15px;
    line-height: 1;
    margin-left: 1px;
}
</style>
