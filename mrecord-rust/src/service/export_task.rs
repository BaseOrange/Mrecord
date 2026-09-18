//! 导出任务服务
//!
//! 对应 Java:
//! - `com.dcz.mrecord.service.SysExportTaskService`（创建任务 / 分页查询）
//! - `com.dcz.mrecord.service.impl.SysExportTaskServiceImpl`
//! - `com.dcz.mrecord.service.ExportTaskExecutorService`（异步执行导出）
//! - `com.dcz.mrecord.service.impl.ExportTaskExecutorServiceImpl`
//!
//! 设计要点（与 Java `ExportTaskExecutorServiceImpl` 对齐）：
//! - **任务粒度**：一次导出请求只创建一个任务。`bookId` 为空时导出用户全部
//!   账簿，生成一个「每账簿一个 Sheet」的 Excel；`bookId` 非空时只导该账簿。
//! - **Excel 布局**：表头为「统计月份 / 总资产 / 总负债 / 净资产 / 环比 / 同比
//!   / 各资产明细项… / 各负债明细项…」，每月一行，明细按 `templateItemId`
//!   映射到对应列，缺失填 0；环比 / 同比输出 `"x%"` 字符串。
//! - **文件**：`账簿导出_yyyyMMddHHmmss.xlsx`，写入工作目录下的
//!   `exports/` 子目录（容器化时随 `/app` 数据卷持久化，见项目 Dockerfile）。
//! - **状态流转**：WAIT → RUN →（SUCCESS 邮件通知 | FAIL 记录原因并删除文件）。

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::Context;
use chrono::Local;
use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_xlsxwriter::{Workbook, XlsxError};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};
use tokio::task;
use uuid::Uuid;

use crate::{
    common::res_code::ResCode,
    constant::{export_task::ExportTaskStatus, template_item::TempItemType},
    entity::{
        fin_book::{Column as BookCol, Entity as BookEntity, Model as BookModel},
        fin_month_item_record::{
            Column as MonthItemCol, Entity as MonthItemEntity, Model as MonthItemModel,
        },
        fin_month_record::{
            Column as MonthRecordCol, Entity as MonthRecordEntity, Model as MonthRecordModel,
        },
        fin_template_item::{
            Column as TemplateItemCol, Entity as TemplateItemEntity, Model as TemplateItemModel,
        },
        sys_export_task::{ActiveModel as ExportTaskActive, Model as ExportTaskModel},
        sys_user::{Column as UserCol, Entity as UserEntity, Model as UserModel},
    },
    error::AppError,
    model::{finance::ExportBookDto, mail_params::MailParams},
    service::email::EmailService,
};

/// 失败原因写入 DB 时的最大长度（对齐 Java `StrUtil.sub(e.getMessage(), 0, 500)`）
const FAIL_REASON_MAX_LEN: usize = 500;

/// Excel Sheet 名称的最大长度（Excel 硬限制）
const SHEET_NAME_MAX_LEN: usize = 31;

/// 导出任务服务，持有邮件服务用于任务完成通知。
pub struct ExportTaskService {
    email_service: Arc<EmailService>,
}

