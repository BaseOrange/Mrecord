import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface ConfigInfo {
    key: string
    value: string
    description?: string
    updateTime?: string
}

export interface EmailConfig {
    hostName: string
    sslSmtpPort: number
    smtpPort: number
    ssl: boolean
    username: string
    password: string
    from: string
}

export interface UpdateEmailConfigParams {
    hostName: string
    sslSmtpPort: number
    smtpPort: number
    ssl: boolean
    userName: string
    password: string
    from: string
}

export interface SiteConfig {
    webSite: string
    adminMail: string
    registerEnabled: boolean
}

// ==================== 接口方法 ====================

/** 刷新配置项缓存 */
export function refreshCache() {
    return post<void>('/config/refreshCache')
}

/** 获取是否开启注册功能 */
export function getRegisterEnabled() {
    return post<boolean>('/config/registerEnabled')
}

/** 获取邮件配置 */
export function getEmailConfig() {
    return post<EmailConfig | null>('/config/getEmailConfig')
}

/** 获取站点配置 */
export function getSiteConfig() {
    return post<SiteConfig>('/config/getSiteConfig')
}

/** 修改邮件配置 */
export function updateEmailConfig(data: UpdateEmailConfigParams) {
    return post<void>('/config/updateEmailConfig', data)
}

/** 修改站点配置 */
export function updateSiteConfig(data: SiteConfig) {
    return post<void>('/config/updateSiteConfig', data)
}
