//! 诊断信息 DTO
//!
//! 供前端「我的 → 点击顶部标题 5 次」打开的诊断面板展示，用于用户在 GitHub
//! 提 issue 时一键复制环境信息，帮助开发者快速定位问题。
//!
//! 【隐私红线】月衡是记账应用，诊断信息会被用户粘贴到**公开**的 GitHub issue
//! 里，因此本 DTO 只包含环境元数据与**计数**：
//! - 绝不包含任何金额、资产/负债数值；
//! - 绝不包含邮箱、昵称、手机号、密码、token；
//! - 绝不包含账簿名、模板条目名、备注内容；
//! - 只有 ID 级信息（后端按当前登录用户过滤后返回的计数）。

use serde::Serialize;

/// 诊断信息响应
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticResponse {
    /// 后端实现类型：Rust 版固定 `rust`（Java 版为 `java`），用于区分 fnOS 版与 Docker 版
    pub backend: String,
    /// 后端框架（如 `axum 0.8`）
    pub backend_framework: String,
    /// 后端应用版本（`CARGO_PKG_VERSION`，与 `manifest` / `Cargo.toml` 一致）
    pub app_version: String,
    /// Rust 工具链版本（由 `build.rs` 编译期注入）
    pub rust_version: String,
    /// 后端二进制构建时间（UTC，由 `build.rs` 编译期注入）
    pub build_time: String,
    /// 后端进程已运行时长（秒）
    pub uptime_secs: u64,
    /// 宿主操作系统（`std::env::consts::OS`，如 `linux` / `macos`）
    pub os: String,
    /// CPU 架构（`std::env::consts::ARCH`，如 `x86_64` / `aarch64`）
    pub arch: String,
    /// 部署模式：fnOS 网关模式返回前缀，独立部署返回 `standalone`
    pub deploy_mode: String,
    /// 数据库类型（固定 `sqlite`）
    pub db_type: String,
    /// 数据库文件大小（字节）
    pub db_size_bytes: u64,
    /// 当前登录用户的数据规模计数（不含金额）
    pub data_counts: DataCounts,
}

/// 当前登录用户的数据规模计数
///
/// 【隐私】只有各表行数，不含任何金额或名称。
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCounts {
    /// 账簿数量
    pub book_count: u64,
    /// 模板条目数量
    pub template_item_count: u64,
    /// 月度汇总记录数量
    pub month_record_count: u64,
    /// 月度明细记录数量
    pub month_item_count: u64,
}
