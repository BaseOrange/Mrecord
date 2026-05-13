# 环境变量使用指南

## 🚀 快速开始

### 开发模式（默认）
```bash
yarn dev
```
- 自动加载 `.env.development`
- API 代理到: `http://127.0.0.1:2333`
- 应用标题: "月衡 Mrecord - Dev"

### 生产模式
```bash
yarn build
```
- 自动加载 `.env.production`
- API 地址: `/api/v2`（需要配置实际的生产服务器地址）
- 应用标题: "月衡 Mrecord"

## 📝 环境文件说明

### `.env` - 通用配置（所有环境共享）
```env
VITE_API_BASE_URL=/api/v2
VITE_APP_TITLE=月衡 Mrecord
VITE_PROXY_TARGET=http://127.0.0.1:2333
```

### `.env.development` - 开发环境
```env
VITE_API_BASE_URL=/api/v2
VITE_APP_TITLE=月衡 Mrecord - Dev
VITE_PROXY_TARGET=http://127.0.0.1:2333
```

### `.env.production` - 生产环境
```env
VITE_API_BASE_URL=/api/v2
VITE_APP_TITLE=月衡 Mrecord
VITE_API_URL=https://example.com/api/v2
```

## 🔧 如何修改配置

### 1. 修改开发环境 API 地址

编辑 `.env.development`：
```env
VITE_PROXY_TARGET=http://your-dev-server.com:3000
```

然后重启开发服务器：
```bash
# Ctrl+C 停止当前服务
yarn dev  # 重新启动
```

### 2. 修改生产环境 API 地址

编辑 `.env.production`：
```env
VITE_API_URL=https://api.yourdomain.com/api/v2
```

然后重新构建：
```bash
yarn build
```

### 3. 添加新的环境变量

**步骤 1**: 在所有 `.env` 文件中添加变量（必须以 `VITE_` 开头）
```env
# .env.development
VITE_MY_VAR=development_value

# .env.production
VITE_MY_VAR=production_value
```

**步骤 2**: 在 `src/types/varlet.d.ts` 中添加类型声明
```typescript
interface ImportMetaEnv {
    readonly VITE_API_BASE_URL: string
    readonly VITE_APP_TITLE: string
    readonly VITE_PROXY_TARGET?: string
    readonly VITE_API_URL?: string
    readonly VITE_MY_VAR?: string  // 新增
}
```

**步骤 3**: 在代码中使用
```typescript
const myVar = import.meta.env.VITE_MY_VAR
console.log(myVar)
```

## 🎯 常用命令

```bash
# 开发环境
yarn dev              # 启动开发服务器

# 生产环境
yarn build            # 构建生产版本
yarn preview          # 预览生产构建

# 其他
yarn lint             # 代码检查（如果配置了）
yarn type-check       # 类型检查
```

## 📊 查看当前环境信息

在浏览器控制台运行：
```javascript
console.log('模式:', import.meta.env.MODE)
console.log('开发环境:', import.meta.env.DEV)
console.log('生产环境:', import.meta.env.PROD)
console.log('API基础URL:', import.meta.env.VITE_API_BASE_URL)
console.log('应用标题:', import.meta.env.VITE_APP_TITLE)
```

## ⚠️ 注意事项

1. **修改环境变量后必须重启服务**
   - 开发环境：停止后重新运行 `yarn dev`
   - 生产环境：重新运行 `yarn build`

2. **环境变量必须以 `VITE_` 开头**
   - ✅ `VITE_API_URL` - 正确
   - ❌ `API_URL` - 错误（不会被暴露给前端）

3. **不要提交敏感信息**
   - `.env` 文件已添加到 `.gitignore`
   - 敏感信息应该通过 CI/CD 或部署平台配置

4. **优先级顺序**
   ```
   .env.[mode].local > .env.[mode] > .env.local > .env
   ```
   - `.env.development.local` 优先级最高（但已被 git 忽略）
   - `.env.development` 次之
   - `.env` 优先级最低

## 🔍 调试技巧

### 查看所有环境变量
在 `main.ts` 中临时添加：
```typescript
console.log('所有环境变量:', import.meta.env)
```

### 检查代理是否生效
开发环境下，请求 `/api/v2/xxx` 会被代理到 `VITE_PROXY_TARGET` 配置的地址

### 验证配置是否加载
```typescript
// 在任何组件中
onMounted(() => {
  console.log('当前环境配置:', {
    mode: import.meta.env.MODE,
    apiBase: import.meta.env.VITE_API_BASE_URL,
    appTitle: import.meta.env.VITE_APP_TITLE,
  })
})
```

## 💡 最佳实践

1. **为不同开发者创建本地配置**
   ```bash
   # 创建个人本地配置（不会被 git 追踪）
   cp .env.development .env.development.local
   # 编辑 .env.development.local 中的个性化配置
   ```

2. **使用注释说明每个变量的用途**
   ```env
   # API 基础路径
   VITE_API_BASE_URL=/api/v2
   
   # 后端服务器地址（仅开发环境）
   VITE_PROXY_TARGET=http://127.0.0.1:2333
   ```

3. **保持环境文件同步**
   - 添加新变量时，确保所有环境文件都有该变量
   - 使用默认值避免运行时错误

## 🐛 常见问题

### Q: 修改了 .env 文件但不生效？
A: 必须重启开发服务器或重新构建

### Q: 如何为测试环境配置？
A: 创建 `.env.test` 文件，然后运行：
```bash
vite --mode test
```

### Q: 环境变量在哪里可以看到？
A: 
- 开发时：浏览器控制台 `import.meta.env`
- 构建后：会被替换为实际值，可以在打包后的代码中搜索

### Q: 可以在 Node.js 代码中使用吗？
A: 不可以，这些是前端环境变量。如果需要后端环境变量，使用 `process.env`

---

更多信息请参考 [Vite 环境变量文档](https://cn.vitejs.dev/guide/env-and-mode.html)
