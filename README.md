<p align="center">
  <img src="README/logo1.png" alt="Mrecord" width="128" height="128">
</p>


<h1 align="center">Mrecord 月衡</h1>

<p align="center">
  <strong>每月快照账簿、资产负债全景浏览 & 同比环比智能分析平台</strong><br/>
  <a href="https://mr.660066.xyz"><strong>访问 Mrecord</strong></a>
</p>


<p align="center">
  一套前端 + 两个可互换后端（Rust / Java）+ SQLite 的快照式月度记账服务。<br/>
  多用户、多账簿、环比同比数据分析、一键导出、月度提醒
</p>


<p align="center">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge"></a>
</p>


---

[![Mrecord 主界面](README/promotion.png)](README/promotion.png)

---

# 目录 <!-- omit in toc -->

- [Mrecord 是什么](#mrecord-是什么)
- [为什么是 Mrecord](#为什么是-mrecord)
- [功能特性](#功能特性)
- [双后端架构](#双后端架构)
- [部署指南](#部署指南)
- [构建与打包](#构建与打包)
- [项目结构](#项目结构)
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

<a name="为什么是-mrecord"></a>

# 为什么是 Mrecord

Mrecord 是你的专属月度财务管理助手，告别繁琐，轻松管控资产负债：

- **月度快照核心** —— 无需逐笔录入，每月仅记各账户资产负债余额，快速完成财务盘点。
- **多维度数据管理** —— 账户分类、清晰视图+搜索拖拽，资产状况一目了然。
- **贴心辅助功能** —— 支持数据导出备份，搭配每月记账提醒，避免遗漏。

---

<a name="功能特性"></a>

# 功能特性

### 👤 用户体系

- **注册 / 登录 / 邮箱激活 / 找回密码 / 修改密码 / 注销账户** —— 完整的账号生命周期，邮件令牌全程闭环。
- **多用户数据隔离** —— 每个用户只能看到自己的账簿与记录。
- **管理员体系** —— 内置管理后台，可管理用户、查看操作审计日志、配置站点与邮件服务。

### 🗂️ 账簿与模板项

- **多账本支持** —— 可创建多个独立账本，分类管理不同场景财务（如个人、家庭、备用金等），清晰区分，互不干扰。
- **自定义模板项** —— 每个账本可定义资产 / 负债 / 仅记录三类条目，自带图标与排序，支持拖拽调整，记账时按模板逐项填写即可。

### 📸 月度快照记账

- **按月记录各账户余额** —— 告别逐笔流水，一个月份只存一份余额快照，自动汇总当月总资产、总负债与净资产。

### 📊 数据分析统计

- **数据统计模块** —— 基于每月账户余额数据，自动完成数据分析，清晰呈现资产负债变化趋势，助力精准掌握财务状况。
- **环比 / 同比** —— 自动计算本月相对上月、以及相对去年同期的变动百分比，数值口径与精度与 Java 版完全对齐。
- **趋势图表** —— 首页仪表盘 + 账簿维度统计页，Chart.js 渲染资产趋势。

### 📁 数据导出

- **异步导出任务** —— 提交导出请求后后台生成 Excel，任务列表可查进度，完成后邮件附件送达。
- **批量 / 自定义导出** —— 支持导出单个账簿或用户全部账簿（一个账簿一个 Sheet），灵活备份，数据管理更自主。

### 📧 邮件提醒

- **月度记账提醒** —— 自定义提醒日期，到点自动推送提醒邮件，避免遗漏。
- **年度财务总结** —— 每个财务年度伊始，自动汇总上一年度资产变动并发送总结邮件。
- **全链路邮件** —— 注册欢迎、账号激活、密码重置、导出完成，均走统一邮件模板。

---

<a name="双后端架构"></a>

# 双后端架构

Mrecord 后端有 **Java 版**与 **Rust 版**两套实现，共用同一个前端（`mrecord-vue`）与同一份 SQLite 表结构，数据互相兼容，可按需选择：

|              | Rust 版（当前推荐）                                | Java 版                                         |
|--------------|------------------------------------------------|-------------------------------------------------|
| **状态**     | 功能已与 Java 版完全对齐（38 个接口 0 缺失），活跃开发 | v1.0.0 已发布版本，稳定可用                          |
| **技术栈**   | axum 0.8 + sea-orm + tokio                     | Spring Boot 4 + MyBatis-Flex                    |
| **默认端口** | `2333`（`MRECORD_PORT` 可改）                     | `2333`                                          |
| **数据目录** | `./data.db` + `./exports/`                     | `./data/mrecord.db`                             |
| **前端嵌入** | **编译期**内嵌进二进制（`include_dir!`），单文件部署 | 作为 classpath 静态资源打包进 jar                     |
| **运行依赖** | 无（musl 静态链接单二进制，开箱即用）                    | 需 JDK 17                                        |
| **构建工具** | Cargo / Docker 多阶段构建                        | Maven / Docker                                  |

> 两版后端的 API 前缀均为 `/api/v2`，且**统一使用 POST**（含查询类接口），返回体均为 `{ code, message, data }`，前端无需为后端版本做任何适配。
>
> 表结构保持一致（表名 / 列名大写命名），切换后端时同一份 SQLite 数据库可直接沿用。

---

<a name="部署指南"></a>

# 部署指南

Mrecord 是一个**自托管 Web 服务**，部署后在浏览器中访问对应端口即可使用。以下按推荐程度给出三种方式。

## 方式一：Docker 部署 Rust 版（推荐）

单二进制、无运行时依赖、前端已内嵌，是 deployment 最省心的版本。

```bash
# 1. 构建前端（vite outDir 已指向 ../mrecord-rust/static/）
git clone https://github.com/BaseOrange/Mrecord.git
cd Mrecord/mrecord-vue
yarn install && yarn build

# 2. 构建镜像（多阶段：依赖缓存 → 编译 → alpine 瘦身）
cd ../mrecord-rust
docker build -t mrecord-rust .

# 3. 运行容器：映射 2333 端口，命名数据卷持久化 data.db 与 exports/
docker volume create mrecord-data-rs
docker run -d -p 2333:2333 -v mrecord-data-rs:/app --name mrecord-rust mrecord-rust
```

> ⚠️ **必须先执行第 1 步**：`static/` 由前端构建生成且被 `.gitignore` 忽略，克隆后不存在；Dockerfile 会把它 `COPY` 进构建上下文并内嵌进二进制，目录缺失时构建会失败。

> ⚠️ **数据卷请使用命名卷**（如 `mrecord-data-rs`）。二进制位于 `/app` 下，命名卷首次挂载时 Docker 会自动把 `/app` 内容复制进卷；若直接把**空主机目录**绑定挂载到 `/app`，会遮盖二进制导致容器无法启动（确需主机目录挂载时，需先把镜像内 `/app` 的文件拷入主机目录）。

镜像支持的环境变量：

| 环境变量                | 说明                                          | 默认值          |
|-------------------|---------------------------------------------|--------------|
| `MRECORD_HOST`    | 监听地址（容器内必须为 `0.0.0.0`，镜像已默认设置）                | `127.0.0.1`  |
| `MRECORD_PORT`    | 监听端口（与 Java 版统一为 `2333`）                | `2333`       |
| `MRECORD_STATIC_DIR` | 设为磁盘路径可切回运行时读盘，在不重编译后端的前提下热替换前端               | 未设置（用内嵌资源）   |
| `RUST_LOG`        | 日志级别，如 `mrecord_rust=info`                    | 镜像内置默认值       |

## 方式二：Docker 部署 Java 版

```bash
cd Mrecord/mrecord-java

# 1. 打包后端（需 JDK 17 环境）
mvn clean install          # 产物：target/mrecord-1.0.0.jar

# 2. 构建并运行（构建上下文为项目根目录）
cd ..
docker build -t mrecord -f dockerfile .
docker run -d -p 2333:2333 -v ./mrecord-data:/app/data --name mrecord mrecord
```

## 方式三：直接运行预编译 jar

[v1.0.0](https://github.com/BaseOrange/Mrecord/releases/tag/v1.0.0) 发布了 Java 版 jar（内含前端，约 77MB），下载后可直接运行：

```bash
java -jar mrecord-1.0.0.jar        # 启动后访问 http://localhost:2333
```

从 [GitHub Releases](https://github.com/BaseOrange/Mrecord/releases) 可浏览所有版本。

### 前置条件

- **Docker 部署**：Docker 20+（Rust 版需能访问 Docker Hub 拉取 `rust:alpine` / `alpine` 基础镜像）
- **源码构建**：Node.js 20.19+（Vite 8 要求）与 JDK 17（仅 Java 版）/ Rust toolchain（仅 Rust 版）
- MacOS、Windows 10+ 或 Linux

---

<a name="构建与打包"></a>

# 构建与打包

#### 前端（两个后端共用）

```bash
cd Mrecord/mrecord-vue

# 安装依赖
yarn install

# 开发模式启动（/api 代理到 http://127.0.0.1:2333，见 .env.development）
yarn dev

# 生产构建（产物直接输出到 ../mrecord-rust/static/）
yarn build

# 类型检查
yarn typecheck
```

> 前端产物**同时**是两个后端的输入：Rust 版经 `include_dir!` 在编译期内嵌，Java 版则需将构建产物放入 `mrecord-java/src/main/resources/static/`（仓库已提交一份构建好的产物）。

#### Rust 后端

> 前端产物在**编译期**内嵌进 Rust 二进制（`include_dir!`），因此构建后端前必须先构建前端：

```bash
# 1. 构建前端（vite 的 outDir 已指向 ../mrecord-rust/static/）
cd Mrecord/mrecord-vue
yarn install && yarn build

# 2. 构建 / 运行后端（static/ 已内嵌，运行时不再依赖该目录与 cwd）
cd ../mrecord-rust
cargo run               # 开发模式，http://127.0.0.1:2333
cargo build --release   # 生产构建
```

若跳过第 1 步，`build.rs` 会生成「前端未构建」占位页并打印 `cargo:warning`（不影响
`cargo check` / `cargo test`）。设置环境变量 `MRECORD_STATIC_DIR=/path/to/static`
可切回运行时读盘，在不重编译后端的前提下替换前端。

数据库在首次启动时自动由 `schema.sql` 建表，无需单独执行迁移；删除 `data.db` 即可重置。

#### Java 后端

```bash
cd Mrecord/mrecord-java

# 安装依赖并打包（需 JDK 17 环境）
mvn clean install          # 产物：target/mrecord-1.0.0.jar

# 本地运行
java -jar target/mrecord-1.0.0.jar     # http://localhost:2333
```

数据库表结构在启动时由 `spring.sql.init` 自动执行 `schema.sql` 初始化，无需手动建表。

---

<a name="项目结构"></a>

# 项目结构

```
Mrecord/
├── README.md                       # 项目主文档
├── LICENSE                         # MIT 开源协议
├── dockerfile                      # Java 版容器构建文件
├── .gitignore                      # Git 忽略规则
├── README/                         # 文档图片资源
│   ├── logo1.png
│   └── promotion.png
│
├── mrecord-rust/                    # 后端服务 - Rust 版（axum + sea-orm，当前推荐）
│   ├── Cargo.toml                  # Rust 工程配置（edition 2024）
│   ├── Cargo.lock                  # 依赖锁定文件
│   ├── Dockerfile                  # 多阶段构建（rust:alpine 编译 → alpine 运行）
│   ├── build.rs                    # 静态资源构建守卫（static/ 缺失时生成占位页）
│   ├── schema.sql                  # 建表脚本（启动时经 include_str! 自动执行）
│   ├── rustfmt.toml                # 代码格式配置
│   ├── resources/                  # 编译期内嵌的资源
│   │   └── mail/                   # 邮件模板（注册 / 激活 / 提醒 / 导出 / 年度总结 …）
│   ├── static/                     # 前端构建产物（include_dir! 内嵌，gitignore 忽略）
│   └── src/
│       ├── main.rs                 # 应用入口（建库 → 加载配置 → 路由 → 绑定端口）
│       ├── db.rs                   # SQLite 连接、schema 初始化、金额列类型补丁
│       ├── config.rs               # 安全配置（JWT 密钥 / 过期时长，从 SYS_CONFIG 加载）
│       ├── error.rs                # AppError：统一错误响应（对应 Java 全局异常处理）
│       ├── static_files.rs         # 前端静态资源托管（内嵌 / MRECORD_STATIC_DIR 覆盖）
│       ├── router/                 # 路由表（/api/v2 前缀，注册全部接口）
│       ├── handler/                # 请求处理器（对齐 Java 各 Controller）
│       ├── service/                # 业务逻辑 + 定时任务（月度提醒 / 年度总结 / 注销清理 / 导出）
│       ├── model/                  # 请求 / 响应 DTO（与实体解耦）
│       ├── entity/                 # sea-orm 实体，一表一文件（含 SYS_BACKUP_* 备份表）
│       ├── common/                 # 统一响应体、状态码、金额序列化、分页、用户上下文
│       ├── constant/               # 领域常量与状态枚举
│       ├── middleware/             # 中间件（操作日志审计）
│       └── util/                   # 工具（JWT、令牌、任务调度）
│
├── mrecord-java/                   # 后端服务 - Java 版（Spring Boot 4）
│   ├── pom.xml                     # Maven 构建配置
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/com/dcz/mrecord/
│   │   │   │   ├── Application.java          # 应用入口
│   │   │   │   ├── bo/                       # 业务对象（邮件 / 站点配置）
│   │   │   │   ├── common/                   # 响应封装、状态码、用户上下文、管理员校验
│   │   │   │   ├── config/                   # Spring 配置（异步、Web、MyBatisFlex、SPA 转发 …）
│   │   │   │   ├── constant/                 # 常量与枚举定义
│   │   │   │   ├── controller/               # 控制器层（REST API）
│   │   │   │   ├── dto/                      # 数据传输对象 (DTO)
│   │   │   │   ├── entity/                   # 数据库实体类
│   │   │   │   │   └── backup/               # 备份表实体（SYS_BACKUP_*）
│   │   │   │   ├── exception/                # 全局异常处理
│   │   │   │   ├── interceptor/              # 拦截器（登录 / 管理员 / 日志 / 审计字段）
│   │   │   │   ├── mapper/                   # MyBatis-Flex 数据访问层
│   │   │   │   ├── service/                  # 业务逻辑层（接口 + impl）
│   │   │   │   ├── task/                     # 定时任务（月度提醒 / 年度总结 / 注销清理）
│   │   │   │   └── util/                     # 工具类（JWT 等）
│   │   │   └── resources/
│   │   │       ├── application.yaml          # 主配置（端口 2333、SQLite、Druid 连接池）
│   │   │       ├── schema.sql                # 建表脚本（启动时自动执行）
│   │   │       ├── mail/                     # 邮件模板
│   │   │       ├── mapper/                   # MyBatis XML 映射文件
│   │   │       └── static/                   # 前端构建产物（classpath 静态资源）
│   │   └── test/                             # 单元测试
│   └── target/                               # Maven 构建输出目录
│
├── mrecord-vue/                    # 前端应用（Vue 3 + TypeScript）
│   ├── package.json                # 项目依赖配置
│   ├── yarn.lock                   # Yarn 依赖锁定
│   ├── vite.config.ts              # Vite 构建配置（outDir → ../mrecord-rust/static）
│   ├── index.html                  # 入口 HTML
│   ├── tsconfig*.json              # TypeScript 配置
│   ├── auto-imports.d.ts           # 自动导入类型声明
│   ├── components.d.ts             # 组件类型声明
│   ├── .env / .env.development / .env.production   # 环境变量
│   ├── doc/                        # 前端文档（环境变量 / 快速参考 / 安全说明）
│   ├── public/                     # 静态资源（不经过编译）
│   └── src/
│       ├── App.vue                 # 根组件
│       ├── main.ts                 # 入口脚本（Varlet 主题配置）
│       ├── style.css               # 全局样式
│       ├── api/                    # API 接口封装
│       │   ├── modules/            # 按模块拆分的 API（book / user / config / export …）
│       │   ├── types.ts            # API 类型定义
│       │   └── index.ts            # 统一导出
│       ├── assets/                 # 静态资源（图片、用户协议 Markdown）
│       ├── components/             # 公共组件（PageHeader / BookCard / TrendChart / IconPicker …）
│       ├── router/                 # Vue Router 路由配置与导航守卫
│       ├── stores/                 # Pinia 状态管理
│       ├── types/                  # TypeScript 类型声明
│       ├── utils/                  # 工具函数（请求封装 / 格式化 / 安全处理）
│       └── views/                  # 页面视图（首页 / 统计 / 账簿 / 记账 / 导出 / 管理后台 …）
│
├── mrecord-sql/                    # 数据库脚本（历史归档）
│   ├── README.md                   # 脚本执行顺序说明
│   └── all/DDL/ddl.sql             # 全量建表脚本
│
└── mrecord-portal/                 # 静态门户/落地页
    ├── index.html                  # 门户首页
    ├── 404.html                    # 自定义 404 页面
    ├── logo.jpg
    └── xct.png
```

> 💡 两个后端启动时均会**自动初始化 SQLite 表结构**，日常使用无需手动执行 `mrecord-sql/` 中的脚本，该目录仅作历史归档保留。

---

<a name="技术栈"></a>

# 技术栈

| 分类              | 技术                                                     |
|-----------------|--------------------------------------------------------|
| 后端 Rust 版（推荐）   | Rust (edition 2024)、axum 0.8、sea-orm 1、tokio、rust_xlsxwriter、lettre |
| 后端 Java 版       | JDK 17、Spring Boot 4.0.6、MyBatis-Flex 1.11.7、Druid、jjwt、Apache Fesod |
| 后端构建工具          | Cargo / Maven 3.9                                      |
| 前端技术栈           | Vue 3.5、TypeScript 6、Pinia 3、Vue Router 4、Varlet UI 3、Chart.js 4、Axios |
| 前端构建工具          | Vite 8（需 Node.js 20.19+）                                 |
| 数据库             | SQLite（WAL 模式，两版后端表结构兼容）                                |
| 容器化             | Docker（多阶段构建）                                           |

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
