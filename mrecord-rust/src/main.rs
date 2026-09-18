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
mod static_files;
mod util;

use std::sync::Arc;

use sea_orm::DatabaseConnection;
use std::net::SocketAddr;

use crate::service::{
    cancel_cleanup_task::CancelCleanupTask, email::EmailService, export_task::ExportTaskService,
    monthly_reminder_task::MonthlyReminderTask, sys_config::SysConfigService,
    sys_user_operate_log::SysUserOperateLogService, yearly_summary_task::YearlySummaryTask,
};

/// 全局应用状态，由 Axum 的 `with_state` 注入到所有 handler。
///
/// 安全配置项（三个密钥 + JWT 过期时长）在启动时由
/// [`config::load_security_config`] 从 `SYS_CONFIG` 表加载；密钥缺失时会自动
/// 生成并回写数据库，因此重启后保持稳定，已签发的 token 能跨重启被解析。
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
    /// 登录 JWT 过期时长（秒），由 `mr.jwtExpire`（毫秒）换算而来
    pub jwt_expire_secs: i64,
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
    /// 月度记账提醒定时任务
    ///
    /// 对应 Java `@Resource MonthlyReminderTask`，由启动流程注册 Tokio 后台循环执行。
    pub monthly_reminder_task: Arc<MonthlyReminderTask>,
    /// 年度总结邮件定时任务
    ///
    /// Java 端 `EmailService.sendNewYearReminderEmail` 原本无调用入口，由 Rust 端补齐
    /// 业务定义后两侧同步：每年 1 月 1 日 08:08 发送上一年度的财务总结邮件。
    pub yearly_summary_task: Arc<YearlySummaryTask>,
    /// 用户注销清理定时任务
    ///
    /// Java 端该方法仅预留注释（`SysUserServiceImpl.canceledMyUser` 中「后续会有单独的定时任务」），
    /// 由 Rust 端补齐：每日扫描冷静期已过的待注销用户并清理数据。
    pub cancel_cleanup_task: Arc<CancelCleanupTask>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mrecord_rust=info,tower_http=info".into()),
        )
        .init();

    let db = db::connect().await;

    // 安全配置（JWT / 令牌密钥、JWT 过期时长）从 SYS_CONFIG 加载，缺失则生成并回写
    let security = config::load_security_config(&db)
        .await
        .expect("Failed to load security config from SYS_CONFIG");

    let config_service = SysConfigService::new();
    let email_service = EmailService::new(config_service.clone());
    let export_task_service = ExportTaskService::new(email_service.clone());
    let operate_log_service = SysUserOperateLogService::new();
    let monthly_reminder_task = MonthlyReminderTask::new(email_service.clone());
    monthly_reminder_task.clone().start(db.clone());

    let yearly_summary_task = YearlySummaryTask::new(email_service.clone());
    yearly_summary_task.clone().start(db.clone());

    let cancel_cleanup_task = CancelCleanupTask::new();
    cancel_cleanup_task.clone().start(db.clone());

    let jwt_expire_secs = security.jwt_expire_secs();
    let state = AppState {
        db,
        jwt_secret: security.jwt_secret,
        activate_token_secret: security.activate_token_secret,
        reset_pwd_token_secret: security.reset_pwd_token_secret,
        jwt_expire_secs,
        config_service,
        email_service,
        export_task_service,
        operate_log_service,
        monthly_reminder_task,
        yearly_summary_task,
        cancel_cleanup_task,
    };

    let app = router::build(state);

    // 监听地址可由环境变量覆盖（`MRECORD_HOST` / `MRECORD_PORT`），默认仍为
    // 127.0.0.1:3000，保持本地 `cargo run` 行为不变；容器化部署时 Dockerfile
    // 设置 `MRECORD_HOST=0.0.0.0`，否则绑定 loopback 地址宿主机无法访问。
    let host = std::env::var("MRECORD_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("MRECORD_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("MRECORD_HOST / MRECORD_PORT 无法解析为 SocketAddr");
    println!("Mrecord-rs server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
