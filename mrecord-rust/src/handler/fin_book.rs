//! 财务账簿模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinBookController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinBookServiceImpl`

use std::collections::HashMap;

use axum::{Json, extract::State};
use chrono::{Datelike, Months, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{page::PageResult, res_code::ResCode, result::ApiResponse, user_context::AuthUser},
    entity::{
        fin_book::{self, ActiveModel as BookActive, Column as BookCol, Entity as BookEntity},
        fin_month_item_record::{Column as MonthItemCol, Entity as MonthItemEntity},
        fin_month_record::{Column as MonthRecordCol, Entity as MonthRecordEntity},
        fin_template_item::{Column as TemplateItemCol, Entity as TemplateItemEntity},
        sys_backup_book::ActiveModel as BackupBookActive,
    },
    error::AppError,
    model::{
        finance::{
            CreateUpdateBookDto, DataStatisticsResponse, FinBookRecordResponse, FinBookResponse,
            MonthRecordResponse, QueryFinBookDto,
        },
        id_dto::IdDto,
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

/// 构造无权限业务异常。
///
/// 对应 Java: `new MrecordException(ResCode.NO_PERMISSION.getCode(), message)`。
fn no_permission(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::NoPermission.code().to_string(),
        message: msg.into(),
    }
}

/// 校验账簿存在且属于当前登录用户。
///
/// 对应 Java: `FinBookServiceImpl.checkUpdateMyFinBook(String finBookId, String userId)`。
async fn check_book_ownership(
    state: &AppState,
    book_id: &str,
    user_id: &str,
) -> Result<fin_book::Model, AppError> {
    let book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::IsDeleted.eq(0))
        .one(&state.db)
        .await?
        .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;

    if book.user_id != user_id {
        return Err(no_permission("无该账簿权限，相关操作已记录"));
    }

    Ok(book)
}

/// 创建账簿：`POST /book/create`。
///
/// 对应 Java: `FinBookController.create` 与 `FinBookServiceImpl.createFinBook`。
pub async fn create(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<CreateUpdateBookDto>,
) -> Result<Json<ApiResponse<FinBookResponse>>, AppError> {
    if params.book_name.trim().is_empty() {
        return Err(param_err("账簿名称不能为空"));
    }

    let active = BookActive {
        id: Set(Uuid::new_v4().simple().to_string()),
        user_id: Set(user_id.clone()),
        book_name: Set(params.book_name),
        create_by: Set(Some(user_id)),
        ..Default::default()
    };
    let book = active.insert(&state.db).await?;

    Ok(Json(ApiResponse::success(book.into())))
}

/// 更新账簿：`POST /book/update`。
///
/// 对应 Java: `FinBookController.update` 与 `FinBookServiceImpl.updateFinBook`。
pub async fn update(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<CreateUpdateBookDto>,
) -> Result<Json<ApiResponse<FinBookResponse>>, AppError> {
    let book_id = params
        .id
        .as_deref()
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| param_err("账簿ID不能为空"))?;
    if params.book_name.trim().is_empty() {
        return Err(param_err("账簿名称不能为空"));
    }

    let book = check_book_ownership(&state, book_id, &user_id).await?;
    let mut active: BookActive = book.into();
    active.book_name = Set(params.book_name);
    active.update_by = Set(Some(user_id));
    active.update_time = Set(Some(Utc::now().naive_utc()));
    let updated = active.update(&state.db).await?;

    Ok(Json(ApiResponse::success(updated.into())))
}

/// 删除账簿：`POST /book/delete`。
///
/// 对应 Java: `FinBookController.delete` 与 `FinBookServiceImpl.deleteFinBook`。
/// 删除时会级联清理月度明细、月度汇总、模板项，并先备份账簿到 `SYS_BACKUP_BOOK`。
pub async fn delete(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<IdDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let book_id = params.id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    let book = check_book_ownership(&state, book_id, &user_id).await?;

    MonthItemEntity::delete_many()
        .filter(MonthItemCol::BookId.eq(book.id.clone()))
        .exec(&state.db)
        .await?;
    MonthRecordEntity::delete_many()
        .filter(MonthRecordCol::BookId.eq(book.id.clone()))
        .exec(&state.db)
        .await?;
    TemplateItemEntity::delete_many()
        .filter(TemplateItemCol::BookId.eq(book.id.clone()))
        .exec(&state.db)
        .await?;

    BackupBookActive {
        id: Set(book.id.clone()),
        user_id: Set(book.user_id.clone()),
        book_name: Set(book.book_name.clone()),
        create_by: Set(book.create_by.clone()),
        create_time: Set(book.create_time),
        update_by: Set(book.update_by.clone()),
        update_time: Set(book.update_time),
        is_deleted: Set(book.is_deleted),
    }
    .insert(&state.db)
    .await?;

    BookEntity::delete_by_id(book.id).exec(&state.db).await?;

    Ok(Json(ApiResponse::<()>::success_empty()))
}

