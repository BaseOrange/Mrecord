<script setup lang="ts">
import { computed } from 'vue'
import { formatMoney } from '@/utils/format'

/**
 * 金额排版组件（Q9-③）。
 *
 * - 等宽数字（tabular-nums），财务排版基本功
 * - 货币符号弱化（小一号、次级色）
 * - 百分比变化（环比/同比）请用 ChangeText 组件，二者分工不混用
 */
const props = withDefaults(
    defineProps<{
        value?: number | null
        /** 小数位数，默认 2 */
        decimals?: number
        /** 货币符号，默认 ¥；传空串则不显示 */
        symbol?: string
        /** 尺寸档位 */
        size?: 'sm' | 'md' | 'lg' | 'hero'
    }>(),
    {
        decimals: 2,
        symbol: '¥',
        size: 'md',
    },
)

const text = computed(() => formatMoney(props.value, props.decimals))
</script>

<template>
    <span class="money-text numeric" :class="`money-text--${size}`">
        <span v-if="symbol" class="money-symbol">{{ symbol }}</span>{{ text }}
    </span>
</template>

<style scoped>
.money-text {
    font-weight: var(--weight-bold);
    letter-spacing: -0.5px;
}

.money-symbol {
    font-size: 0.62em;
    font-weight: var(--weight-medium);
    opacity: 0.65;
    margin-right: 0.08em;
}

.money-text--sm {
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
}
.money-text--md {
    font-size: var(--text-body);
}
.money-text--lg {
    font-size: 22px;
}
.money-text--hero {
    font-size: var(--text-money);
    letter-spacing: -1px;
}
</style>
