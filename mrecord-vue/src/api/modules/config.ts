import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface ConfigInfo {
    key: string
    value: string
    description?: string
    updateTime?: string
}

// ==================== 接口方法 ====================

/** 刷新配置项缓存 */
export function refreshCache() {
    return post<void>('/config/refreshCache')
}
