# 快速参考指南

## 新增的工具函数

### 格式化工具 (`@/utils/format`)

```typescript
import { formatMoney, getChangeColor, getChangePrefix, formatDate, debounce, throttle } from '@/utils/format'

// 格式化金额
formatMoney(1234.5) // "1,234.50"

// 获取变化颜色
getChangeColor(100) // "#34c759" (绿色)
getChangeColor(-100) // "#ff3b30" (红色)
getChangeColor(0) // "#8e8e93" (灰色)

// 获取变化前缀
getChangePrefix(100) // "+"
getChangePrefix(-100) // "-"
getChangePrefix(0) // ""

// 格式化日期
formatDate(new Date()) // "2026-05-13 10:30:00"
formatDate(Date.now(), 'YYYY-MM-DD') // "2026-05-13"

// 防抖
const debouncedSearch = debounce((query) => { /* search logic */ }, 300)

// 节流
const throttledScroll = throttle(() => { /* scroll logic */ }, 100)
```

### 安全工具 (`@/utils/security`)

```typescript
import { 
  checkPasswordStrength, 
  isValidEmail, 
  sanitizeInput,
  generateRandomString,
  encryptStorage,
  decryptStorage,
  clearSensitiveFields 
} from '@/utils/security'

// 检查密码强度 (0-4)
checkPasswordStrength('abc123') // 2
checkPasswordStrength('Abc123!@#') // 4

// 验证邮箱
isValidEmail('test@example.com') // true
isValidEmail('invalid') // false

// 清理输入（防止XSS）
sanitizeInput('<script>alert("xss")</script>') // "&lt;script&gt;...&lt;/script&gt;"

// 生成随机字符串
generateRandomString(16) // "aB3dE5gH7jK9mN1p"

// 加密存储
const encrypted = encryptStorage({ key: 'value' })
const decrypted = decryptStorage(encrypted) // { key: 'value' }

// 清除敏感字段
clearSensitiveFields({ name: 'John', password: '123' }) // { name: 'John' }
```

## 新增的组件

### PageHeader 组件

```vue
<script setup lang="ts">
import PageHeader from '@/components/PageHeader.vue'
</script>

<template>
  <div>
    <!-- 基础用法 -->
    <PageHeader title="页面标题" />
    
    <!-- 带返回按钮 -->
    <PageHeader title="详情" :show-back="true" />
    
    <!-- 自定义返回路径 -->
    <PageHeader title="编辑" :show-back="true" back-path="/home" />
  </div>
</template>
```

## 环境变量使用

### 在代码中使用

```typescript
// 获取 API 基础URL
const baseURL = import.meta.env.VITE_API_BASE_URL

// 获取应用标题
const appTitle = import.meta.env.VITE_APP_TITLE

// 条件性使用
if (import.meta.env.DEV) {
  console.log('开发环境')
}
```

### 配置文件

**.env.development**
```env
VITE_API_BASE_URL=/api/v2
VITE_APP_TITLE=月衡 Mrecord - Dev
VITE_PROXY_TARGET=http://127.0.0.1:2333
```

**.env.production**
```env
VITE_API_BASE_URL=/api/v2
VITE_APP_TITLE=月衡 Mrecord
VITE_API_URL=https://api.mrecord.com/api/v2
```

## 请求拦截器高级用法

### 跳过 Loading

```typescript
import { get } from '@/utils/request'

// 某些请求不需要显示 loading
get('/api/data', { skipLoading: true } as any)
```

### 错误处理

所有请求都会自动：
- 显示全局 Loading
- 处理业务错误（code !== '00000'）
- 处理 HTTP 错误（401, 403, 404, 500等）
- Token 过期自动登出
- 显示友好的错误提示

```typescript
try {
  const data = await get('/api/data')
  // 处理成功响应
} catch (error) {
  // 错误已被拦截器处理并显示提示
  // 这里可以进行额外的错误处理
}
```

## User Store 使用

```typescript
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

// 获取用户信息
console.log(userStore.userInfo)
console.log(userStore.token)
console.log(userStore.isLoggedIn)

// 设置 Token（自动加密存储）
userStore.setToken('your-token-here')

// 设置用户信息（自动清除密码字段）
userStore.setUserInfo({ id: '1', email: 'test@example.com', nickname: 'Test' })

// 登出（自动清除所有数据）
userStore.logout()
```

## 最佳实践

### 1. 组件中使用工具函数

```vue
<script setup lang="ts">
import { formatMoney, getChangeColor } from '@/utils/format'

const amount = 1234.56
</script>

<template>
  <div :style="{ color: getChangeColor(amount) }">
    {{ formatMoney(amount) }}
  </div>
</template>
```

### 2. 表单验证

```vue
<script setup lang="ts">
import { isValidEmail, checkPasswordStrength } from '@/utils/security'
import { Snackbar } from '@varlet/ui'

const email = ref('')
const password = ref('')

const onSubmit = () => {
  if (!isValidEmail(email.value)) {
    Snackbar.warning('请输入有效的邮箱地址')
    return
  }
  
  const strength = checkPasswordStrength(password.value)
  if (strength < 3) {
    Snackbar.warning('密码强度不足，请使用更复杂的密码')
    return
  }
  
  // 提交表单
}
</script>
```

### 3. 搜索防抖

```vue
<script setup lang="ts">
import { debounce } from '@/utils/format'
import { ref, watch } from 'vue'

const searchQuery = ref('')
const results = ref([])

const performSearch = debounce(async (query: string) => {
  if (!query) {
    results.value = []
    return
  }
  // 执行搜索逻辑
  results.value = await searchAPI(query)
}, 300)

watch(searchQuery, (newVal) => {
  performSearch(newVal)
})
</script>
```

## 常见问题

### Q: 如何添加新的环境变量？

A: 
1. 在 `.env`、`.env.development`、`.env.production` 中添加变量（以 `VITE_` 开头）
2. 在 `src/types/varlet.d.ts` 中的 `ImportMetaEnv` 接口添加类型声明
3. 在代码中使用 `import.meta.env.VITE_YOUR_VAR`

### Q: 如何在某些请求中禁用全局 Loading？

A:
```typescript
get('/api/data', { skipLoading: true } as any)
post('/api/data', data, { skipLoading: true } as any)
```

### Q: Token 加密是否足够安全？

A: 当前使用的是简单异或加密 + Base64，可以防止明文存储。但对于高安全性要求的应用，建议：
- 使用 HTTPS
- 实施 Token 刷新机制
- 考虑使用 HttpOnly Cookie 存储 Token
- 后端实施 Rate Limiting

### Q: 如何测试优化后的功能？

A:
```bash
# 开发环境测试
yarn dev

# 构建生产版本
yarn build

# 预览生产构建
yarn preview

# 类型检查
yarn vue-tsc --noEmit
```
