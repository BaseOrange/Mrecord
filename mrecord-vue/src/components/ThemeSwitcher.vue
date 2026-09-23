<script setup lang="ts">
import { useThemeStore, type ThemeMode } from '@/stores/theme'

/**
 * 三态主题切换器（Q7 → C）：跟随系统 / 浅色 / 深色。
 * Apple segmented control 样式，放「我的」页面。
 */
const theme = useThemeStore()

const options: { value: ThemeMode; label: string }[] = [
    { value: 'system', label: '跟随系统' },
    { value: 'light', label: '浅色' },
    { value: 'dark', label: '深色' },
]
</script>

<template>
    <div class="theme-switcher" role="radiogroup" aria-label="主题模式">
        <button
            v-for="opt in options"
            :key="opt.value"
            type="button"
            class="theme-switcher__item"
            :class="{ 'is-active': theme.mode === opt.value }"
            role="radio"
            :aria-checked="theme.mode === opt.value"
            @click="theme.setMode(opt.value)"
        >
            {{ opt.label }}
        </button>
    </div>
</template>

<style scoped>
.theme-switcher {
    display: flex;
    gap: 2px;
    padding: 3px;
    background: var(--bg-surface-2);
    border-radius: var(--radius-pill);
    /* 在卡片行内不被压缩，避免「跟随系统」被挤到换行 */
    flex-shrink: 0;
}

.theme-switcher__item {
    flex: 1;
    height: 32px;
    min-height: 32px;
    padding: 0 12px;
    border-radius: var(--radius-pill);
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    color: var(--text-secondary);
    cursor: pointer;
    white-space: nowrap;
    -webkit-tap-highlight-color: transparent;
    transition:
        color var(--duration-fast) var(--ease-out),
        background var(--duration-fast) var(--ease-out);
}

.theme-switcher__item.is-active {
    background: var(--bg-surface);
    color: var(--text-primary);
    font-weight: var(--weight-semibold);
    box-shadow: var(--shadow-sm);
}
</style>
