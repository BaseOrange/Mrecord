import {post} from '@/utils/request'

// ==================== 类型定义 ====================

/** 当前用户的数据规模计数（只有计数，不含金额） */
export interface DiagnosticDataCounts {
    /** 账簿数量 */
    bookCount: number
    /** 模板条目数量 */
    templateItemCount: number
    /** 月度汇总记录数量 */
    monthRecordCount: number
    /** 月度明细记录数量 */
    monthItemCount: number
}

/** 后端诊断信息（全部为环境元数据与计数，不含业务/敏感数据） */
export interface DiagnosticInfo {
    /** 后端实现类型：rust（fnOS 版） / java（Docker 版） */
    backend: string
    /** 后端框架，如 "axum 0.8" / "Spring Boot 3.x" */
    backendFramework: string
    /** 后端应用版本（与 manifest / Cargo.toml / pom.xml 一致） */
    appVersion: string
    /** Rust 工具链版本 */
    rustVersion: string
    /** 后端二进制构建时间（UTC） */
    buildTime: string
    /** 后端进程已运行时长（秒） */
    uptimeSecs: number
    /** 宿主操作系统 */
    os: string
    /** CPU 架构 */
    arch: string
    /** 部署模式：fnos-gateway (前缀) 或 standalone */
    deployMode: string
    /** 数据库类型 */
    dbType: string
    /** 数据库文件大小（字节） */
    dbSizeBytes: number
    /** 当前用户数据规模计数 */
    dataCounts: DiagnosticDataCounts
}

// ==================== 接口 ====================

/**
 * 查询诊断信息
 *
 * 供「我的 → 点击顶部标题 5 次」诊断面板调用。需要登录（按当前用户过滤数据计数）。
 */
export function queryDiagnostic() {
    return post<DiagnosticInfo>('/diagnostic/query')
}
