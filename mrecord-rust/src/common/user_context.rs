//! 当前登录用户上下文
//!
//! 对应 Java: `com.dcz.mrecord.common.UserContext`（基于 ThreadLocal）。
//!
//! Rust/Axum 没有 ThreadLocal 概念，改为通过自定义提取器从请求头解析 JWT，
//! 直接拿到 `userId` 注入到 handler。
//!
//! 用法：
//! ```ignore
//! async fn my_handler(AuthUser(user_id): AuthUser, State(state): State<AppState>) { ... }
//! ```
//!
//! 管理员校验：使用 [`AdminUser`] 提取器（对应 Java 的 `@CheckAdmin` 注解）。

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sea_orm::EntityTrait;

use crate::{
    common::res_code::ResCode, constant::user_status::UserStatus, entity::sys_user, error::AppError,
    util::jwt, AppState,
};

/// 已登录用户提取器，仅包含 userId
///
/// 对应 Java: `UserContext.getUserId()`
pub struct AuthUser(pub String);

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // 从 Authorization 头读取 Bearer token
        // 兼容 Java 端的 "token" header 写法：优先 Authorization，再退回 token
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_start_matches("Bearer ").trim().to_string())
            .or_else(|| {
                parts
                    .headers
                    .get("token")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string())
            })
            .ok_or(AppError::ResCode(ResCode::LoginExpire))?;

        // 解析 JWT，失败或过期均视作登录过期
        let user_id = jwt::parse_user_id(&token, &app_state.jwt_secret)
            .ok_or(AppError::ResCode(ResCode::LoginExpire))?;

        Ok(AuthUser(user_id))
    }
}

/// 管理员用户提取器
///
/// 对应 Java: `@CheckAdmin` 注解 + `CheckAdminAspect` 切面。
///
/// 提取 JWT 后会再查一次数据库确认 admin 标志和用户状态，
/// 与 Java 切面行为一致。
pub struct AdminUser(pub String);

impl<S> FromRequestParts<S> for AdminUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 先复用普通用户校验逻辑
        let AuthUser(user_id) = AuthUser::from_request_parts(parts, state).await?;
        let app_state = AppState::from_ref(state);

        // 查库确认是管理员，且账号状态正常
        let user = sys_user::Entity::find_by_id(user_id.clone())
            .one(&app_state.db)
            .await?
            .ok_or(AppError::ResCode(ResCode::NoPermission))?;

        if user.admin != 1 {
            return Err(AppError::ResCode(ResCode::NoPermission));
        }
        if user.status != UserStatus::Normal as i32 {
            return Err(AppError::ResCode(ResCode::UserStatusError));
        }

        Ok(AdminUser(user_id))
    }
}
