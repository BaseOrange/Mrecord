declare module '@varlet/ui/es/style' {
}

// Vite 环境变量类型声明
interface ImportMetaEnv {
    readonly VITE_API_BASE_URL: string
    readonly VITE_APP_TITLE: string
    readonly VITE_PROXY_TARGET?: string
    readonly VITE_API_URL?: string
}

interface ImportMeta {
    readonly env: ImportMetaEnv
}
