//! 财务账簿模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinBookController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinBookServiceImpl`

use std::collections::HashMap;

use axum::{Json, extract::State};
use chrono::{Datelike, Months, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
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
        sys_backup_month_item_record::{
            ActiveModel as BackupMonthItemActive, Entity as BackupMonthItemEntity,
        },
        sys_backup_month_record::{
            ActiveModel as BackupMonthRecordActive, Entity as BackupMonthRecordEntity,
        },
        sys_backup_template_item::{
            ActiveModel as BackupTemplateItemActive, Entity as BackupTemplateItemEntity,
        },
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
async fn check_book_ownership<C>(
    db: &C,
    book_id: &str,
    user_id: &str,
) -> Result<fin_book::Model, AppError>
where
    C: ConnectionTrait,
{
    let book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::IsDeleted.eq(0))
        .one(db)
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
        create_time: Set(Utc::now().naive_utc()),
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

    let book = check_book_ownership(&state.db, book_id, &user_id).await?;
    let mut active: BookActive = book.into();
    active.book_name = Set(params.book_name);
    active.update_by = Set(Some(user_id));
    active.update_time = Set(Some(Utc::now().naive_utc()));
    let updated = active.update(&state.db).await?;

    Ok(Json(ApiResponse::success(updated.into())))
}

/// 备份指定账簿下的月度财务明细项。
///
/// 对应 Java: `SysBackupMonthItemRecordMapper.backupByBookId(String bookId)`。
pub(crate) async fn backup_month_item_records<C>(db: &C, book_id: &str) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    let records = MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(book_id))
        .filter(MonthItemCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if records.is_empty() {
        return Ok(());
    }

    let backups = records.into_iter().map(|record| BackupMonthItemActive {
        id: Set(record.id),
        year: Set(record.year),
        month: Set(record.month),
        book_id: Set(record.book_id),
        template_item_id: Set(record.template_item_id),
        item_value: Set(record.item_value),
        create_by: Set(record.create_by),
        create_time: Set(record.create_time),
        update_by: Set(record.update_by),
        update_time: Set(record.update_time),
        is_deleted: Set(record.is_deleted),
    });
    BackupMonthItemEntity::insert_many(backups).exec(db).await?;

    Ok(())
}

/// 备份指定账簿下的月度财务汇总。
///
/// 对应 Java: `SysBackupMonthRecordMapper.backupByBookId(String bookId)`。
pub(crate) async fn backup_month_records<C>(db: &C, book_id: &str) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    let records = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if records.is_empty() {
        return Ok(());
    }

    let backups = records.into_iter().map(|record| BackupMonthRecordActive {
        id: Set(record.id),
        user_id: Set(record.user_id),
        book_id: Set(record.book_id),
        year: Set(record.year),
        month: Set(record.month),
        total_asset: Set(record.total_asset),
        total_liability: Set(record.total_liability),
        net_asset: Set(record.net_asset),
        month_on_month: Set(record.month_on_month),
        year_on_year: Set(record.year_on_year),
        note: Set(record.note),
        create_by: Set(record.create_by),
        create_time: Set(record.create_time),
        update_by: Set(record.update_by),
        update_time: Set(record.update_time),
        is_deleted: Set(record.is_deleted),
    });
    BackupMonthRecordEntity::insert_many(backups)
        .exec(db)
        .await?;

    Ok(())
}

/// 备份指定账簿下的记账模板项。
///
/// 对应 Java: `SysBackupTemplateItemMapper.backupByBookId(String bookId)`。
pub(crate) async fn backup_template_items<C>(db: &C, book_id: &str) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    let items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if items.is_empty() {
        return Ok(());
    }

    let backups = items.into_iter().map(|item| BackupTemplateItemActive {
        id: Set(item.id),
        book_id: Set(item.book_id),
        item_name: Set(item.item_name),
        item_type: Set(item.item_type),
        icon: Set(item.icon),
        sort: Set(item.sort),
        create_by: Set(item.create_by),
        create_time: Set(item.create_time),
        update_by: Set(item.update_by),
        update_time: Set(item.update_time),
        is_deleted: Set(item.is_deleted),
    });
    BackupTemplateItemEntity::insert_many(backups)
        .exec(db)
        .await?;

    Ok(())
}

/// 备份指定账簿主数据。
///
/// 对应 Java: `SysBackupBookMapper.backupByBookId(String bookId)`。
pub(crate) async fn backup_book<C>(db: &C, book: &fin_book::Model) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
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
    .insert(db)
    .await?;

    Ok(())
}

