//! 月度财务账目模块 HTTP 处理函数
//!
//! 对应 Java: `com.dcz.mrecord.controller.FinMonthItemRecordController`
//! 对应业务实现: `com.dcz.mrecord.service.impl.FinMonthItemRecordServiceImpl`

use std::collections::HashMap;

use axum::{Json, extract::State};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{
        money::{calculate_growth_rate, round_money, zero_money},
        res_code::ResCode,
        result::ApiResponse,
        user_context::AuthUser,
    },
    entity::{
        fin_book,
        fin_book::{Column as BookCol, Entity as BookEntity},
        fin_month_item_record::{
            ActiveModel as MonthItemActive, Column as MonthItemCol, Entity as MonthItemEntity,
        },
        fin_month_record::{
            self, ActiveModel as MonthRecordActive, Column as MonthRecordCol,
            Entity as MonthRecordEntity,
        },
        fin_template_item::{self, Column as TemplateItemCol, Entity as TemplateItemEntity},
    },
    error::AppError,
    model::finance::{MonthItemDto, MonthItemEntry, MonthItemRecordResponse},
};

/// 构造参数错误业务异常。
fn param_err(msg: impl Into<String>) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: msg.into(),
    }
}

/// 校验账簿存在且属于当前登录用户。
async fn check_book_ownership<C>(
    db: &C,
    book_id: &str,
    user_id: &str,
) -> Result<fin_book::Model, AppError>
where
    C: ConnectionTrait,
{
    let book = BookEntity::find_by_id(book_id.to_string())
        .filter(BookCol::UserId.eq(user_id))
        .filter(BookCol::IsDeleted.eq(0))
        .one(db)
        .await?
        .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;

    Ok(book)
}

/// 校验账簿存在且属于当前登录用户，并返回模板项列表。
async fn check_book_and_get_template_items<C>(
    db: &C,
    book_id: &str,
    user_id: &str,
) -> Result<Vec<fin_template_item::Model>, AppError>
where
    C: ConnectionTrait,
{
    // 先校验账簿权限
    let _ = check_book_ownership(db, book_id, user_id).await?;

    // 查询模板项
    let items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .all(db)
        .await?;

    if items.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    Ok(items)
}

/// 获取指定年月的月度汇总记录。
async fn get_month_record<C>(
    db: &C,
    book_id: &str,
    year: i32,
    month: i32,
) -> Result<Option<fin_month_record::Model>, AppError>
where
    C: ConnectionTrait,
{
    let record = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id))
        .filter(MonthRecordCol::Year.eq(year))
        .filter(MonthRecordCol::Month.eq(month))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .one(db)
        .await?;
    Ok(record)
}

/// 计算上个月的年月。
fn prev_month(year: i32, month: i32) -> (i32, i32) {
    if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    }
}

/// 计算下个月的年月。
fn next_month(year: i32, month: i32) -> (i32, i32) {
    if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    }
}

/// 月度汇总计算结果。
struct CalculatedMonthRecord {
    total_asset: Decimal,
    total_liability: Decimal,
    net_asset: Decimal,
    month_on_month: Decimal,
    year_on_year: Decimal,
}

