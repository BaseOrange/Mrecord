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

/** 创建/更新账簿传参 */
export interface SaveBookParams {
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
