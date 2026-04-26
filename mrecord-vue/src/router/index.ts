import {createRouter, createWebHistory} from 'vue-router'
import {useUserStore} from '@/stores/user'

// 无需登录即可访问的页面
const PUBLIC_PAGES = ['/login', '/register', '/forgot-password', '/reset-password']

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
            path: '/book/:bookId/template',
            name: 'TempItemEditor',
            component: () => import('@/views/TempItemEditor.vue'),
            meta: {title: '编辑账目模板'}
        },
        {
            path: '/book/:bookId/records',
            name: 'BookRecord',
            component: () => import('@/views/BookRecordPage.vue'),
            meta: {title: '账簿记录', transition: 'slide-up'}
        },
        {
            path: '/book/:bookId/record',
            name: 'Record',
            component: () => import('@/views/RecordPage.vue'),
            meta: {title: '记账', transition: 'slide-up'}
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

    const userStore = useUserStore()
    const isPublic = PUBLIC_PAGES.includes(to.path)

    if (userStore.token && isPublic) {
        // 已登录访问公开页面 → 跳转首页
        next('/home')
    } else if (!userStore.token && !isPublic) {
        // 未登录访问需认证页面 → 跳转登录
        next('/login')
    } else {
        next()
    }
})

export default router