impl ExportTaskService {
    /// 创建导出任务服务实例。
    pub fn new(email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { email_service })
    }

    /// 创建导出任务（一次请求一个任务）并异步执行。
    ///
    /// 对应 Java: `SysExportTaskController.export` →
    /// `SysExportTaskServiceImpl.createExportTask` + `ExportTaskExecutorService.executeExport`。
    /// 返回创建好的任务实体，供 controller 直接响应给前端（含 `taskId`）。
    pub async fn create_export_task(
        self: Arc<Self>,
        db: DatabaseConnection,
        user_id: String,
        params: ExportBookDto,
    ) -> Result<ExportTaskModel, AppError> {
        let book_id = params.book_id.filter(|s| !s.trim().is_empty());
        let start = params.start_year_month.filter(|s| !s.trim().is_empty());
        let end = params.end_year_month.filter(|s| !s.trim().is_empty());
        validate_year_month_range(start.as_deref(), end.as_deref())?;

        // 指定账簿时校验归属；不指定时稍后在执行阶段再查全部账簿
        if let Some(id) = book_id.as_deref() {
            check_book_ownership(&db, &user_id, id).await?;
        }

        let task = ExportTaskActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            user_id: Set(user_id.clone()),
            book_id: Set(book_id),
            start_year_month: Set(start),
            end_year_month: Set(end),
            status: Set(ExportTaskStatus::Wait.as_str().to_string()),
            create_by: Set(Some(user_id.clone())),
            create_time: Set(Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await?;

        let service = self.clone();
        let db_for_task = db.clone();
        let task_model = task.clone();
        task::spawn(async move {
            if let Err(e) = service.process_task(db_for_task, task_model).await {
                tracing::error!("导出任务后台处理失败: {:?}", e);
            }
        });

        Ok(task)
    }

    /// 处理单个导出任务：RUN → 生成 Excel → SUCCESS 发邮件 / FAIL 记原因。
    ///
    /// 对应 Java: `ExportTaskExecutorServiceImpl.executeExport`（`@Async`）。
    async fn process_task(
        self: Arc<Self>,
        db: DatabaseConnection,
        task: ExportTaskModel,
    ) -> Result<(), AppError> {
        self.update_status(&db, &task, ExportTaskStatus::Run, None, None)
            .await?;

        match self.execute_export(&db, &task).await {
            Ok(file_path) => {
                let file_name = file_path
                    .file_name()
                    .and_then(|v| v.to_str())
                    .unwrap_or("mrecord-export.xlsx")
                    .to_string();
                self.update_status(
                    &db,
                    &task,
                    ExportTaskStatus::Success,
                    Some(file_name.clone()),
                    None,
                )
                .await?;

                if let Err(e) = self
                    .send_completion_email(&db, &task, &file_name, file_path)
                    .await
                {
                    tracing::warn!("导出任务完成邮件发送失败，任务保持成功状态: {:?}", e);
                }
                Ok(())
            }
            Err(e) => {
                let reason = truncate_fail_reason(&format!("{:?}", e));
                self.update_status(&db, &task, ExportTaskStatus::Fail, None, Some(reason))
                    .await?;
                Err(e)
            }
        }
    }

    /// 生成导出 Excel 文件（多账簿 → 多 Sheet）。
    ///
    /// 对应 Java: `ExportTaskExecutorServiceImpl` 中更新 RUN → 查账簿 →
    /// `ExcelWriter` 逐 Sheet 写入 → 更新 SUCCESS 的主流程。
    async fn execute_export(
        &self,
        db: &DatabaseConnection,
        task: &ExportTaskModel,
    ) -> Result<PathBuf, AppError> {
        // 用户邮箱校验：没有邮箱就无法寄出导出文件
        let user = UserEntity::find_by_id(task.user_id.clone())
            .filter(UserCol::IsDeleted.eq(0))
            .one(db)
            .await?
            .ok_or_else(|| param_error("用户不存在"))?;
        if user.email.trim().is_empty() {
            return Err(param_error("用户邮箱未配置，无法发送导出文件"));
        }

        let books = resolve_books(db, &task.user_id, task.book_id.as_deref()).await?;
        if books.is_empty() {
            return Err(param_error("无可导出的账簿数据"));
        }

        // 逐账簿取数（任一账簿缺模板项即抛 FinItemTempNotExist，与 Java 一致）
        let mut sheets = Vec::with_capacity(books.len());
        for book in &books {
            sheets.push(fetch_book_sheet_data(db, book, task).await?);
        }

        let file_name = format!("账簿导出_{}.xlsx", Local::now().format("%Y%m%d%H%M%S"));
        // 相对工作目录（容器内即 `/app`），随数据卷持久化；目录由 write_excel 自动创建
        let export_dir = PathBuf::from("exports");
        let file_path = export_dir.join(&file_name);

        let path_for_blocking = file_path.clone();
        task::spawn_blocking(move || write_excel(path_for_blocking, &sheets))
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))??;

        Ok(file_path)
    }

    /// 更新导出任务状态。
    ///
    /// 对应 Java `SysExportTask` 状态流转字段更新（`updateById`）。
    async fn update_status(
        &self,
        db: &DatabaseConnection,
        task: &ExportTaskModel,
        status: ExportTaskStatus,
        file_name: Option<String>,
        fail_reason: Option<String>,
    ) -> Result<(), AppError> {
        let mut active: ExportTaskActive = task.clone().into();
        active.status = Set(status.as_str().to_string());
        active.file_name = Set(file_name);
        active.fail_reason = Set(fail_reason);
        active.update_by = Set(Some(task.user_id.clone()));
        active.update_time = Set(Some(Local::now().naive_local()));
        active.update(db).await?;
        Ok(())
    }

    /// 发送导出完成邮件。
    ///
    /// 对应 Java: `EmailServiceImpl.sendExportSuccessEmail`。
    async fn send_completion_email(
        &self,
        db: &DatabaseConnection,
        task: &ExportTaskModel,
        file_name: &str,
        file_path: PathBuf,
    ) -> Result<(), AppError> {
        let user = UserEntity::find_by_id(task.user_id.clone())
            .filter(UserCol::IsDeleted.eq(0))
            .one(db)
            .await?
            .ok_or(AppError::ResCode(ResCode::DataNotExist))?;
        let params = build_export_mail_params(user, file_name);
        self.email_service
            .send_export_book_email(db, params, &file_path)
            .await
    }
}

