/**
 * 格式化工具函数
 */

/**
 * 格式化金额为千分位字符串
 * @param val 金额数值
 * @param decimals 小数位数，默认2位
 * @returns 格式化后的字符串
 */
export function formatMoney(val: number, decimals = 2): string {
    return val.toLocaleString('zh-CN', {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals
    })
}

/**
 * 获取变化值的颜色
 * @param val 变化值
 * @returns 颜色值
 */
export function getChangeColor(val: number): string {
    if (val === 0) return '#8e8e93'
    return val > 0 ? '#34c759' : '#ff3b30'
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
