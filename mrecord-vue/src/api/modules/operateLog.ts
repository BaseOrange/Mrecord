import {post} from '@/utils/request'
import type {BaseEntity, PageParams, PageResult} from '@/api/types'

// ==================== 类型定义 ====================

/** 操作审计日志 */
export interface OperateLogInfo extends BaseEntity {
    /** 操作用户ID */
    userId?: string
    /** 操作类型（LOGIN-登录，LOGOUT-登出，UPDATE-数据修改，EXPORT-导出，CANCEL-注销/撤销注销，RESET_PWD-密码重置） */
    operateType?: string
    /** 操作内容详细描述 */
    content?: string
    /** 操作IP地址 */
    ip?: string
}

// ==================== 接口方法 ====================

/** 分页查询用户操作审计日志列表 */
export function listOperateLogs(data?: PageParams) {
    return post<PageResult<OperateLogInfo>>('/operateLog/list', data)
}