/// 计算月度汇总数据。
async fn calculate_month_record_data<C>(
    db: &C,
    book_id: &str,
    year: i32,
    month: i32,
    item_list: &[MonthItemEntry],
    template_items: &[fin_template_item::Model],
) -> Result<CalculatedMonthRecord, AppError>
where
    C: ConnectionTrait,
{
    // 构建模板项类型映射
    let type_map: HashMap<_, _> = template_items
        .iter()
        .map(|item| (item.id.clone(), item.item_type))
        .collect();

    // 计算总资产和总负债（不逐项舍入，对齐 Java：明细存原值）
    let mut total_asset = Decimal::ZERO;
    let mut total_liability = Decimal::ZERO;

    for item in item_list {
        if let Some(&item_type) = type_map.get(&item.template_item_id) {
            match item_type {
                1 => total_asset += item.item_value,      // 资产
                -1 => total_liability += item.item_value, // 负债
                0 => {}                                   // 仅记录，不计入
                _ => {}
            }
        }
    }

    // 总资产/总负债不单独舍入（对齐 Java）；仅 netAsset 保留 2 位
    let net_asset = round_money(total_asset - total_liability);

    // 计算环比（与上月比较）
    let (prev_year, prev_month) = prev_month(year, month);
    let prev_record = get_month_record(db, book_id, prev_year, prev_month).await?;
    let month_on_month = prev_record
        .map(|r| calculate_growth_rate(net_asset, r.net_asset))
        .unwrap_or_else(zero_money);

    // 计算同比（与去年同月比较）
    let last_year_record = get_month_record(db, book_id, year - 1, month).await?;
    let year_on_year = last_year_record
        .map(|r| calculate_growth_rate(net_asset, r.net_asset))
        .unwrap_or_else(zero_money);

    Ok(CalculatedMonthRecord {
        total_asset,
        total_liability,
        net_asset,
        month_on_month,
        year_on_year,
    })
}

/// 插入或更新月度汇总记录。
async fn upsert_month_record<C>(
    db: &C,
    book_id: &str,
    year: i32,
    month: i32,
    user_id: &str,
    note: Option<String>,
    calculated: &CalculatedMonthRecord,
) -> Result<fin_month_record::Model, AppError>
where
    C: ConnectionTrait,
{
    let existing = get_month_record(db, book_id, year, month).await?;

    let record = if let Some(existing) = existing {
        let mut active: MonthRecordActive = existing.into();
        active.total_asset = Set(calculated.total_asset);
        active.total_liability = Set(calculated.total_liability);
        active.net_asset = Set(calculated.net_asset);
        active.month_on_month = Set(calculated.month_on_month);
        active.year_on_year = Set(calculated.year_on_year);
        active.note = Set(note);
        active.update_by = Set(Some(user_id.to_string()));
        active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
        active.update(db).await?
    } else {
        let active = MonthRecordActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            user_id: Set(user_id.to_string()),
            book_id: Set(book_id.to_string()),
            year: Set(year),
            month: Set(month),
            total_asset: Set(calculated.total_asset),
            total_liability: Set(calculated.total_liability),
            net_asset: Set(calculated.net_asset),
            month_on_month: Set(calculated.month_on_month),
            year_on_year: Set(calculated.year_on_year),
            note: Set(note),
            create_by: Set(Some(user_id.to_string())),
            create_time: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        active.insert(db).await?
    };

    Ok(record)
}

