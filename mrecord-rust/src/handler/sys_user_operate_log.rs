//! 用户操作审计日志模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.SysUserOperateLogController`。

use axum::{Json, extract::State};

use crate::{
    AppState,
    common::{page::PageResult, result::ApiResponse, user_context::AdminUser},
    error::AppError,
    model::sys_user_operate_log::{OperateLogResponse, QueryOperateLogDto},
};

/// 分页查询用户操作审计日志列表：`POST /operateLog/list`。
///
/// 对应 Java: `SysUserOperateLogController.list`，仅管理员可访问。
pub async fn list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(params): Json<QueryOperateLogDto>,
) -> Result<Json<ApiResponse<PageResult<OperateLogResponse>>>, AppError> {
    let page_num = params.page.page_num.max(1) as u64;
    let page_size = params.page.page_size.max(1) as u64;
    let result = state
        .operate_log_service
        .query_list(&state.db, page_num, page_size)
        .await?;
    Ok(Json(ApiResponse::success(result)))
}
