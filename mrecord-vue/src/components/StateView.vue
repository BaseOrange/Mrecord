<script setup lang="ts">
import AppIcon from './AppIcon.vue'

/**
 * 三态视图：loading / empty / error 的统一渲染（Q8）。
 *
 * 治掉 HomePage / BookPage / StatsPage 各写一遍的 spinner 与空状态 SVG。
 * idle 状态直接渲染默认插槽，可无侵入地包在页面外层：
 *
 * <StateView state="loading" empty-text="还没有账簿">
 *   <div class="content" />
 * </StateView>
 */
type ViewState = 'idle' | 'loading' | 'empty' | 'error'

withDefaults(
    defineProps<{
        state: ViewState
        /** 空状态主文案 */
        emptyText?: string
        /** 空状态副文案 */
        emptySub?: string
        /** 空状态图标（Lucide 名） */
        emptyIcon?: string
        /** 失败文案 */
        errorText?: string
    }>(),
    {
        emptyText: '暂无数据',
        emptySub: '',
        emptyIcon: 'inbox',
        errorText: '加载失败',
    },
)
</script>

<template>
    <template v-if="state === 'idle'">
        <slot />
    </template>

    <div v-else class="state-view" :class="`state-view--${state}`">
        <!-- 加载中 -->
        <div v-if="state === 'loading'" class="state-loading">
            <div class="state-spinner" role="status" aria-label="加载中"></div>
        </div>

        <!-- 空状态 -->
        <div v-else-if="state === 'empty'" class="state-empty">
            <div class="state-icon-circle">
                <AppIcon :name="emptyIcon" :size="28" :stroke-width="1.5" />
            </div>
            <p class="state-text">{{ emptyText }}</p>
            <p v-if="emptySub" class="state-sub">{{ emptySub }}</p>
            <div class="state-action">
                <slot name="empty-action" />
            </div>
        </div>

        <!-- 失败 -->
        <div v-else-if="state === 'error'" class="state-error">
            <div class="state-icon-circle state-icon-circle--danger">
                <AppIcon name="circle-alert" :size="28" :stroke-width="1.5" />
            </div>
            <p class="state-text">{{ errorText }}</p>
            <div class="state-action">
                <slot name="error-action" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.state-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px var(--space-4) 40px;
}

/* ==================== 加载中 ==================== */
.state-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px 0;
}

.state-spinner {
    width: 28px;
    height: 28px;
    border: 3px solid var(--separator);
    border-top-color: var(--brand);
    border-radius: 50%;
    animation: state-spin 0.8s linear infinite;
}

@keyframes state-spin {
    to {
        transform: rotate(360deg);
    }
}

/* ==================== 空 / 失败 ==================== */
.state-icon-circle {
    width: 64px;
    height: 64px;
    border-radius: var(--radius-xl);
    background: var(--bg-surface-2);
    color: var(--text-tertiary);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: var(--space-5);
}

.state-icon-circle--danger {
    color: var(--semantic-danger);
}

.state-text {
    font-size: var(--text-title-3);
    font-weight: var(--weight-medium);
    color: var(--text-secondary);
}

.state-sub {
    margin-top: var(--space-1);
    font-size: var(--text-sm);
    color: var(--text-tertiary);
}

.state-action {
    margin-top: var(--space-5);
}
</style>
