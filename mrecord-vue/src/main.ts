import {createApp} from 'vue'
import {createPinia} from 'pinia'
import './style.css'
import '@varlet/ui/es/snackbar/style/index.mjs'
import App from './App.vue'
import router from './router'
import {StyleProvider} from '@varlet/ui'
import {pushErrorLog} from './utils/errorLog'

// 配置月衡 Mrecord 橙色主题
StyleProvider({
    '--color-primary': '#FF6500',
    '--color-primary-container': '#FF6500',
    '--color-on-primary': '#ffffff',
    '--color-on-primary-container': '#ffffff',
})

// ==================== 全局错误捕获（便于定位生产环境白屏问题） ====================
// 每个钩子在保留原有「白屏兜底渲染」的同时，额外写入错误日志环形缓冲区
// （见 utils/errorLog），供诊断面板展示，帮用户提 issue 时带上报错上下文。
window.addEventListener('error', (event) => {
    pushErrorLog({
        type: 'js-error',
        message: event.message || 'unknown',
        source: event.filename || undefined,
        detail: event.lineno ? `line ${event.lineno}:${event.colno}` : undefined,
    })
    const el = document.getElementById('app')
    if (el && el.children.length === 0) {
        el.innerHTML = `<div style="padding:20px;color:red;font-family:monospace;">
            <h3>JS 运行时错误</h3>
            <pre>${event.message || 'unknown'}</pre>
            <p>文件: ${event.filename || 'unknown'}</p>
        </div>`
    }
})
window.addEventListener('unhandledrejection', (event) => {
    const reason = String(event.reason)
    pushErrorLog({
        type: 'unhandledrejection',
        message: reason,
    })
    const el = document.getElementById('app')
    if (el && el.children.length === 0) {
        el.innerHTML = `<div style="padding:20px;color:red;font-family:monospace;">
            <h3>未捕获的 Promise 错误</h3>
            <pre>${reason}</pre>
        </div>`
    }
})

const app = createApp(App)
app.config.errorHandler = (err, _instance, info) => {
    console.error('[Vue Error]', err, info)
    pushErrorLog({
        type: 'vue',
        message: String(err),
        detail: info || undefined,
    })
    const el = document.getElementById('app')
    if (el && el.children.length === 0) {
        el.innerHTML = `<div style="padding:20px;color:red;font-family:monospace;">
            <h3>Vue 运行时错误</h3>
            <pre>${String(err)}</pre>
            <p>信息: ${info}</p>
        </div>`
    }
}

app.use(createPinia())
app.use(router)

// 路由加载失败处理
router.onError((error) => {
    console.error('[Router Error]', error)
    pushErrorLog({
        type: 'router',
        message: String(error),
    })
    const el = document.getElementById('app')
    if (el) {
        el.innerHTML = `<div style="padding:20px;color:red;font-family:monospace;">
            <h3>路由加载失败</h3>
            <pre>${String(error)}</pre>
            <p>可能原因：静态资源路径错误或文件缺失</p>
        </div>`
    }
})

app.mount('#app')