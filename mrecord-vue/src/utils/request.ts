import axios, {
    AxiosError, type AxiosInstance, type AxiosRequestConfig, type AxiosResponse,
    type InternalAxiosRequestConfig
} from 'axios'
import {Snackbar} from '@varlet/ui'
import {useUserStore} from '@/stores/user'

// 创建 axios 实例
const request: AxiosInstance = axios.create({
    baseURL: '/api/v2',
    timeout: 15000,
    headers: {
        'Content-Type': 'application/json',
    },
})

// 请求拦截器
request.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
        const userStore = useUserStore()
        if (userStore.token && config.headers) {
            config.headers.Authorization = `Bearer ${userStore.token}`
        }
        return config
    },
    (error: AxiosError) => {
        return Promise.reject(error)
    }
)

// 后端统一响应格式
interface Result<T = unknown> {
    code: string
    message: string
    data: T
}

// 业务成功码
const SUCCESS_CODE = '00000'

// 响应拦截器
request.interceptors.response.use(
    (response: AxiosResponse<Result>) => {
        const res = response.data
        // 业务失败：弹出错误提示并 reject
        if (res.code !== SUCCESS_CODE) {
            Snackbar.error(res.message || '请求失败')
            return Promise.reject(new Error(res.message || '请求失败'))
        }
        // 业务成功：自动解包，直接返回 data
        return res.data as any
    },
    (error: AxiosError) => {
        const {response} = error
        let message = '网络错误，请稍后重试'

        if (response) {
            const status = response.status
            switch (status) {
                case 401:
                    message = '登录已过期，请重新登录'
                    useUserStore().logout()
                    window.location.href = '/login'
                    break
                case 403:
                    message = '没有权限访问'
                    break
                case 404:
                    message = '请求的资源不存在'
                    break
                case 500:
                    message = '服务器内部错误'
                    break
                default:
                    message = `请求失败 (${status})`
            }
        } else if (error.message.includes('timeout')) {
            message = '请求超时，请稍后重试'
        } else if (error.message.includes('Network Error')) {
            message = '网络连接失败，请检查网络'
        }

        Snackbar.error(message)
        return Promise.reject(error)
    }
)

export default request

// 封装通用请求方法
export function get<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
    return request.get(url, config) as Promise<T>
}

export function post<T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
    return request.post(url, data, config) as Promise<T>
}

export function put<T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
    return request.put(url, data, config) as Promise<T>
}

export function del<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
    return request.delete(url, config) as Promise<T>
}
