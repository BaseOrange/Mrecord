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

// Unix domain socket 仅存在于 Unix 平台；Windows 构建跳过网关分支（见 main 中的 cfg）
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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

    // 飞牛 fnOS 统一网关模式：监听 Unix socket 而非 TCP。
    //
    // 飞牛把 `/app/mrecord-fnos` 下的请求校验登录态后转发到 `$TRIM_APPDEST/app.sock`，
    // 应用自身不需要（也不应该）占用任何宿主机 TCP 端口——这正是飞牛版能与
    // Docker 版（`-p 2333:2333`）同时运行、数据互相独立的原因。
    //
    // 这是**可选开关**：未设置 `MRECORD_GATEWAY_SOCKET` 时直接跳到下面的 TCP 分支，
    // 本地 `cargo run`、Docker 部署的路由与监听逻辑与改造前完全一致。
    #[cfg(unix)]
    if let Some(socket_path) = env_nonempty("MRECORD_GATEWAY_SOCKET") {
        // 残留的旧 socket 文件会让 bind 失败（飞牛 cmd/main 也会先删，这里双保险）
        let _ = std::fs::remove_file(&socket_path);
        let listener = tokio::net::UnixListener::bind(&socket_path)
            .unwrap_or_else(|e| panic!("无法绑定 Unix socket「{socket_path}」: {e}"));
        // 网关转发进程与应用可能不是同一用户，放宽权限避免连接被拒
        let _ = std::fs::set_permissions(&socket_path, PermissionsExt::from_mode(0o666));
        println!("Mrecord-rs server listening on unix:{socket_path}");
        axum::serve(listener, app).await.unwrap();
        return;
    }

    // 监听地址可由环境变量覆盖（`MRECORD_HOST` / `MRECORD_PORT`），默认仍为
    // 127.0.0.1:2333（与 Java 版对齐，前端 dev 代理也指向该端口），保持本地
    // `cargo run` 行为不变；容器化部署时 Dockerfile 设置 `MRECORD_HOST=0.0.0.0`，
    // 否则绑定 loopback 地址宿主机无法访问。
    let host = std::env::var("MRECORD_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("MRECORD_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(2333);
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("MRECORD_HOST / MRECORD_PORT 无法解析为 SocketAddr");
    println!("Mrecord-rs server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

/// 读取环境变量：未设置或空白统一视作未设置
///
/// 供网关模式两个开关使用（`MRECORD_GATEWAY_SOCKET` / `MRECORD_GATEWAY_PREFIX`），
/// 与现有 `MRECORD_HOST` 等变量的「空值即默认」语义保持一致。
fn env_nonempty(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    }
}
