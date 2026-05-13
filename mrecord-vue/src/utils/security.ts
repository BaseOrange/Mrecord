/**
 * 安全工具函数
 */

/**
 * 简单的密码强度检查
 * @param password 密码
 * @returns 密码强度等级 (0-4)
 */
export function checkPasswordStrength(password: string): number {
    let strength = 0
    
    // 长度检查
    if (password.length >= 8) strength++
    if (password.length >= 12) strength++
    
    // 包含数字
    if (/\d/.test(password)) strength++
    
    // 包含字母
    if (/[a-zA-Z]/.test(password)) strength++
    
    // 包含特殊字符
    if (/[^a-zA-Z0-9]/.test(password)) strength++
    
    return strength
}

/**
 * 验证邮箱格式
 * @param email 邮箱地址
 * @returns 是否为有效邮箱
 */
export function isValidEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    return emailRegex.test(email)
}

/**
 * 清理用户输入，防止XSS攻击
 * @param input 用户输入
 * @returns 清理后的字符串
 */
export function sanitizeInput(input: string): string {
    return input
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#x27;')
}

/**
 * 生成随机字符串（用于token等）
 * @param length 字符串长度
 * @returns 随机字符串
 */
export function generateRandomString(length = 32): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
    let result = ''
    for (let i = 0; i < length; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length))
    }
    return result
}

/**
 * 本地存储加密（简单异或加密，仅用于非敏感数据）
 * @param data 要存储的数据
 * @param key 加密密钥
 * @returns 加密后的字符串
 */
export function encryptStorage(data: any, key = 'mrecord-key'): string {
    const jsonStr = JSON.stringify(data)
    let encrypted = ''
    for (let i = 0; i < jsonStr.length; i++) {
        const charCode = jsonStr.charCodeAt(i) ^ key.charCodeAt(i % key.length)
        encrypted += String.fromCharCode(charCode)
    }
    return btoa(encrypted) // Base64编码
}

/**
 * 本地存储解密
 * @param encryptedData 加密的数据
 * @param key 解密密钥
 * @returns 解密后的数据
 */
export function decryptStorage<T>(encryptedData: string, key = 'mrecord-key'): T | null {
    try {
        const decoded = atob(encryptedData) // Base64解码
        let decrypted = ''
        for (let i = 0; i < decoded.length; i++) {
            const charCode = decoded.charCodeAt(i) ^ key.charCodeAt(i % key.length)
            decrypted += String.fromCharCode(charCode)
        }
        return JSON.parse(decrypted) as T
    } catch (error) {
        console.error('解密失败:', error)
        return null
    }
}

/**
 * 清除敏感数据
 * @param data 包含敏感数据的对象
 * @param sensitiveFields 敏感字段列表
 * @returns 清除敏感字段后的对象
 */
export function clearSensitiveFields<T extends Record<string, any>>(
    data: T,
    sensitiveFields: string[] = ['password', 'token', 'secret']
): Partial<T> {
    const cleaned = { ...data }
    sensitiveFields.forEach(field => {
        if (field in cleaned) {
            delete cleaned[field]
        }
    })
    return cleaned
}
