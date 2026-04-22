import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface TempItemInfo {
    id: number | string
    bookId: number | string
    name: string
    type?: number
    amount?: number
    category?: string
    icon?: string
    sort?: number
    status?: number
    createTime?: string
    updateTime?: string
}

export interface CreateTempItemParams {
    bookId: number | string
    name: string
    type?: number
    amount?: number
    category?: string
    icon?: string
}

export interface UpdateTempItemParams {
    id: number | string
    bookId?: number | string
    name?: string
    type?: number
    amount?: number
    category?: string
    icon?: string
}

export interface CopyTempItemParams {
    id: number | string
    targetBookId?: number | string
}

export interface ListTempItemsParams {
    bookId?: number | string
    page?: number
    pageSize?: number
    status?: number
}

export interface ListTempItemsResult {
    list: TempItemInfo[]
    total: number
    page: number
    pageSize: number
}

// ==================== 接口方法 ====================

/** 创建账本模板项 */
export function createTempItem(data: CreateTempItemParams) {
    return post<TempItemInfo>('/tempItem/create', data)
}

/** 更新账本模板项 */
export function updateTempItem(data: UpdateTempItemParams) {
    return post<TempItemInfo>('/tempItem/update', data)
}

/** 复制账本模板项 */
export function copyTempItem(data: CopyTempItemParams) {
    return post<TempItemInfo>('/tempItem/copy', data)
}

/** 查询账本模板项列表 */
export function listTempItems(data?: ListTempItemsParams) {
    return post<ListTempItemsResult>('/tempItem/list', data)
}
