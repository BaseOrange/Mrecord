<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

interface Props {
  title?: string
  showBack?: boolean
  backPath?: string
  /** 返回前钩子：传入则由父组件全权接管返回逻辑（如未保存修改的二次确认），不传走默认逻辑 */
  beforeBack?: () => void
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  showBack: false,
  backPath: '',
  beforeBack: undefined
})

// 标题点击事件：供「我的」页面实现「1.5 秒内点击 5 次打开诊断面板」的彩蛋入口
const emit = defineEmits<{ 'title-click': [] }>()

const router = useRouter()

const displayTitle = computed(() => props.title || document.title.split(' | ')[0])

/** 标题被点击时抛出事件，由父组件决定是否响应（不响应的页面无任何行为变化） */
const handleTitleClick = () => {
  emit('title-click')
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
  <div class="page-header">
    <div v-if="showBack" class="back-btn" @click="handleBack">
      <var-icon name="chevron-left" :size="24" />
    </div>
    <h2 class="header-title" @click="handleTitleClick">{{ displayTitle }}</h2>
    <!-- 右侧插槽：有内容则渲染，无内容且 showBack 时自动补 32px 占位保持视觉平衡 -->
    <div v-if="$slots.right" class="header-right">
      <slot name="right" />
    </div>
    <div v-else-if="showBack" class="header-placeholder"></div>
  </div>
</template>

<style scoped>
.page-header {
  background: #fff;
  padding: calc(16px + env(safe-area-inset-top, 0px)) 16px 16px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid #f0f0f0;
}

.back-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  padding: 0;
  border: none;
  background: none;
  color: #333;
  cursor: pointer;
  border-radius: 50%;
  -webkit-tap-highlight-color: transparent;
  transition: background 0.15s;
  flex-shrink: 0;
}

.back-btn:active {
  background: rgba(0, 0, 0, 0.06);
}

.header-title {
  flex: 1;
  font-size: 18px;
  font-weight: 600;
  color: #1d1d1f;
  margin: 0;
  line-height: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.header-right {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.header-placeholder {
  width: 32px;
  flex-shrink: 0;
}
</style>
