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
