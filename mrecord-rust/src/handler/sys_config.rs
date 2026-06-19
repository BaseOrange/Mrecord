//! 系统配置项 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.SysConfigController`
//!
//! 业务逻辑全部委托给 [`crate::service::sys_config::SysConfigService`]。
//! 管理员保护通过 [`crate::common::user_context::AdminUser`] 提取器实现，
//! 等价于 Java 端的 `@CheckAdmin` 注解。

use axum::{extract::State, Json};

use crate::{
    common::{result::ApiResponse, user_context::AdminUser},
    error::AppError,
    handler::user as user_handler,
    model::{
        email::{EmailConfigBo, TestEmailDto, UpdateEmailConfigDto},
        site::{SiteConfigVo, UpdateSiteConfigDto},
        user::InitAdminDto,
    },
    AppState,
};

/// 刷新缓存：`POST /config/refreshCache`
///
/// 对应 Java: `SysConfigController.refreshCache`
pub async fn refresh_cache(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    tracing::info!("刷新配置项缓存[/config/refreshCache]请求");
    state.config_service.refresh_cache().await;
    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 获取系统初始化状态：`POST /config/initialized`
///
/// 对应 Java: `SysConfigController.initialized`
pub async fn initialized(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    tracing::info!("获取系统初始化状态[/config/initialized]请求");
    let initialized = state.config_service.is_initialized(&state.db).await?;
    Ok(Json(ApiResponse::success(initialized)))
}

/// 获取注册功能开关：`POST /config/registerEnabled`
///
/// 对应 Java: `SysConfigController.registerEnabled`
pub async fn register_enabled(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    tracing::info!("获取是否开启注册功能[/config/registerEnabled]请求");
    let enabled = state.config_service.is_register_enabled(&state.db).await?;
    Ok(Json(ApiResponse::success(enabled)))
}

/// 获取邮件配置（脱敏）：`POST /config/getEmailConfig`
///
/// 对应 Java: `SysConfigController.getEmailConfig`
pub async fn get_email_config(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Option<EmailConfigBo>>>, AppError> {
    tracing::info!("获取邮件配置[/config/getEmailConfig]请求");
    let cfg = state
        .config_service
        .get_masked_email_config(&state.db)
        .await?;
    Ok(Json(ApiResponse::success(cfg)))
}

/// 获取站点配置：`POST /config/getSiteConfig`
///
/// 对应 Java: `SysConfigController.getSiteConfig`
pub async fn get_site_config(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SiteConfigVo>>, AppError> {
    tracing::info!("获取站点配置[/config/getSiteConfig]请求");
    let vo = state.config_service.get_site_config(&state.db).await?;
    Ok(Json(ApiResponse::success(vo)))
}

/// 修改邮件配置：`POST /config/updateEmailConfig`
///
/// 对应 Java: `SysConfigController.updateEmailConfig`
pub async fn update_email_config(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(dto): Json<UpdateEmailConfigDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    tracing::info!("修改邮件配置[/config/updateEmailConfig]请求");
    state
        .config_service
        .update_email_config(&state.db, dto)
        .await?;
    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 修改站点配置：`POST /config/updateSiteConfig`
///
/// 对应 Java: `SysConfigController.updateSiteConfig`
pub async fn update_site_config(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(dto): Json<UpdateSiteConfigDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    tracing::info!("修改站点配置[/config/updateSiteConfig]请求");
    state
        .config_service
        .update_site_config(&state.db, dto)
        .await?;
    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 初始化管理员账户：`POST /config/initAdmin`
///
/// 对应 Java: `SysConfigController.initAdmin`。
/// 业务逻辑与 `/user/initAdmin` 完全相同，这里只是路径别名，直接复用同一个 handler。
pub async fn init_admin(
    state: State<AppState>,
    params: Json<InitAdminDto>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    user_handler::init_admin(state, params).await
}

/// 测试邮件发送：`POST /config/testEmail`
///
/// 对应 Java: `SysConfigController.testEmail`。
///
/// 当前 Rust 端尚未接入邮件发送实现，仅做日志记录后返回成功。
/// TODO: 接入 `EmailService` 后改为真正发送测试邮件。
pub async fn test_email(
    _admin: AdminUser,
    Json(dto): Json<TestEmailDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    tracing::info!(
        "测试邮件发送[/config/testEmail]请求 host={} from={} to={}",
        dto.host_name,
        dto.from,
        dto.test_to
    );
    // TODO: 接入邮件服务后改为真正发送
    Ok(Json(ApiResponse::<()>::success_empty()))
}
