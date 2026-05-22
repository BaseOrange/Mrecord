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

export interface InitAdminParams {
    email: string
    password: string
    nickname: string
}

export interface TestEmailParams {
    hostName: string
    sslSmtpPort: number
    smtpPort: number
    ssl: boolean
    userName: string
    password: string
    from: string
    testTo: string
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

/** 检查系统是否已初始化 */
export function checkInitialized() {
    return post<boolean>('/config/initialized')
}

/** 初始化管理员账户 */
export function initAdmin(data: InitAdminParams) {
    return post<string>('/config/initAdmin', data)
}

/** 发送测试邮件 */
export function testEmail(data: TestEmailParams) {
    return post<void>('/config/testEmail', data)
}
