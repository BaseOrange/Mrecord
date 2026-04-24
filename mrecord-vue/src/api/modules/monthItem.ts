import {post} from '@/utils/request'
import type {BaseEntity} from '@/api/types'

// ==================== 类型定义 ====================

/** 月度记账明细记录 */
export interface FinMonthItemRecord extends BaseEntity {
    /** 统计年份 */
    year?: number
    /** 统计月份 */
    month?: number
    /** 关联账簿ID */
    bookId?: string
    /** 关联模板项ID */
    templateItemId?: string
    /** 当月该记账项实际金额 */
    itemValue?: number
}

/** 月度账目操作传参（插入/更新/查询/查询所有共用） */
export interface MonthItemParams {
    /** 账簿ID */
    bookId?: string
    /** 年份 */
    year?: number
    /** 月份 */
    month?: number
    /** 明细列表 */
    itemList?: FinMonthItemRecord[]
    /** 备注 */
    note?: string
}

// ==================== 接口方法 ====================

/** 插入月度财务账目 */
export function insertMonthItem(data: MonthItemParams) {
    return post<FinMonthItemRecord[]>('/monthItem/insertMonthItem', data)
}

/** 更新月度财务账目 */
export function updateMonthItem(data: MonthItemParams) {
    return post<FinMonthItemRecord[]>('/monthItem/updateMonthItem', data)
}

/** 查询月度财务账目 */
export function queryMonthItem(data: MonthItemParams) {
    return post<FinMonthItemRecord[]>('/monthItem/queryMonthItem', data)
}

/** 查询所有月度财务账目，key为模板项ID */
export function queryAllMonthItems(data?: MonthItemParams) {
    return post<Record<string, FinMonthItemRecord[]>>('/monthItem/queryAll', data)
}
