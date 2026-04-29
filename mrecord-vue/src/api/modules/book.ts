import {post} from '@/utils/request'
import type {BaseEntity, PageParams, PageResult} from '@/api/types'

// ==================== 类型定义 ====================

/** 账簿信息 */
export interface BookInfo extends BaseEntity {
    /** 操作用户ID */
    userId?: string
    /** 账簿名称 */
    bookName?: string
}

/** 账簿统计数据 */
export interface BookStatistics {
    /** 账簿ID */
    bookId?: string
    /** 账簿名称 */
    bookName?: string
    /** 所属用户ID */
    userId?: string
    /** 统计年份 */
    year?: number
    /** 统计月份 */
    month?: number
    /** 当月总资产 */
    totalAsset?: number
    /** 当月总负债 */
    totalLiability?: number
    /** 当月净资产 */
    netAsset?: number
    /** 环比增长/下跌金额 */
    monthOnMonth?: number
    /** 同比增长/下跌金额 */
    yearOnYear?: number
    /** 用户本月汇总备注 */
    note?: string
}

/** 创建/更新账簿传参 */
export interface SaveBookParams {
    /** 账簿ID（更新时必传） */
    id?: string
    /** 账簿名称 */
    bookName: string
}

/** 删除账簿传参 */
export interface DeleteBookParams {
    /** 账簿ID */
    id: string
}

/** 账簿列表查询传参 */
export interface ListBooksParams extends PageParams {
    /** 账簿名称 */
    name?: string
    /** 账簿类型 */
    type?: string
    /** 账簿年份 */
    year?: string
}

/** 指定账簿详细统计响应 */
export interface BookDetailedStatistics {
    /** 开始年月 yyyyMM */
    startYearMonth?: string
    /** 结束年月 yyyyMM */
    endYearMonth?: string
    /** 区间财务汇总数据 */
    recordList?: BookStatistics[]
}

// ==================== 接口方法 ====================

/** 创建账簿 */
export function createBook(data: SaveBookParams) {
    return post<BookInfo>('/book/create', data)
}

/** 更新账簿 */
export function updateBook(data: SaveBookParams) {
    return post<BookInfo>('/book/update', data)
}

/** 删除账簿 */
export function deleteBook(data: DeleteBookParams) {
    return post<void>('/book/delete', data)
}

/** 获取账簿列表 */
export function listBooks(data?: ListBooksParams) {
    return post<PageResult<BookInfo>>('/book/list', data)
}

/** 获取我的所有账簿统计数据 */
export function getMyDataStatistics() {
    return post<BookDetailedStatistics>('/book/getMyDataStatistics')
}

/** 获取指定账簿详细统计数据 */
export function getBookDetailedStatistics(data: DeleteBookParams) {
    return post<BookDetailedStatistics>('/book/getBookDetailedStatistics', data)
}
