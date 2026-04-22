import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface MonthItemInfo {
    id: number | string
    bookId: number | string
    tempItemId?: number | string
    month: string
    year: number
    monthNum: number
    name: string
    type: number
    amount: number
    category?: string
    icon?: string
    remark?: string
    sort?: number
    status?: number
    createTime?: string
    updateTime?: string
}

export interface InsertMonthItemParams {
    bookId: number | string
    tempItemId?: number | string
    month: string
    name?: string
    type?: number
    amount?: number
    category?: string
    icon?: string
    remark?: string
}

export interface UpdateMonthItemParams {
    id: number | string
    month?: string
    name?: string
    type?: number
    amount?: number
    category?: string
    icon?: string
    remark?: string
}

export interface QueryMonthItemParams {
    id: number | string
}

export interface QueryAllMonthItemsParams {
    bookId?: number | string
    month?: string
    year?: number
    type?: number
    page?: number
    pageSize?: number
}

export interface QueryAllMonthItemsResult {
    list: MonthItemInfo[]
    total: number
    page: number
    pageSize: number
}

// ==================== 接口方法 ====================

/** 插入月度财务账目 */
export function insertMonthItem(data: InsertMonthItemParams) {
    return post<MonthItemInfo>('/monthItem/insertMonthItem', data)
}

/** 更新月度财务账目 */
export function updateMonthItem(data: UpdateMonthItemParams) {
    return post<MonthItemInfo>('/monthItem/updateMonthItem', data)
}

/** 查询月度财务账目 */
export function queryMonthItem(data: QueryMonthItemParams) {
    return post<MonthItemInfo>('/monthItem/queryMonthItem', data)
}

/** 查询所有月度财务账目 */
export function queryAllMonthItems(data?: QueryAllMonthItemsParams) {
    return post<QueryAllMonthItemsResult>('/monthItem/queryAll', data)
}
