//! 记账模板明细模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinTemplateItemController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinTemplateItemServiceImpl`

use axum::{Json, extract::State};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait, sea_query::Expr,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{res_code::ResCode, result::ApiResponse, user_context::AuthUser},
    entity::{
        fin_book::{Column as BookCol, Entity as BookEntity},
        fin_template_item::{
            self, ActiveModel as TemplateItemActive, Column as TemplateItemCol,
            Entity as TemplateItemEntity,
        },
    },
    error::AppError,
    model::finance::{FinTempItemDto, TemplateItemEntry, TemplateItemResponse},
};

/// 构造参数错误业务异常。
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 校验账簿存在且属于当前登录用户，并返回模板项列表。
///
/// 对应 Java: `FinTemplateItemServiceImpl.selectByFinBookIdExternal`
async fn check_book_and_get_template_items<C>(
    db: &C,
    book_id: &str,
    user_id: &str,
) -> Result<Vec<fin_template_item::Model>, AppError>
where
    C: ConnectionTrait,
{
    // 通过 JOIN 查询校验所有权并获取模板项
    // 对应 SQL: SELECT fti.* FROM FIN_BOOK fb INNER JOIN FIN_TEMPLATE_ITEM fti ...
    let _book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .one(db)
        .await?
        .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;

    // 如果账簿存在，查询模板项
    let items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .order_by_asc(Expr::cust("CAST(MR_SORT AS INTEGER)"))
        .all(db)
        .await?;

    Ok(items)
}

/// 校验账簿存在且属于当前登录用户。
///
/// 【有意收紧 / 修复 Java 缺陷】Java 的 `ceateFinTemplateItemList` 与
/// `copyTemplateItem` 都**不做账簿归属校验**：前者直接用 `bookId` 插入模板项，
/// 后者甚至用内部 `selectByFinBookId`（不带 userId 过滤）读取源账簿模板——
/// 任意登录用户可读取他人模板并写入任意账簿（IDOR 读写）。Rust 要求操作涉及的
/// 账簿必须属于当前登录用户，否则返回 `FinBookNotFound`。
async fn check_book_ownership<C>(db: &C, book_id: &str, user_id: &str) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    let exists = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .one(db)
        .await?
        .is_some();
    if !exists {
        return Err(AppError::ResCode(ResCode::FinBookNotFound));
    }
    Ok(())
}

/// 查询账簿的在册模板项（按 sort 升序），不校验归属、空列表不报错。
async fn get_active_template_items<C>(
    db: &C,
    book_id: &str,
) -> Result<Vec<fin_template_item::Model>, AppError>
where
    C: ConnectionTrait,
{
    let items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .order_by_asc(Expr::cust("CAST(MR_SORT AS INTEGER)"))
        .all(db)
        .await?;
    Ok(items)
}

/// 校验模板项类型合法（对齐 Java `ceateFinTemplateItemList` 的类型校验）。
///
/// - 缺失 / null → `FinItemTempTypeRequired`（14303），交业务层校验而非反序列化裸 400；
/// - 非 -1 / 0 / 1 → `FinItemTempTypeError`（14304）。
///
/// 返回解析后的类型值供插入使用。
fn validate_item_type(item_type: Option<i32>) -> Result<i32, AppError> {
    let item_type = item_type.ok_or(AppError::ResCode(ResCode::FinItemTempTypeRequired))?;
    // -1: 负债, 0: 仅记录, 1: 资产
    if !(-1..=1).contains(&item_type) {
        return Err(AppError::ResCode(ResCode::FinItemTempTypeError));
    }
    Ok(item_type)
}

