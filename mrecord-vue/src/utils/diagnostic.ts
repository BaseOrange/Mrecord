/**
 * 诊断信息采集与格式化
 *
 * 供 DiagnosticSheet 面板使用：先采集纯前端环境信息，再调用后端诊断接口拿到
 * 服务端信息，最后拼成一段可直接粘贴到 GitHub issue 的 markdown。
 *
 * 【隐私红线】月衡是记账应用，诊断信息会被用户贴到**公开**的 issue 里，因此
 * 只采集环境元数据与计数：
 * - 绝不含金额、资产/负债数值；
 * - 绝不含邮箱、昵称、密码、token 明文；
 * - 绝不含账簿名、模板条目名、备注内容；
 * - 只有用户 ID（后端按它过滤数据计数）与各类计数。
 */

import {useUserStore} from '@/stores/user'
import {getErrorLogs, type ErrorLogEntry} from '@/utils/errorLog'
import {queryDiagnostic, type DiagnosticInfo} from '@/api'

/** 前端环境信息（全部可在浏览器端直接取得，不依赖后端） */
export interface FrontendEnvInfo {
    /** 应用版本（构建期注入，如 2.1.0+abc1234） */
    appVersion: string
    /** 前端构建时间（构建期注入） */
    appBuildTime: string
    /** 部署模式：飞牛网关 / 独立部署（由 base 路径前缀判断） */
    deployMode: string
    /** API 基础路径 */
    apiBaseUrl: string
    /** User-Agent（含操作系统、浏览器内核信息） */
    userAgent: string
    /** 屏幕宽高（CSS 像素） */
    screenSize: string
    /** 设备像素比 */
    dpr: number
    /** 语言 */
    language: string
    /** 时区（月度记账涉及时区，排障关键字段） */
    timeZone: string
    /** 网络在线状态 */
    online: boolean
    /** 当前完整 URL（含网关前缀，能反映真实入口） */
    currentUrl: string
    /** localStorage 已用空间估算（KB） */
    storageUsedKb: number
    /** 登录状态 */
    loggedIn: boolean
    /** 用户 ID（不含邮箱/昵称） */
    userId: string | null
    /** 是否管理员 */
    isAdmin: boolean
}

/** 采集前端环境信息 */
export function collectFrontendEnv(): FrontendEnvInfo {
    const userStore = useUserStore()

    // 估算 localStorage 已用空间：遍历所有 key 累加长度（UTF-16 每字符约 2 字节）
    let storageBytes = 0
    try {
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i)
            if (key === null) continue
            const value = localStorage.getItem(key) || ''
            storageBytes += (key.length + value.length) * 2
        }
    } catch {
        // 隐私模式 / 禁用 localStorage 时忽略
    }

    const baseUrl = import.meta.env.BASE_URL || '/'
    return {
        appVersion: typeof __APP_VERSION__ !== 'undefined' ? __APP_VERSION__ : 'unknown',
        appBuildTime: typeof __APP_BUILD_TIME__ !== 'undefined' ? __APP_BUILD_TIME__ : 'unknown',
        // 飞牛网关把应用挂在 /app/... 前缀下（见 utils/request.ts 的 GATEWAY_MODE 判断）
        deployMode: baseUrl.startsWith('/app/') ? 'fnos-gateway' : 'standalone',
        apiBaseUrl: import.meta.env.VITE_API_BASE_URL || '/api/v2',
        userAgent: navigator.userAgent,
        screenSize: `${window.screen.width}x${window.screen.height}`,
        dpr: window.devicePixelRatio,
        language: navigator.language,
        timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone || 'unknown',
        online: navigator.onLine,
        currentUrl: window.location.href,
        storageUsedKb: Math.round(storageBytes / 1024),
        loggedIn: userStore.isLoggedIn,
        userId: userStore.userInfo?.id || null,
        isAdmin: userStore.userInfo?.admin === 1,
    }
}

/** 将秒数格式化为「x天 x小时 x分 x秒」的可读时长 */
function formatUptime(secs: number): string {
    const d = Math.floor(secs / 86400)
    const h = Math.floor((secs % 86400) / 3600)
    const m = Math.floor((secs % 3600) / 60)
    const s = secs % 60
    const parts: string[] = []
    if (d > 0) parts.push(`${d}天`)
    if (h > 0) parts.push(`${h}小时`)
    if (m > 0) parts.push(`${m}分`)
    parts.push(`${s}秒`)
    return parts.join(' ')
}

/** 将字节格式化为人类可读的文件大小 */
function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`
    const mb = bytes / (1024 * 1024)
    return `${mb.toFixed(2)} MB`
}

/** 错误日志格式化为 markdown 列表行 */
function formatErrorLog(entry: ErrorLogEntry): string {
    const time = entry.time.replace('T', ' ').replace(/\.\d+Z$/, 'Z')
    const source = entry.source ? ` \`${entry.source}\`` : ''
    const detail = entry.detail ? ` (${entry.detail})` : ''
    return `- [${time}] [${entry.type}] ${entry.message}${source}${detail}`
}

