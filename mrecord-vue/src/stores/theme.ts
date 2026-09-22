import { ref, computed, watch } from 'vue'
import { defineStore } from 'pinia'

/**
 * 主题模式（Q7 → C 三态）
 *
 * - mode：用户选择，持久化到 localStorage
 * - resolved：实际生效的主题；「跟随系统」时由 matchMedia 实时解析
 *
 * 通过 document.documentElement.dataset.theme 应用主题，供 themes/*.css
 * 的 [data-theme] 选择器覆盖令牌；同时设 color-scheme 让原生控件
 * （滚动条、输入框、alert）跟随。
 */
export type ThemeMode = 'system' | 'light' | 'dark'
export type ResolvedTheme = 'light' | 'dark'

const STORAGE_KEY = 'mrecord-theme-mode'
const VALID_MODES: ThemeMode[] = ['system', 'light', 'dark']

function readStoredMode(): ThemeMode {
    try {
        const raw = localStorage.getItem(STORAGE_KEY)
        if (raw && VALID_MODES.includes(raw as ThemeMode)) return raw as ThemeMode
    } catch {
        // 隐私模式等无 localStorage 场景，回落默认值
    }
    return 'system'
}

function readSystemTheme(): ResolvedTheme {
    if (typeof window === 'undefined' || !window.matchMedia) return 'light'
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

export const useThemeStore = defineStore('theme', () => {
    const mode = ref<ThemeMode>(readStoredMode())
    const systemTheme = ref<ResolvedTheme>(readSystemTheme())

    const resolved = computed<ResolvedTheme>(() =>
        mode.value === 'system' ? systemTheme.value : mode.value,
    )

    /** 写入 DOM：data-theme 供 CSS 覆盖令牌，color-scheme 驱动原生控件 */
    function apply(theme: ResolvedTheme) {
        if (typeof document === 'undefined') return
        document.documentElement.dataset.theme = theme
        document.documentElement.style.colorScheme = theme
    }

    function setMode(next: ThemeMode) {
        mode.value = next
        try {
            localStorage.setItem(STORAGE_KEY, next)
        } catch {
            // 持久化失败不影响当次使用
        }
    }

    // resolved 变化（用户切换或系统切换）即时应用
    watch(resolved, (theme) => apply(theme), { immediate: true })

    // 监听系统主题变化（仅在「跟随系统」时影响 resolved）
    if (typeof window !== 'undefined' && window.matchMedia) {
        const media = window.matchMedia('(prefers-color-scheme: dark)')
        const onChange = (e: MediaQueryListEvent) => {
            systemTheme.value = e.matches ? 'dark' : 'light'
        }
        // addEventListener 在 Safari 14+ 可用，回落 addListener 兼容更早版本
        if (typeof media.addEventListener === 'function') {
            media.addEventListener('change', onChange)
        } else if (typeof (media as any).addListener === 'function') {
            ;(media as any).addListener(onChange)
        }
    }

    return { mode, resolved, setMode }
})
