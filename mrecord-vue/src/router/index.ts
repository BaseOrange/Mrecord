import {createRouter, createWebHistory} from 'vue-router'
import {useUserStore} from '@/stores/user'

// 无需登录即可访问的页面
const PUBLIC_PAGES = ['/login', '/register', '/forgot-password', '/reset-password', '/activate-account']

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            component: () => import('@/views/Layout.vue'),
            redirect: '/home',
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
                    path: 'export',
                    name: 'Export',
                    component: () => import('@/views/ExportPage.vue'),
                    meta: {title: '数据导出'}
                },
                {
                    path: 'profile',
                    name: 'Profile',
                    component: () => import('@/views/ProfilePage.vue'),
                    meta: {title: '我的'}
                },
                {
                    path: 'change-password',
                    name: 'ChangePassword',
                    component: () => import('@/views/ChangePassword.vue'),
                    meta: {title: '修改密码'}
                },
                {
                    path: 'profile-edit',
                    name: 'ProfileEdit',
                    component: () => import('@/views/ProfileEditPage.vue'),
                    meta: {title: '个人资料'}
                },
                {
                    path: 'admin',
                    name: 'Admin',
                    component: () => import('@/views/AdminPage.vue'),
                    meta: {title: '管理中心', requiresAdmin: true}
                },
                {
                    path: 'admin/users',
                    name: 'AdminUsers',
                    component: () => import('@/views/AdminUsersPage.vue'),
                    meta: {title: '用户管理', requiresAdmin: true}
                },
                {
                    path: 'admin/logs',
                    name: 'AdminLogs',
                    component: () => import('@/views/AdminLogsPage.vue'),
                    meta: {title: '操作日志', requiresAdmin: true}
                },
                {
                    path: 'admin/config',
                    name: 'AdminConfig',
                    component: () => import('@/views/AdminConfigPage.vue'),
                    meta: {title: '系统配置', requiresAdmin: true}
                },
                {
                    path: 'admin/email-config',
                    name: 'AdminEmailConfig',
                    component: () => import('@/views/AdminEmailConfigPage.vue'),
                    meta: {title: '邮箱服务配置', requiresAdmin: true}
                },
                {
                    path: 'admin/site-config',
                    name: 'AdminSiteConfig',
                    component: () => import('@/views/AdminSiteConfigPage.vue'),
                    meta: {title: '站点信息配置', requiresAdmin: true}
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
            path: '/activate-account',
            name: 'ActivateAccount',
            component: () => import('@/views/ActivateAccount.vue'),
            meta: {title: '账户激活'}
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
            path: '/book/:bookId/stats',
            name: 'BookStatsDetail',
            component: () => import('@/views/BookStatsDetail.vue'),
            meta: {title: '账簿统计', transition: 'slide-up'}
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
    } else if (to.meta.requiresAdmin && userStore.userInfo?.admin !== 1) {
        // 非管理员访问管理员页面 → 跳转首页
        next('/home')
    } else {
        next()
    }
})

export default router
