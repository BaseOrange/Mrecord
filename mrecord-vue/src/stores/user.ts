import {defineStore} from 'pinia'
import {ref, computed} from 'vue'
import type {SysUser} from '@/api'
import {encryptStorage, decryptStorage} from '@/utils/security'

export const useUserStore = defineStore('user', () => {
    // ==================== State ====================
    // 尝试解密token，如果失败则使用原始值
    const storedToken = localStorage.getItem('token') || ''
    const token = ref<string>(storedToken ? (decryptStorage<string>(storedToken, 'mrecord-token-key') || storedToken) : '')
    const userInfo = ref<SysUser | null>(
        JSON.parse(localStorage.getItem('userInfo') || 'null')
    )

    // ==================== Getters ====================
    const isLoggedIn = computed(() => !!token.value)

    // ==================== Actions ====================
    /** 设置 token */
    function setToken(newToken: string) {
        token.value = newToken
        // 对token进行简单加密存储
        localStorage.setItem('token', encryptStorage(newToken, 'mrecord-token-key'))
    }

    /** 清除 token */
    function clearToken() {
        token.value = ''
        userInfo.value = null
        localStorage.removeItem('token')
        localStorage.removeItem('userInfo')
    }

    /** 设置用户信息 */
    function setUserInfo(info: SysUser) {
        userInfo.value = info
        // 对用户信息进行加密存储（不包含敏感字段）
        const safeInfo = { ...info }
        delete safeInfo.password
        localStorage.setItem('userInfo', JSON.stringify(safeInfo))
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