/// 修改历史月份明细后，联动重算「下月汇总的环比」与「明年同月汇总的同比」。
///
/// 对应 Java: `FinMonthRecordServiceImpl.recalculateFinMonthRecord` 中更新下月 / 明年同月
/// 汇总记录的段落（`nextMonthRecord.setMonthOnMonth(...)` / `nextYearRecord.setYearOnYear(...)`）。
///
/// 与 Java 严格对齐的三点语义：
/// - **下月汇总只更新 `month_on_month` 单字段**：以本月（刚重算）净资产为基数、以下月汇总
///   已存在的净资产为当期值计算环比，不覆盖 totalAsset / totalLiability / netAsset /
///   yearOnYear / note 等其它字段；
/// - **明年同月汇总只更新 `year_on_year` 单字段**：基数同样是本月净资产，当期值为明年同月
///   汇总已存在的净资产；
/// - **只要汇总记录存在就更新，不要求其明细仍然存在**：明细被删除时 Java 仍以已存在的净资产
///   为基准更新 MoM/YoY，故此处不再查询下月/明年同月的明细（原先的 `!next_items.is_empty()`
///   跳过逻辑已移除）。
async fn recalculate_related_months<C>(
    db: &C,
    book_id: &str,
    year: i32,
    month: i32,
    user_id: &str,
    curr_record: &fin_month_record::Model,
) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    // 更新下个月的环比（仅 month_on_month 单字段）
    let (next_year, next_month_val) = next_month(year, month);
    if let Some(next_record) = get_month_record(db, book_id, next_year, next_month_val).await? {
        // Java: getMonthOnMonthVal(currMonthRecord, nextMonthRecord)
        // = (nextMonthRecord.netAsset - currMonthRecord.netAsset) / |currMonthRecord.netAsset|
        let month_on_month = calculate_growth_rate(next_record.net_asset, curr_record.net_asset);
        let mut active: MonthRecordActive = next_record.into();
        active.month_on_month = Set(month_on_month);
        active.update_by = Set(Some(user_id.to_string()));
        active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
        active.update(db).await?;
    }

    // 更新明年同月的同比（仅 year_on_year 单字段）
    if let Some(next_year_record) = get_month_record(db, book_id, year + 1, month).await? {
        // Java: getYearOnYearVal(currMonthRecord, nextYearRecord)
        // = (nextYearRecord.netAsset - currMonthRecord.netAsset) / |currMonthRecord.netAsset|
        let year_on_year = calculate_growth_rate(next_year_record.net_asset, curr_record.net_asset);
        let mut active: MonthRecordActive = next_year_record.into();
        active.year_on_year = Set(year_on_year);
        active.update_by = Set(Some(user_id.to_string()));
        active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
        active.update(db).await?;
    }

    Ok(())
}

/// 插入月度财务账目：`POST /monthItem/insertMonthItem`
///
/// 对应 Java: `FinMonthItemRecordController.insertMonthItem` 与 `FinMonthItemRecordServiceImpl.insertMonthItemRecord`
pub async fn insert_month_item(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthItemDto>,
) -> Result<Json<ApiResponse<Vec<MonthItemRecordResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }
    let year = params.year.ok_or_else(|| param_err("年份不能为空"))?;
    let month = params.month.ok_or_else(|| param_err("月份不能为空"))?;

    let item_list = params
        .item_list
        .ok_or_else(|| param_err("账目列表不能为空"))?;
    if item_list.is_empty() {
        return Err(param_err("账目列表不能为空"));
    }

    let txn = state.db.begin().await?;
    // 校验账簿权限并获取模板项
    let template_items = check_book_and_get_template_items(&txn, book_id, &user_id).await?;

    // 校验明细项
    let template_ids: std::collections::HashSet<_> =
        template_items.iter().map(|i| i.id.as_str()).collect();
    for item in &item_list {
        if item.template_item_id.trim().is_empty() {
            return Err(param_err("模板项ID不能为空"));
        }
        if !template_ids.contains(item.template_item_id.as_str()) {
            return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
        }
        if item.item_value < Decimal::ZERO {
            return Err(param_err("账目金额不能为负数"));
        }
    }

    // 插入明细项
    let mut result: Vec<MonthItemRecordResponse> = Vec::with_capacity(item_list.len());
    for item in item_list {
        let active = MonthItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            year: Set(year),
            month: Set(month),
            book_id: Set(book_id.to_string()),
            template_item_id: Set(item.template_item_id),
            item_value: Set(item.item_value),
            create_by: Set(Some(user_id.clone())),
            create_time: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        let model = active.insert(&txn).await?;
        result.push(model.into());
    }

    // 重新构建条目用于计算
    let entries: Vec<_> = result
        .iter()
        .map(|r: &MonthItemRecordResponse| MonthItemEntry {
            id: Some(r.id.clone()),
            template_item_id: r.template_item_id.clone(),
            item_value: r.item_value,
        })
        .collect();

    // 计算并插入月度汇总
    let calculated =
        calculate_month_record_data(&txn, book_id, year, month, &entries, &template_items).await?;
    let curr_record = upsert_month_record(
        &txn,
        book_id,
        year,
        month,
        &user_id,
        params.note,
        &calculated,
    )
    .await?;

    // 重新计算相关月份（下月环比 / 明年同月同比）
    recalculate_related_months(&txn, book_id, year, month, &user_id, &curr_record).await?;
    txn.commit().await?;

    Ok(Json(ApiResponse::success(result)))
}

