mod common;
mod config;
mod constant;
mod db;
mod entity;
mod error;
mod handler;
mod middleware;
mod model;
mod router;
mod service;
mod util;

use std::sync::Arc;

use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tracing_subscriber;

use crate::service::{
    email::EmailService, export_task::ExportTaskService, sys_config::SysConfigService,
    sys_user_operate_log::SysUserOperateLogService,
};

/// 全局应用状态，由 Axum 的 `with_state` 注入到所有 handler。
///
/// 暂时把 token 密钥硬编码在此处；后续将由 `config.rs` 统一加载（环境变量 / `SYS_CONFIG` 表）。
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接
    pub db: DatabaseConnection,
    /// 登录 JWT 密钥（对应 Java `MrConf.jwtSecret`）
    pub jwt_secret: String,
    /// 账户激活令牌密钥（对应 Java `MrConf.activateTokenSecret`）
    pub activate_token_secret: String,
    /// 重置密码令牌密钥（对应 Java `MrConf.resetPwdTokenSecret`）
    pub reset_pwd_token_secret: String,
    /// 系统配置项服务（持有进程级缓存）
    ///
    /// 对应 Java `@Resource SysConfigService`。使用 `Arc` 在多个 handler 间共享同一份缓存。
    pub config_service: Arc<SysConfigService>,
    /// 邮件发送服务
    ///
    /// 对应 Java `@Resource EmailService`。SMTP 配置从 `config_service` 现取，便于热更新。
    pub email_service: Arc<EmailService>,
    /// 导出任务服务
    ///
    /// 对应 Java `@Resource ExportTaskService`，负责创建异步导出任务并生成 Excel 附件。
    pub export_task_service: Arc<ExportTaskService>,
    /// 用户操作审计日志服务
    ///
    /// 对应 Java `@Resource SysUserOperateLogService`，负责记录请求日志和管理员查询审计日志。
    pub operate_log_service: Arc<SysUserOperateLogService>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = db::connect().await;
    let config_service = SysConfigService::new();
    let email_service = EmailService::new(config_service.clone());
    let export_task_service = ExportTaskService::new(email_service.clone());
    let operate_log_service = SysUserOperateLogService::new();

    // TODO: 这些密钥应从配置文件 / 环境变量加载，避免硬编码
    let state = AppState {
        db,
        jwt_secret: "mrecord-dev-jwt-secret-please-change-me".to_string(),
        activate_token_secret: "mrecord-dev-activate-secret-please-change".to_string(),
        reset_pwd_token_secret: "mrecord-dev-reset-pwd-secret-please-change".to_string(),
        config_service,
        email_service,
        export_task_service,
        operate_log_service,
    };

    let app = router::build(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Mrecord-rs server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
