/**
 * 格式化工具函数
 */

/**
 * 格式化金额为千分位字符串。
 *
 * 领域字段（netAsset / totalAsset 等）在类型上全部可选，直接
 * `val.toLocaleString()` 在 undefined / null 时会抛 TypeError——这正是
 * 各页面不敢复用、各自重写一份空值保护的根因（Q3）。
 *
 * @param val 金额数值；undefined / null / NaN 时返回 '--'
 * @param decimals 小数位数，默认2位
 * @returns 格式化后的字符串
 */
export function formatMoney(val?: number | null, decimals = 2): string {
    if (val === undefined || val === null || Number.isNaN(val)) return '--'
    return val.toLocaleString('zh-CN', {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals
    })
}

/**
 * 将金额按后端 `round_money` 规则舍入到分（两位小数、HALF_UP 远离零）。
 *
 * 对齐 Rust `common/money.rs::round_money`（`round_dp_with_strategy(2, MidpointAwayFromZero)`）
 * 与 Java `BigDecimal.setScale(2, RoundingMode.HALF_UP)`。前端在解析输入、汇总、提交前
 * 统一舍入，避免 `0.1 + 0.2` 类浮点误差导致前端汇总与后端落库值不一致（B7）。
 *
 * @param val 金额数值
 * @returns 舍入到两位小数的数值（-0 规整为 0）
 */
export function roundMoney(val: number): number {
    if (val === 0) return 0
    return Math.sign(val) * Math.round(Math.abs(val) * 100 + Number.EPSILON * 100) / 100
}

/**
 * 获取变化值（环比/同比、净资产增减）的展示颜色。
 *
 * 全站统一「正红负绿」：上涨为红（#ff3b30）、下跌为绿（#34c759），与 A 股等
 * 中式金融惯例一致；0 / undefined / null / NaN 为中性灰（#8e8e93）。
 * 该方向为明确的产品决策（D1），各页不得再自带相反的本地实现。
 *
 * @param val 变化值
 * @returns 颜色值
 */
export function getChangeColor(val?: number | null): string {
    if (val === undefined || val === null || Number.isNaN(val) || val === 0) return '#8e8e93'
    return val > 0 ? '#ff3b30' : '#34c759'
}

/**
 * 获取变化值的前缀符号
 * @param val 变化值
 * @returns 前缀符号
 */
export function getChangePrefix(val: number): string {
    if (val === 0) return ''
    return val > 0 ? '+' : ''
}

/**
 * 格式化环比 / 同比增长率为展示文本。
 *
 * 后端 monthOnMonth / yearOnYear 返回的是**百分比**值（`(本月-上月)/|上月|*100`，
 * 见 B6），本函数统一为：空值 → '--'；0 → '持平'；其余 → 带 '+' 的两位小数百分比。
 *
 * @param val 百分比数值
 * @returns 展示文本
 */
export function getChangeText(val?: number | null): string {
    if (val === undefined || val === null) return '--'
    if (val === 0) return '持平'
    return (val > 0 ? '+' : '') + val.toFixed(2) + '%'
}

/**
 * 格式化日期
 * @param date 日期对象或时间戳
 * @param format 格式化模板，默认 'YYYY-MM-DD HH:mm:ss'
 * @returns 格式化后的日期字符串
 */
/**
 * 将后端返回的时间值解析为 Date。
 *
 * 后端（Rust `model/*.rs` 的各 DTO、Java）把时间序列化为 `"2024-01-01 00:00:00"` 这类
 * **空格分隔**的字符串；iOS Safari 的 `new Date("2024-01-01 00:00:00")` 不接受该格式，
 * 会得到 Invalid Date，页面渲染出 "NaN-NaN-NaN"（Q4）。这里把开头的空格替换为 ISO 8601
 * 的 `T` 后再解析，同时兼容时间戳数字、含时区的 ISO 字符串与已是 Date 的入参。
 *
 * @param date 时间值；null/undefined 返回 Invalid Date，formatDate 会渲染为占位符
 * @returns Date 对象；无法解析时为 Invalid Date（调用方可按需判断）
 */
export function parseDate(date: Date | number | string | null | undefined): Date {
    if (date === null || date === undefined) return new Date(NaN)
    if (date instanceof Date) return date
    if (typeof date === 'number') return new Date(date)
    // 仅替换「日期 时间」之间的那个空格，避免误伤其他位
    return new Date(String(date).replace(/^(\d{4}-\d{1,2}-\d{1,2})\s+/, '$1T'))
}

export function formatDate(date: Date | number | string | null | undefined, format = 'YYYY-MM-DD HH:mm:ss'): string {
    const d = parseDate(date)
    // 解析失败时返回占位符，而不是把 "NaN-NaN-NaN" 渲染到界面上（Q4）
    if (isNaN(d.getTime())) return '-'
    
    const year = d.getFullYear()
    const month = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const hours = String(d.getHours()).padStart(2, '0')
    const minutes = String(d.getMinutes()).padStart(2, '0')
    const seconds = String(d.getSeconds()).padStart(2, '0')
    
    return format
        .replace('YYYY', String(year))
        .replace('MM', month)
        .replace('DD', day)
        .replace('HH', hours)
        .replace('mm', minutes)
        .replace('ss', seconds)
}

/**
 * 防抖函数
 * @param fn 要防抖的函数
 * @param delay 延迟时间（毫秒）
 * @returns 防抖后的函数
 */
export function debounce<T extends (...args: any[]) => any>(fn: T, delay = 300): (...args: Parameters<T>) => void {
    let timer: ReturnType<typeof setTimeout> | null = null
    
    return function(this: ThisParameterType<T>, ...args: Parameters<T>) {
        if (timer) clearTimeout(timer)
        timer = setTimeout(() => {
            fn.apply(this, args)
        }, delay)
    }
}

/**
 * 节流函数
 * @param fn 要节流的函数
 * @param interval 间隔时间（毫秒）
 * @returns 节流后的函数
 */
export function throttle<T extends (...args: any[]) => any>(fn: T, interval = 300): (...args: Parameters<T>) => void {
    let lastTime = 0
    
    return function(this: ThisParameterType<T>, ...args: Parameters<T>) {
        const now = Date.now()
        if (now - lastTime >= interval) {
            lastTime = now
            fn.apply(this, args)
        }
    }
}
