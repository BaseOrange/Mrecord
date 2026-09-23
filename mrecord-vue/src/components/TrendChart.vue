<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { Chart, LineController, LineElement, PointElement, LinearScale, CategoryScale, Tooltip, Legend, Title, Filler } from 'chart.js'
import { useThemeStore } from '@/stores/theme'

Chart.register(LineController, LineElement, PointElement, LinearScale, CategoryScale, Tooltip, Legend, Title, Filler)

interface Dataset {
  label: string
  data: number[]
  /** 线条色：hex 或 CSS 变量（如 var(--brand)），变量在渲染时解析 */
  color: string
  fill?: boolean
}

const props = defineProps<{
  labels: string[]
  datasets: Dataset[]
  title?: string
}>()

const themeStore = useThemeStore()

const canvasRef = ref<HTMLCanvasElement | null>(null)
let chartInstance: Chart | null = null

/**
 * 解析颜色：支持 var(--xxx) 形式的令牌，运行时从 documentElement 读取计算值。
 * 这样图表配色能随主题切换自动变化，无需页面层各自维护两套色值。
 */
const resolveColor = (input: string): string => {
  const match = /^var\((--[\w-]+)\)$/.exec(input.trim())
  if (!match) return input
  return getComputedStyle(document.documentElement).getPropertyValue(match[1]).trim() || input
}

/** hex → rgba，用于面积填充 */
const hexToRgba = (hex: string, alpha: number): string => {
  const m = /^#?([\da-f]{2})([\da-f]{2})([\da-f]{2})$/i.exec(hex)
  if (!m) return hex
  const r = parseInt(m[1], 16)
  const g = parseInt(m[2], 16)
  const b = parseInt(m[3], 16)
  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

const renderChart = () => {
  if (!canvasRef.value) return

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = canvasRef.value.getContext('2d')
  if (!ctx) return

  // 语义色统一在此解析，页面层只传令牌
  const tickColor = resolveColor('var(--text-tertiary)')
  const gridColor = resolveColor('var(--separator)')
  const legendColor = resolveColor('var(--text-secondary)')
  const surfaceColor = resolveColor('var(--bg-surface)')
  const onSurfaceColor = resolveColor('var(--text-primary)')
  const pointBg = resolveColor('var(--bg-surface)')

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels: props.labels,
      datasets: props.datasets.map(ds => {
        const color = resolveColor(ds.color)
        return {
          label: ds.label,
          data: ds.data,
          borderColor: color,
          backgroundColor: ds.fill ? hexToRgba(color, 0.12) : color,
          borderWidth: 2,
          pointRadius: 3,
          pointHoverRadius: 5,
          pointBackgroundColor: pointBg,
          pointBorderColor: color,
          pointBorderWidth: 2,
          tension: 0.35,
          fill: ds.fill || false,
        }
      }),
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      interaction: {
        mode: 'index',
        intersect: false,
      },
      plugins: {
        title: props.title
          ? {
              display: true,
              text: props.title,
              font: { size: 14, weight: '600' },
              color: onSurfaceColor,
              padding: { top: 8, bottom: 12 },
            }
          : undefined,
        legend: {
          position: 'top',
          labels: {
            usePointStyle: true,
            pointStyle: 'circle',
            padding: 16,
            font: { size: 12 },
            color: legendColor,
          },
        },
        tooltip: {
          backgroundColor: surfaceColor,
          titleColor: onSurfaceColor,
          bodyColor: legendColor,
          titleFont: { size: 12 },
          bodyFont: { size: 12 },
          padding: 10,
          cornerRadius: 8,
          displayColors: true,
        },
      },
      scales: {
        x: {
          grid: { display: false },
          ticks: {
            font: { size: 11 },
            color: tickColor,
          },
        },
        y: {
          border: { display: false },
          grid: {
            color: gridColor,
            drawBorder: false,
          },
          ticks: {
            font: { size: 11 },
            color: tickColor,
            callback: (value: number | string) => {
              const num = typeof value === 'string' ? parseFloat(value) : value
              if (Math.abs(num) >= 10000) {
                return (num / 10000).toFixed(1) + 'w'
              }
              return num.toLocaleString()
            },
          },
        },
      },
    },
  })
}

onMounted(() => {
  renderChart()
})

onUnmounted(() => {
  if (chartInstance) {
    chartInstance.destroy()
    chartInstance = null
  }
})

watch(() => [props.labels, props.datasets], () => {
  renderChart()
}, { deep: true })

// 主题切换时重渲染，让图表配色随深浅色模式变化
watch(() => themeStore.resolved, () => {
  nextTick(renderChart)
})
</script>

<template>
  <div class="trend-chart">
    <canvas ref="canvasRef"></canvas>
  </div>
</template>

<style scoped>
.trend-chart {
  position: relative;
  width: 100%;
  height: 260px;
}
</style>
