import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface MonthRecordInfo {
    month: string
    year: number
    monthNum: number
    bookId?: number | string
    totalIncome: number
    totalExpense: number
    totalBalance: number
    itemCount: number
    categoryStats?: Record<string, number>
}

export interface YearRecordItem {
    month: string
    year: number
    monthNum: number
    totalIncome: number
    totalExpense: number
    totalBalance: number
}

export interface GetMonthRecordParams {
    bookId?: number | string
    month: string
}

export interface GetYearRecordListParams {
    bookId?: number | string
    year: number
}

export interface GetYearRecordListResult {
    list: YearRecordItem[]
    year: number
    totalIncome: number
    totalExpense: number
    totalBalance: number
}

// ==================== 接口方法 ====================

/** 获取月度财务汇总 */
export function getMonthRecord(data: GetMonthRecordParams) {
    return post<MonthRecordInfo>('/monthRecord/getMonthRecord', data)
}

/** 获取年度财务汇总列表 */
export function getYearRecordList(data: GetYearRecordListParams) {
    return post<GetYearRecordListResult>('/monthRecord/getYearRecordList', data)
}
