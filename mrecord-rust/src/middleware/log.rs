//! 用户操作审计日志中间件
//!
//! 对应 Java: `LogInterceptor` + `CachedBodyFilter`。

use axum::{
    body::{Body, Bytes, to_bytes},
    extract::State,
    http::{Method, Request, StatusCode, header::HeaderMap},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use std::time::Instant;

use crate::{AppState, util::jwt};

const MAX_BODY_BYTES: usize = 1024 * 1024;
const SENSITIVE_FIELDS: &[&str] = &[
    "password",
    "oldPassword",
    "newPassword",
    "confirmPassword",
    "token",
    "jwtSecret",
    "activateTokenSecret",
    "resetPwdTokenSecret",
    "mailPass",
];

/// 写入操作审计日志，并把请求体重新放回请求中继续交给后续 handler。
///
/// 对应 Java: `LogInterceptor.preHandle`，排除规则对应 `WebConfig.addInterceptors`。
pub async fn operate_log(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let started_at = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let headers = request.headers().clone();
    let client_ip = extract_client_ip(&headers);
    let user_id = extract_user_id(&headers, &state.jwt_secret);
    let skip_audit_log = should_skip_log(&path);
    let (parts, body) = request.into_parts();
    let body_bytes = if should_cache_body(&method) {
        match to_bytes(body, MAX_BODY_BYTES).await {
            Ok(bytes) => bytes,
            Err(err) => {
                tracing::warn!(method = %method, path = %path, client_ip = %client_ip, "读取请求体失败: {:?}", err);
                return StatusCode::BAD_REQUEST.into_response();
            }
        }
    } else {
        Bytes::new()
    };
    let content = String::from_utf8_lossy(&body_bytes).to_string();
    let request_params = mask_request_body(&content);

    tracing::info!(
        method = %method,
        path = %path,
        client_ip = %client_ip,
        user_id = user_id.as_deref().unwrap_or(""),
        request_params = %request_params,
        "收到接口请求"
    );

    let request = Request::from_parts(parts, Body::from(body_bytes));

    if !skip_audit_log
        && let Err(err) = state
            .operate_log_service
            .save_log(
                &state.db,
                user_id.clone(),
                path.clone(),
                content,
                client_ip.clone(),
            )
            .await
    {
        tracing::error!(method = %method, path = %path, "保存操作审计日志失败: {:?}", err);
        return err.into_response();
    }

    let response = next.run(request).await;
    let status = response.status();
    let elapsed_ms = started_at.elapsed().as_millis();
    if status.is_server_error() {
        tracing::error!(method = %method, path = %path, status = %status, elapsed_ms, "接口请求异常结束");
    } else if status.is_client_error() {
        tracing::warn!(method = %method, path = %path, status = %status, elapsed_ms, "接口请求参数或客户端错误");
    } else {
        tracing::info!(method = %method, path = %path, status = %status, elapsed_ms, "接口请求处理完成");
    }
    response
}

/// 判断当前路径是否不需要写入操作日志。
///
/// 对应 Java `WebConfig.addInterceptors` 中 `logInterceptor` 的 `excludePathPatterns`：
/// `/api/v2/operateLog/list`、`/api/v2/config/initialized`、`/api/v2/config/registerEnabled`。
///
/// 【为什么必须排除 `/operateLog/list`】查询操作日志的接口本身若被记录，
/// 管理员每查看一次日志列表就新增一条日志，日志表会随查看次数无限膨胀
/// （前端 `AdminLogsPage` 还带下拉刷新 + 滚动分页，每次加载都触发新日志），
/// 形成事实上的死循环。这与 Java 端的排除语义一致。
///
/// 【前缀容错】Axum 的 `nest("/api/v2", ...)` 会剥掉外层前缀，中间件实际看到的是
/// `/operateLog/list`（已通过探针测试确认）。但为了防止日后把中间件挂到外层路由
/// （此时路径带 `/api/v2` 前缀）导致排除规则静默失效，这里统一先剥掉可选前缀再匹配，
/// 两种挂载位置下都能正确跳过。
fn should_skip_log(path: &str) -> bool {
    let normalized = path.strip_prefix("/api/v2").unwrap_or(path);
    matches!(
        normalized,
        "/operateLog/list" | "/config/initialized" | "/config/registerEnabled"
    )
}

/// 脱敏请求参数后输出到控制台，便于排查请求反序列化等进入 handler 前的问题。
fn mask_request_body(content: &str) -> String {
    if content.trim().is_empty() {
        return String::new();
    }

    match serde_json::from_str::<Value>(content) {
        Ok(mut value) => {
            mask_json_value(&mut value);
            value.to_string()
        }
        Err(_) => content.to_string(),
    }
}

/// 递归脱敏 JSON 中的敏感字段。
fn mask_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if SENSITIVE_FIELDS
                    .iter()
                    .any(|field| field.eq_ignore_ascii_case(key))
                {
                    *value = Value::String("******".to_string());
                } else {
                    mask_json_value(value);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                mask_json_value(item);
            }
        }
        _ => {}
    }
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

#[cfg(test)]
mod tests {
    //! 操作日志跳过规则的单元测试。
    //!
    //! 对应 REFACTOR_TODO 3.10：`should_skip_log` 必须对 `/api/v2` 前缀有无都正确跳过
    //! `/operateLog/list`，否则「查看日志 → 产生日志」形成死循环。

    use super::should_skip_log;

    #[test]
    fn skips_operate_log_list_without_prefix() {
        // Axum 的 nest 会剥掉 /api/v2 前缀，中间件实际看到的是这个路径
        assert!(should_skip_log("/operateLog/list"));
    }

    #[test]
    fn skips_operate_log_list_with_prefix() {
        // 防御：若日后把中间件挂到外层路由，路径会带 /api/v2 前缀，同样必须跳过
        assert!(should_skip_log("/api/v2/operateLog/list"));
    }

    #[test]
    fn skips_config_probe_endpoints() {
        // 前端登录前高频轮询的两个接口，对齐 Java excludePathPatterns
        assert!(should_skip_log("/config/initialized"));
        assert!(should_skip_log("/config/registerEnabled"));
        assert!(should_skip_log("/api/v2/config/initialized"));
        assert!(should_skip_log("/api/v2/config/registerEnabled"));
    }

    #[test]
    fn does_not_skip_business_endpoints() {
        // 业务接口必须正常记录
        assert!(!should_skip_log("/user/login"));
        assert!(!should_skip_log("/api/v2/user/login"));
        assert!(!should_skip_log("/book/create"));
        assert!(!should_skip_log("/exportTask/list"));
        // 仅前缀相似但路径不同，不能误跳过
        assert!(!should_skip_log("/operateLog/listDetail"));
        assert!(!should_skip_log("/api/v2/config/getEmailConfig"));
    }
}
