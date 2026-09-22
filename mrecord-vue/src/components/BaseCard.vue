<script setup lang="ts">
/**
 * 卡片基类：统一表面色、圆角、阴影（Q8 令牌化，治掉各页手写卡片样式）。
 */
withDefaults(
    defineProps<{
        /** 内边距，默认 16px */
        padding?: string
    }>(),
    {
        padding: 'var(--space-4)',
    },
)
</script>

<template>
    <div class="base-card">
        <slot />
    </div>
</template>

<style scoped>
.base-card {
    background: var(--bg-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    padding: v-bind(padding);
}
</style>