/**
 * 拼接完整诊断信息 markdown
 *
 * @param backend 后端诊断信息；接口失败时传 null，面板会标注「后端接口不可用」
 * @param errors 最近错误日志（为空时显示「无」）
 */
export function buildDiagnosticMarkdown(
    frontend: FrontendEnvInfo,
    backend: DiagnosticInfo | null,
    errors: ErrorLogEntry[],
): string {
    const lines: string[] = []
    lines.push('## 月衡 Mrecord 诊断信息')
    lines.push('')
    lines.push('> 由「我的 → 点击顶部标题 5 次」生成。本信息不含任何金额、账号与账簿内容，可放心附到 issue。')
    lines.push('')

    lines.push('### 前端')
    lines.push(`- 应用版本: \`${frontend.appVersion}\``)
    lines.push(`- 前端构建时间: \`${frontend.appBuildTime}\``)
    lines.push(`- 部署模式: \`${frontend.deployMode}\``)
    lines.push(`- API 基础路径: \`${frontend.apiBaseUrl}\``)
    lines.push(`- 操作系统/浏览器: \`${frontend.userAgent}\``)
    lines.push(`- 屏幕: \`${frontend.screenSize}\` @${frontend.dpr}x`)
    lines.push(`- 语言/时区: \`${frontend.language}\` / \`${frontend.timeZone}\``)
    lines.push(`- 网络在线: \`${frontend.online}\``)
    lines.push(`- 当前 URL: \`${frontend.currentUrl}\``)
    lines.push(`- 本地存储占用: \`${frontend.storageUsedKb} KB\``)
    lines.push(`- 登录状态: \`${frontend.loggedIn}\`${frontend.loggedIn ? ` (用户ID: \`${frontend.userId}\`${frontend.isAdmin ? ', 管理员' : ''})` : ''}`)
    lines.push('')

    lines.push('### 后端')
    if (backend) {
        // 后端类型友好显示：rust → Rust 版（fnOS），java → Java 版（Docker / 独立部署）
        const backendName = backend.backend === 'java' ? 'Java 版（Docker / 独立部署）' : 'Rust 版（fnOS）'
        lines.push(`- 后端类型: \`${backendName}\`（${backend.backend}）`)
        lines.push(`- 后端框架: \`${backend.backendFramework}\``)
        // Rust 工具链版本只有 Rust 版后端会返回
        if (backend.backend === 'rust' && backend.rustVersion) {
            lines.push(`- Rust 版本: \`${backend.rustVersion}\``)
        }
        lines.push(`- 应用版本: \`${backend.appVersion}\``)
        lines.push(`- 构建时间: \`${backend.buildTime}\``)
        lines.push(`- 已运行: \`${formatUptime(backend.uptimeSecs)}\``)
        lines.push(`- 操作系统: \`${backend.os}\` / \`${backend.arch}\``)
        lines.push(`- 部署模式: \`${backend.deployMode}\``)
        lines.push(`- 数据库: \`${backend.dbType}\`，${formatBytes(backend.dbSizeBytes)}`)
        lines.push(`- 数据规模: 账簿 ${backend.dataCounts.bookCount} 个，模板条目 ${backend.dataCounts.templateItemCount} 条，月度汇总 ${backend.dataCounts.monthRecordCount} 条，月度明细 ${backend.dataCounts.monthItemCount} 条`)
    } else {
        // 后端接口失败本身也是重要信息（可能是服务未启动或网关转发异常）
        lines.push('- ⚠️ 后端诊断接口不可用（可能服务未启动、网关转发异常或登录已过期）')
    }
    lines.push('')

    lines.push('### 最近错误日志')
    if (errors.length === 0) {
        lines.push('- 无')
    } else {
        // 倒序展示：最新的在最前
        [...errors].reverse().forEach((entry) => lines.push(formatErrorLog(entry)))
    }
    lines.push('')

    return lines.join('\n')
}

/**
 * 一键采集并拼接诊断信息（面板打开时调用）
 *
 * @returns markdown 文本；后端接口失败不影响前端部分的生成
 */
export async function collectDiagnosticText(): Promise<string> {
    const frontend = collectFrontendEnv()
    const errors = getErrorLogs()

    let backend: DiagnosticInfo | null = null
    try {
        backend = await queryDiagnostic()
    } catch {
        // 拦截器已弹错误提示；这里只标标注后端不可用，前端信息仍要可用
    }

    return buildDiagnosticMarkdown(frontend, backend, errors)
}