/// 更新月度财务账目：`POST /monthItem/updateMonthItem`
///
/// 对应 Java: `FinMonthItemRecordController.updateMonthItem` 与 `FinMonthItemRecordServiceImpl.updateMonthItemRecord`
pub async fn update_month_item(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthItemDto>,
) -> Result<Json<ApiResponse<Vec<MonthItemRecordResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }
    let year = params.year.ok_or_else(|| param_err("年份不能为空"))?;
    let month = params.month.ok_or_else(|| param_err("月份不能为空"))?;

    let item_list = params
        .item_list
        .ok_or_else(|| param_err("账目列表不能为空"))?;
    if item_list.is_empty() {
        return Err(param_err("账目列表不能为空"));
    }

    let txn = state.db.begin().await?;
    // 校验账簿权限并获取模板项
    let template_items = check_book_and_get_template_items(&txn, book_id, &user_id).await?;

    // 校验明细项
    let template_ids: std::collections::HashSet<_> =
        template_items.iter().map(|i| i.id.as_str()).collect();
    for item in &item_list {
        if item.template_item_id.trim().is_empty() {
            return Err(param_err("模板项ID不能为空"));
        }
        if !template_ids.contains(item.template_item_id.as_str()) {
            return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
        }
        if item.item_value < Decimal::ZERO {
            return Err(param_err("账目金额不能为负数"));
        }
    }

    // 获取现有记录
    let existing_items = MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(book_id))
        .filter(MonthItemCol::Year.eq(year))
        .filter(MonthItemCol::Month.eq(month))
        .filter(MonthItemCol::IsDeleted.eq(0))
        .all(&txn)
        .await?;
    let existing_map: std::collections::HashMap<_, _> = existing_items
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect();

    // 更新或插入明细项
    let mut result: Vec<MonthItemRecordResponse> = Vec::with_capacity(item_list.len());
    for item in item_list {
        match item.id {
            Some(ref id) if !id.trim().is_empty() => {
                let id_str: &str = id;
                // 更新现有项
                let existing = existing_map
                    .get(id_str)
                    .ok_or(AppError::ResCode(ResCode::FinItemNotFound))?;
                let mut active: MonthItemActive = existing.clone().into();
                active.template_item_id = Set(item.template_item_id);
                active.item_value = Set(item.item_value);
                active.update_by = Set(Some(user_id.clone()));
                active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
                let model = active.update(&txn).await?;
                result.push(model.into());
            }
            _ => {
                // 新增项
                let active = MonthItemActive {
                    id: Set(Uuid::new_v4().simple().to_string()),
                    year: Set(year),
                    month: Set(month),
                    book_id: Set(book_id.to_string()),
                    template_item_id: Set(item.template_item_id),
                    item_value: Set(item.item_value),
                    create_by: Set(Some(user_id.clone())),
                    create_time: Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };
                let model = active.insert(&txn).await?;
                result.push(model.into());
            }
        }
    }

    // 重新构建条目用于计算
    let entries: Vec<_> = result
        .iter()
        .map(|r: &MonthItemRecordResponse| MonthItemEntry {
            id: Some(r.id.clone()),
            template_item_id: r.template_item_id.clone(),
            item_value: r.item_value,
        })
        .collect();

    // 计算并更新月度汇总
    let calculated =
        calculate_month_record_data(&txn, book_id, year, month, &entries, &template_items).await?;
    let curr_record = upsert_month_record(
        &txn,
        book_id,
        year,
        month,
        &user_id,
        params.note,
        &calculated,
    )
    .await?;

    // 重新计算相关月份（下月环比 / 明年同月同比）
    recalculate_related_months(&txn, book_id, year, month, &user_id, &curr_record).await?;
    txn.commit().await?;

    Ok(Json(ApiResponse::success(result)))
}

