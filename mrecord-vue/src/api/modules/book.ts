import {post} from '@/utils/request'

// ==================== 类型定义 ====================

export interface BookInfo {
    id: number | string
    name: string
    description?: string
    icon?: string
    color?: string
    currency?: string
    sort?: number
    status?: number
    createTime?: string
    updateTime?: string
}

export interface CreateBookParams {
    name: string
    description?: string
    icon?: string
    color?: string
    currency?: string
}

export interface UpdateBookParams {
    id: number | string
    name?: string
    description?: string
    icon?: string
    color?: string
    currency?: string
}

export interface DeleteBookParams {
    id: number | string
}

export interface ListBooksParams {
    page?: number
    pageSize?: number
    status?: number
}

export interface ListBooksResult {
    list: BookInfo[]
    total: number
    page: number
    pageSize: number
}

// ==================== 接口方法 ====================

/** 创建账簿 */
export function createBook(data: CreateBookParams) {
    return post<BookInfo>('/book/create', data)
}

/** 更新账簿 */
export function updateBook(data: UpdateBookParams) {
    return post<BookInfo>('/book/update', data)
}

/** 删除账簿 */
export function deleteBook(data: DeleteBookParams) {
    return post<void>('/book/delete', data)
}

/** 获取账簿列表 */
export function listBooks(data?: ListBooksParams) {
    return post<ListBooksResult>('/book/list', data)
}
