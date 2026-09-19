# Mrecord 飞牛 fnOS 打包目录

把 Rust 版 Mrecord 打包成飞牛 fnOS 应用（`.fpk`）的所有文件都在这个目录里。
**本目录对仓库其他部分零侵入**：构建脚本只读取 `mrecord-vue` / `mrecord-rust` 的产物，
不会修改它们的内容。

## 目录结构

```
mrecord-fnos/
├── manifest                        # 应用包描述（appname / 版本 / 入口 / 运行控制）
├── config/
│   ├── privilege                   # 以专用包用户 mrecord-fnos 运行
│   └── resource                    # 资源声明（当前为空，仅 JSON 占位）
├── cmd/                            # 生命周期脚本
│   ├── main                        # start/stop/status：选二进制 + cd 数据目录 + 传网关变量
│   ├── install_init / install_callback
│   ├── upgrade_init / upgrade_callback
│   ├── uninstall_init / uninstall_callback
│   └── config_init / config_callback
├── wizard/
│   └── install                     # 安装向导（仅说明，无需用户配置）
├── app/
│   ├── ui/
│   │   ├── config                  # 统一网关入口：/app/mrecord-fnos → app.sock
│   │   └── images/                 # 桌面图标 icon_64.png / icon_256.png
│   └── mrecord-rust-{x86_64,aarch64}  # 编译产物，由 build.sh 生成（已 gitignore）
├── scripts/
│   └── build.sh                    # 一键打包脚本
├── ICON.PNG                        # 64×64 包图标
└── ICON_256.PNG                    # 256×256 包图标
```

## 打包

```bash
./scripts/build.sh
```

脚本会依次：检查工具链 → 构建网关版前端 → 交叉编译 x86_64/aarch64 musl 二进制 →
拷进 `app/` → `fnpack build` → 生成 `mrecord-fnos.fpk`。

### 前置条件

| 工具 | 安装方式 |
|---|---|
| Node.js + yarn | https://nodejs.org |
| Rust | https://rustup.rs |
| cargo-zigbuild | `cargo install cargo-zigbuild`（musl 交叉编译，按其文档装 zig） |
| fnpack | 见 https://developer.fnnas.com/docs/cli/fnpack/#下载 |

脚本会自动 `rustup target add` 两个 musl target，缺工具时会给出安装提示。

### 常用参数

```bash
./scripts/build.sh x86_64        # 只编 x86_64（默认双架构）
./scripts/build.sh --no-restore  # 不恢复 mrecord-rust/static/ 的默认构建
```

## 设计要点

### 访问方式：统一网关（零端口冲突）

应用监听 `$TRIM_APPDEST/app.sock`（Unix socket），飞牛把
`https://你的NAS域名/app/mrecord-fnos` 的请求校验登录态后转发到该 socket。

- **不占用任何宿主机端口**，因此可以和 Docker 版（`-p 2333:2333`）同时运行，数据互相独立
- 前端与 API 路径都在 `/app/mrecord-fnos/` 前缀下：
  - 前端 `vite build --base=/app/mrecord-fnos/`
  - API 地址由构建期 `VITE_API_BASE_URL=/app/mrecord-fnos/api/v2` 指定
    （`mrecord-vue/src/utils/request.ts` 已支持该环境变量，**前端零代码改动**）
  - Rust 侧路由按 `MRECORD_GATEWAY_PREFIX` 挂到该前缀下

### 数据持久化

`data.db` 与 `exports/` 由应用按**工作目录相对路径**写入，`cmd/main` 启动前
`cd "$TRIM_PKGVAR"`，于是数据全部落在 fnOS 的持久化数据目录，跨升级保留。
SQLite 首次启动自动建表，JWT/令牌密钥自动生成并回写，**安装无需任何配置**。

### 架构支持

包内同时携带两个 musl 静态二进制，`cmd/main` 按 `TRIM_SYS_ARCH` 运行时选择，
因此 `manifest` 声明 `platform=all`。若后续发现 fnOS 对此有更严格的校验，
改为按架构分别出包即可（build.sh 已支持单架构参数）。

## ✅ Rust 侧网关改造已完成

`cmd/main` 启动应用时传入两个环境变量，`mrecord-rust` 已实现支持（均为**可选开关**，
不设置时行为完全不变，独立部署与 Docker 部署零影响）：

| 环境变量 | 作用 | 状态 |
|---|---|---|
| `MRECORD_GATEWAY_SOCKET` | 监听该 Unix socket 而非 TCP（不占任何宿主机端口） | ✅ 已实现并验证 |
| `MRECORD_GATEWAY_PREFIX` | 路由挂到 `/app/mrecord-fnos` 前缀下 | ✅ 已实现并验证 |

实现要点（详见 `mrecord-rust/src/main.rs` 与 `src/router/mod.rs`）：

- socket 模式下跳过 TCP 绑定，socket 权限设为 `0666`，避免网关进程与应用用户不同时连接被拒
- 路由 `nest` 到前缀下；另为「精确前缀根」（`/app/mrecord-fnos` 与 `/app/mrecord-fnos/`）
  显式挂了返回 SPA 首页的路由——axum 嵌套 fallback 只覆盖 `prefix/*`，不补这两条的话
  飞牛桌面入口打开会 404 白屏
- 已通过本地冒烟测试：前缀根 / SPA 路由 / 静态资源 / API 全部正常，TCP 端口确认未占用

后续可选增强：读取网关转发的 `X-Trim-Userid` / `X-Trim-Isadmin` / `X-Trim-Username`
header 作为可信身份上下文（应用自身的 JWT 用户体系保持不变，非必需）。

## 参考文档

- 打包工具 fnpack：https://developer.fnnas.com/docs/cli/fnpack/
- 应用框架 / 生命周期：https://developer.fnnas.com/docs/core-concepts/framework/
- Manifest：https://developer.fnnas.com/docs/core-concepts/manifest/
- 环境变量：https://developer.fnnas.com/docs/core-concepts/environment-variables/
- 统一网关：https://developer.fnnas.com/docs/core-concepts/gateway-registration/
- Native 应用案例：https://developer.fnnas.com/docs/examples/native/