/// 查询月度财务账目：`POST /monthItem/queryMonthItem`
///
/// 对应 Java: `FinMonthItemRecordController.queryMonthItem` 与 `FinMonthItemRecordServiceImpl.queryByBookIdAndMonth`
pub async fn query_month_item(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthItemDto>,
) -> Result<Json<ApiResponse<Vec<MonthItemRecordResponse>>>, AppError> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }
    let year = params.year.ok_or_else(|| param_err("年份不能为空"))?;
    let month = params.month.ok_or_else(|| param_err("月份不能为空"))?;

    // 校验账簿权限
    let _ = check_book_ownership(&state.db, book_id, &user_id).await?;

    let items = MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(book_id))
        .filter(MonthItemCol::Year.eq(year))
        .filter(MonthItemCol::Month.eq(month))
        .filter(MonthItemCol::IsDeleted.eq(0))
        .all(&state.db)
        .await?;

    let result = items
        .into_iter()
        .map(MonthItemRecordResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(result)))
}

/// 查询所有月度财务账目：`POST /monthItem/queryAll`
///
/// 对应 Java: `FinMonthItemRecordController.queryAll` 与 `FinMonthItemRecordServiceImpl.queryAllByBookId`
pub async fn query_all(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(params): Json<MonthItemDto>,
) -> Result<
    Json<ApiResponse<std::collections::HashMap<String, Vec<MonthItemRecordResponse>>>>,
    AppError,
> {
    let book_id = params.book_id.trim();
    if book_id.is_empty() {
        return Err(param_err("账簿ID不能为空"));
    }

    // 校验账簿权限并获取模板项（用于排序）
    let template_items = check_book_and_get_template_items(&state.db, book_id, &user_id).await?;
    let sort_map: HashMap<_, _> = template_items
        .into_iter()
        .map(|item| {
            let sort_order = item.sort.parse::<i32>().unwrap_or(i32::MAX);
            (item.id, sort_order)
        })
        .collect();

    // 查询所有明细项
    let items = MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(book_id))
        .filter(MonthItemCol::IsDeleted.eq(0))
        .order_by_asc(MonthItemCol::Year)
        .order_by_asc(MonthItemCol::Month)
        .all(&state.db)
        .await?;

    // 按年月分组
    let mut grouped: std::collections::HashMap<String, Vec<MonthItemRecordResponse>> =
        std::collections::HashMap::new();
    for item in items {
        let key = format!("{:04}{:02}", item.year, item.month);
        grouped.entry(key).or_default().push(item.into());
    }

    // 对每个月的条目按模板项排序
    for list in grouped.values_mut() {
        list.sort_by_key(|item| {
            sort_map
                .get(&item.template_item_id)
                .copied()
                .unwrap_or(i32::MAX)
        });
    }

    Ok(Json(ApiResponse::success(grouped)))
}

#[cfg(test)]
mod tests {
    //! `recalculate_related_months` 的验收测试。
    //!
    //! 对应 REFACTOR_TODO 3.1「月度汇总重算范围」：修改历史月份明细后，下月/明年同月的
    //! MoM/YoY 流转须与 Java `FinMonthRecordServiceImpl.recalculateFinMonthRecord` 一致。

    use super::*;
    use crate::common::money::calculate_growth_rate;
    use rust_decimal::Decimal;
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

    /// 构造 `Decimal`（整数 / 10^scale）。
    fn dec(val: i64, scale: u32) -> Decimal {
        Decimal::new(val, scale)
    }

