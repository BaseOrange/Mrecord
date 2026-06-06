<p align="center">
  <img src="README/logo1.jpg" alt="Netcatty" width="128" height="128">
</p>

<h1 align="center">Mrecord</h1>

<p align="center">
  <strong>每月快照账簿、资产负债全景浏览 & 同比环比智能分析平台</strong><br/>
  <a href="https://mr.660066.xyz"><strong>访问Mrecord</strong></a>
</p>



<p align="center">
  一个基于 Java、Vue3 构建的快照式月度记账服务。<br/>
  多用户、多账簿、环比同比数据分析、一键导出、月度提醒
</p>


<p align="center">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge"></a>
</p>

<p align="center">
  <a href="./README.md">English</a> · <a href="./README.zh-CN.md">简体中文</a>
</p>



---

[![Mrecord 主界面](README/promotion.png)](README/promotion.png)

---

# 目录 <!-- omit in toc -->

- [Mrecord 是什么](#mrecord-是什么)
- [为什么是 Mrecord](#为什么是-Mrecord)
- [功能特性](#功能特性)
- [部署指南](#部署指南)
- [构建与打包](#构建与打包)
- [技术栈](#技术栈)
- [参与贡献](#参与贡献)
- [贡献者](#贡献者)
- [Star 历史](#star-历史)
- [开源协议](#开源协议)

---

<a name="mrecord-是什么"></a>

# Mrecord是什么

**Mrecord**是一款主打月度账户快照的极简记账App，告别繁琐流水记录，专为需要轻松管理个人及家庭财务、精准掌握资产状况的用户设计，以每月余额为核心，让财务管理更高效、更省心。

- **Mrecord 是便捷的月度账户快照工具**，打破传统流水记账的繁琐模式，每月只需记录各账户的资产、负债余额，无需逐笔录入收支，3秒即可完成月度财务盘点，省时又省力，让记账不再成为负担。
- **Mrecord 是专业的财务数据分析助手**
  ，基于每月记录的账户余额数据，自动生成直观的财务分析报告，清晰呈现资产负债变化趋势、收支结构占比，帮你快速洞察财务状况，轻松掌握财富流向，为消费规划和理财决策提供参考依据。
- **Mrecord 支持灵活的数据导出功能**，可将月度账户余额、财务分析结果一键导出备份，支持多种导出格式，方便你随时留存数据、跨设备查看或与其他财务工具联动，实现数据自主掌控，无需担心数据丢失或迁移不便。
- **Mrecord 支持贴心的每月提醒功能**，可自定义月度记账提醒时间，到点自动推送通知，帮你养成定期记录账户余额的习惯，避免遗漏，确保每一期财务数据完整，让财务管理更具规律性和连贯性，彻底告别忘记记账的烦恼。
- **Mrecord 不是传统流水记账工具**，它以“月度余额快照”为核心，不强调逐笔收支记录，专注于资产负债的定期盘点与数据分析，让你跳出繁琐记账，聚焦核心财务状况，轻松实现财务可视化管理。

---

Mrecord<a name="为什么是-Mrecord"></a>

# 为什么是 Mrecord

Mrecord 是你的专属月度财务管理助手，告别繁琐，轻松管控资产负债：

- **月度快照核心** —— 无需逐笔录入，每月仅记各账户资产负债余额，快速完成财务盘点。
- **多维度数据管理** —— 账户分类、清晰视图+搜索拖拽，资产状况一目了然。
- **贴心辅助功能** —— 支持数据导出备份，搭配每月记账提醒，避免遗漏。

---

<a name="功能特性"></a>

# 功能特性

### 🗂️ 账户与账本管理

- **多账本支持** —— 可创建多个独立账本，分类管理不同场景财务（如个人、家庭、备用金等），清晰区分，互不干扰。

### 📊 数据分析统计

- **数据统计模块** —— 基于每月账户余额数据，自动完成数据分析，清晰呈现资产负债变化趋势，助力精准掌握财务状况。
- **多维度分析** —— 直观展示账户余额波动、账本收支关联，为财务规划和理财决策提供参考。

### 📁 数据导出

- **批量/自定义导出** —— 支持批量导出多账本数据，或自定义选择导出范围，灵活备份，数据管理更自主。
- **便捷编辑** —— 内置编辑功能，快速修改账户余额及账本信息，操作简单高效。

---

<a name="部署指南"></a>

# 部署指南

### 下载

从 [GitHub Releases](https://github.com/BaseOrange/Mrecord/releases/latest)
下载适合您平台的最新版本，或在 [GitHub Releases](https://github.com/BaseOrange/Mrecord/releases) 浏览所有版本。

| 操作系统                                | 支持情况        |
|:------------------------------------|:------------|
| **MacOS** / **Windows** / **Linux** | x64 / arm64 |

> 目前服务端为Java17开发，远期规划将使用Rust重构后端，数据库数据会尽可能保证兼容。
>
> 待Rust服务端发布后，会根据情况，决定后续是否继续维护Java版本服务端。

### 前置条件

- Node.js 18+ 和 npm
- MacOS、Windows 10+ 或 Linux

<a name="构建与打包"></a>

### 构建与打包

#### 前端

```bash
# 克隆仓库
git clone https://github.com/BaseOrange/Mrecord.git
cd Mrecord/mrecord-vue

# 安装依赖
yarn install

# 开发模式启动
yarn dev

# 生产构建
yarn build
```

#### 后端（Java 版，当前主力）

```bash
cd Mrecord/mrecord-java

# 安装依赖并打包（需 JDK 17 环境）
mvn clean install
```

#### 后端（Rust 版，**重构中，暂不可用**！）

```bash
cd Mrecord/mrecord-rust

# 开发模式运行
cargo run

# 生产构建
cargo build --release
```

#### Docker 部署

```bash
# 在项目根目录构建镜像
docker build -t mrecord -f dockerfile .

# 运行容器
docker run -d -p 2333:2333 -v ./mrecord-data:/app/data --name mrecord mrecord
```

### 项目结构

```
Mrecord/
├── README.md                       # 项目主文档
├── LICENSE                         # MIT 开源协议
├── dockerfile                      # Docker 容器构建文件
├── .gitignore                      # Git 忽略规则
├── README/                         # 文档图片资源
│   ├── logo1.jpg
│   └── promotion.png
│
├── mrecord-java/                   # 后端服务 - Java 版（Spring Boot，当前主力）
│   ├── pom.xml                     # Maven 构建配置
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/com/dcz/mrecord/
│   │   │   │   ├── Application.java          # 应用入口
│   │   │   │   ├── bo/                       # 业务对象 (Business Object)
│   │   │   │   ├── common/                   # 通用组件与响应封装
│   │   │   │   ├── config/                   # Spring 配置类
│   │   │   │   ├── constant/                 # 常量与枚举定义
│   │   │   │   ├── controller/               # 控制器层 (REST API)
│   │   │   │   ├── dto/                      # 数据传输对象 (DTO)
│   │   │   │   ├── entity/                   # 数据库实体类
│   │   │   │   ├── exception/                # 全局异常处理
│   │   │   │   ├── interceptor/              # 请求拦截器（认证、日志等）
│   │   │   │   ├── mapper/                   # MyBatis 数据访问层
│   │   │   │   ├── service/                  # 业务逻辑层
│   │   │   │   └── util/                     # 工具类
│   │   │   └── resources/
│   │   │       ├── application.yaml          # 主配置文件
│   │   │       ├── application-dev.yaml      # 开发环境配置
│   │   │       ├── application-sit.yaml      # 测试环境配置
│   │   │       ├── mail/                     # 邮件模板
│   │   │       └── mapper/                   # MyBatis XML 映射文件
│   │   └── test/                             # 单元测试
│   └── target/                               # Maven 构建输出目录
│
├── mrecord-rust/                   # 后端服务 - Rust 版（axum + sea-orm，重构中）
│   ├── Cargo.toml                  # Rust 项目配置
│   ├── Cargo.lock                  # 依赖锁定文件
│   ├── data.db                     # SQLite 数据库文件
│   ├── migration/                  # 数据库迁移模块
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                          # 迁移入口
│   │       ├── m20240604_000001_create_record.rs
│   │       ├── m20240604_000002_create_core_tables.rs
│   │       ├── m20240604_000003_create_sys_tables.rs
│   │       └── m20240604_000004_create_backup_tables.rs
│   └── src/
│       ├── main.rs                # 应用入口
│       ├── config.rs              # 配置管理
│       ├── db.rs                  # 数据库连接
│       ├── error.rs               # 错误类型定义
│       ├── common/                # 通用模块
│       │   ├── mod.rs
│       │   ├── res_code.rs        # 响应状态码
│       │   └── result.rs          # 统一响应体
│       ├── constant/              # 常量与状态枚举
│       │   ├── mod.rs
│       │   ├── export_task.rs
│       │   ├── template_item.rs
│       │   └── user_status.rs
│       ├── entity/                # ORM 实体定义 (sea-orm)
│       │   ├── mod.rs
│       │   ├── fin_book.rs
│       │   ├── fin_month_item_record.rs
│       │   ├── fin_month_record.rs
│       │   ├── fin_template_item.rs
│       │   ├── record.rs
│       │   ├── sys_backup_book.rs
│       │   ├── sys_backup_month_item_record.rs
│       │   ├── sys_backup_month_record.rs
│       │   ├── sys_backup_template_item.rs
│       │   ├── sys_config.rs
│       │   ├── sys_export_task.rs
│       │   ├── sys_user.rs
│       │   └── sys_user_operate_log.rs
│       ├── handler/               # 请求处理器 (路由逻辑)
│       │   ├── mod.rs
│       │   └── record.rs
│       ├── middleware/            # 中间件
│       │   └── mod.rs
│       ├── model/                 # 请求/响应模型 (DTO)
│       │   ├── mod.rs
│       │   ├── email.rs
│       │   ├── finance.rs
│       │   ├── id_dto.rs
│       │   ├── page_info.rs
│       │   ├── record.rs
│       │   ├── site.rs
│       │   └── user.rs
│       └── router/                # 路由注册
│           └── mod.rs
│
├── mrecord-vue/                    # 前端应用（Vue 3 + TypeScript）
│   ├── package.json                # 项目依赖配置
│   ├── yarn.lock                   # Yarn 依赖锁定
│   ├── vite.config.ts              # Vite 构建配置
│   ├── index.html                  # 入口 HTML
│   ├── tsconfig.json               # TypeScript 配置
│   ├── auto-imports.d.ts           # 自动导入类型声明
│   ├── components.d.ts             # 组件类型声明
│   ├── .env                        # 环境变量
│   ├── .env.development            # 开发环境变量
│   ├── .env.production             # 生产环境变量
│   ├── doc/                        # 前端文档
│   │   ├── ENV_GUIDE.md
│   │   ├── QUICK_REFERENCE.md
│   │   └── SECURITY.md
│   ├── public/                     # 静态资源（不经过编译）
│   │   ├── favicon.svg
│   │   └── icons.svg
│   └── src/
│       ├── App.vue                 # 根组件
│       ├── main.ts                 # 入口脚本
│       ├── style.css               # 全局样式
│       ├── api/                    # API 接口封装
│       │   ├── index.ts
│       │   ├── modules/            # 按模块拆分的 API
│       │   └── types.ts            # API 类型定义
│       ├── assets/                 # 静态资源（图片、Markdown 等）
│       ├── components/             # 公共组件
│       │   ├── AgreementPopup.vue
│       │   ├── AuthLayout.vue
│       │   ├── BookCard.vue
│       │   ├── HelloWorld.vue
│       │   ├── IconPicker.vue
│       │   ├── PageHeader.vue
│       │   └── TrendChart.vue
│       ├── router/                 # Vue Router 路由配置
│       ├── stores/                 # Pinia 状态管理
│       ├── types/                  # TypeScript 类型声明
│       ├── utils/                  # 工具函数
│       └── views/                  # 页面视图
│           ├── Home.vue
│           ├── Login.vue
│           ├── Register.vue
│           ├── Layout.vue
│           ├── BookPage.vue
│           ├── RecordPage.vue
│           ├── StatsPage.vue
│           ├── ExportPage.vue
│           ├── ProfilePage.vue
│           ├── AdminPage.vue
│           └── ...                 # 更多页面组件
│
├── mrecord-sql/                    # 数据库脚本
│   ├── README.md
│   ├── all/                        # 完整脚本（初次部署使用）
│   │   ├── DDL/
│   │   │   └── ddl.sql             # 完整数据库表结构
│   │   └── DML/                    # 初始数据
│   └── add/                        # 增量脚本（版本升级使用）
│       ├── DDL/
│       └── DML/
│
└── mrecord-portal/                 # 静态门户/落地页
    ├── index.html                  # 门户首页
    ├── 404.html                    # 自定义 404 页面
    ├── logo.jpg
    └── xct.png
```

---

<a name="技术栈"></a>

# 技术栈

| 分类         | 技术                               |
|------------|----------------------------------|
| 后端技术栈Java版 | JDK17、Spring Boot 4、MyBatis Flex |
| 后端技术栈Rust版 | Rust、axum 0.8、sea-orm、tokio（重构中） |
| 后端构建工具     | Maven 3.9 / Cargo                |
| 前端技术栈      | Vue 3、TypeScript、Pinia、Varlet UI |
| 前端构建工具     | Vite 8、Node.js 20.19             |
| 数据库        | SQLite                           |
| 容器化        | Docker                           |

---

<a name="参与贡献"></a>

# 参与贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建你的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

---

<a name="贡献者"></a>

# 贡献者

感谢所有参与贡献的人！

<a href="https://github.com/BaseOrange/Mrecord/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=BaseOrange/Mrecord" />
</a>

---

<a name="开源协议"></a>

# 开源协议

本项目采用 **MIT 协议** 开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

<a name="star-历史"></a>

# Star 历史

<a href="https://star-history.com/#BaseOrange/Mrecord&Date">

 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=BaseOrange/Mrecord&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=BaseOrange/Mrecord&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=BaseOrange/Mrecord&type=Date" />
 </picture>

</a>

---

<p>
作者的文采实在是太差，所以Readme模板是抄的Netcatty项目的哈哈哈哈。
</p>
<p>
我特喜欢Netcatty，媲美Termius欢迎大家去Start一下他的项目
</p>
<p>
项目地址：https://github.com/binaricat/Netcatty  
</p>