/// 备份并逻辑删除单个账簿及其全部关联数据。
///
/// 抽取自 `FinBookController.delete`（`FinBookServiceImpl.deleteFinBook`）的备份+删除流程，
/// 按 Java Service 顺序备份并清理月度明细、月度汇总、模板项和账簿主数据。
/// 调用方负责事务边界（`begin`/`commit`）与账簿归属校验。
///
/// 【对齐 Java】Java `BaseEntity` 的 `MR_IS_DELETED` 标注 `isLogicDelete = true`，
/// `deleteById` / `deleteByQuery` 一律是 `UPDATE ... SET is_deleted = 1`，即全链路逻辑删除。
/// Rust 同样改为逻辑删除：先备份 `is_deleted = 0` 的在册数据，再将其置为
/// `is_deleted = 1`。所有业务查询本就过滤 `is_deleted = 0`，故删除后数据对用户立即不可见，
/// 同时保留可恢复性（将来支持「撤销删除」只需回写 `is_deleted = 0`）。备份表保持
/// `is_deleted = 0` 的在册快照不变。
pub(crate) async fn purge_book<C>(db: &C, book: &fin_book::Model) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    backup_month_item_records(db, &book.id).await?;
    MonthItemEntity::update_many()
        .col_expr(MonthItemCol::IsDeleted, 1.into())
        .filter(MonthItemCol::BookId.eq(book.id.clone()))
        .exec(db)
        .await?;

    backup_month_records(db, &book.id).await?;
    MonthRecordEntity::update_many()
        .col_expr(MonthRecordCol::IsDeleted, 1.into())
        .filter(MonthRecordCol::BookId.eq(book.id.clone()))
        .exec(db)
        .await?;

    backup_template_items(db, &book.id).await?;
    TemplateItemEntity::update_many()
        .col_expr(TemplateItemCol::IsDeleted, 1.into())
        .filter(TemplateItemCol::BookId.eq(book.id.clone()))
        .exec(db)
        .await?;

    backup_book(db, book).await?;
    BookEntity::update_many()
        .col_expr(BookCol::IsDeleted, 1.into())
        .filter(BookCol::Id.eq(book.id.clone()))
        .exec(db)
        .await?;

    Ok(())
}

/// 备份并逻辑删除指定用户名下的全部账簿及其关联数据。
///
/// 供用户注销清理定时任务（[`crate::service::cancel_cleanup_task`]）复用账簿删除流程。
/// 返回清理的账簿数量。调用方负责事务边界。
pub(crate) async fn delete_user_books<C>(db: &C, user_id: &str) -> Result<usize, AppError>
where
    C: ConnectionTrait,
{
    let books = BookEntity::find()
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .all(db)
        .await?;

    for book in &books {
        purge_book(db, book).await?;
    }

    Ok(books.len())
}