/// 创建账本模板项：`POST /tempItem/create`
///
/// 对应 Java: `FinTemplateItemController.create` 与 `FinTemplateItemServiceImpl.ceateFinTemplateItemList`
///
/// 【有意收紧】账簿归属校验见 [`check_book_ownership`]——Java 不做校验，存在 IDOR 写入。
/// 注意：此处**不**要求账簿已有模板项，否则新账簿永远无法创建第一个模板项（死路）。
pub async fn create(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 校验账簿权限（仅归属，不要求已有模板项——否则新账簿无法创建首个模板项）
    check_book_ownership(&state.db, book_id, &user_id).await?;

    // 模板项列表为空（含字段缺失）→ FinItemTempNotExist，对齐 Java
    let item_list = require_template_item_list(params.item_list)?;

    let mut result = Vec::with_capacity(item_list.len());

    for item in item_list {
        if item.item_name.trim().is_empty() {
            return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
        }
        let item_type = validate_item_type(item.item_type)?;

        let active = TemplateItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            book_id: Set(book_id.to_string()),
            item_name: Set(item.item_name),
            item_type: Set(item_type),
            icon: Set(item.icon),
            sort: Set(item.sort),
            create_by: Set(Some(user_id.clone())),
            create_time: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        let model = active.insert(&state.db).await?;
        result.push(model.into());
    }

    Ok(Json(ApiResponse::success(result)))
}

/// 校验模板项列表非空。
///
/// 对齐 Java `FinTemplateItemServiceImpl.ceateFinTemplateItemList`：
/// `itemList` 为 `null` 或空列表时抛 `FIN_ITEM_TEMP_IS_NOT`（14301）。
/// Rust 此前对「字段缺失」返回 `ParamError`（10001），与 Java 不一致，此处统一为 14301。
fn require_template_item_list(
    item_list: Option<Vec<TemplateItemEntry>>,
) -> Result<Vec<TemplateItemEntry>, AppError> {
    item_list
        .filter(|list| !list.is_empty())
        .ok_or(AppError::ResCode(ResCode::FinItemTempNotExist))
}

/// 更新账本模板项：`POST /tempItem/update`
///
/// 对应 Java: `FinTemplateItemController.update` 与 `FinTemplateItemServiceImpl.updateFinTemplateItemList`
///
/// 【有意收紧 / 修复 Java 缺陷】
/// - 账簿归属校验（Java 无）；现有项的 `id` 必须属于本账簿（Java 用全库匹配 `dbList`，
///   实际也限定同一账簿，但 Rust 显式拒绝跨账簿 id）；类型禁止修改（对齐 Java）；
/// - 新增项的名称非空 + 类型范围校验：Java `updateFinTemplateItemList` 对新增项**不做任何
///   校验**直接插入（脏数据风险），Rust 补齐（复用 create 的校验）。
pub async fn update(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 模板项列表为空（含字段缺失）→ FinItemTempNotExist，对齐 Java
    let item_list = require_template_item_list(params.item_list)?;

    let txn = state.db.begin().await?;
    // 校验账簿权限（仅归属；已有模板项可为空——允许给无模板账簿新增首项）
    check_book_ownership(&txn, book_id, &user_id).await?;
    let existing_items = get_active_template_items(&txn, book_id).await?;
    let existing_map: std::collections::HashMap<_, _> = existing_items
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

    let mut result = Vec::with_capacity(item_list.len());

    for item in item_list {
        match item.id {
            Some(ref id) if !id.trim().is_empty() => {
                let id_str: &str = id;
                // 更新现有项（item.id 归属校验：不存在于本账簿时拒绝）
                let existing = existing_map
                    .get(id_str)
                    .ok_or(AppError::ResCode(ResCode::FinItemTempUpdateError))?;

                // 禁止修改类型（itemType 缺失或与库内不一致均视为修改 → 拒绝，对齐 Java）
                if item.item_type != Some(existing.item_type) {
                    return Err(AppError::ResCode(ResCode::FinItemTempUpdateError));
                }

                if item.item_name.trim().is_empty() {
                    return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
                }

                let mut active: TemplateItemActive = existing.clone().into();
                active.item_name = Set(item.item_name);
                active.icon = Set(item.icon);
                active.sort = Set(item.sort);
                active.update_by = Set(Some(user_id.clone()));
                active.update_time = Set(Some(chrono::Utc::now().naive_utc()));

                let model = active.update(&txn).await?;
                result.push(model.into());
            }
            _ => {
                // 新增项：补齐 Java 缺失的名称非空 + 类型范围校验
                if item.item_name.trim().is_empty() {
                    return Err(AppError::ResCode(ResCode::FinItemTempNameRequired));
                }
                let item_type = validate_item_type(item.item_type)?;

                let active = TemplateItemActive {
                    id: Set(Uuid::new_v4().simple().to_string()),
                    book_id: Set(book_id.to_string()),
                    item_name: Set(item.item_name),
                    item_type: Set(item_type),
                    icon: Set(item.icon),
                    sort: Set(item.sort),
                    create_by: Set(Some(user_id.clone())),
                    create_time: Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };

                let model = active.insert(&txn).await?;
                result.push(model.into());
            }
        }
    }

    txn.commit().await?;
    Ok(Json(ApiResponse::success(result)))
}

