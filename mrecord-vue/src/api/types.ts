// ==================== 通用基础类型 ====================

/** 审计字段基类 */
export interface BaseEntity {
    /** 主键 */
    id?: string
    /** 创建人 */
    createBy?: string
    /** 创建时间 */
    createTime?: string
    /** 更新人 */
    updateBy?: string
    /** 更新时间 */
    updateTime?: string
}

/** 通用分页查询参数 */
export interface PageParams {
    /** 页码，默认1 */
    pageNum?: number
    /** 每页数量，默认10 */
    pageSize?: number
}

/** 通用分页响应对象 */
export interface PageResult<T> {
    /** 当前页数据 */
    records: T[]
    /** 当前页码 */
    pageNumber: number
    /** 每页数据数量 */
    pageSize: number
    /** 总页数 */
    totalPage: number
    /** 总数据数量 */
    totalRow: number
}
