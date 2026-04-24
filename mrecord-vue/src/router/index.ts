import {createRouter, createWebHistory} from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            redirect: '/login'
        },
        {
            path: '/',
            component: () => import('@/views/Layout.vue'),
            children: [
                {
                    path: 'home',
                    name: 'Home',
                    component: () => import('@/views/HomePage.vue'),
                    meta: {title: '首页'}
                },
                {
                    path: 'stats',
                    name: 'Stats',
                    component: () => import('@/views/StatsPage.vue'),
                    meta: {title: '统计'}
                },
                {
                    path: 'book',
                    name: 'Book',
                    component: () => import('@/views/BookPage.vue'),
                    meta: {title: '账簿'}
                },
                {
                    path: 'profile',
                    name: 'Profile',
                    component: () => import('@/views/ProfilePage.vue'),
                    meta: {title: '我的'}
                },
            ]
        },
        {
            path: '/login',
            name: 'Login',
            component: () => import('@/views/Login.vue'),
            meta: {title: '登录'}
        },
        {
            path: '/register',
            name: 'Register',
            component: () => import('@/views/Register.vue'),
            meta: {title: '注册'}
        },
        {
            path: '/forgot-password',
            name: 'ForgotPassword',
            component: () => import('@/views/ForgotPassword.vue'),
            meta: {title: '找回密码'}
        },
        {
            path: '/reset-password',
            name: 'ResetPassword',
            component: () => import('@/views/ResetPassword.vue'),
            meta: {title: '重置密码'}
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
