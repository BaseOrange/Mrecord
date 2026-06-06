# 月衡 Mrecord (Vue 前端)

基于 Vue 3 + TypeScript + Vite 构建的个人财务管理应用前端，提供记账、多账簿管理、资产统计与数据导出等功能。

## 功能概览

- **用户系统**：注册、登录、找回密码、账户激活、修改密码
- **系统初始化**：首次部署时的管理员注册与系统引导
- **首页仪表盘**：净资产总览、环比变动、快捷入口
- **多账簿管理**：创建与管理多个独立账簿，每个账簿独立记账
- **记账面板**：按月份记录收支，支持自定义账目模板
- **统计分析**：资产趋势图表、各账簿独立统计
- **数据导出**：支持导出账簿数据
- **管理后台**：用户管理、系统配置、邮箱服务配置、站点信息配置、操作日志

## 技术栈

| 类别 | 技术 |
|------|------|
| 框架 | Vue 3 (Composition API + `<script setup>`) |
| 语言 | TypeScript |
| 构建 | Vite |
| 状态管理 | Pinia |
| 路由 | Vue Router 4 |
| UI 组件库 | Varlet UI |
| 图表 | Chart.js |
| HTTP | Axios |
| 拖拽排序 | vuedraggable |
| 密码哈希 | js-md5 |

## 项目结构

```
src/
├── api/                  # API 请求层
│   ├── modules/          # 按模块拆分的接口（book, user, config, export 等）
│   ├── types.ts          # 通用类型定义
│   └── index.ts          # API 统一导出
├── assets/               # 静态资源
├── components/           # 公共组件
├── router/               # 路由配置与导航守卫
├── stores/               # Pinia 状态管理
├── utils/                # 工具函数（格式化、请求封装、安全处理）
├── views/                # 页面视图
│   ├── HomePage.vue      # 首页仪表盘
│   ├── StatsPage.vue     # 统计页
│   ├── BookPage.vue      # 账簿列表
│   ├── BookRecordPage.vue # 账簿记录列表
│   ├── RecordPage.vue    # 记账页
│   ├── ExportPage.vue    # 数据导出
│   ├── ProfilePage.vue   # 个人中心
│   ├── Login.vue         # 登录
│   ├── Register.vue      # 注册
│   ├── SystemInitPage.vue # 系统初始化
│   ├── AdminPage.vue     # 管理后台
│   └── Layout.vue        # 主布局（含底部导航栏）
└── main.ts               # 应用入口
```

## 快速开始

### 环境要求

- Node.js >= 18
- 后端服务运行中（参见 `mrecord-rust` 项目）

### 安装与运行

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build

# 预览生产构建
npm run preview

# 类型检查
npm run typecheck
```

### 开发环境配置

默认情况下开发服务器会将 API 请求代理到 `http://127.0.0.1:8250`。可通过 Vite 配置文件修改代理目标。

## 路由一览

| 路径 | 页面 | 权限 |
|------|------|------|
| `/login` | 登录 | 公开 |
| `/register` | 注册 | 公开 |
| `/forgot-password` | 找回密码 | 公开 |
| `/reset-password` | 重置密码 | 公开 |
| `/activate-account` | 账户激活 | 公开 |
| `/init` | 系统初始化 | 公开 |
| `/home` | 首页 | 需登录 |
| `/stats` | 统计 | 需登录 |
| `/book` | 账簿列表 | 需登录 |
| `/book/:bookId/records` | 账簿记录 | 需登录 |
| `/book/:bookId/record` | 记账 | 需登录 |
| `/book/:bookId/template` | 账目模板 | 需登录 |
| `/book/:bookId/stats` | 账簿统计 | 需登录 |
| `/export` | 数据导出 | 需登录 |
| `/profile` | 个人中心 | 需登录 |
| `/change-password` | 修改密码 | 需登录 |
| `/profile-edit` | 编辑资料 | 需登录 |
| `/admin` | 管理后台 | 管理员 |
| `/admin/users` | 用户管理 | 管理员 |
| `/admin/logs` | 操作日志 | 管理员 |
| `/admin/config` | 系统配置 | 管理员 |
| `/admin/email-config` | 邮箱配置 | 管理员 |
| `/admin/site-config` | 站点配置 | 管理员 |

## 主题色

项目使用自定义橙色主题（`#FF6500`），通过 Varlet UI 的 `StyleProvider` 在 `main.ts` 中全局配置。
