import {post} from '@/utils/request'
import type {BaseEntity} from '@/api/types'

// ==================== 类型定义 ====================

/** 月度财务汇总记录 */
export interface FinMonthRecord extends BaseEntity {
    /** 所属用户ID */
    userId?: string
    /** 所属账簿ID */
    bookId?: string
    /** 统计年份 */
    year?: number
    /** 统计月份 */
    month?: number
    /** 当月总资产 */
    totalAsset?: number
    /** 当月总负债 */
    totalLiability?: number
    /** 当月净资产（总资产-总负债） */
    netAsset?: number
    /** 环比增长/下跌金额（对比上月） */
    monthOnMonth?: number
    /** 同比增长/下跌金额（对比去年同月） */
    yearOnYear?: number
    /** 用户本月汇总备注 */
    note?: string
}

/** 月度/年度汇总查询传参 */
export interface MonthRecordParams {
    /** 账簿ID */
    bookId?: string
    /** 年份 */
    year?: number
    /** 月份 */
    month?: number
    /** 备注 */
    note?: string
}

// ==================== 接口方法 ====================

/** 获取月度财务汇总 */
export function getMonthRecord(data: MonthRecordParams) {
    return post<FinMonthRecord>('/monthRecord/getMonthRecord', data)
}

/** 获取年度财务汇总列表 */
export function getYearRecordList(data: MonthRecordParams) {
    return post<FinMonthRecord[]>('/monthRecord/getYearRecordList', data)
}
