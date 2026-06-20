//! 导出任务模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.ExportTaskController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.ExportTaskServiceImpl`

use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::{
    AppState,
    common::{page::PageResult, res_code::ResCode, result::ApiResponse, user_context::AuthUser},
    constant::export_task::ExportTaskStatus,
    entity::sys_export_task::{Column as ExportTaskCol, Entity as ExportTaskEntity},
    error::AppError,
    model::{
        export_task::{ExportTaskResponse, QueryExportTaskDto},
        finance::ExportBookDto,
    },
};

/// 构造参数错误业务异常。
///
/// 对应 Java: `new MrecordException(ResCode.PARAM_ERROR.getCode(), message)`。
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 提交导出任务：`POST /exportTask/export`。
///
/// 对应 Java: `ExportTaskController.export` 与 `ExportTaskServiceImpl.export`。
pub async fn export(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<ExportBookDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state
        .export_task_service
        .clone()
        .create_export_tasks(state.db.clone(), user_id, params)
        .await?;
    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 查询当前用户导出任务列表：`POST /exportTask/list`。
///
/// 对应 Java: `ExportTaskController.list`。
pub async fn list(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<QueryExportTaskDto>,
) -> Result<Json<ApiResponse<PageResult<ExportTaskResponse>>>, AppError> {
    let mut q = ExportTaskEntity::find()
        .filter(ExportTaskCol::UserId.eq(user_id))
        .filter(ExportTaskCol::IsDeleted.eq(0));

    if let Some(status) = params.status.filter(|s| !s.trim().is_empty()) {
        let normalized = status.trim().to_ascii_uppercase();
        ExportTaskStatus::from_str(&normalized).ok_or_else(|| param_err("导出任务状态错误"))?;
        q = q.filter(ExportTaskCol::Status.eq(normalized));
    }

    let q = q.order_by_desc(ExportTaskCol::CreateTime);
    let page_num = params.page.page_num.max(1) as u64;
    let page_size = params.page.page_size.max(1) as u64;
    let paginator = q.paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let records = paginator.fetch_page(page_num - 1).await?;
    let result = PageResult::new(
        records.into_iter().map(ExportTaskResponse::from).collect(),
        total,
        page_num,
        page_size,
    );

    Ok(Json(ApiResponse::success(result)))
}