/// 获取当前用户账簿列表：`POST /book/list`。
///
/// 对应 Java: `FinBookController.list` 与 `FinBookServiceImpl.getMyFinBook`。
/// 支持按账簿名称模糊查询，并返回 MyBatis-Flex `Page` 对齐的分页结构。
pub async fn list(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<QueryFinBookDto>,
) -> Result<Json<ApiResponse<PageResult<FinBookResponse>>>, AppError> {
    let mut q = BookEntity::find()
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0));
    if let Some(name) = params.name.filter(|s| !s.trim().is_empty()) {
        q = q.filter(BookCol::BookName.contains(&name));
    }
    let q = q.order_by_desc(BookCol::CreateTime);

    let page_num = params.page.page_num.max(1) as u64;
    let page_size = params.page.page_size.max(1) as u64;
    let paginator = q.paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let records = paginator.fetch_page(page_num - 1).await?;
    let result = PageResult::new(
        records.into_iter().map(FinBookResponse::from).collect(),
        total,
        page_num,
        page_size,
    );

    Ok(Json(ApiResponse::success(result)))
}

/// 获取当前用户所有账簿的最新统计数据：`POST /book/getMyDataStatistics`。
///
/// 对应 Java: `FinBookController.getMyDataStatistics` 与 `FinBookServiceImpl.getMyDataStatistics`。
/// 每个账簿最多返回一条最新月度汇总记录，用于首页账户统计展示。
pub async fn get_my_data_statistics(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DataStatisticsResponse<FinBookRecordResponse>>>, AppError> {
    let books = BookEntity::find()
        .filter(BookCol::UserId.eq(user_id.clone()))
        .filter(BookCol::IsDeleted.eq(0))
        .all(&state.db)
        .await?;

    if books.is_empty() {
        return Ok(Json(ApiResponse::success(DataStatisticsResponse {
            start_year_month: String::new(),
            end_year_month: String::new(),
            record_list: Vec::new(),
        })));
    }

    let book_map: HashMap<String, String> = books
        .into_iter()
        .map(|book| (book.id, book.book_name))
        .collect();
    let records = MonthRecordEntity::find()
        .filter(MonthRecordCol::UserId.eq(user_id))
        .filter(MonthRecordCol::BookId.is_in(book_map.keys().cloned()))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .order_by_desc(MonthRecordCol::Year)
        .order_by_desc(MonthRecordCol::Month)
        .all(&state.db)
        .await?;

    let mut latest_by_book = HashMap::new();
    for record in records {
        latest_by_book
            .entry(record.book_id.clone())
            .or_insert(record);
    }

    let mut record_list = Vec::new();
    for (book_id, book_name) in book_map {
        if let Some(record) = latest_by_book.remove(&book_id) {
            record_list.push(FinBookRecordResponse::new(book_id, book_name, record));
        }
    }

    Ok(Json(ApiResponse::success(DataStatisticsResponse {
        start_year_month: String::new(),
        end_year_month: String::new(),
        record_list,
    })))
}

/// 获取指定账簿过去 12 个月的详细统计数据：`POST /book/getBookDetailedStatistics`。
///
/// 对应 Java: `FinBookController.getBookDetailedStatistics` 与 `FinBookServiceImpl.getBookDetailedStatistics`。
/// 先校验账簿归属，再按年月范围查询月度汇总记录。
pub async fn get_book_detailed_statistics(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<IdDto>,
) -> Result<Json<ApiResponse<DataStatisticsResponse<MonthRecordResponse>>>, AppError> {
    let book_id = params.id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }
    check_book_ownership(&state, book_id, &user_id).await?;

    let today = Utc::now().date_naive();
    let start = today.checked_sub_months(Months::new(12)).unwrap_or(today);
    let start_year_month = format!("{:04}{:02}", start.year(), start.month());
    let end_year_month = format!("{:04}{:02}", today.year(), today.month());

    let records = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id.to_string()))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .filter(
            Condition::any()
                .add(MonthRecordCol::Year.gt(start.year()))
                .add(
                    Condition::all()
                        .add(MonthRecordCol::Year.eq(start.year()))
                        .add(MonthRecordCol::Month.gte(start.month() as i32)),
                ),
        )
        .filter(
            Condition::any()
                .add(MonthRecordCol::Year.lt(today.year()))
                .add(
                    Condition::all()
                        .add(MonthRecordCol::Year.eq(today.year()))
                        .add(MonthRecordCol::Month.lte(today.month() as i32)),
                ),
        )
        .order_by_asc(MonthRecordCol::Year)
        .order_by_asc(MonthRecordCol::Month)
        .all(&state.db)
        .await?;

    Ok(Json(ApiResponse::success(DataStatisticsResponse {
        start_year_month,
        end_year_month,
        record_list: records.into_iter().map(MonthRecordResponse::from).collect(),
    })))
}
