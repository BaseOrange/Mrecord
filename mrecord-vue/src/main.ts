import {createApp} from 'vue'
import {createPinia} from 'pinia'
import './style.css'
import App from './App.vue'
import router from './router'
import Varlet, {StyleProvider} from '@varlet/ui'
import '@varlet/ui/es/style'

// 配置月衡 Mrecord 橙色主题
StyleProvider({
    '--color-primary': '#FF6500',
    '--color-primary-container': '#FF6500',
    '--color-on-primary': '#ffffff',
    '--color-on-primary-container': '#ffffff',
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(Varlet)

app.mount('#app')