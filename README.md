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
- **Mrecord 是专业的财务数据分析助手**，基于每月记录的账户余额数据，自动生成直观的财务分析报告，清晰呈现资产负债变化趋势、收支结构占比，帮你快速洞察财务状况，轻松掌握财富流向，为消费规划和理财决策提供参考依据。
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

从 [GitHub Releases](https://github.com/BaseOrange/Mrecord/releases/latest) 下载适合您平台的最新版本，或在 [GitHub Releases](https://github.com/BaseOrange/Mrecord/releases) 浏览所有版本。

| 操作系统                            | 支持情况    |
| :---------------------------------- | :---------- |
| **MacOS** / **Windows** / **Linux** | x64 / arm64 |

> 目前服务端为Java17开发，远期规划将使用Rust重构后端，数据库数据会尽可能保证兼容。
>
> 待Rust服务端发布后，会根据情况，决定后续是否继续维护Java版本服务端。

### 前置条件
- Node.js 18+ 和 npm
- MacOS、Windows 10+ 或 Linux

<a name="构建与打包"></a>

### 构建与打包

```bash
# 克隆仓库
git clone https://github.com/BaseOrange/Mrecord.git

# 客户端打包
cd Mrecord/mrecord-vue
# 安装依赖
yarn install
# 构建dist
yarn build

# 服务端打包
cd Mrecord/mrecord-java
# 安装依赖及其打包(需JDK17环境)
mvn clean install
```

### 项目结构

```
Mrecord/
├── README.md                 # 项目主文档
├── LICENSE                   # MIT 开源协议
├── README/                   # 文档图片资源
│   ├── logo1.jpg
│   └── promotion.png
│
├── mrecord-java/             # 后端服务（Spring Boot）
│   ├── pom.xml               # Maven 构建配置
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/com/dcz/mrecord/
│   │   │   │   ├── Application.java
│   │   │   │   ├── bo/                   # 业务对象
│   │   │   │   ├── common/               # 通用组件
│   │   │   │   ├── config/               # 配置类
│   │   │   │   ├── constant/             # 常量定义
│   │   │   │   ├── controller/           # 控制器层
│   │   │   │   ├── dto/                  # 数据传输对象
│   │   │   │   ├── entity/               # 实体类
│   │   │   │   ├── exception/            # 异常处理
│   │   │   │   ├── interceptor/          # 拦截器
│   │   │   │   ├── mapper/               # 数据访问层
│   │   │   │   ├── service/              # 业务逻辑层
│   │   │   │   └── util/                 # 工具类
│   │   │   └── resources/
│   │   │       ├── application.yaml      # 主配置文件
│   │   │       ├── application-dev.yaml  # 开发环境配置
│   │   │       ├── application-sit.yaml  # 测试环境配置
│   │   │       ├── mail/                 # 邮件模板
│   │   │       └── mapper/               # MyBatis XML 映射文件
│   │   └── test/                         # 单元测试
│   └── target/                           # 构建输出目录
│
├── mrecord-vue/              # 前端应用（Vue 3）
│   ├── package.json          # 项目依赖配置
│   ├── vite.config.ts        # Vite 构建配置
│   ├── index.html            # 入口 HTML
│   ├── tsconfig.json         # TypeScript 配置
│   ├── auto-imports.d.ts     # 自动导入类型声明
│   ├── components.d.ts       # 组件类型声明
│   ├── public/               # 静态资源
│   └── src/
│       ├── App.vue           # 根组件
│       ├── main.ts           # 入口脚本
│       ├── api/              # API 接口封装
│       ├── assets/           # 静态资源（图片、Markdown）
│       ├── components/       # 公共组件
│       ├── router/           # 路由配置
│       ├── stores/           # Pinia 状态管理
│       ├── types/            # TypeScript 类型声明
│       ├── utils/            # 工具函数
│       ├── views/            # 页面视图
│       └── style.css         # 全局样式
│
└── mrecord-sql/              # 数据库脚本
    ├── README.md
    ├── all/                  # 完整脚本
    │   ├── DDL/
    │   │   └── ddl.sql       # 完整数据库表结构
    │   └── DML/
    └── add/                  # 增量脚本
        ├── DDL/
        └── DML/
```

---

<a name="技术栈"></a>
# 技术栈

| 分类         | 技术                              |
| ------------ | --------------------------------- |
| 后端技术栈   | JDK17、Springboot4、MybatisFlex   |
| 后端构建工具 | Maven3.9                          |
| 前端技术栈   | Vue3、TypeScript、Pina、Varlet UI |
| 前端构建工具 | Vite8、NodeJs20.19                |

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


