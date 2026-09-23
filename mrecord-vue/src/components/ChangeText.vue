<script setup lang="ts">
import { computed } from 'vue'
import { getChangeText } from '@/utils/format'
import AppIcon from './AppIcon.vue'

/**
 * 变化率展示（环比/同比）：百分比文本 + 红/绿语义色 + 趋势图标（D1 正红负绿）。
 *
 * 与 MoneyText 分工：MoneyText 管金额（¥ + 等宽数字），ChangeText 管百分比变化。
 * 语义色用令牌而非 format.ts 的 hex，保证深浅色模式对比度。
 */
const props = defineProps<{
    /** 百分比值（如 3.2 表示 +3.20%） */
    value?: number | null
    /** 是否显示趋势图标，默认显示 */
    icon?: boolean
}>()

const text = computed(() => getChangeText(props.value))

const color = computed(() => {
    const v = props.value
    if (v === undefined || v === null || Number.isNaN(v)) return 'var(--text-tertiary)'
    if (v === 0) return 'var(--text-tertiary)'
    return v > 0 ? 'var(--semantic-up)' : 'var(--semantic-down)'
})

const trendIcon = computed(() => {
    const v = props.value
    if (v === undefined || v === null || v === 0) return 'minus'
    return v > 0 ? 'trending-up' : 'trending-down'
})
</script>

<template>
    <span class="change-text numeric" :style="{ color }">
        <AppIcon v-if="icon !== false" :name="trendIcon" :size="12" :stroke-width="2.4" />
        {{ text }}
    </span>
</template>

<style scoped>
.change-text {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
}
</style>
