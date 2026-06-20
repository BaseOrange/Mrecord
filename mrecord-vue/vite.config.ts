import {defineConfig, loadEnv, type Plugin} from 'vite'
import vue from '@vitejs/plugin-vue'
import components from 'unplugin-vue-components/vite'
import autoImport from 'unplugin-auto-import/vite'
import {VarletImportResolver} from '@varlet/import-resolver'
import {resolve} from 'path'

/**
 * 修复 Rolldown 1.0.3 的 bug：代码分割时生成 init_reactivity_esm_bundler() 调用
 * 但丢失其函数定义。此插件在后处理阶段为包含该调用的 chunk 注入空实现。
 */
function fixReactivityBundlerInit(): Plugin {
    return {
        name: 'fix-reactivity-bundler-init',
        apply: 'build',
        generateBundle(_options, bundle) {
            for (const chunk of Object.values(bundle)) {
                if (chunk.type === 'chunk' && chunk.code.includes('init_reactivity_esm_bundler()')) {
                    chunk.code = 'var init_reactivity_esm_bundler=function(){};' + chunk.code
                    console.log(`  [fix] injected init_reactivity_esm_bundler into ${chunk.fileName}`)
                }
            }
        },
    }
}

// https://vite.dev/config/
export default defineConfig(({mode}) => {
    // 加载环境变量
    const env = loadEnv(mode, process.cwd(), '')

    return {
        // 显式声明 base，确保构建产物路径绝对正确
        base: '/',
        build: {
            // 直接输出到 Rust 后端 static 目录，避免手动复制
            outDir: '../mrecord-rust/static',
            emptyOutDir: true,
            chunkSizeWarningLimit: 700,
            rollupOptions: {
                output: {
                    manualChunks(id) {
                        if (id.includes('node_modules/chart.js')) return 'vendor-chart'
                        if (id.includes('node_modules/@varlet/ui')) return 'vendor-varlet'
                        if (id.includes('node_modules/vue-router')) return 'vendor-router'
                        if (id.includes('node_modules/vuedraggable')) return 'vendor-draggable'
                    }
                }
            }
        },
        plugins: [
            vue(),
            components({
                resolvers: [VarletImportResolver()]
            }),
            autoImport({
                resolvers: [VarletImportResolver({autoImport: true})]
            }),
            fixReactivityBundlerInit(),
        ],
        resolve: {
            alias: {
                '@': resolve(__dirname, 'src')
            }
        },
        server: {
            proxy: {
                '/api': {
                    target: env.VITE_PROXY_TARGET || 'http://127.0.0.1:2333',
                    changeOrigin: true,
                    secure: false,
                }
            }
        }
    }
})
