<script setup lang="ts">
/**
 * 分组列表容器（Apple grouped list）。
 *
 * 圆角卡片 + 自动分割线（最后一项无线）。slot 内放 `<ListCell>`。
 */
</script>

<template>
    <div class="list-group">
        <slot />
    </div>
</template>

<style scoped>
.list-group {
    background: var(--bg-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    overflow: hidden;
}

/* 自动分割线：非最后一个子项的底部 */
.list-group > :deep(*) {
    position: relative;
}
.list-group > :deep(*:not(:last-child))::after {
    content: '';
    position: absolute;
    left: var(--space-4);
    right: 0;
    bottom: 0;
    height: 1px;
    background: var(--separator);
}
</style>