// ==================== 数据查询 ====================

/// 校验指定账簿存在且属于当前用户。
///
/// 对应 Java: `getExportBooks` 中 `book.getUserId()` 权限校验分支。
async fn check_book_ownership(
    db: &DatabaseConnection,
    user_id: &str,
    book_id: &str,
) -> Result<(), AppError> {
    let owned = BookEntity::find()
        .filter(BookCol::Id.eq(book_id.to_string()))
        .filter(BookCol::UserId.eq(user_id.to_string()))
        .filter(BookCol::IsDeleted.eq(0))
        .one(db)
        .await?;
    if owned.is_none() {
        return Err(AppError::ResCode(ResCode::NoPermission));
    }
    Ok(())
}

/// 根据任务解析要导出的账簿集合。
///
/// 对应 Java: `getExportBooks`。`book_id` 为空时返回用户全部账簿。
async fn resolve_books(
    db: &DatabaseConnection,
    user_id: &str,
    book_id: Option<&str>,
) -> Result<Vec<BookModel>, AppError> {
    let mut q = BookEntity::find()
        .filter(BookCol::UserId.eq(user_id.to_string()))
        .filter(BookCol::IsDeleted.eq(0));
    if let Some(book_id) = book_id.filter(|id| !id.trim().is_empty()) {
        q = q.filter(BookCol::Id.eq(book_id.to_string()));
    }
    Ok(q.order_by_asc(BookCol::CreateTime).all(db).await?)
}

/// 单个账簿的 Sheet 数据（模板项 + 月度汇总 + 月度明细）
struct BookSheetData {
    /// 账簿（Sheet 名取自 `book_name`）
    book: BookModel,
    /// 资产类模板项（按 sort 升序）
    asset_items: Vec<TemplateItemModel>,
    /// 负债类模板项（按 sort 升序）
    liability_items: Vec<TemplateItemModel>,
    /// 月度汇总（按年月升序）
    month_records: Vec<MonthRecordModel>,
    /// 月度明细
    month_items: Vec<MonthItemModel>,
}