/// 复制账本模板项：`POST /tempItem/copy`
///
/// 对应 Java: `FinTemplateItemController.copy` 与 `FinTemplateItemServiceImpl.copyTemplateItem`
///
/// 【有意收紧 / 修复 Java 缺陷】Java 用内部 `selectByFinBookId`（不带 userId）读源账簿模板、
/// 也不校验新账簿归属，任意登录用户可读他人模板并写入他人账簿（IDOR 读写）。
/// Rust 要求源/目标账簿**均**属于当前登录用户。源账簿无模板项时返回空列表（对齐 Java）。
pub async fn copy(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let old_book_id = params
        .old_book_id
        .ok_or_else(|| param_err("原账簿ID不能为空"))?;
    let old_book_id = old_book_id.trim();
    if old_book_id.is_empty() {
        return Err(param_err("原账簿ID不能为空"));
    }

    let new_book_id = params.book_id.trim();
    if new_book_id.is_empty() {
        return Err(param_err("新账簿ID不能为空"));
    }

    // 校验源/目标账簿均属于当前登录用户（Java 无此校验 → IDOR）
    check_book_ownership(&state.db, old_book_id, &user_id).await?;
    check_book_ownership(&state.db, new_book_id, &user_id).await?;

    let source_items = get_active_template_items(&state.db, old_book_id).await?;
    if source_items.is_empty() {
        return Ok(Json(ApiResponse::success(vec![])));
    }

    let mut result = Vec::with_capacity(source_items.len());

    for item in source_items {
        let active = TemplateItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            book_id: Set(new_book_id.to_string()),
            item_name: Set(item.item_name),
            item_type: Set(item.item_type),
            icon: Set(item.icon),
            sort: Set(item.sort),
            create_by: Set(Some(user_id.clone())),
            create_time: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        let model = active.insert(&state.db).await?;
        result.push(model.into());
    }

    Ok(Json(ApiResponse::success(result)))
}

/// 查询账本模板项列表：`POST /tempItem/list`
///
/// 对应 Java: `FinTemplateItemController.list` 与 `FinTemplateItemServiceImpl.selectByFinBookIdExternal`
pub async fn list(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<FinTempItemDto>,
) -> Result<Json<ApiResponse<Vec<TemplateItemResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    let items = check_book_and_get_template_items(&state.db, book_id, &user_id).await?;

    if items.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    let result = items.into_iter().map(TemplateItemResponse::from).collect();

    Ok(Json(ApiResponse::success(result)))
}

#[cfg(test)]
mod tests {
    //! 模板项校验测试。
    //!
    //! - 列表空值校验对应 Java `FinTemplateItemServiceImpl.ceateFinTemplateItemList`：
    //!   `itemList` 为 null 或空列表时抛 `FIN_ITEM_TEMP_IS_NOT`（14301）；
    //! - 类型校验对应同方法：`itemType` 缺失 → 14303、越界 → 14304（3.7）；
    //! - handler 级测试覆盖 3.7 的有意收紧（账簿归属）与缺陷修复
    //!   （create/update 对无模板账簿新增首项的「死路」）。

