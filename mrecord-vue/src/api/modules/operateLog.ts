import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface OperateLogInfo {
    id: number | string
    userId?: number | string
    username?: string
    module?: string
    action?: string
    description?: string
    ip?: string
    params?: string
    result?: string
    status?: number
    duration?: number
    createTime?: string
}

export interface ListOperateLogsParams {
    page?: number
    pageSize?: number
    userId?: number | string
    module?: string
    action?: string
    startTime?: string
    endTime?: string
}

export interface ListOperateLogsResult {
    list: OperateLogInfo[]
    total: number
    page: number
    pageSize: number
}

// ==================== 接口方法 ====================

/** 分页查询用户操作审计日志列表 */
export function listOperateLogs(data?: ListOperateLogsParams) {
    return post<ListOperateLogsResult>('/operateLog/list', data)
}
