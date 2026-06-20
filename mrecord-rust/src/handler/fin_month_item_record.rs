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
        money::{round_money, zero_money},
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

/// 计算同比/环比增长率。
/// 返回百分比值（如 10.50 表示 10.50%），对应 Java BigDecimal 的两位 HALF_UP 规则。
fn calculate_growth_rate(current: Decimal, base: Decimal) -> Decimal {
    if base.is_zero() {
        return zero_money();
    }
    round_money((current - base) / base.abs() * Decimal::from(100))
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

    // 计算总资产和总负债
    let mut total_asset = Decimal::ZERO;
    let mut total_liability = Decimal::ZERO;

    for item in item_list {
        let item_value = round_money(item.item_value);
        if let Some(&item_type) = type_map.get(&item.template_item_id) {
            match item_type {
                1 => total_asset += item_value,      // 资产
                -1 => total_liability += item_value, // 负债
                0 => {}                              // 仅记录，不计入
                _ => {}
            }
        }
    }

    // 保留两位小数
    total_asset = round_money(total_asset);
    total_liability = round_money(total_liability);
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
            ..Default::default()
        };
        active.insert(db).await?
    };

    Ok(record)
}

/// 重新计算后续月份的环比和明年同月的同比。
async fn recalculate_related_months<C>(
    db: &C,
    book_id: &str,
    year: i32,
    month: i32,
    user_id: &str,
    template_items: &[fin_template_item::Model],
) -> Result<(), AppError>
where
    C: ConnectionTrait,
{
    // 更新下个月的环比
    let (next_year, next_month_val) = next_month(year, month);
    if let Some(next_record) = get_month_record(db, book_id, next_year, next_month_val).await? {
        // 获取下个月的明细项
        let next_items = MonthItemEntity::find()
            .filter(MonthItemCol::BookId.eq(book_id))
            .filter(MonthItemCol::Year.eq(next_year))
            .filter(MonthItemCol::Month.eq(next_month_val))
            .filter(MonthItemCol::IsDeleted.eq(0))
            .all(db)
            .await?;

        if !next_items.is_empty() {
            let entries: Vec<_> = next_items
                .into_iter()
                .map(|item| MonthItemEntry {
                    id: Some(item.id),
                    template_item_id: item.template_item_id,
                    item_value: item.item_value,
                })
                .collect();

            let calculated = calculate_month_record_data(
                db,
                book_id,
                next_year,
                next_month_val,
                &entries,
                template_items,
            )
            .await?;

            // 更新下个月记录（保持原有的 note）
            let mut active: MonthRecordActive = next_record.clone().into();
            active.total_asset = Set(calculated.total_asset);
            active.total_liability = Set(calculated.total_liability);
            active.net_asset = Set(calculated.net_asset);
            active.month_on_month = Set(calculated.month_on_month);
            active.year_on_year = Set(calculated.year_on_year);
            active.update_by = Set(Some(user_id.to_string()));
            active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
            active.update(db).await?;
        }
    }

    // 更新明年同月的同比
    if let Some(next_year_record) = get_month_record(db, book_id, year + 1, month).await? {
        // 获取明年同月的明细项
        let next_year_items = MonthItemEntity::find()
            .filter(MonthItemCol::BookId.eq(book_id))
            .filter(MonthItemCol::Year.eq(year + 1))
            .filter(MonthItemCol::Month.eq(month))
            .filter(MonthItemCol::IsDeleted.eq(0))
            .all(db)
            .await?;

        if !next_year_items.is_empty() {
            let entries: Vec<_> = next_year_items
                .into_iter()
                .map(|item| MonthItemEntry {
                    id: Some(item.id),
                    template_item_id: item.template_item_id,
                    item_value: item.item_value,
                })
                .collect();

            let calculated =
                calculate_month_record_data(db, book_id, year + 1, month, &entries, template_items)
                    .await?;

            // 更新明年同月记录
            let mut active: MonthRecordActive = next_year_record.clone().into();
            active.total_asset = Set(calculated.total_asset);
            active.total_liability = Set(calculated.total_liability);
            active.net_asset = Set(calculated.net_asset);
            active.month_on_month = Set(calculated.month_on_month);
            active.year_on_year = Set(calculated.year_on_year);
            active.update_by = Set(Some(user_id.to_string()));
            active.update_time = Set(Some(chrono::Utc::now().naive_utc()));
            active.update(db).await?;
        }
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
            item_value: Set(round_money(item.item_value)),
            create_by: Set(Some(user_id.clone())),
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
    let _ = upsert_month_record(
        &txn,
        book_id,
        year,
        month,
        &user_id,
        params.note,
        &calculated,
    )
    .await?;

    // 重新计算相关月份
    recalculate_related_months(&txn, book_id, year, month, &user_id, &template_items).await?;
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
                    item_value: Set(round_money(item.item_value)),
                    create_by: Set(Some(user_id.clone())),
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
    let _ = upsert_month_record(
        &txn,
        book_id,
        year,
        month,
        &user_id,
        params.note,
        &calculated,
    )
    .await?;

    // 重新计算相关月份
    recalculate_related_months(&txn, book_id, year, month, &user_id, &template_items).await?;
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
