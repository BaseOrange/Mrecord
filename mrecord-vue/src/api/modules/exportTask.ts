import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface ExportTaskInfo {
    id: number | string
    bookId?: number | string
    fileName?: string
    fileUrl?: string
    status: number
    statusText?: string
    createTime?: string
    finishTime?: string
}

export interface ExportBookDataParams {
    bookId?: number | string
    month?: string
    year?: number
    format?: string
}

export interface ListExportTasksParams {
    page?: number
    pageSize?: number
    status?: number
}

export interface ListExportTasksResult {
    list: ExportTaskInfo[]
    total: number
    page: number
    pageSize: number
}

// ==================== 接口方法 ====================

/** 发起账簿数据导出任务 */
export function exportBookData(data?: ExportBookDataParams) {
    return post<ExportTaskInfo>('/exportTask/export', data)
}

/** 查询当前用户的导出任务列表 */
export function listExportTasks(data?: ListExportTasksParams) {
    return post<ListExportTasksResult>('/exportTask/list', data)
}
