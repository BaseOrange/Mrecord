<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import AppIcon from './AppIcon.vue'

interface Props {
    title?: string
    showBack?: boolean
    backPath?: string
    /** 返回前钩子：传入则由父组件全权接管返回逻辑（如未保存修改的二次确认），不传走默认逻辑 */
    beforeBack?: () => void
    /** 大标题模式：仅用于 tab 根视图（Q14 → A，遵循 Apple HIG） */
    large?: boolean
    /** 大标题下的副标题（如首页的「9 月」） */
    subtitle?: string
}

const props = withDefaults(defineProps<Props>(), {
    title: '',
    showBack: false,
    backPath: '',
    beforeBack: undefined,
    large: false,
    subtitle: '',
})

// 标题点击事件：供「我的」页面实现「1.5 秒内点击 5 次打开诊断面板」的彩蛋入口
const emit = defineEmits<{ 'title-click': [] }>()

const router = useRouter()

const displayTitle = computed(() => props.title || document.title.split(' | ')[0])

// ==================== 大标题滚动折叠（Q9-①）====================
// 页面滚动超过阈值时，大标题区收起，顶部栏转为毛玻璃并显示小标题。
const COLLAPSE_THRESHOLD = 24
const collapsed = ref(false)

const onScroll = () => {
    collapsed.value = window.scrollY > COLLAPSE_THRESHOLD
}

onMounted(() => {
    if (props.large) {
        window.addEventListener('scroll', onScroll, { passive: true })
        onScroll()
    }
})

onUnmounted(() => {
    window.removeEventListener('scroll', onScroll)
})

/** 大标题模式下，只有大标题本身响应标题点击（彩蛋）；小标题栏仅在非大标题模式响应 */
const onCompactTitleClick = () => {
    if (!props.large) emit('title-click')
}

/**
 * 返回逻辑（I11）：
 * - 父组件传了 beforeBack（如未保存修改的二次确认）→ 全权交由它接管
 * - 有 backPath → replace 到指定路由（不往历史栈里追加记录）
 * - 无 backPath → 先判 window.history.length：深链接进入时（length ≤ 1）没有可返回的
 *   历史栈，router.back() 会退出应用或无响应，此时降级到 /home
 */
const handleBack = () => {
    if (props.beforeBack) {
        props.beforeBack()
    } else if (props.backPath) {
        router.replace(props.backPath)
    } else if (window.history.length <= 1) {
        router.replace('/home')
    } else {
        router.back()
    }
}
</script>

<template>
    <div
        class="page-header"
        :class="{ 'page-header--large': large, 'is-collapsed': collapsed }"
    >
        <!-- 紧凑栏：始终存在；大标题模式下初始透明，滚动后转毛玻璃并显示小标题 -->
        <div class="header-bar">
            <div v-if="showBack" class="back-btn" @click="handleBack">
                <AppIcon name="chevron-left" :size="24" />
            </div>
            <h2
                class="header-title"
                :class="{ 'header-title--hidden': large && !collapsed }"
                @click="onCompactTitleClick"
            >
                {{ displayTitle }}
            </h2>
            <!-- 右侧插槽：有内容则渲染，无内容且 showBack 时自动补 32px 占位保持视觉平衡 -->
            <div v-if="$slots.right" class="header-right">
                <slot name="right" />
            </div>
            <div v-else-if="showBack" class="header-placeholder"></div>
        </div>

        <!-- 大标题区：仅 large 模式，随滚动平滑收起 -->
        <div v-if="large" class="header-large" @click="emit('title-click')">
            <div class="header-large-inner">
                <h1 class="header-large-title">{{ displayTitle }}</h1>
                <p v-if="subtitle" class="header-subtitle">{{ subtitle }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.page-header {
    position: relative;
    z-index: var(--z-header);
}

.page-header--large {
    /* 大标题模式吸顶，内容从下方滚过时呈现毛玻璃穿透效果 */
    position: sticky;
    top: 0;
}

.header-bar {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: calc(var(--space-4) + env(safe-area-inset-top, 0px)) var(--page-padding) var(--space-4);
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--separator);
    transition:
        background var(--duration-base) var(--ease-out),
        border-color var(--duration-base) var(--ease-out);
}

/* 大标题模式：顶部栏初始透明无描边 */
.page-header--large .header-bar {
    background: transparent;
    border-bottom-color: transparent;
}

/* 滚动后转毛玻璃 */
.page-header--large.is-collapsed .header-bar {
    background: var(--header-bg);
    backdrop-filter: blur(20px) saturate(1.4);
    -webkit-backdrop-filter: blur(20px) saturate(1.4);
    border-bottom-color: var(--separator);
}

.back-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    color: var(--text-primary);
    cursor: pointer;
    border-radius: 50%;
    -webkit-tap-highlight-color: transparent;
    transition: background var(--duration-fast);
    flex-shrink: 0;
}

.back-btn:active {
    background: var(--separator);
}

.header-title {
    flex: 1;
    font-size: var(--text-title-3);
    font-weight: var(--weight-semibold);
    color: var(--text-primary);
    margin: 0;
    line-height: 1.2;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: opacity var(--duration-fast) var(--ease-out);
}

/* 大标题模式下，小标题在未滚动时隐藏（位置仍保留，作为折叠态占位） */
.header-title--hidden {
    opacity: 0;
}

.header-right {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    gap: var(--space-1);
}

.header-placeholder {
    width: 32px;
    flex-shrink: 0;
}

/* ==================== 大标题区 ==================== */
/* 用 grid-template-rows 1fr→0fr 实现无 max-height 估算滞后的高度动画 */
.header-large {
    display: grid;
    grid-template-rows: 1fr;
    padding: 0 var(--page-padding);
    cursor: pointer;
    transition:
        grid-template-rows var(--duration-base) var(--ease-out),
        opacity var(--duration-fast) var(--ease-out);
}

.header-large-inner {
    overflow: hidden;
    min-height: 0;
    padding-bottom: var(--space-4);
}

.is-collapsed .header-large {
    grid-template-rows: 0fr;
    opacity: 0;
    pointer-events: none;
}

.header-large-title {
    font-size: var(--text-large-title);
    font-weight: var(--weight-bold);
    letter-spacing: -1px;
    color: var(--text-primary);
    line-height: 1.15;
    margin: 0;
}

.header-subtitle {
    margin-top: var(--space-1);
    font-size: var(--text-sm);
    color: var(--text-secondary);
    line-height: 1.4;
}
</style>
