//! 月度财务汇总模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinMonthRecordController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinMonthRecordServiceImpl`

use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    AppState,
    common::{res_code::ResCode, result::ApiResponse, user_context::AuthUser},
    entity::{
        fin_book,
        fin_book::{Column as BookCol, Entity as BookEntity},
        fin_month_record::{Column as MonthRecordCol, Entity as MonthRecordEntity},
    },
    error::AppError,
    model::{
        finance::{MonthRecordDto, MonthRecordResponse},
    },
};

/// 构造参数错误业务异常。
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 校验账簿存在且属于当前登录用户。
async fn check_book_ownership(
    state: &AppState,
    book_id: &str,
    user_id: &str,
) -> Result<fin_book::Model, AppError> {
    let book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;

    Ok(book)
}

/// 获取月度财务汇总：`POST /monthRecord/getMonthRecord`
///
/// 对应 Java: `FinMonthRecordController.getMonthRecord` 与 `FinMonthRecordServiceImpl.getMonthRecord`
pub async fn get_month_record(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthRecordDto>,
) -> Result<Json<ApiResponse<Option<MonthRecordResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }
    let year = params.year.ok_or_else(|| param_err("年份不能为空"))?;
    let month = params.month.ok_or_else(|| param_err("月份不能为空"))?;

    // 校验账簿权限
    let _ = check_book_ownership(&state, book_id, &user_id).await?;

    let record = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id))
        .filter(MonthRecordCol::Year.eq(year))
        .filter(MonthRecordCol::Month.eq(month))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?;

    let response = record.map(MonthRecordResponse::from);
    Ok(Json(ApiResponse::success(response)))
}

/// 获取年度财务汇总列表：`POST /monthRecord/getYearRecordList`
///
/// 对应 Java: `FinMonthRecordController.getYearRecordList` 与 `FinMonthRecordServiceImpl.getYearRecordList`
pub async fn get_year_record_list(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthRecordDto>,
) -> Result<Json<ApiResponse<Vec<MonthRecordResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 校验账簿权限
    let _ = check_book_ownership(&state, book_id, &user_id).await?;

    let mut query = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id))
        .filter(MonthRecordCol::IsDeleted.eq(0));

    // 如果指定了年份，添加年份过滤
    if let Some(year) = params.year {
        query = query.filter(MonthRecordCol::Year.eq(year));
    }

    let records = query
        .order_by_asc(MonthRecordCol::Year)
        .order_by_asc(MonthRecordCol::Month)
        .all(&state.db)
        .await?;

    let result = records.into_iter().map(MonthRecordResponse::from).collect();

    Ok(Json(ApiResponse::success(result)))
}
