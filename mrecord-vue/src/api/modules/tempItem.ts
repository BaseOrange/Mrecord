import {post} from '@/utils/request'
import type {BaseEntity} from '@/api/types'

// ==================== 类型定义 ====================

/** 账本模板项 */
export interface FinTemplateItem extends BaseEntity {
    /** 所属账簿ID */
    bookId?: string
    /** 记账项名称，如招行储蓄卡、花呗 */
    itemName?: string
    /** 账簿类型：-1负债，0不统计仅记录，1资产 */
    itemType?: number
    /** 图标标识 */
    icon?: string
    /** 展示排序号，数值越小越靠前 */
    sort?: string
}

/** 创建/更新/复制/查询模板项传参 */
export interface SaveTempItemParams {
    /** 账本ID */
    bookId?: string
    /** 旧账簿ID */
    oldBookId?: string
    /** 账本模板项列表 */
    itemList?: FinTemplateItem[]
}

/** 删除模板项传参 */
export interface DeleteTempItemParams {
    /** 账簿ID */
    bookId?: string
    /** 模板项ID */
    templateItemId?: string
}

// ==================== 接口方法 ====================

/** 创建账本模板项 */
export function createTempItem(data: SaveTempItemParams) {
    return post<FinTemplateItem[]>('/tempItem/create', data)
}

/** 更新账本模板项 */
export function updateTempItem(data: SaveTempItemParams) {
    return post<FinTemplateItem[]>('/tempItem/update', data)
}

/** 复制账本模板项 */
export function copyTempItem(data: SaveTempItemParams) {
    return post<FinTemplateItem[]>('/tempItem/copy', data)
}

/** 查询账本模板项列表 */
export function listTempItems(data?: SaveTempItemParams) {
    return post<FinTemplateItem[]>('/tempItem/list', data)
}

/**
 * 删除账本模板项
 *
 * 业务规则：仅允许删除「尚无任何月份记账记录」的模板项；已有记录的科目
 * 后端返回 14306（为保护历史快照），调用方据此给出提示。
 */
export function deleteTempItem(data: DeleteTempItemParams) {
    return post<void>('/tempItem/delete', data)
}
