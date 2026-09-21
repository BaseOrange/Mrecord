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
export function formatDate(date: Date | number | string, format = 'YYYY-MM-DD HH:mm:ss'): string {
    const d = new Date(date)
    
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
