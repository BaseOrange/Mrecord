//! 用户操作审计日志中间件
//!
//! 对应 Java: `LogInterceptor` + `CachedBodyFilter`。

use axum::{
    body::{Body, to_bytes},
    extract::State,
    http::{Method, Request, StatusCode, header::HeaderMap},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{AppState, util::jwt};

const MAX_BODY_BYTES: usize = 1024 * 1024;

/// 写入操作审计日志，并把请求体重新放回请求中继续交给后续 handler。
///
/// 对应 Java: `LogInterceptor.preHandle`，排除规则对应 `WebConfig.addInterceptors`。
pub async fn operate_log(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if should_skip_log(request.uri().path()) {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let headers = request.headers().clone();
    let (parts, body) = request.into_parts();
    let body_bytes = if should_cache_body(&method) {
        match to_bytes(body, MAX_BODY_BYTES).await {
            Ok(bytes) => bytes,
            Err(_) => return StatusCode::BAD_REQUEST.into_response(),
        }
    } else {
        Default::default()
    };
    let content = String::from_utf8_lossy(&body_bytes).to_string();
    let request = Request::from_parts(parts, Body::from(body_bytes));

    if let Err(err) = state
        .operate_log_service
        .save_log(
            &state.db,
            extract_user_id(&headers, &state.jwt_secret),
            path,
            content,
            extract_client_ip(&headers),
        )
        .await
    {
        tracing::error!("保存操作审计日志失败: {:?}", err);
        return err.into_response();
    }

    next.run(request).await
}

/// 判断当前路径是否不需要写入操作日志。
fn should_skip_log(path: &str) -> bool {
    matches!(
        path,
        "/operateLog/list" | "/config/initialized" | "/config/registerEnabled"
    )
}

/// 判断请求方法是否需要缓存请求体。
fn should_cache_body(method: &Method) -> bool {
    matches!(method, &Method::POST | &Method::PUT | &Method::PATCH)
}

/// 从请求头中解析当前用户 ID。
fn extract_user_id(headers: &HeaderMap, jwt_secret: &str) -> Option<String> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").trim().to_string())
        .or_else(|| {
            headers
                .get("token")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })?;
    jwt::parse_user_id(&token, jwt_secret)
}

/// 获取客户端 IP。
fn extract_client_ip(headers: &HeaderMap) -> String {
    headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(str::trim)
        .filter(|v| !v.is_empty() && !v.eq_ignore_ascii_case("unknown"))
        .or_else(|| {
            headers
                .get("Proxy-Client-IP")
                .and_then(|v| v.to_str().ok())
                .map(str::trim)
                .filter(|v| !v.is_empty() && !v.eq_ignore_ascii_case("unknown"))
        })
        .or_else(|| {
            headers
                .get("X-Real-IP")
                .and_then(|v| v.to_str().ok())
                .map(str::trim)
                .filter(|v| !v.is_empty() && !v.eq_ignore_ascii_case("unknown"))
        })
        .unwrap_or("")
        .to_string()
}
