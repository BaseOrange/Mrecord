import { ref, onUnmounted } from 'vue'

/**
 * 邮件 / 验证码接口的冷却倒计时（I13）。
 *
 * 忘记密码、重发激活邮件这类接口后端通常有发送频率限制，前端在发送成功后锁定按钮
 * 并显示剩余秒数，防止用户连点触发「请求过于频繁」或刷出大量邮件。倒计时随组件
 * 卸载自动清理，不泄漏定时器。
 *
 * @param seconds 冷却总时长（秒），默认 60
 * @returns `remaining` 剩余秒数（0 表示可再次发送）；`start` 开始倒计时
 */
export function useCountdown(seconds = 60) {
    const remaining = ref(0)
    let timer: ReturnType<typeof setInterval> | null = null

    const clear = () => {
        if (timer) {
            clearInterval(timer)
            timer = null
        }
    }

    const start = () => {
        clear()
        remaining.value = seconds
        timer = setInterval(() => {
            remaining.value--
            if (remaining.value <= 0) {
                remaining.value = 0
                clear()
            }
        }, 1000)
    }

    onUnmounted(clear)

    return { remaining, start }
}
