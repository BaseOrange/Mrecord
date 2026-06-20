//! 统一响应体
//!
//! 对应 Java: `com.dcz.mrecord.common.Result<T>`
//!
//! 注意：Rust 标准库已有 `Result` 类型，因此命名为 `ApiResponse` 避免冲突。

use axum::{Json, response::IntoResponse};
use serde::Serialize;

use super::res_code::ResCode;

/// 统一 API 响应结构
///
/// 所有接口返回 `{ "code": "...", "message": "...", "data": ... }` 格式。
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    /// 响应码（如 "00000" 成功）
    pub code: String,
    /// 响应消息
    pub message: String,
    /// 响应数据，失败时为 null
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功返回（带数据）
    pub fn success(data: T) -> Self {
        Self {
            code: ResCode::Success.code().to_string(),
            message: ResCode::Success.message().to_string(),
            data: Some(data),
        }
    }

    /// 成功返回（无数据）
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            code: ResCode::Success.code().to_string(),
            message: ResCode::Success.message().to_string(),
            data: None,
        }
    }

    /// 失败返回（使用预定义响应码）
    pub fn fail(res_code: ResCode) -> Self {
        Self {
            code: res_code.code().to_string(),
            message: res_code.message().to_string(),
            data: None,
        }
    }

    /// 失败返回（自定义 code + message）
    pub fn fail_with(code: String, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
