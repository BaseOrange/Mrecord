//! 业务服务层
//!
//! 对应 Java 中各 `@Service` 类。Rust 项目早期把业务逻辑直接写在 handler 里，
//! 但像 `SysConfigService` 这类需要进程级缓存的服务必须独立放在这一层，
//! 通过 [`crate::AppState`] 的 `Arc<...>` 共享。
//!
//! 当前已实现：
//! - [`sys_config::SysConfigService`] — 系统配置项服务（邮件 SMTP、站点地址、注册开关等）

pub mod sys_config;
