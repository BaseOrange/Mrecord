//! HTTP 路由表
//!
//! 对应 Java 中各 `@RestController` 的 `@RequestMapping`。
//! 所有路由集中在此处注册，方便统一查阅。

use axum::{Router, middleware, routing::post};

use crate::{AppState, handler};

/// 构建应用路由
pub fn build(state: AppState) -> Router {
    let api_routes = Router::new()
        // ==================== 用户模块 ====================
        // 对应 Java: SysUserController（@RequestMapping("/user")）
        .route("/user/initAdmin", post(handler::user::init_admin))
        .route("/user/register", post(handler::user::register))
        .route("/user/activate", post(handler::user::activate))
        .route(
            "/user/resendActivateEmail",
            post(handler::user::resend_activate_email),
        )
        .route("/user/login", post(handler::user::login))
        .route("/user/logout", post(handler::user::logout))
        .route("/user/forgotPassword", post(handler::user::forgot_password))
        .route("/user/resetPassword", post(handler::user::reset_password))
        .route("/user/changePassword", post(handler::user::change_password))
        .route("/user/queryMyInfo", post(handler::user::query_my_info))
        .route("/user/updateMyInfo", post(handler::user::update_my_info))
        .route(
            "/user/canceledMyUser",
            post(handler::user::canceled_my_user),
        )
        // 撤销注销（免登录：注销冷静期内用户无法登录，入口在登录页）
        .route("/user/revokeCancel", post(handler::user::revoke_cancel))
        // 管理员接口（内部通过 AdminUser 提取器进行权限校验）
        .route("/user/list", post(handler::user::admin_query_list))
        .route(
            "/user/queryUserInfo",
            post(handler::user::admin_query_user_info),
        )
        .route(
            "/user/adminResetPassword",
            post(handler::user::admin_reset_password),
        )
        .route(
            "/user/enableOrDisableUser",
            post(handler::user::admin_enable_or_disable),
        )
        .route("/user/deleteUser", post(handler::user::admin_delete_user))
        // ==================== 账簿模块 ====================
        // 对应 Java: FinBookController（@RequestMapping("/book")）
        .route("/book/create", post(handler::fin_book::create))
        .route("/book/update", post(handler::fin_book::update))
        .route("/book/delete", post(handler::fin_book::delete))
        .route("/book/list", post(handler::fin_book::list))
        .route(
            "/book/getMyDataStatistics",
            post(handler::fin_book::get_my_data_statistics),
        )
        .route(
            "/book/getBookDetailedStatistics",
            post(handler::fin_book::get_book_detailed_statistics),
        )
        // ==================== 模板项模块 ====================
        // 对应 Java: FinTemplateItemController（@RequestMapping("/tempItem")）
        .route("/tempItem/create", post(handler::fin_template_item::create))
        .route("/tempItem/update", post(handler::fin_template_item::update))
        .route("/tempItem/copy", post(handler::fin_template_item::copy))
        .route("/tempItem/list", post(handler::fin_template_item::list))
        // ==================== 月度汇总模块 ====================
        // 对应 Java: FinMonthRecordController（@RequestMapping("/monthRecord")）
        .route(
            "/monthRecord/getMonthRecord",
            post(handler::fin_month_record::get_month_record),
        )
        .route(
            "/monthRecord/getYearRecordList",
            post(handler::fin_month_record::get_year_record_list),
        )
        // ==================== 月度明细模块 ====================
        // 对应 Java: FinMonthItemRecordController（@RequestMapping("/monthItem")）
        .route(
            "/monthItem/insertMonthItem",
            post(handler::fin_month_item_record::insert_month_item),
        )
        .route(
            "/monthItem/updateMonthItem",
            post(handler::fin_month_item_record::update_month_item),
        )
        .route(
            "/monthItem/queryMonthItem",
            post(handler::fin_month_item_record::query_month_item),
        )
        .route(
            "/monthItem/queryAll",
            post(handler::fin_month_item_record::query_all),
        )
        // ==================== 导出任务模块 ====================
        // 对应 Java: ExportTaskController（@RequestMapping("/exportTask")）
        .route("/exportTask/export", post(handler::sys_export_task::export))
        .route("/exportTask/list", post(handler::sys_export_task::list))
        // ==================== 操作审计日志模块 ====================
        // 对应 Java: SysUserOperateLogController（@RequestMapping("/operateLog")）
        .route(
            "/operateLog/list",
            post(handler::sys_user_operate_log::list),
        )
        // ==================== 配置项模块 ====================
        // 对应 Java: SysConfigController（@RequestMapping("/config")）
        .route(
            "/config/refreshCache",
            post(handler::sys_config::refresh_cache),
        )
        .route(
            "/config/initialized",
            post(handler::sys_config::initialized),
        )
        .route(
            "/config/registerEnabled",
            post(handler::sys_config::register_enabled),
        )
        .route(
            "/config/getEmailConfig",
            post(handler::sys_config::get_email_config),
        )
        .route(
            "/config/getSiteConfig",
            post(handler::sys_config::get_site_config),
        )
        .route(
            "/config/updateEmailConfig",
            post(handler::sys_config::update_email_config),
        )
        .route(
            "/config/updateSiteConfig",
            post(handler::sys_config::update_site_config),
        )
        .route("/config/initAdmin", post(handler::sys_config::init_admin))
        .route("/config/testEmail", post(handler::sys_config::test_email))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::log::operate_log,
        ));

    Router::new()
        .nest("/api/v2", api_routes)
        // 静态资源走编译期内嵌（详见 `static_files` 模块）：不再依赖运行时 cwd
        // 与磁盘上的 `static/` 目录，容器外启动、命名数据卷升级等场景下前端
        // 不会再整体 404。`MRECORD_STATIC_DIR` 可切回磁盘模式以热替换前端。
        .fallback_service(crate::static_files::static_service())
        .with_state(state)
}

