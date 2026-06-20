//! HTTP 路由表
//!
//! 对应 Java 中各 `@RestController` 的 `@RequestMapping`。
//! 所有路由集中在此处注册，方便统一查阅。

use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, handler};

/// 构建应用路由
pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello! Welcome To Mrecord!" }))
        // record 模块（示例 / 旧测试用）
        .route(
            "/records",
            get(handler::record::list).post(handler::record::create),
        )
        .route(
            "/records/{id}",
            get(handler::record::get).delete(handler::record::delete),
        )
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
        .with_state(state)
}
