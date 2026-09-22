<script setup lang="ts">
import AppIcon from './AppIcon.vue'

/**
 * Apple list cell：图标 + 标题 + （值 + 副值）+ chevron。
 * 在 `<ListGroup>` 内使用时自动处理分割线（最后一项无线）。
 */
withDefaults(
    defineProps<{
        /** 左侧图标（AppIcon 名），不传则留出图标位以对齐 */
        icon?: string
        label: string
        /** 右侧主值 */
        value?: string
        /** 右侧副值（主值下方小字） */
        subValue?: string
        /** 副值颜色（语义色用令牌，如 var(--semantic-up)） */
        subValueColor?: string
        /** 是否显示右侧箭头，默认显示 */
        arrow?: boolean
        /** 危险样式（注销/删除类） */
        danger?: boolean
    }>(),
    {
        arrow: true,
    },
)

defineEmits<{
    click: []
}>()
</script>

<template>
    <button type="button" class="list-cell" :class="{ 'list-cell--danger': danger }" @click="$emit('click')">
        <div v-if="icon || $slots.icon" class="cell-icon">
            <slot name="icon">
                <AppIcon v-if="icon" :name="icon" :size="20" />
            </slot>
        </div>
        <span class="cell-label">{{ label }}</span>
        <div class="cell-value-group">
            <span v-if="value" class="cell-value numeric">{{ value }}</span>
            <span
                v-if="subValue"
                class="cell-subvalue"
                :style="subValueColor ? { color: subValueColor } : undefined"
            >{{ subValue }}</span>
        </div>
        <span v-if="arrow" class="cell-arrow" aria-hidden="true">
            <AppIcon name="chevron-right" :size="16" />
        </span>
    </button>
</template>

<style scoped>
.list-cell {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    min-height: 48px;
    padding: var(--space-2) var(--space-4);
    text-align: left;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
    transition: background-color var(--duration-fast) var(--ease-out);
}

.list-cell:active {
    background-color: var(--bg-surface-2);
}

.cell-icon {
    width: 24px;
    height: 24px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
}

.list-cell--danger .cell-icon,
.list-cell--danger .cell-label {
    color: var(--semantic-danger);
}

.cell-label {
    flex: 1;
    min-width: 0;
    font-size: var(--text-body);
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.cell-value-group {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 1px;
    flex-shrink: 0;
}

.cell-value {
    font-size: var(--text-body);
    font-weight: var(--weight-semibold);
    color: var(--text-primary);
}

.cell-subvalue {
    font-size: var(--text-xs);
    color: var(--text-tertiary);
    line-height: 1.3;
}

.cell-arrow {
    color: var(--text-tertiary);
    flex-shrink: 0;
    display: flex;
    align-items: center;
}
</style>