/// 取单个账簿生成 Sheet 所需的全部数据。
///
/// 对应 Java: `writeBookData` 的取数部分。模板项不存在时抛
/// `ResCode::FinItemTempNotExist`（Java `ResCode.FIN_ITEM_TEMP_IS_NOT`）。
async fn fetch_book_sheet_data(
    db: &DatabaseConnection,
    book: &BookModel,
    task: &ExportTaskModel,
) -> Result<BookSheetData, AppError> {
    let template_items = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book.id.clone()))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    if template_items.is_empty() {
        return Err(AppError::ResCode(ResCode::FinItemTempNotExist));
    }

    // 按类型分拣并按 sort 升序（Java `parseSort`：非数字排在最后）
    let mut asset_items = Vec::new();
    let mut liability_items = Vec::new();
    for item in template_items {
        match TempItemType::from_i32(item.item_type) {
            Some(TempItemType::Asset) => asset_items.push(item),
            Some(TempItemType::Liability) => liability_items.push(item),
            _ => {}
        }
    }
    asset_items.sort_by_key(|i| parse_sort(&i.sort));
    liability_items.sort_by_key(|i| parse_sort(&i.sort));

    let month_records = query_month_records(db, &book.id, task).await?;
    let month_items = query_month_items(db, &book.id, task).await?;

    Ok(BookSheetData {
        book: book.clone(),
        asset_items,
        liability_items,
        month_records,
        month_items,
    })
}

/// 查询导出范围内的月度汇总（按年月升序）。
async fn query_month_records(
    db: &DatabaseConnection,
    book_id: &str,
    task: &ExportTaskModel,
) -> Result<Vec<MonthRecordModel>, AppError> {
    let mut q = MonthRecordEntity::find()
        .filter(MonthRecordCol::BookId.eq(book_id.to_string()))
        .filter(MonthRecordCol::IsDeleted.eq(0));
    if let Some(cond) = year_month_range_condition(
        task.start_year_month.as_deref(),
        task.end_year_month.as_deref(),
        MonthRecordCol::Year,
        MonthRecordCol::Month,
    )? {
        q = q.filter(cond);
    }
    Ok(q.order_by_asc(MonthRecordCol::Year)
        .order_by_asc(MonthRecordCol::Month)
        .all(db)
        .await?)
}

/// 查询导出范围内的月度明细。
async fn query_month_items(
    db: &DatabaseConnection,
    book_id: &str,
    task: &ExportTaskModel,
) -> Result<Vec<MonthItemModel>, AppError> {
    let mut q = MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(book_id.to_string()))
        .filter(MonthItemCol::IsDeleted.eq(0));
    if let Some(cond) = year_month_range_condition(
        task.start_year_month.as_deref(),
        task.end_year_month.as_deref(),
        MonthItemCol::Year,
        MonthItemCol::Month,
    )? {
        q = q.filter(cond);
    }
    Ok(q.order_by_asc(MonthItemCol::Year)
        .order_by_asc(MonthItemCol::Month)
        .all(db)
        .await?)
}

/// 按 yyyyMM 范围构造过滤条件；两端皆为空时返回 `None`（不限范围）。
///
/// 对应 Java: `filterByYearMonth`（字符串比较，左闭右闭）。
fn year_month_range_condition<C>(
    start: Option<&str>,
    end: Option<&str>,
    year_col: C,
    month_col: C,
) -> Result<Option<Condition>, AppError>
where
    C: ColumnTrait,
{
    let (start_year, start_month) = match start {
        Some(s) => Some(parse_year_month(s)?),
        None => None,
    }
    .unzip();
    let (end_year, end_month) = match end {
        Some(s) => Some(parse_year_month(s)?),
        None => None,
    }
    .unzip();

    let mut cond = Condition::all();
    if let (Some(sy), Some(sm)) = (start_year, start_month) {
        cond = cond.add(
            Condition::any()
                .add(year_col.gt(sy))
                .add(Condition::all().add(year_col.eq(sy)).add(month_col.gte(sm))),
        );
    }
    if let (Some(ey), Some(em)) = (end_year, end_month) {
        cond = cond.add(
            Condition::any()
                .add(year_col.lt(ey))
                .add(Condition::all().add(year_col.eq(ey)).add(month_col.lte(em))),
        );
    }
    if start_year.is_none() && end_year.is_none() {
        return Ok(None);
    }
    Ok(Some(cond))
}

