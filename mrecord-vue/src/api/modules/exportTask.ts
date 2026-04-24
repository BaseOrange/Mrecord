import {post} from '@/utils/request'
import type {BaseEntity, PageParams, PageResult} from '@/api/types'

// ==================== 类型定义 ====================

/** 导出任务记录 */
export interface ExportTaskInfo extends BaseEntity {
    /** 操作用户ID */
    userId?: string
    /** 导出账簿ID */
    bookId?: string
    /** 账簿类型（YEARLY-年度，CATEGORY-分类） */
    bookType?: string
    /** 导出开始年月，格式yyyyMM */
    startYearMonth?: string
    /** 导出结束年月，格式yyyyMM */
    endYearMonth?: string
    /** 任务状态（WAIT-待执行，RUN-执行中，SUCCESS-成功，FAIL-失败） */
    status?: string
    /** 生成的Excel文件名 */
    fileName?: string
    /** 任务失败原因 */
    failReason?: string
}

/** 发起导出任务传参 */
export interface ExportBookDataParams {
    /** 账簿ID，不传则导出全部账簿 */
    bookId?: string
    /** 导出起始年月，格式yyyyMM */
    startYearMonth?: string
    /** 导出结束年月，格式yyyyMM */
    endYearMonth?: string
}

// ==================== 接口方法 ====================

/** 发起账簿数据导出任务 */
export function exportBookData(data?: ExportBookDataParams) {
    return post<ExportTaskInfo>('/exportTask/export', data)
}

/** 查询当前用户的导出任务列表 */
export function listExportTasks(data?: PageParams) {
    return post<PageResult<ExportTaskInfo>>('/exportTask/list', data)
}
