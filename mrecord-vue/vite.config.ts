import {defineConfig, loadEnv} from 'vite'
import vue from '@vitejs/plugin-vue'
import components from 'unplugin-vue-components/vite'
import autoImport from 'unplugin-auto-import/vite'
import {VarletImportResolver} from '@varlet/import-resolver'
import {resolve} from 'path'

// https://vite.dev/config/
export default defineConfig(({mode}) => {
    // 加载环境变量
    const env = loadEnv(mode, process.cwd(), '')
    
    return {
        plugins: [
            vue(),
            components({
                resolvers: [VarletImportResolver()]
            }),
            autoImport({
                resolvers: [VarletImportResolver({autoImport: true})]
            })
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