// ==================== Excel 写入 ====================

/// 写入 Excel 文件：每个账簿一个 Sheet。
///
/// 对应 Java: `executeExport` 中 `for (FinBook book : books)` 的 Sheet 循环。
fn write_excel(file_path: PathBuf, sheets: &[BookSheetData]) -> Result<(), AppError> {
    std::fs::create_dir_all(
        file_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new(".")),
    )
    .context("创建导出目录失败")?;

    let mut workbook = Workbook::new();
    let mut used_names: HashMap<String, ()> = HashMap::new();
    for data in sheets {
        let title = unique_sheet_name(&data.book.book_name, &mut used_names);
        let worksheet = workbook
            .add_worksheet()
            .set_name(title)
            .map_err(map_xlsx_error)?;
        write_book_sheet(worksheet, data)?;
    }
    // 保存失败时删除残留的半成品文件（对齐 Java 的 `excelFile.delete()`）
    if let Err(e) = workbook.save(&file_path) {
        let _ = std::fs::remove_file(&file_path);
        return Err(map_xlsx_error(e));
    }
    Ok(())
}

/// 写入单个账簿的 Sheet。
///
/// 对应 Java: `writeBookData`。表头：统计月份 / 总资产 / 总负债 / 净资产 /
/// 环比 / 同比 / 各资产明细项… / 各负债明细项…；数据行每月一行。
fn write_book_sheet(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    data: &BookSheetData,
) -> Result<(), AppError> {
    // 表头
    let mut headers: Vec<String> = ["统计月份", "总资产", "总负债", "净资产", "环比", "同比"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for item in &data.asset_items {
        headers.push(item.item_name.clone());
    }
    for item in &data.liability_items {
        headers.push(item.item_name.clone());
    }
    for (col, header) in headers.iter().enumerate() {
        worksheet
            .write_string(0, col as u16, header)
            .map_err(map_xlsx_error)?;
    }

    // 明细按 yyyyMM 分组，组内以 templateItemId 索引
    let item_map: HashMap<String, HashMap<String, Decimal>> =
        data.month_items
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                let ym = format!("{:04}{:02}", item.year, item.month);
                let values = acc.entry(ym).or_default();
                values
                    .entry(item.template_item_id.clone())
                    .or_insert(item.item_value);
                acc
            });

    // 数据行
    for (row_idx, record) in data.month_records.iter().enumerate() {
        let row = row_idx as u32 + 1;
        worksheet
            .write_string(row, 0, format!("{:04}-{:02}", record.year, record.month))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 1, decimal_to_f64(record.total_asset))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 2, decimal_to_f64(record.total_liability))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 3, decimal_to_f64(record.net_asset))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_string(row, 4, format_percent(record.month_on_month))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_string(row, 5, format_percent(record.year_on_year))
            .map_err(map_xlsx_error)?;

        let ym = format!("{:04}{:02}", record.year, record.month);
        let values = item_map.get(&ym);
        let mut col: u16 = 6;
        for item in &data.asset_items {
            let value = values
                .and_then(|m| m.get(&item.id))
                .copied()
                .unwrap_or(Decimal::ZERO);
            worksheet
                .write_number(row, col, decimal_to_f64(value))
                .map_err(map_xlsx_error)?;
            col += 1;
        }
        for item in &data.liability_items {
            let value = values
                .and_then(|m| m.get(&item.id))
                .copied()
                .unwrap_or(Decimal::ZERO);
            worksheet
                .write_number(row, col, decimal_to_f64(value))
                .map_err(map_xlsx_error)?;
            col += 1;
        }
    }
    Ok(())
}

// ==================== 辅助函数 ====================

/// 构造导出完成邮件参数。
fn build_export_mail_params(user: UserModel, file_name: &str) -> MailParams {
    let mut params = MailParams::new();
    params.to = user.email.clone();
    params.user_email = user.email;
    params.user_name = user.nickname;
    params.file_name = file_name.to_string();
    params
}

