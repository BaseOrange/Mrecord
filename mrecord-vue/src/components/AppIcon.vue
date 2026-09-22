<script setup lang="ts">
import { computed, type Component } from 'vue'
import {
    House,
    ChevronLeft,
    ChevronRight,
    CircleUserRound,
    CircleAlert,
    CircleOff,
    TrendingUp,
    TrendingDown,
    Minus,
    Plus,
    CalendarDays,
    Settings,
    Trash2,
    Download,
    FileText,
    Lock,
    NotebookText,
    CirclePlus,
    Share,
    ShieldCheck,
    TriangleAlert,
    X,
    Wrench,
    Inbox,
} from 'lucide-vue-next'

/**
 * AppIcon —— 统一图标层（Q10 → B）。
 *
 * Lucide 是最接近 Apple SF Symbols 的开源图标集（细描边、圆角线帽）。
 * 这里显式导入所需图标以保证 tree-shaking（切勿 import * 全量引入）。
 *
 * 查表兼容两种命名：
 * - 旧 Varlet / Material 名（home / cog-outline …）：迁移期页面直接套用
 * - Lucide 原生 kebab 名（house / settings …）：新代码统一用这个
 * 未命中则渲染占位图标并在开发环境告警，便于迁移期发现遗漏。
 */
const ICONS: Record<string, Component> = {
    // ---- 旧 Varlet / Material 名 → Lucide ----
    home: House,
    'account-circle': CircleUserRound,
    'account-circle-outline': CircleUserRound,
    'alert-circle-outline': CircleAlert,
    'arrow-up-bold-box-outline': TrendingUp,
    'calendar-month': CalendarDays,
    'cog-outline': Settings,
    delete: Trash2,
    'download-outline': Download,
    'file-document-outline': FileText,
    'file-text-outline': FileText,
    'lock-outline': Lock,
    notebook: NotebookText,
    'plus-circle': CirclePlus,
    'shield-outline': ShieldCheck,
    warning: TriangleAlert,
    'window-close': X,
    right: ChevronRight,
    // ---- Lucide 原生名（新代码统一使用） ----
    house: House,
    'circle-user-round': CircleUserRound,
    'circle-alert': CircleAlert,
    'trending-up': TrendingUp,
    'calendar-days': CalendarDays,
    settings: Settings,
    'trash-2': Trash2,
    download: Download,
    'file-text': FileText,
    lock: Lock,
    'notebook-text': NotebookText,
    'circle-plus': CirclePlus,
    share: Share,
    'shield-check': ShieldCheck,
    'triangle-alert': TriangleAlert,
    x: X,
    wrench: Wrench,
    'chevron-left': ChevronLeft,
    'chevron-right': ChevronRight,
    inbox: Inbox,
    'trending-down': TrendingDown,
    minus: Minus,
    plus: Plus,
}

const props = withDefaults(
    defineProps<{
        /** 图标名：旧 Varlet 名或 Lucide 原生 kebab 名 */
        name: string
        /** 尺寸（px），默认 22 */
        size?: number | string
        /** 描边颜色，默认继承 currentColor */
        color?: string
        /** 描边宽度，默认 1.8（贴近 SF Symbols 的纤细感） */
        strokeWidth?: number
        /** 无障碍标题：传入则视为语义图标并朗读，否则标记为装饰性 */
        title?: string
    }>(),
    {
        size: 22,
        strokeWidth: 1.8,
    },
)

const resolved = computed<Component>(() => {
    const icon = ICONS[props.name]
    if (!icon && import.meta.env.DEV) {
        console.warn(`[AppIcon] 未知图标名:「${props.name}」，请在 ICONS 中补充导入`)
    }
    return icon || CircleOff
})

const isDecorative = computed(() => !props.title)
</script>

<template>
    <component
        :is="resolved"
        class="app-icon"
        :size="size"
        :color="color"
        :stroke-width="strokeWidth"
        :role="isDecorative ? undefined : 'img'"
        :aria-label="title"
        :aria-hidden="isDecorative ? 'true' : undefined"
        focusable="false"
    />
</template>

<style scoped>
.app-icon {
    display: inline-flex;
    flex-shrink: 0;
}
</style>
