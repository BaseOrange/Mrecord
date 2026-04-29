<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { Chart, LineController, LineElement, PointElement, LinearScale, CategoryScale, Tooltip, Legend, Title, Filler } from 'chart.js'

Chart.register(LineController, LineElement, PointElement, LinearScale, CategoryScale, Tooltip, Legend, Title, Filler)

interface Dataset {
  label: string
  data: number[]
  color: string
  fill?: boolean
}

const props = defineProps<{
  labels: string[]
  datasets: Dataset[]
  title?: string
}>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
let chartInstance: Chart | null = null

const renderChart = () => {
  if (!canvasRef.value) return

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = canvasRef.value.getContext('2d')
  if (!ctx) return

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels: props.labels,
      datasets: props.datasets.map(ds => ({
        label: ds.label,
        data: ds.data,
        borderColor: ds.color,
        backgroundColor: ds.fill ? ds.color + '20' : ds.color,
        borderWidth: 2,
        pointRadius: 3,
        pointHoverRadius: 5,
        pointBackgroundColor: '#fff',
        pointBorderColor: ds.color,
        pointBorderWidth: 2,
        tension: 0.35,
        fill: ds.fill || false,
      })),
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
              color: '#333',
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
            color: '#666',
          },
        },
        tooltip: {
          backgroundColor: 'rgba(0,0,0,0.8)',
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
            color: '#999',
          },
        },
        y: {
          border: { display: false },
          grid: {
            color: '#f0f0f0',
            drawBorder: false,
          },
          ticks: {
            font: { size: 11 },
            color: '#999',
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