#[cfg(test)]
mod tests {

    //! 操作日志中间件挂载在真实路由树上的回归测试。
    //!
    //! 对应 REFACTOR_TODO 3.10：验证「查询操作日志」本身不会被写入操作日志，
    //! 避免日志表随查看次数无限膨胀（前端 `AdminLogsPage` 还带下拉刷新 + 滚动分页）。
    //!
    //! 之所以在 `router` 模块做集成测试而非只测 `should_skip_log` 纯函数：
    //! 跳过逻辑依赖 Axum `nest("/api/v2", ...)` 会剥掉外层前缀这一行为，
    //! 只有驱动完整路由树才能证明端到端确实跳过（曾用探针确认中间件看到的是
    //! `/operateLog/list` 而非 `/api/v2/operateLog/list`）。

    /// 原生 COUNT 查询行（Sea-ORM 2.0 的 `query_one` 只接受 `StatementBuilder`，原生 SQL 走 `find_by_statement`）。
    #[derive(Clone, Debug, PartialEq, FromQueryResult)]
    struct CountRow {
        c: i64,
    }

    use super::*;
    use crate::service::{
        cancel_cleanup_task::CancelCleanupTask, email::EmailService,
        export_task::ExportTaskService, monthly_reminder_task::MonthlyReminderTask,
        sys_config::SysConfigService, sys_user_operate_log::SysUserOperateLogService,
        yearly_summary_task::YearlySummaryTask,
    };
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use sea_orm::{
        ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult,
        Statement,
    };
    use tower::util::ServiceExt;

    /// 建日志表与用户表（`/user/login` 与中间件落库需要）。
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接内存库失败");
        for ddl in [
            "CREATE TABLE IF NOT EXISTS SYS_USER_OPERATE_LOG (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_OPERATE_TYPE TEXT,
                MR_CONTENT TEXT, MR_IP TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_USER (
                MR_ID TEXT PRIMARY KEY, MR_EMAIL TEXT, MR_PASSWORD TEXT, MR_NICKNAME TEXT,
                MR_ADMIN INTEGER DEFAULT 0, MR_STATUS INTEGER DEFAULT 1, MR_AVATAR TEXT,
                MR_REMIND_ENABLED INTEGER, MR_REMIND_DAY INTEGER, MR_CANCEL_TIME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db
    }

    fn test_state(db: DatabaseConnection) -> AppState {
        let config_service = SysConfigService::new();
        let email_service = EmailService::new(config_service.clone());
        AppState {
            db,
            jwt_secret: "test-secret".to_string(),
            activate_token_secret: "test-secret".to_string(),
            reset_pwd_token_secret: "test-secret".to_string(),
            jwt_expire_secs: 604800,
            config_service,
            email_service: email_service.clone(),
            export_task_service: ExportTaskService::new(email_service.clone()),
            operate_log_service: SysUserOperateLogService::new(),
            monthly_reminder_task: MonthlyReminderTask::new(email_service.clone()),
            yearly_summary_task: YearlySummaryTask::new(email_service.clone()),
            cancel_cleanup_task: CancelCleanupTask::new(),
        }
    }

    async fn count_logs(db: &DatabaseConnection) -> i64 {
        let row = CountRow::find_by_statement(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS c FROM SYS_USER_OPERATE_LOG",
        ))
        .one(db)
        .await
        .unwrap()
        .unwrap();
        row.c
    }

    /// 查询操作日志列表本身**不落日志**——这是防止「查看日志 → 产生日志」死循环的关键。
    #[tokio::test]
    async fn listing_operate_logs_does_not_write_a_log_row() {
        let db = setup_db().await;
        let app = build(test_state(db.clone()));

        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v2/operateLog/list")
                    .header("Content-Type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 未登录会返回业务错误（LoginExpire），但中间件早已执行完毕——
        // 关键断言在库里的行数，而非响应状态
        assert_ne!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            count_logs(&db).await,
            0,
            "查询操作日志不得写入日志行，否则会形成死循环导致日志无限膨胀"
        );
    }

    /// 反证：非排除列表的接口（登录）会正常落一条日志，证明跳过是「仅排除列表生效」而非整个中间件失效。
    #[tokio::test]
    async fn non_excluded_endpoint_writes_a_log_row() {
        let db = setup_db().await;
        let app = build(test_state(db.clone()));

        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v2/user/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        r#"{"email":"nobody@example.com","password":"x"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            count_logs(&db).await,
            1,
            "登录接口应写入一条操作日志（中间件确实在工作）"
        );
    }
}
