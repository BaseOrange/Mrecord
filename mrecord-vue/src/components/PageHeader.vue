<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

interface Props {
  title?: string
  showBack?: boolean
  backPath?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  showBack: false,
  backPath: ''
})

const router = useRouter()

const displayTitle = computed(() => props.title || document.title.split(' | ')[0])

const handleBack = () => {
  if (props.backPath) {
    router.push(props.backPath)
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
    <h2 class="header-title">{{ displayTitle }}</h2>
  </div>
</template>

<style scoped>
.page-header {
  background: #fff;
  padding: 16px;
  padding-top: calc(16px + env(safe-area-inset-top, 0px));
  display: flex;
  align-items: center;
  gap: 12px;
  border-bottom: 1px solid #f0f0f0;
}

.back-btn {
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  transition: background-color 0.2s;
  -webkit-tap-highlight-color: transparent;
}

.back-btn:active {
  background-color: rgba(0, 0, 0, 0.05);
}

.header-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0;
  flex: 1;
}
</style>
