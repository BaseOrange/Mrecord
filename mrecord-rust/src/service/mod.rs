//! 业务服务层
//!
//! 对应 Java 中各 `@Service` 类。Rust 项目早期把业务逻辑直接写在 handler 里，
//! 但像 `SysConfigService`/`EmailService` 这类需要进程级缓存或可复用客户端的服务
//! 必须独立放在这一层，通过 [`crate::AppState`] 的 `Arc<...>` 共享。
//!
//! 当前已实现：
//! - [`sys_config::SysConfigService`] — 系统配置项服务（邮件 SMTP、站点地址、注册开关等）
//! - [`email::EmailService`] — 邮件发送服务（SMTP/SMTPS via lettre）

pub mod email;
pub mod export_task;
pub mod sys_config;
