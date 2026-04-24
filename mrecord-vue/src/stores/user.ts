import {defineStore} from 'pinia'
import {ref, computed} from 'vue'
import type {SysUser} from '@/api'

export const useUserStore = defineStore('user', () => {
    // ==================== State ====================
    const token = ref<string>(localStorage.getItem('token') || '')
    const userInfo = ref<SysUser | null>(null)

    // ==================== Getters ====================
    const isLoggedIn = computed(() => !!token.value)

    // ==================== Actions ====================
    /** 设置 token */
    function setToken(newToken: string) {
        token.value = newToken
        localStorage.setItem('token', newToken)
    }

    /** 清除 token */
    function clearToken() {
        token.value = ''
        userInfo.value = null
        localStorage.removeItem('token')
    }

    /** 设置用户信息 */
    function setUserInfo(info: SysUser) {
        userInfo.value = info
    }

    /** 登出 */
    function logout() {
        clearToken()
    }

    return {
        token,
        userInfo,
        isLoggedIn,
        setToken,
        clearToken,
        setUserInfo,
        logout,
    }
})