/// 校验年月范围参数（仅在两端都给定时校验）。
fn validate_year_month_range(start: Option<&str>, end: Option<&str>) -> Result<(), AppError> {
    match (start, end) {
        (Some(s), Some(e)) if !is_year_month(s) || !is_year_month(e) || s > e => {
            Err(param_error("导出年月范围错误"))
        }
        (Some(s), None) | (None, Some(s)) if !is_year_month(s) => {
            Err(param_error("导出年月范围错误"))
        }
        _ => Ok(()),
    }
}

/// 解析 yyyyMM 字符串为 (年, 月)。
fn parse_year_month(v: &str) -> Result<(i32, i32), AppError> {
    if !is_year_month(v) {
        return Err(param_error("导出年月范围错误"));
    }
    Ok((v[0..4].parse().unwrap_or(0), v[4..6].parse().unwrap_or(0)))
}

/// 判断字符串是否为 yyyyMM。
fn is_year_month(v: &str) -> bool {
    if v.len() != 6 || !v.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let month: i32 = v[4..6].parse().unwrap_or(0);
    (1..=12).contains(&month)
}

/// 解析排序字段为整数；失败时排在最后（对齐 Java `parseSort` 返回 MAX_VALUE）。
fn parse_sort(sort: &str) -> i32 {
    sort.trim().parse::<i32>().unwrap_or(i32::MAX)
}

/// 格式化百分比：`12.5` → `"12.5%"`。
///
/// 对应 Java: `formatPercent`。Java 端入参为 `BigDecimal`，空值输出 `"-"`；
/// Rust 端该字段不可空（汇总总是写入），因此只处理非空分支。
fn format_percent(value: Decimal) -> String {
    format!("{}%", value)
}

/// 将 Decimal 转为 Excel 数字单元格需要的浮点值。
fn decimal_to_f64(value: Decimal) -> f64 {
    value.to_f64().unwrap_or(0.0)
}

/// 把失败原因截断到 DB 列允许的最大长度（对齐 Java `StrUtil.sub(..., 0, 500)`）。
fn truncate_fail_reason(s: &str) -> String {
    if s.len() <= FAIL_REASON_MAX_LEN {
        return s.to_string();
    }
    // 按字符截断，避免从多字节字符中间切断
    s.chars().take(FAIL_REASON_MAX_LEN).collect()
}

/// 构造参数错误（业务码沿用 Java `ResCode.PARAM_ERROR` + 自定义消息）。
fn param_error(message: &str) -> AppError {
    AppError::Business {
        code: ResCode::ParamError.code().to_string(),
        message: message.to_string(),
    }
}

/// 把账簿名转为合法且不重复的 Sheet 名。
///
/// Excel 限制 Sheet 名不超过 31 字符且不能包含 `: \ / ? * [ ]`，
/// 同一工作簿内还不允许重名（Java POI 重名会直接抛错，这里改为自动加序号）。
/// 注意：所有截断都按字符进行，避免从多字节字符（如中文）中间切断。
fn unique_sheet_name(name: &str, used: &mut HashMap<String, ()>) -> String {
    /// 按字符裁剪到指定长度
    fn truncate(chars: &[char], max: usize) -> String {
        chars.iter().take(max).collect()
    }

    let base_chars: Vec<char> = name
        .chars()
        .map(|c| match c {
            ':' | '\\' | '/' | '?' | '*' | '[' | ']' => '_',
            c => c,
        })
        .collect();
    let base = truncate(&base_chars, SHEET_NAME_MAX_LEN);
    if used.insert(base.clone(), ()).is_none() {
        return base;
    }
    // 重名时追加序号（序号也计入 31 字符上限）
    for seq in 2..1000u32 {
        let suffix = format!(" ({})", seq);
        let max_base = SHEET_NAME_MAX_LEN.saturating_sub(suffix.chars().count());
        let candidate = format!("{}{}", truncate(&base_chars, max_base), suffix);
        if used.insert(candidate.clone(), ()).is_none() {
            return candidate;
        }
    }
    base
}

