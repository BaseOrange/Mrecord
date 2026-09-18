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
///
/// 【有意收紧】Java 的查询接口（`queryByBookIdAndMonth` / `queryAllByBookId`）只校验
/// 账簿ID 非空（`checkBookId`），查询时不带 `userId` 过滤，任意登录用户猜对 `bookId`
/// 即可读取他人数据（IDOR 越权读取）。Rust 要求账簿必须属于当前登录用户，否则返回
/// `FinBookNotFound`。
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

/// 校验单个记账明细条目，对齐 Java `FinMonthItemRecordServiceImpl.checkFinItemList`
/// 的逐项校验，并保留 Rust 更严格的模板项归属校验。
///
/// Java 逐项校验顺序：`templateItemId` 非空 → `itemValue` 非空 → `itemValue` 非负 →
/// `bookId` 非空 → `year` / `month` 非空。其中后三项在 Java **插入**路径里会先用 DTO 级
/// 值覆盖逐项字段再校验，故等价于恒通过；只有**更新**路径才真正校验逐项字段。本函数用
/// `is_insert` 参数复现这一语义。
///
/// 与 Java 的两处有意差异（已记录于 REFACTOR_TODO 3.2）：
/// - Rust 额外校验 `template_item_id` 必须属于当前账簿（Java 静默接受跨账簿模板项，
///   仅在汇总时不计入），返回 `FinItemTempNotExist`；
/// - 逐项 `book_id` / `year` / `month` 仅校验、**不用于写库**：写库统一取 DTO 级值，
///   避免逐项 `bookId` 指向其它账簿造成跨库写入（与 `item.id` 归属校验同一类收紧）。
fn validate_item_entry(
    item: &MonthItemEntry,
    template_ids: &std::collections::HashSet<&str>,
    is_insert: bool,
) -> Result<(), AppError> {
    if item.template_item_id.trim().is_empty() {
        return Err(param_err("记账账目ID不能为空"));
    }
    // 模板项归属校验（Rust 严格化：拒绝跨账簿模板项）
    if !template_ids.contains(item.template_item_id.trim()) {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }
    // itemValue 非空（对齐 Java itemValue == null 判断，原依赖反序列化裸 400）
    let item_value = item
        .item_value
        .ok_or_else(|| param_err("记账账目金额不能为空"))?;
    if item_value < Decimal::ZERO {
        return Err(param_err("记账账目金额必须大于等于零"));
    }
    if !is_insert {
        // 更新路径逐项字段必填（Java checkFinItemList 在 isInsert=false 时生效）
        if item
            .book_id
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            return Err(param_err("账簿ID不能为空"));
        }
        if item.year.is_none() || item.month.is_none() {
            return Err(param_err("年份和月份不能为空"));
        }
    }
    Ok(())
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
            // 调用方已通过 validate_item_entry 校验金额非空，此处兜底取 0
            let item_value = item.item_value.unwrap_or(Decimal::ZERO);
            match item_type {
                1 => total_asset += item_value,      // 资产
                -1 => total_liability += item_value, // 负债
                0 => {}                              // 仅记录，不计入
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

    // 校验明细项（对齐 Java checkFinItemList，is_insert=true）
    let template_ids: std::collections::HashSet<_> =
        template_items.iter().map(|i| i.id.as_str()).collect();
    for item in &item_list {
        validate_item_entry(item, &template_ids, true)?;
    }

    // 插入明细项
    let mut result: Vec<MonthItemRecordResponse> = Vec::with_capacity(item_list.len());
    for item in item_list {
        // 校验已保证 item_value 非空；写库统一用 DTO 级 bookId/year/month（见 validate_item_entry 注释）
        let active = MonthItemActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            year: Set(year),
            month: Set(month),
            book_id: Set(book_id.to_string()),
            template_item_id: Set(item.template_item_id),
            item_value: Set(item.item_value.unwrap_or(Decimal::ZERO)),
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
            item_value: Some(r.item_value),
            book_id: Some(r.book_id.clone()),
            year: Some(r.year),
            month: Some(r.month),
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

    // 校验明细项（对齐 Java checkFinItemList，is_insert=false：逐项 bookId/year/month 必填）
    let template_ids: std::collections::HashSet<_> =
        template_items.iter().map(|i| i.id.as_str()).collect();
    for item in &item_list {
        validate_item_entry(item, &template_ids, false)?;
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
                // 更新现有项（item.id 归属校验：不存在于本账簿本月时拒绝，修复 Java 的越权写入）
                let existing = existing_map
                    .get(id_str)
                    .ok_or(AppError::ResCode(ResCode::FinItemNotFound))?;
                let mut active: MonthItemActive = existing.clone().into();
                active.template_item_id = Set(item.template_item_id);
                active.item_value = Set(item.item_value.unwrap_or(Decimal::ZERO));
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
                    item_value: Set(item.item_value.unwrap_or(Decimal::ZERO)),
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
            item_value: Some(r.item_value),
            book_id: Some(r.book_id.clone()),
            year: Some(r.year),
            month: Some(r.month),
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

    // 【有意收紧】账簿归属校验：修复 Java `queryByBookIdAndMonth` 的 IDOR 越权读取（见 check_book_ownership）
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
///
/// 与 Java 的差异：账簿无模板项时返回空分组数据而非抛 `FinItemTempNotExist`——对齐
/// Java `queryAllByBookId`「无明细记录时返回空 Map」的主语义（Java 仅在「有明细但无
/// 模板项」这一自相矛盾的场景下才经 `getSortMap` → `selectByFinBookIdExternal` 抛
/// 14301；该场景在正常业务流中不可能出现，Rust 统一返回空分组更稳健）。
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

    // 【有意收紧】账簿归属校验：修复 Java `queryAllByBookId` 的 IDOR 越权读取（见 check_book_ownership）
    let _ = check_book_ownership(&state.db, book_id, &user_id).await?;

    // 模板项仅用于排序，缺失时 sort_map 为空（排序兜底 i32::MAX），不抛异常
    let sort_map: HashMap<_, _> = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .all(&state.db)
        .await?
        .into_iter()
        .map(|item| (item.id, item.sort.parse::<i32>().unwrap_or(i32::MAX)))
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
    //! 月度明细校验、查询与重算的验收测试。
    //!
    //! - 校验部分对应 REFACTOR_TODO 3.2 与 Java
    //!   `FinMonthItemRecordServiceImpl.checkFinItemList`；
    //! - 查询部分对应 REFACTOR_TODO 3.3：归属校验（修复 Java IDOR）与
    //!   `queryAll` 无模板项时返回空分组（对齐 Java 空结果语义）；
    //! - 重算部分对应 REFACTOR_TODO 3.1：修改历史月份明细后，下月/明年同月的
    //!   MoM/YoY 流转须与 Java `FinMonthRecordServiceImpl.recalculateFinMonthRecord` 一致。

    use super::*;
    use crate::AppState;
    use crate::common::money::calculate_growth_rate;
    use crate::error::AppError;
    use crate::model::finance::{MonthItemDto, MonthItemEntry};
    use crate::service::{
        cancel_cleanup_task::CancelCleanupTask, email::EmailService,
        export_task::ExportTaskService, monthly_reminder_task::MonthlyReminderTask,
        sys_config::SysConfigService, sys_user_operate_log::SysUserOperateLogService,
    };
    use axum::extract::State;
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

    // ==================== validate_item_entry 测试 ====================
    //
    // 对应 Java: FinMonthItemRecordServiceImpl.checkFinItemList

    /// 提取 AppError 的响应码字符串（ParamError→10001，ResCode→对应码）。
    fn err_code(err: &AppError) -> &str {
        match err {
            AppError::Business { code, .. } => code.as_str(),
            AppError::ResCode(rc) => rc.code(),
            AppError::NotFound => ResCode::DataNotExist.code(),
            AppError::Internal(_) => ResCode::Error.code(),
        }
    }

    /// 本账簿的模板项 ID 集合：含 tpl-a，不含 tpl-other（模拟跨账簿模板项）。
    fn book_template_ids() -> std::collections::HashSet<&'static str> {
        ["tpl-a"].into_iter().collect()
    }

    /// 构造明细条目；`item_value` / `book_id` / `year` / `month` 按需置空以覆盖各校验分支。
    fn entry_of(
        template_item_id: &str,
        item_value: Option<Decimal>,
        book_id: Option<&str>,
        year: Option<i32>,
        month: Option<i32>,
    ) -> MonthItemEntry {
        MonthItemEntry {
            id: None,
            template_item_id: template_item_id.to_string(),
            item_value,
            book_id: book_id.map(str::to_string),
            year,
            month,
        }
    }

    /// 合法条目（插入 / 更新路径都应通过）。
    #[test]
    fn valid_entry_passes_both_paths() {
        let ids = book_template_ids();
        let item = entry_of("tpl-a", Some(dec(100, 0)), Some("bk1"), Some(2026), Some(3));
        validate_item_entry(&item, &ids, true).expect("插入校验应通过");
        validate_item_entry(&item, &ids, false).expect("更新校验应通过");
    }

    /// templateItemId 为空 → ParamError(10001)。
    #[test]
    fn blank_template_item_id_rejected() {
        let ids = book_template_ids();
        let item = entry_of(" ", Some(dec(100, 0)), Some("bk1"), Some(2026), Some(3));
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, true).unwrap_err()),
            "10001"
        );
    }

    /// itemValue 缺失 / null → 业务响应 ParamError(10001)，不再是反序列化的裸 400。
    #[test]
    fn missing_item_value_rejected_as_business_error() {
        let ids = book_template_ids();
        let item = entry_of("tpl-a", None, Some("bk1"), Some(2026), Some(3));
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, true).unwrap_err()),
            "10001"
        );
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, false).unwrap_err()),
            "10001"
        );
    }

    /// itemValue 为负 → ParamError(10001)。
    #[test]
    fn negative_item_value_rejected() {
        let ids = book_template_ids();
        let item = entry_of("tpl-a", Some(dec(-5, 0)), Some("bk1"), Some(2026), Some(3));
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, true).unwrap_err()),
            "10001"
        );
    }

    /// 模板项不属于本账簿 → FinItemTempNotExist(14301)（Rust 比 Java 更严格，有意保留）。
    #[test]
    fn cross_book_template_item_rejected() {
        let ids = book_template_ids();
        let item = entry_of(
            "tpl-other",
            Some(dec(100, 0)),
            Some("bk1"),
            Some(2026),
            Some(3),
        );
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, false).unwrap_err()),
            "14301"
        );
    }

    /// 插入路径不要求逐项 bookId/year/month（Java 插入时先用 DTO 级值覆盖，等价恒通过）。
    #[test]
    fn insert_tolerates_missing_per_item_fields() {
        let ids = book_template_ids();
        let item = entry_of("tpl-a", Some(dec(100, 0)), None, None, None);
        validate_item_entry(&item, &ids, true).expect("插入路径不应校验逐项 bookId/year/month");
    }

    /// 更新路径逐项 bookId 缺失 → ParamError(10001)。
    #[test]
    fn update_requires_per_item_book_id() {
        let ids = book_template_ids();
        let item = entry_of("tpl-a", Some(dec(100, 0)), None, Some(2026), Some(3));
        assert_eq!(
            err_code(&validate_item_entry(&item, &ids, false).unwrap_err()),
            "10001"
        );
    }

    /// 更新路径逐项 year/month 缺失 → ParamError(10001)。
    #[test]
    fn update_requires_per_item_year_and_month() {
        let ids = book_template_ids();
        let no_year = entry_of("tpl-a", Some(dec(100, 0)), Some("bk1"), None, Some(3));
        assert_eq!(
            err_code(&validate_item_entry(&no_year, &ids, false).unwrap_err()),
            "10001"
        );
        let no_month = entry_of("tpl-a", Some(dec(100, 0)), Some("bk1"), Some(2026), None);
        assert_eq!(
            err_code(&validate_item_entry(&no_month, &ids, false).unwrap_err()),
            "10001"
        );
    }

    /// 请求 JSON 反序列化：itemValue 缺失 / null → None（交业务层校验，不再是裸 400），
    /// 数字 / 字符串 → Some；逐项 bookId/year/month 按驼峰映射。
    #[test]
    fn month_item_entry_tolerates_missing_and_null_value() {
        // 缺失 itemValue
        let no_value: MonthItemEntry = serde_json::from_str(
            r#"{"templateItemId":"tpl-a","bookId":"bk1","year":2026,"month":3}"#,
        )
        .unwrap();
        assert!(no_value.item_value.is_none());
        assert_eq!(no_value.book_id.as_deref(), Some("bk1"));
        assert_eq!(no_value.year, Some(2026));

        // 显式 null
        let null_value: MonthItemEntry =
            serde_json::from_str(r#"{"templateItemId":"tpl-a","itemValue":null}"#).unwrap();
        assert!(null_value.item_value.is_none());

        // 数字与字符串金额
        let num: MonthItemEntry =
            serde_json::from_str(r#"{"templateItemId":"tpl-a","itemValue":12.5}"#).unwrap();
        assert_eq!(num.item_value, Some(dec(1250, 2)));
        let str_val: MonthItemEntry =
            serde_json::from_str(r#"{"templateItemId":"tpl-a","itemValue":"12.5"}"#).unwrap();
        assert_eq!(str_val.item_value, Some(dec(1250, 2)));
    }

    // ==================== query_all 测试 ====================
    //
    // 对应 REFACTOR_TODO 3.3：归属校验（修复 Java IDOR）+
    // 账簿无模板项时返回空分组数据（对齐 Java，原先抛 FinItemTempNotExist）

    /// 建好账簿 / 模板项 / 明细三张表，并写入一个属于 u1 的账簿。
    async fn setup_query_db() -> DatabaseConnection {
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
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db.execute_unprepared(
            "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('bk1', 'u1', '我的账簿')",
        )
        .await
        .unwrap();
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
            cancel_cleanup_task: CancelCleanupTask::new(),
        }
    }

    fn query_all_params(book_id: &str) -> MonthItemDto {
        MonthItemDto {
            book_id: book_id.to_string(),
            year: None,
            month: None,
            item_list: None,
            note: None,
        }
    }

    /// 账簿无模板项且无明细时返回空分组（不抛 FinItemTempNotExist），对齐 Java 空结果。
    #[tokio::test]
    async fn query_all_returns_empty_when_book_has_no_template_items() {
        let db = setup_query_db().await;
        let state = test_state(db);

        let res = query_all(
            AuthUser("u1".to_string()),
            State(state),
            Json(query_all_params("bk1")),
        )
        .await
        .expect("无模板项时应返回空分组而非报错");

        let grouped = res.0.data.unwrap();
        assert!(
            grouped.is_empty(),
            "无模板项无明细时应返回空分组: {grouped:?}"
        );
    }

    /// 有模板项与明细时按年月分组、按模板项 sort 排序。
    #[tokio::test]
    async fn query_all_groups_by_month_and_sorts_by_template() {
        let db = setup_query_db().await;
        // 模板项：tpl-b 排序 1，tpl-a 排序 2
        for (id, sort) in [("tpl-b", "1"), ("tpl-a", "2")] {
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_TEMPLATE_ITEM (MR_ID, MR_BOOK_ID, MR_ITEM_NAME, MR_ITEM_TYPE, MR_ICON, MR_SORT)
                 VALUES ('{id}', 'bk1', '{id}', 1, 'icon', '{sort}')"
            ))
            .await
            .unwrap();
        }
        // 明细：tpl-a 在 2026-03，tpl-b 在 2026-03，另一条在 2026-01
        for (id, year, month, value) in [
            ("i1", 2026, 3, 100),
            ("i2", 2026, 3, 200),
            ("i3", 2026, 1, 50),
        ] {
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_MONTH_ITEM_RECORD (MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE)
                 VALUES ('{id}', {year}, {month}, 'bk1', 'tpl-{}', {value})",
                if id == "i1" { "a" } else { "b" }
            ))
            .await
            .unwrap();
        }

        let state = test_state(db);
        let res = query_all(
            AuthUser("u1".to_string()),
            State(state),
            Json(query_all_params("bk1")),
        )
        .await
        .unwrap();
        let grouped = res.0.data.unwrap();

        assert_eq!(grouped.len(), 2, "应按年月分两组: {grouped:?}");
        // 2026 年 3 月：tpl-b（sort=1）排在 tpl-a（sort=2）之前
        let mar = grouped.get("202603").expect("202603 分组必须存在");
        assert_eq!(mar.len(), 2);
        assert_eq!(mar[0].template_item_id, "tpl-b");
        assert_eq!(mar[1].template_item_id, "tpl-a");
        assert_eq!(grouped.get("202601").unwrap()[0].item_value, dec(5000, 2));
    }

    /// 账簿属于他人时拒绝读取（修复 Java 的 IDOR 越权读取）。
    #[tokio::test]
    async fn query_all_rejects_book_of_other_user() {
        let db = setup_query_db().await;
        let state = test_state(db);

        let err = query_all(
            AuthUser("attacker".to_string()),
            State(state),
            Json(query_all_params("bk1")),
        )
        .await
        .unwrap_err();

        // 【有意收紧】Java 不带 userId 过滤可读他人账簿；Rust 返回账簿不存在
        assert!(matches!(err, AppError::ResCode(ResCode::FinBookNotFound)));
    }
}
