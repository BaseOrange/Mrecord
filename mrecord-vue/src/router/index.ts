import {createRouter, createWebHistory} from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'Home',
            component: () => import('@/views/Home.vue'),
            meta: {title: '首页'}
        },
        {
            path: '/login',
            name: 'Login',
            component: () => import('@/views/Login.vue'),
            meta: {title: '登录'}
        },
        {
            path: '/:pathMatch(.*)*',
            name: 'NotFound',
            component: () => import('@/views/NotFound.vue'),
            meta: {title: '页面不存在'}
        }
    ]
})

router.beforeEach((to, _from, next) => {
    // 设置页面标题
    document.title = to.meta.title ? `${to.meta.title} | 月衡 Mrecord` : '月衡 Mrecord'
    next()
})

export default router