    use super::*;
    use crate::error::AppError;
    use crate::model::finance::{FinTempItemDto, TemplateItemEntry};
    use crate::service::{
        cancel_cleanup_task::CancelCleanupTask, email::EmailService,
        export_task::ExportTaskService, monthly_reminder_task::MonthlyReminderTask,
        sys_config::SysConfigService, sys_user_operate_log::SysUserOperateLogService,
        yearly_summary_task::YearlySummaryTask,
    };
    use axum::extract::State;
    use sea_orm::{
        ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult,
        Statement,
    };

    fn entry(name: &str) -> TemplateItemEntry {
        TemplateItemEntry {
            id: None,
            item_name: name.to_string(),
            item_type: Some(1),
            icon: String::new(),
            sort: "1".to_string(),
        }
    }

    fn entry_with_type(name: &str, item_type: Option<i32>) -> TemplateItemEntry {
        let mut e = entry(name);
        e.item_type = item_type;
        e
    }

    #[test]
    fn missing_list_returns_fin_item_temp_not_exist() {
        // 字段缺失（null）：此前返回 ParamError(10001)，对齐 Java 后应为 14301
        let err = require_template_item_list(None).unwrap_err();
        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempNotExist)
        ));
    }

    #[test]
    fn empty_list_returns_fin_item_temp_not_exist() {
        let err = require_template_item_list(Some(vec![])).unwrap_err();
        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempNotExist)
        ));
    }

    #[test]
    fn non_empty_list_is_returned_unchanged() {
        let list = require_template_item_list(Some(vec![entry("现金"), entry("基金")])).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].item_name, "现金");
    }

    // ==================== 类型校验（3.7：itemType → Option）====================

    #[test]
    fn validate_item_type_rejects_missing_as_14303() {
        // itemType 缺失：此前是反序列化裸 400，改为业务响应 14303（对齐 Java）
        let err = validate_item_type(None).unwrap_err();
        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempTypeRequired)
        ));
    }

    #[test]
    fn validate_item_type_rejects_out_of_range_as_14304() {
        for t in [2, -2, 99, i32::MIN, i32::MAX] {
            let err = validate_item_type(Some(t)).unwrap_err();
            assert!(
                matches!(err, AppError::ResCode(ResCode::FinItemTempTypeError)),
                "itemType={t} 应拒绝为 14304"
            );
        }
    }

    #[test]
    fn validate_item_type_accepts_minus_one_zero_one() {
        for t in [-1, 0, 1] {
            assert_eq!(validate_item_type(Some(t)).unwrap(), t);
        }
    }

    #[test]
    fn item_type_missing_in_json_deserializes_to_none() {
        // 不含 itemType 的 JSON 反序列化为 None（而非失败 → 裸 400），icon 走默认空串
        let json = r#"[{"itemName":"现金","sort":"1"}]"#;
        let list: Vec<TemplateItemEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].item_type, None);
        assert_eq!(list[0].item_name, "现金");
        assert!(list[0].icon.is_empty());
    }

    // ==================== handler 级测试 ====================

    /// 建好 FIN_BOOK / FIN_TEMPLATE_ITEM（对齐 `schema.sql`）。
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
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db
    }

    /// 用内存库构造最小可用的 `AppState`（handler 只用到 db 与 AuthUser）。
    fn test_state(db: DatabaseConnection) -> AppState {
        let config_service = SysConfigService::new();
        let email_service = EmailService::new(config_service.clone());
        AppState {
            db,
            jwt_secret: "test-secret".to_string(),
            activate_token_secret: "test-secret".to_string(),
            reset_pwd_token_secret: "test-secret".to_string(),
            jwt_expire_secs: 604800,
            config_service,
            email_service: email_service.clone(),
            export_task_service: ExportTaskService::new(email_service.clone()),
            operate_log_service: SysUserOperateLogService::new(),
            monthly_reminder_task: MonthlyReminderTask::new(email_service.clone()),
            yearly_summary_task: YearlySummaryTask::new(email_service.clone()),
            cancel_cleanup_task: CancelCleanupTask::new(),
        }
    }

    async fn insert_book(db: &DatabaseConnection, id: &str, user_id: &str) {
        db.execute_unprepared(&format!(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('{id}', '{user_id}', '账簿')"
        ))
        .await
        .unwrap();
    }

    async fn insert_template(
        db: &DatabaseConnection,
        id: &str,
        book_id: &str,
        name: &str,
        item_type: i32,
    ) {
        db.execute_unprepared(&format!(
            "INSERT INTO FIN_TEMPLATE_ITEM (MR_ID, MR_BOOK_ID, MR_ITEM_NAME, MR_ITEM_TYPE, MR_ICON, MR_SORT)
             VALUES ('{id}', '{book_id}', '{name}', {item_type}, 'icon', '1')"
        ))
        .await
        .unwrap();
    }

    async fn count_active_templates(db: &DatabaseConnection, book_id: &str) -> i64 {
        // Sea-ORM 2.0 起 `query_one` 只接受 `StatementBuilder`；原生 SQL 走 `find_by_statement`。
        CountRow::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS c FROM FIN_TEMPLATE_ITEM WHERE MR_IS_DELETED = 0 AND MR_BOOK_ID = ?",
            [book_id.into()],
        ))
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .c
    }

    /// 原生 COUNT 查询行（Sea-ORM 2.0 的 `query_one` 只接受 `StatementBuilder`）。
    #[derive(Clone, Debug, PartialEq, FromQueryResult)]
    struct CountRow {
        c: i64,
    }

    /// 【死路修复】无模板项的新账簿也能创建首个模板项（此前 create 要求账簿已有模板项 → 死路）。
    #[tokio::test]
    async fn create_allows_first_template_for_empty_book() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;

        let res = create(
            AuthUser("u1".into()),
            State(test_state(db.clone())),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![entry("现金"), entry_with_type("房贷", Some(-1))]),
            }),
        )
        .await
        .expect("新账簿创建首个模板项失败");

        assert_eq!(res.0.data.unwrap().len(), 2);
        assert_eq!(count_active_templates(&db, "bk1").await, 2);
    }

    /// 【有意收紧】无归属的用户不能给他人的账簿创建模板项（Java 无此校验 → IDOR 写入）。
    #[tokio::test]
    async fn create_rejects_other_users_book() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "owner").await;

        let err = create(
            AuthUser("attacker".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![entry("现金")]),
            }),
        )
        .await
        .expect_err("越权创建应被拒绝");

        assert!(matches!(err, AppError::ResCode(ResCode::FinBookNotFound)));
    }

    /// create 时 itemType 缺失 → 14303 业务响应（而非裸 400）。
    #[tokio::test]
    async fn create_rejects_missing_item_type_as_14303() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;

        let err = create(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![entry_with_type("现金", None)]),
            }),
        )
        .await
        .expect_err("itemType 缺失应返回 14303");

        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempTypeRequired)
        ));
    }

    /// 【有意收紧】copy 要求源/目标账簿均属于当前用户（Java 用内部无 userId 查询 → IDOR 读写）。
    #[tokio::test]
    async fn copy_rejects_source_book_not_owned_by_user() {
        let db = setup_db().await;
        insert_book(&db, "src", "owner").await;
        insert_book(&db, "dst", "u1").await;

        let err = copy(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "dst".to_string(),
                old_book_id: Some("src".to_string()),
                item_list: None,
            }),
        )
        .await
        .expect_err("越权复制源账簿应被拒绝");

        assert!(matches!(err, AppError::ResCode(ResCode::FinBookNotFound)));
    }

    /// 源账簿无模板项时 copy 返回空列表（对齐 Java：`Collections.emptyList()`）。
    #[tokio::test]
    async fn copy_returns_empty_when_source_has_no_templates() {
        let db = setup_db().await;
        insert_book(&db, "src", "u1").await;
        insert_book(&db, "dst", "u1").await;

        let res = copy(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "dst".to_string(),
                old_book_id: Some("src".to_string()),
                item_list: None,
            }),
        )
        .await
        .expect("空源账簿复制应返回空列表");

        assert!(res.0.data.unwrap().is_empty());
    }

    #[tokio::test]
    async fn copy_duplicates_templates_to_new_book() {
        let db = setup_db().await;
        insert_book(&db, "src", "u1").await;
        insert_book(&db, "dst", "u1").await;
        insert_template(&db, "t1", "src", "现金", 1).await;

        let res = copy(
            AuthUser("u1".into()),
            State(test_state(db.clone())),
            Json(FinTempItemDto {
                book_id: "dst".to_string(),
                old_book_id: Some("src".to_string()),
                item_list: None,
            }),
        )
        .await
        .expect("复制模板项失败");

        let copied = res.0.data.unwrap();
        assert_eq!(copied.len(), 1);
        assert_eq!(copied[0].book_id, "dst");
        assert_eq!(copied[0].item_name, "现金");
        assert_eq!(count_active_templates(&db, "dst").await, 1);
    }

    /// 更新现有项时缺失 itemType 视为类型修改 → 14305（对齐 Java 的 null != 库内值拒绝语义）。
    #[tokio::test]
    async fn update_rejects_type_change_when_item_type_omitted() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;
        insert_template(&db, "t1", "bk1", "现金", 1).await;

        let mut existing = entry("现金");
        existing.id = Some("t1".to_string());
        existing.item_type = None;

        let err = update(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![existing]),
            }),
        )
        .await
        .expect_err("itemType 缺失应视为类型修改被拒绝");

        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempUpdateError)
        ));
    }

    /// 更新时新增项缺失类型 → 14303（补齐 Java 对新增项不校验的缺陷）。
    #[tokio::test]
    async fn update_validates_missing_type_for_new_item_as_14303() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;
        insert_template(&db, "t1", "bk1", "现金", 1).await;

        let mut existing = entry("现金");
        existing.id = Some("t1".to_string());

        let err = update(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![existing, entry_with_type("基金", None)]),
            }),
        )
        .await
        .expect_err("新增项缺失类型应返回 14303");

        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempTypeRequired)
        ));
    }

    /// 更新时新增项类型越界 → 14304（补齐 Java 对新增项不校验的缺陷）。
    #[tokio::test]
    async fn update_rejects_new_item_with_invalid_type() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;
        insert_template(&db, "t1", "bk1", "现金", 1).await;

        let mut existing = entry("现金");
        existing.id = Some("t1".to_string());

        let err = update(
            AuthUser("u1".into()),
            State(test_state(db)),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![existing, entry_with_type("基金", Some(2))]),
            }),
        )
        .await
        .expect_err("新增项类型越界应返回 14304");

        assert!(matches!(
            err,
            AppError::ResCode(ResCode::FinItemTempTypeError)
        ));
    }

    /// 【死路修复】无模板项的账簿可经 update 新增首项（此前 update 要求账簿已有模板项 → 死路）。
    #[tokio::test]
    async fn update_can_add_first_item_to_empty_book() {
        let db = setup_db().await;
        insert_book(&db, "bk1", "u1").await;

        let res = update(
            AuthUser("u1".into()),
            State(test_state(db.clone())),
            Json(FinTempItemDto {
                book_id: "bk1".to_string(),
                old_book_id: None,
                item_list: Some(vec![entry("现金")]),
            }),
        )
        .await
        .expect("空账簿新增首个模板项失败");

        assert_eq!(res.0.data.unwrap().len(), 1);
        assert_eq!(count_active_templates(&db, "bk1").await, 1);
    }
}
