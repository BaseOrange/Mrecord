/**
 * 全局错误日志环形缓冲区
 *
 * 供「我的 → 点击顶部标题 5 次」诊断面板展示，帮助用户在 GitHub 提 issue 时
 * 附上最近的报错上下文。只保留最近 N 条，内存占用恒定。
 *
 * 记录来源：
 * - `main.ts` 的全局 error / unhandledrejection / Vue errorHandler / router.onError
 * - `utils/request.ts` 的 axios 响应拦截器失败分支（失败的业务接口）
 *
 * 【隐私】不记录任何请求体、token、金额；HTTP 失败只记 URL、状态码与业务码。
 */

/** 单条错误日志 */
export interface ErrorLogEntry {
    /** ISO 时间戳 */
    time: string
    /** 类型：js-error / unhandledrejection / vue / router / http */
    type: string
    /** 错误消息 */
    message: string
    /** 来源：脚本文件、接口 URL 或组件名 */
    source?: string
    /** 附加信息：HTTP 状态码、业务 code、行列号等 */
    detail?: string
}

/** 缓冲区容量：最近 10 条足够定位问题，又不会让诊断面板一屏放不下 */
const MAX_ENTRIES = 10

const entries: ErrorLogEntry[] = []

/**
 * 追加一条错误日志。超出容量时丢弃最早的一条（FIFO 环形）。
 *
 * @param entry 不含时间戳的日志内容，时间戳由本函数补齐
 */
export function pushErrorLog(entry: Omit<ErrorLogEntry, 'time'>): void {
    entries.push({time: new Date().toISOString(), ...entry})
    while (entries.length > MAX_ENTRIES) {
        entries.shift()
    }
}

/** 读取当前缓冲区副本（诊断面板展示用，返回新数组避免外部篡改内部状态） */
export function getErrorLogs(): ErrorLogEntry[] {
    return [...entries]
}

/** 清空缓冲区（诊断面板提供「清空」按钮时使用） */
export function clearErrorLogs(): void {
    entries.length = 0
}