/// 把 Excel 库错误映射为统一业务错误。
fn map_xlsx_error(e: XlsxError) -> AppError {
    AppError::Business {
        code: ResCode::Error.code().to_string(),
        message: format!("生成导出文件失败：{}", e),
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Reader;
    use rust_decimal::Decimal;
    use sea_orm::{ConnectionTrait, Database};

    #[test]
    fn format_percent_appends_percent_sign() {
        assert_eq!(format_percent(Decimal::from(12)), "12%");
        assert_eq!(format_percent(Decimal::new(125, 1)), "12.5%");
    }

    #[test]
    fn parse_sort_falls_back_to_max() {
        assert_eq!(parse_sort("10"), 10);
        assert_eq!(parse_sort("abc"), i32::MAX);
        assert_eq!(parse_sort(" 3 "), 3);
    }

    #[test]
    fn truncate_fail_reason_respects_limit() {
        let short = "oops";
        assert_eq!(truncate_fail_reason(short), "oops");
        let long = "x".repeat(600);
        let truncated = truncate_fail_reason(&long);
        assert_eq!(truncated.chars().count(), FAIL_REASON_MAX_LEN);
    }

    #[test]
    fn year_month_range_condition_open_ended() {
        // 两端皆空 → 不过滤
        let cond =
            year_month_range_condition(None, None, MonthRecordCol::Year, MonthRecordCol::Month)
                .unwrap();
        assert!(cond.is_none());

        // 只给起始
        let cond = year_month_range_condition(
            Some("202501"),
            None,
            MonthRecordCol::Year,
            MonthRecordCol::Month,
        )
        .unwrap();
        assert!(cond.is_some());

        // 非法格式
        assert!(
            year_month_range_condition(
                Some("2025"),
                None,
                MonthRecordCol::Year,
                MonthRecordCol::Month,
            )
            .is_err()
        );
    }

    #[test]
    fn validate_range_rejects_inverted() {
        assert!(validate_year_month_range(Some("202501"), Some("202401")).is_err());
        assert!(validate_year_month_range(Some("202501"), Some("202501")).is_ok());
        assert!(validate_year_month_range(None, None).is_ok());
    }

    #[test]
    fn unique_sheet_name_sanizes_and_dedupes() {
        let mut used = HashMap::new();
        assert_eq!(unique_sheet_name("日常账本", &mut used), "日常账本");
        // 重名追加序号
        assert_eq!(unique_sheet_name("日常账本", &mut used), "日常账本 (2)");
        // 非法字符替换
        assert_eq!(unique_sheet_name("a/b:c", &mut used), "a_b_c");
    }

    #[test]
    fn unique_sheet_name_truncates_to_limit() {
        let mut used = HashMap::new();
        let long = "账".repeat(40);
        let name = unique_sheet_name(&long, &mut used);
        assert!(name.chars().count() <= SHEET_NAME_MAX_LEN);
    }

    #[test]
    fn unique_sheet_name_dedupes_multibyte_names() {
        // 中文账簿重名时追加序号，不能因多字节字符截断而 panic
        let mut used = HashMap::new();
        let long = "账本".repeat(20);
        let first = unique_sheet_name(&long, &mut used);
        let second = unique_sheet_name(&long, &mut used);
        assert!(first.chars().count() <= SHEET_NAME_MAX_LEN);
        assert!(second.chars().count() <= SHEET_NAME_MAX_LEN);
        assert_ne!(first, second);
        assert!(second.ends_with("(2)"));
    }

    /// 验收：多账簿导出为一个多 Sheet Excel，表头/数据行/百分比/缺省 0 正确。
    #[tokio::test]
    async fn write_excel_produces_multi_sheet_layout() {
        let db = Database::connect("sqlite::memory:?cache=shared")
            .await
            .expect("连接内存库失败");
        // 建表（每条语句单独执行，sqlx 的 execute 不支持批量多语句）
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
                MR_ID TEXT PRIMARY KEY, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER, MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }

        let now = Local::now().naive_local();
        // 两个账簿
        let book1 = BookModel {
            id: "b1".into(),
            user_id: "u1".into(),
            book_name: "日常账本".into(),
            create_by: None,
            create_time: now,
            update_by: None,
            update_time: None,
            is_deleted: 0,
        };
        let book2 = BookModel {
            id: "b2".into(),
            book_name: "年度账本".into(),
            ..book1.clone()
        };

        let task = ExportTaskModel {
            id: "t1".into(),
            user_id: "u1".into(),
            book_id: None,
            start_year_month: None,
            end_year_month: None,
            status: "WAIT".into(),
            file_name: None,
            fail_reason: None,
            create_by: None,
            create_time: now,
            update_by: None,
            update_time: None,
            is_deleted: 0,
        };

        // 模板项：b1 含资产与负债各一项，b2 只含一项资产
        for (book_id, item_type, item_id) in [
            ("b1", 1, "tpl-asset"),
            ("b1", -1, "tpl-liab"),
            ("b2", 1, "tpl-b2"),
        ] {
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_TEMPLATE_ITEM (MR_ID, MR_BOOK_ID, MR_ITEM_NAME, MR_ITEM_TYPE, MR_ICON, MR_SORT)
                 VALUES ('{}', '{}', '{}', {}, '', '1')",
                item_id, book_id, item_id, item_type
            ))
            .await
            .unwrap();
        }

        // b1 的 2026-01 汇总与明细：明细只填了资产项，负债项缺失（应为 0）
        db.execute_unprepared(
            "INSERT INTO FIN_MONTH_ITEM_RECORD (MR_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE)
             VALUES ('mi1', 'b1', 2026, 1, 'tpl-asset', 100)",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "INSERT INTO FIN_MONTH_RECORD (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR)
             VALUES ('mr1', 'u1', 'b1', 2026, 1, 100, 40, 60, 5, 12)",
        )
        .await
        .unwrap();

        // 取数（必须在种子数据写入之后）
        let sheets = vec![
            fetch_book_sheet_data(&db, &book1, &task).await.unwrap(),
            fetch_book_sheet_data(&db, &book2, &task).await.unwrap(),
        ];

        let dir = std::env::temp_dir().join("mrecord").join("export-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("账簿导出_20260421120000.xlsx");
        write_excel(path.clone(), &sheets).expect("写入 Excel 失败");

        // 读回校验
        let mut workbook =
            calamine::open_workbook::<calamine::Xlsx<_>, _>(&path).expect("读取 Excel 失败");
        let names = workbook.sheet_names();
        assert_eq!(names, vec!["日常账本", "年度账本"]);

        let sheet = workbook
            .worksheet_range("日常账本")
            .expect("读取 Sheet 失败");
        let to_row = |rows: &calamine::Range<calamine::Data>, idx: usize| -> Vec<String> {
            rows.rows()
                .nth(idx)
                .unwrap()
                .iter()
                .map(|c| c.to_string())
                .collect()
        };
        // 表头：统计月份 / 总资产 / 总负债 / 净资产 / 环比 / 同比 / 资产项 / 负债项
        assert_eq!(
            to_row(&sheet, 0),
            vec![
                "统计月份",
                "总资产",
                "总负债",
                "净资产",
                "环比",
                "同比",
                "tpl-asset",
                "tpl-liab"
            ]
        );
        let data_row = to_row(&sheet, 1);
        assert_eq!(data_row[0], "2026-01");
        assert_eq!(data_row[1], "100");
        assert_eq!(data_row[3], "60");
        assert_eq!(data_row[4], "5%");
        assert_eq!(data_row[5], "12%");
        assert_eq!(data_row[6], "100"); // tpl-asset
        assert_eq!(data_row[7], "0"); // tpl-liab 缺省为 0

        // b2 没有任何月度数据，只有表头行
        let sheet2 = workbook
            .worksheet_range("年度账本")
            .expect("读取 Sheet 失败");
        assert_eq!(to_row(&sheet2, 0).len(), 7); // 6 个固定列 + 1 个资产项
    }
}