    /// 建好 `FIN_MONTH_RECORD` 表的内存库连接。
    ///
    /// 刻意**不建** `FIN_MONTH_ITEM_RECORD` 表：重算逻辑对齐 Java 后只读汇总记录、
    /// 不再查询下游明细，因此下游明细「不存在 / 已删除」时仍须更新 MoM/YoY。
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接内存库失败");
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        )
        .await
        .expect("建表失败");
        db
    }

    /// 写入一条月度汇总记录。totalAsset / totalLiability / note 使用固定的哨兵值，
    /// 用于验证重算不会覆盖这些字段。
    async fn insert_record(
        db: &DatabaseConnection,
        id: &str,
        year: i32,
        month: i32,
        net_asset: &str,
        month_on_month: &str,
        year_on_year: &str,
    ) {
        db.execute_unprepared(&format!(
            "INSERT INTO FIN_MONTH_RECORD
                (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH,
                 MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET,
                 MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR, MR_NOTE)
             VALUES ('{id}', 'u1', 'bk1', {year}, {month},
                 999, 11, {net_asset}, {month_on_month}, {year_on_year}, '原始备注')"
        ))
        .await
        .unwrap();
    }

    /// 构造本月（刚重算完成）的汇总记录，供 `recalculate_related_months` 使用。
    fn curr_model(
        year: i32,
        month: i32,
        net_asset: Decimal,
    ) -> crate::entity::fin_month_record::Model {
        crate::entity::fin_month_record::Model {
            id: "curr".to_string(),
            user_id: "u1".to_string(),
            book_id: "bk1".to_string(),
            year,
            month,
            total_asset: dec(999, 0),
            total_liability: dec(11, 0),
            net_asset,
            month_on_month: dec(5, 0),
            year_on_year: dec(12, 0),
            note: Some("本月".to_string()),
            create_by: None,
            create_time: chrono::Utc::now().naive_utc(),
            update_by: None,
            update_time: None,
            is_deleted: 0,
        }
    }

    /// 下月汇总只更新 `month_on_month`，其它字段（含 note）保持不变。
    #[tokio::test]
    async fn next_month_updates_only_month_on_month() {
        let db = setup_db().await;
        insert_record(&db, "curr", 2026, 3, "100", "5", "12").await;
        insert_record(&db, "next", 2026, 4, "120", "9", "7").await;

        let curr = curr_model(2026, 3, dec(100, 0));
        recalculate_related_months(&db, "bk1", 2026, 3, "u1", &curr)
            .await
            .expect("重算失败");

        let next = get_month_record(&db, "bk1", 2026, 4)
            .await
            .unwrap()
            .expect("下月记录必须存在");

        // Java: getMonthOnMonthVal(curr, next) = (120 - 100) / |100| * 100 = 20.00
        assert_eq!(next.month_on_month, dec(2000, 2));
        assert_eq!(
            next.month_on_month,
            calculate_growth_rate(dec(120, 0), dec(100, 0))
        );
        // 其余 4 个财务字段 + note 不被覆盖（Java 只 setMonthOnMonth 单字段）
        assert_eq!(next.total_asset, dec(999, 0));
        assert_eq!(next.total_liability, dec(11, 0));
        assert_eq!(next.net_asset, dec(120, 0));
        assert_eq!(next.year_on_year, dec(7, 0));
        assert_eq!(next.note.as_deref(), Some("原始备注"));
    }

    /// 明年同月汇总只更新 `year_on_year`，其它字段保持不变。
    #[tokio::test]
    async fn next_year_updates_only_year_on_year() {
        let db = setup_db().await;
        insert_record(&db, "curr", 2026, 3, "100", "5", "12").await;
        insert_record(&db, "next_year", 2027, 3, "150", "9", "7").await;

        let curr = curr_model(2026, 3, dec(100, 0));
        recalculate_related_months(&db, "bk1", 2026, 3, "u1", &curr)
            .await
            .expect("重算失败");

        let next_year = get_month_record(&db, "bk1", 2027, 3)
            .await
            .unwrap()
            .expect("明年同月记录必须存在");

        // Java: getYearOnYearVal(curr, nextYear) = (150 - 100) / |100| * 100 = 50.00
        assert_eq!(next_year.year_on_year, dec(5000, 2));
        assert_eq!(
            next_year.year_on_year,
            calculate_growth_rate(dec(150, 0), dec(100, 0))
        );
        assert_eq!(next_year.total_asset, dec(999, 0));
        assert_eq!(next_year.total_liability, dec(11, 0));
        assert_eq!(next_year.net_asset, dec(150, 0));
        assert_eq!(next_year.month_on_month, dec(9, 0));
        assert_eq!(next_year.note.as_deref(), Some("原始备注"));
    }

    /// 下月/明年同月汇总存在但明细已删除（此处连明细表都没有）时仍更新 MoM/YoY——
    /// 对齐 Java「以已存在的净资产为基准」，去掉原先 `!next_items.is_empty()` 的跳过逻辑。
    #[tokio::test]
    async fn still_updates_when_downstream_items_deleted() {
        let db = setup_db().await;
        insert_record(&db, "curr", 2026, 3, "100", "5", "12").await;
        insert_record(&db, "next", 2026, 4, "80", "9", "7").await;
        insert_record(&db, "next_year", 2027, 3, "60", "9", "7").await;

        let curr = curr_model(2026, 3, dec(100, 0));
        recalculate_related_months(&db, "bk1", 2026, 3, "u1", &curr)
            .await
            .expect("重算失败");

        // 80 vs 100 → -20.00
        let next = get_month_record(&db, "bk1", 2026, 4)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(next.month_on_month, dec(-2000, 2));

        // 60 vs 100 → -40.00
        let next_year = get_month_record(&db, "bk1", 2027, 3)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(next_year.year_on_year, dec(-4000, 2));
    }

    /// 本月净资产为 0 时，下月/明年同月的增长率按 Java 约定置 0.00（分母为 0 不计算）。
    #[tokio::test]
    async fn zero_current_net_asset_zeroes_downstream_growth() {
        let db = setup_db().await;
        insert_record(&db, "curr", 2026, 3, "0", "5", "12").await;
        insert_record(&db, "next", 2026, 4, "120", "9", "7").await;
        insert_record(&db, "next_year", 2027, 3, "150", "9", "7").await;

        let curr = curr_model(2026, 3, Decimal::ZERO);
        recalculate_related_months(&db, "bk1", 2026, 3, "u1", &curr)
            .await
            .expect("重算失败");

        let next = get_month_record(&db, "bk1", 2026, 4)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(next.month_on_month, Decimal::ZERO);

        let next_year = get_month_record(&db, "bk1", 2027, 3)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(next_year.year_on_year, Decimal::ZERO);
    }

    /// 12 月重算时：下月跨年落到次年 1 月，明年同月落到次年 12 月，两者各自只更新单字段。
    #[tokio::test]
    async fn december_rolls_over_to_next_year() {
        let db = setup_db().await;
        insert_record(&db, "curr", 2026, 12, "100", "5", "12").await;
        insert_record(&db, "jan", 2027, 1, "130", "9", "7").await;
        insert_record(&db, "next_dec", 2027, 12, "200", "9", "7").await;

        let curr = curr_model(2026, 12, dec(100, 0));
        recalculate_related_months(&db, "bk1", 2026, 12, "u1", &curr)
            .await
            .expect("重算失败");

        // 下月 = 2027-01（跨年）：MoM = (130 - 100) / 100 = 30.00
        let jan = get_month_record(&db, "bk1", 2027, 1)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(jan.month_on_month, dec(3000, 2));
        assert_eq!(jan.year_on_year, dec(7, 0));

        // 明年同月 = 2027-12：YoY = (200 - 100) / 100 = 100.00
        let next_dec = get_month_record(&db, "bk1", 2027, 12)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(next_dec.year_on_year, dec(10000, 2));
        assert_eq!(next_dec.month_on_month, dec(9, 0));
    }

    /// 下月与明年同月汇总都不存在时是空操作，不报错。
    #[tokio::test]
    async fn missing_downstream_records_is_noop() {
        let db = setup_db().await;
        let curr = curr_model(2026, 3, dec(100, 0));
        recalculate_related_months(&db, "bk1", 2026, 3, "u1", &curr)
            .await
            .expect("无下游记录时应为空操作");
    }
}