/// 删除账簿：`POST /book/delete`。
///
/// 对应 Java: `FinBookController.delete` 与 `FinBookServiceImpl.deleteFinBook`。
/// 删除时会按 Java Service 顺序备份并将月度明细、月度汇总、模板项和账簿主数据
/// 逻辑删除（`is_deleted = 1`），数据对用户立即不可见且可恢复。
pub async fn delete(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<IdDto>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let book_id = params.id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    let txn = state.db.begin().await?;
    let book = check_book_ownership(&txn, book_id, &user_id).await?;
    purge_book(&txn, &book).await?;
    txn.commit().await?;

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
    check_book_ownership(&state.db, book_id, &user_id).await?;

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

#[cfg(test)]
mod tests {
    //! 账簿级联删除（逻辑删除）验收测试。
    //!
    //! 对应 REFACTOR_TODO 3.5：账簿删除时月度明细 / 汇总 / 模板项 / 账簿主数据
    //! 一律逻辑删除（`is_deleted = 1`，对齐 Java `BaseEntity.isLogicDelete`），
    //! 删除前先备份在册快照（备份范围 `is_deleted = 0`）。

    use super::*;
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};

    /// 建好业务表与备份表（对齐 `schema.sql`）。
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接内存库失败");
        for ddl in [
            "CREATE TABLE IF NOT EXISTS FIN_BOOK (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_NAME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_TEMPLATE_ITEM (
                MR_ID TEXT PRIMARY KEY, MR_BOOK_ID TEXT, MR_ITEM_NAME TEXT, MR_ITEM_TYPE INTEGER,
                MR_ICON TEXT, MR_SORT TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_BOOK (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_NAME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_TEMPLATE_ITEM (
                MR_ID TEXT PRIMARY KEY, MR_BOOK_ID TEXT, MR_ITEM_NAME TEXT, MR_ITEM_TYPE INTEGER,
                MR_ICON TEXT, MR_SORT TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db
    }

    /// 写入一个属于 u1 的账簿及其模板项 / 月度汇总 / 明细各一条（均在册）。
    async fn insert_book_with_data(db: &DatabaseConnection) {
        for sql in [
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('bk1', 'u1', '我的账簿')",
            "INSERT INTO FIN_TEMPLATE_ITEM (MR_ID, MR_BOOK_ID, MR_ITEM_NAME, MR_ITEM_TYPE, MR_ICON, MR_SORT)
             VALUES ('tpl1', 'bk1', '现金', 1, 'icon', '1')",
            "INSERT INTO FIN_MONTH_RECORD (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH,
                 MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR)
             VALUES ('mr1', 'u1', 'bk1', 2026, 3, 100, 40, 60, 5, 12)",
            "INSERT INTO FIN_MONTH_ITEM_RECORD (MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID,
                 MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE)
             VALUES ('mir1', 2026, 3, 'bk1', 'tpl1', 100)",
        ] {
            db.execute_unprepared(sql).await.unwrap();
        }
    }

    fn test_book() -> fin_book::Model {
        fin_book::Model {
            id: "bk1".to_string(),
            user_id: "u1".to_string(),
            book_name: "我的账簿".to_string(),
            create_by: None,
            create_time: chrono::Utc::now().naive_utc(),
            update_by: None,
            update_time: None,
            is_deleted: 0,
        }
    }

    async fn count_total(db: &DatabaseConnection, table: &str) -> i64 {
        let row = db
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                format!("SELECT COUNT(*) AS c FROM {table}"),
            ))
            .await
            .unwrap()
            .unwrap();
        row.try_get::<i64>("", "c").unwrap()
    }

    /// 统计业务表在册（`is_deleted = 0`）行数。
    async fn count_active(db: &DatabaseConnection, table: &str) -> i64 {
        let row = db
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                format!("SELECT COUNT(*) AS c FROM {table} WHERE MR_IS_DELETED = 0"),
            ))
            .await
            .unwrap()
            .unwrap();
        row.try_get::<i64>("", "c").unwrap()
    }

    /// 级联逻辑删除：业务表行保留但 `is_deleted = 1`，备份表留存在册快照。
    #[tokio::test]
    async fn purge_book_logically_deletes_cascade_and_backs_up() {
        let db = setup_db().await;
        insert_book_with_data(&db).await;

        purge_book(&db, &test_book()).await.expect("级联删除失败");

        // 业务表：行保留但 is_deleted = 1（所有业务查询过滤 is_deleted = 0 → 对用户不可见）
        for table in [
            "FIN_BOOK",
            "FIN_TEMPLATE_ITEM",
            "FIN_MONTH_RECORD",
            "FIN_MONTH_ITEM_RECORD",
        ] {
            assert_eq!(count_active(&db, table).await, 0, "{table} 在册行应为 0");
            assert_eq!(count_total(&db, table).await, 1, "{table} 物理行应保留为 1");
        }
        // 备份表：各留一条在册快照
        for table in [
            "SYS_BACKUP_BOOK",
            "SYS_BACKUP_TEMPLATE_ITEM",
            "SYS_BACKUP_MONTH_RECORD",
            "SYS_BACKUP_MONTH_ITEM_RECORD",
        ] {
            assert_eq!(count_total(&db, table).await, 1, "{table} 应有 1 条备份");
        }
    }

    /// 逻辑删除后账簿对业务查询不可见（`delete_user_books` 只处理在册账簿，幂等）。
    #[tokio::test]
    async fn delete_user_books_is_idempotent_after_logical_delete() {
        let db = setup_db().await;
        insert_book_with_data(&db).await;

        assert_eq!(delete_user_books(&db, "u1").await.unwrap(), 1);
        // 再次清理：账簿已 is_deleted = 1，不在在册查询范围内，不会重复处理
        assert_eq!(
            delete_user_books(&db, "u1").await.unwrap(),
            0,
            "逻辑删除后再次清理不应重复处理"
        );
    }
}
