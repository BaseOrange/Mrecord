<script setup lang="ts">
import { computed } from 'vue'
import { formatMoney } from '@/utils/format'

/**
 * 金额排版组件（Q9-③）。
 *
 * - 等宽数字（tabular-nums），财务排版基本功
 * - 货币符号弱化（小一号、次级色）
 * - 可选语义着色：传入 change 时按 D1（正红负绿）给数字上色
 */
const props = withDefaults(
    defineProps<{
        value?: number | null
        /** 小数位数，默认 2 */
        decimals?: number
        /** 货币符号，默认 ¥；传空串则不显示 */
        symbol?: string
        /** 变化值：传入则按红涨绿跌着色；不传为中性色 */
        change?: number | null
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

const color = computed(() => {
    if (props.change === undefined || props.change === null || Number.isNaN(props.change)) {
        return undefined
    }
    if (props.change === 0) return 'var(--text-secondary)'
    return props.change > 0 ? 'var(--semantic-up)' : 'var(--semantic-down)'
})
</script>

<template>
    <span class="money-text numeric" :class="`money-text--${size}`" :style="{ color }">
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
