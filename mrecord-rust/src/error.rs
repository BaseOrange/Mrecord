//! 统一错误类型与全局异常处理
//!
//! 对应 Java:
//! - `com.dcz.mrecord.exception.MrecordException` — 自定义业务异常
//! - `com.dcz.mrecord.exception.GlobalExceptionHandler` — 全局异常捕获
//!
//! 在 Axum 中，`AppError` 实现 `IntoResponse` 即等价于 `@RestControllerAdvice` 的效果，
//! handler 里直接 `?` 抛出即可被自动转为统一 JSON 响应。

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::common::res_code::ResCode;
use crate::common::result::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    /// 资源不存在
    NotFound,
    /// 业务异常（预定义响应码），对应 Java `new MrecordException(ResCode)`
    ResCode(ResCode),
    /// 业务异常（自定义 code + message），对应 Java `new MrecordException(code, message)`
    Business { code: String, message: String },
    /// 内部错误（兜底），对应 Java `@ExceptionHandler(Exception.class)`
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => {
                let body = ApiResponse::<()>::fail(ResCode::DataNotExist);
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
            AppError::ResCode(rc) => {
                let body = ApiResponse::<()>::fail(rc);
                (StatusCode::OK, Json(body)).into_response()
            }
            AppError::Business { code, message } => {
                let body = ApiResponse::<()>::fail_with(code, message);
                (StatusCode::OK, Json(body)).into_response()
            }
            AppError::Internal(e) => {
                tracing::error!("internal error: {:?}", e);
                let body = ApiResponse::<()>::fail(ResCode::Error);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Internal(e)
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        AppError::Internal(e.into())
    }
}

impl From<ResCode> for AppError {
    fn from(rc: ResCode) -> Self {
        AppError::ResCode(rc)
    }
}
