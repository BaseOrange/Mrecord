//! 导出任务服务
//!
//! 对应 Java: `com.dcz.mrecord.service.ExportTaskService`
//! 实现:    `com.dcz.mrecord.service.impl.ExportTaskServiceImpl`

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::Context;
use chrono::Utc;
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
    constant::export_task::ExportTaskStatus,
    entity::{
        fin_book::{Column as BookCol, Entity as BookEntity, Model as BookModel},
        fin_month_item_record::{
            Column as MonthItemCol, Entity as MonthItemEntity, Model as MonthItemModel,
        },
        fin_month_record::{
            Column as MonthRecordCol, Entity as MonthRecordEntity, Model as MonthRecordModel,
        },
        fin_template_item::{Column as TemplateItemCol, Entity as TemplateItemEntity},
        sys_export_task::{ActiveModel as ExportTaskActive, Model as ExportTaskModel},
        sys_user::{Column as UserCol, Entity as UserEntity, Model as UserModel},
    },
    error::AppError,
    model::{finance::ExportBookDto, mail_params::MailParams},
    service::email::EmailService,
};

/// 导出任务服务，持有邮件服务用于任务完成通知。
pub struct ExportTaskService {
    email_service: Arc<EmailService>,
}

impl ExportTaskService {
    /// 创建导出任务服务实例。
    pub fn new(email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { email_service })
    }

    /// 创建账簿导出任务并异步处理。
    ///
    /// 对应 Java: `ExportTaskServiceImpl.export(ExportBookDTO, SysUser)`。
    pub async fn create_export_tasks(
        self: Arc<Self>,
        db: DatabaseConnection,
        user_id: String,
        params: ExportBookDto,
    ) -> Result<usize, AppError> {
        let books = self
            .resolve_books(&db, &user_id, params.book_id.as_deref())
            .await?;
        if books.is_empty() {
            return Err(AppError::ResCode(ResCode::FinBookNotFound));
        }

        let start_year_month = params
            .start_year_month
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "000101".to_string());
        let end_year_month = params
            .end_year_month
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "999912".to_string());

        validate_year_month_range(&start_year_month, &end_year_month)?;

        for book in books {
            let task = ExportTaskActive {
                id: Set(Uuid::new_v4().simple().to_string()),
                user_id: Set(user_id.clone()),
                book_id: Set(book.id),
                start_year_month: Set(start_year_month.clone()),
                end_year_month: Set(end_year_month.clone()),
                status: Set(ExportTaskStatus::Wait.as_str().to_string()),
                create_by: Set(Some(user_id.clone())),
                ..Default::default()
            }
            .insert(&db)
            .await?;

            let service = self.clone();
            let db_for_task = db.clone();
            task::spawn(async move {
                if let Err(e) = service.process_task(db_for_task, task).await {
                    tracing::error!("导出任务后台处理失败: {:?}", e);
                }
            });
        }

        Ok(1)
    }

    /// 根据请求解析要导出的账簿集合。
    ///
    /// 对应 Java 中按当前用户过滤账簿，避免越权导出。
    async fn resolve_books(
        &self,
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

    /// 处理单个导出任务。
    ///
    /// 对应 Java 异步任务：更新 RUN、生成附件、成功后发邮件，异常时记录 FAIL。
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
                let reason = format!("{:?}", e);
                self.update_status(&db, &task, ExportTaskStatus::Fail, None, Some(reason))
                    .await?;
                Err(e)
            }
        }
    }

    /// 生成导出 Excel 文件。
    ///
    /// 对应 Java 中账簿导出 Excel 的核心业务。
    async fn execute_export(
        &self,
        db: &DatabaseConnection,
        task: &ExportTaskModel,
    ) -> Result<PathBuf, AppError> {
        let book = BookEntity::find_by_id(task.book_id.clone())
            .filter(BookCol::UserId.eq(task.user_id.clone()))
            .filter(BookCol::IsDeleted.eq(0))
            .one(db)
            .await?
            .ok_or(AppError::ResCode(ResCode::FinBookNotFound))?;
        let month_records = query_month_records(db, task).await?;
        let month_items = query_month_items(db, task).await?;
        let template_names = query_template_names(db, &task.book_id).await?;
        let file_name = format!(
            "mrecord-{}-{}-{}.xlsx",
            book.book_name, task.start_year_month, task.end_year_month
        );
        let file_path = PathBuf::from("exports").join(sanitize_file_name(&file_name));
        let path_for_blocking = file_path.clone();

        task::spawn_blocking(move || {
            write_excel(
                path_for_blocking.clone(),
                book,
                month_records,
                month_items,
                template_names,
            )
        })
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))??;

        Ok(file_path)
    }

    /// 更新导出任务状态。
    ///
    /// 对应 Java `SysExportTask` 状态流转字段更新。
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
        active.update_time = Set(Some(Utc::now().naive_utc()));
        active.update(db).await?;
        Ok(())
    }

    /// 发送导出完成邮件。
    ///
    /// 对应 Java: `EmailServiceImpl.sendExportBookEmail`。
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

/// 查询导出范围内的月度汇总。
async fn query_month_records(
    db: &DatabaseConnection,
    task: &ExportTaskModel,
) -> Result<Vec<MonthRecordModel>, AppError> {
    let (start_year, start_month) = parse_year_month(&task.start_year_month)?;
    let (end_year, end_month) = parse_year_month(&task.end_year_month)?;
    Ok(MonthRecordEntity::find()
        .filter(MonthRecordCol::UserId.eq(task.user_id.clone()))
        .filter(MonthRecordCol::BookId.eq(task.book_id.clone()))
        .filter(MonthRecordCol::IsDeleted.eq(0))
        .filter(month_record_range(
            start_year,
            start_month,
            end_year,
            end_month,
        ))
        .order_by_asc(MonthRecordCol::Year)
        .order_by_asc(MonthRecordCol::Month)
        .all(db)
        .await?)
}

/// 查询导出范围内的月度明细。
async fn query_month_items(
    db: &DatabaseConnection,
    task: &ExportTaskModel,
) -> Result<Vec<MonthItemModel>, AppError> {
    let (start_year, start_month) = parse_year_month(&task.start_year_month)?;
    let (end_year, end_month) = parse_year_month(&task.end_year_month)?;
    Ok(MonthItemEntity::find()
        .filter(MonthItemCol::BookId.eq(task.book_id.clone()))
        .filter(MonthItemCol::IsDeleted.eq(0))
        .filter(month_item_range(
            start_year,
            start_month,
            end_year,
            end_month,
        ))
        .order_by_asc(MonthItemCol::Year)
        .order_by_asc(MonthItemCol::Month)
        .all(db)
        .await?)
}

/// 构造月度汇总表的年月范围过滤条件。
fn month_record_range(
    start_year: i32,
    start_month: i32,
    end_year: i32,
    end_month: i32,
) -> Condition {
    Condition::all()
        .add(
            Condition::any()
                .add(MonthRecordCol::Year.gt(start_year))
                .add(
                    Condition::all()
                        .add(MonthRecordCol::Year.eq(start_year))
                        .add(MonthRecordCol::Month.gte(start_month)),
                ),
        )
        .add(
            Condition::any().add(MonthRecordCol::Year.lt(end_year)).add(
                Condition::all()
                    .add(MonthRecordCol::Year.eq(end_year))
                    .add(MonthRecordCol::Month.lte(end_month)),
            ),
        )
}

/// 构造月度明细表的年月范围过滤条件。
fn month_item_range(start_year: i32, start_month: i32, end_year: i32, end_month: i32) -> Condition {
    Condition::all()
        .add(
            Condition::any().add(MonthItemCol::Year.gt(start_year)).add(
                Condition::all()
                    .add(MonthItemCol::Year.eq(start_year))
                    .add(MonthItemCol::Month.gte(start_month)),
            ),
        )
        .add(
            Condition::any().add(MonthItemCol::Year.lt(end_year)).add(
                Condition::all()
                    .add(MonthItemCol::Year.eq(end_year))
                    .add(MonthItemCol::Month.lte(end_month)),
            ),
        )
}

/// 查询模板项名称，用于把明细里的模板项 ID 翻译为用户可读名称。
async fn query_template_names(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<HashMap<String, String>, AppError> {
    let templates = TemplateItemEntity::find()
        .filter(TemplateItemCol::BookId.eq(book_id.to_string()))
        .filter(TemplateItemCol::IsDeleted.eq(0))
        .all(db)
        .await?;
    Ok(templates.into_iter().map(|m| (m.id, m.item_name)).collect())
}

/// 写入 Excel 文件。
fn write_excel(
    file_path: PathBuf,
    book: BookModel,
    month_records: Vec<MonthRecordModel>,
    month_items: Vec<MonthItemModel>,
    template_names: HashMap<String, String>,
) -> Result<(), AppError> {
    std::fs::create_dir_all(
        file_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new(".")),
    )
    .context("创建导出目录失败")?;

    let mut workbook = Workbook::new();
    write_summary_sheet(&mut workbook, &book, &month_records)?;
    write_detail_sheet(&mut workbook, &book, &month_items, &template_names)?;
    workbook.save(&file_path).map_err(map_xlsx_error)?;
    Ok(())
}

/// 写入账簿汇总 Sheet。
fn write_summary_sheet(
    workbook: &mut Workbook,
    book: &BookModel,
    records: &[MonthRecordModel],
) -> Result<(), AppError> {
    let worksheet = workbook
        .add_worksheet()
        .set_name("账簿汇总")
        .map_err(map_xlsx_error)?;
    let headers = [
        "账簿名称",
        "年份",
        "月份",
        "总资产",
        "总负债",
        "净资产",
        "环比",
        "同比",
        "备注",
    ];
    for (idx, header) in headers.iter().enumerate() {
        worksheet
            .write_string(0, idx as u16, *header)
            .map_err(map_xlsx_error)?;
    }
    for (row_idx, record) in records.iter().enumerate() {
        let row = row_idx as u32 + 1;
        worksheet
            .write_string(row, 0, &book.book_name)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 1, record.year)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 2, record.month)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 3, decimal_to_f64(record.total_asset))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 4, decimal_to_f64(record.total_liability))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 5, decimal_to_f64(record.net_asset))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 6, decimal_to_f64(record.month_on_month))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 7, decimal_to_f64(record.year_on_year))
            .map_err(map_xlsx_error)?;
        worksheet
            .write_string(row, 8, record.note.as_deref().unwrap_or(""))
            .map_err(map_xlsx_error)?;
    }
    Ok(())
}

/// 写入月度明细 Sheet。
fn write_detail_sheet(
    workbook: &mut Workbook,
    book: &BookModel,
    items: &[MonthItemModel],
    template_names: &HashMap<String, String>,
) -> Result<(), AppError> {
    let worksheet = workbook
        .add_worksheet()
        .set_name("月度明细")
        .map_err(map_xlsx_error)?;
    let headers = ["账簿名称", "年份", "月份", "记账项", "金额"];
    for (idx, header) in headers.iter().enumerate() {
        worksheet
            .write_string(0, idx as u16, *header)
            .map_err(map_xlsx_error)?;
    }
    for (row_idx, item) in items.iter().enumerate() {
        let row = row_idx as u32 + 1;
        let item_name = template_names
            .get(&item.template_item_id)
            .map(String::as_str)
            .unwrap_or(&item.template_item_id);
        worksheet
            .write_string(row, 0, &book.book_name)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 1, item.year)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 2, item.month)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_string(row, 3, item_name)
            .map_err(map_xlsx_error)?;
        worksheet
            .write_number(row, 4, decimal_to_f64(item.item_value))
            .map_err(map_xlsx_error)?;
    }
    Ok(())
}

/// 将两位小数 Decimal 转为 Excel 数字单元格需要的浮点值。
fn decimal_to_f64(value: Decimal) -> f64 {
    value.to_f64().unwrap_or(0.0)
}

/// 构造导出完成邮件参数。
fn build_export_mail_params(user: UserModel, file_name: &str) -> MailParams {
    let mut params = MailParams::new();
    params.to = user.email.clone();
    params.user_email = user.email;
    params.user_name = user.nickname;
    params.file_name = file_name.to_string();
    params
}

/// 校验年月范围参数。
fn validate_year_month_range(start: &str, end: &str) -> Result<(), AppError> {
    if !is_year_month(start) || !is_year_month(end) || start > end {
        return Err(AppError::Business {
            code: ResCode::ParamError.code().to_string(),
            message: "导出年月范围错误".to_string(),
        });
    }
    Ok(())
}

/// 解析 yyyyMM 字符串。
fn parse_year_month(v: &str) -> Result<(i32, i32), AppError> {
    if !is_year_month(v) {
        return Err(AppError::Business {
            code: ResCode::ParamError.code().to_string(),
            message: "导出年月范围错误".to_string(),
        });
    }
    Ok((v[0..4].parse().unwrap_or(0), v[4..6].parse().unwrap_or(0)))
}

/// 判断字符串是否为 yyyyMM，允许内部默认哨兵年月。
fn is_year_month(v: &str) -> bool {
    if v.len() != 6 || !v.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let month: i32 = v[4..6].parse().unwrap_or(0);
    (1..=12).contains(&month)
}

/// 避免账簿名中的路径分隔符影响导出文件路径。
fn sanitize_file_name(v: &str) -> String {
    v.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

/// 把 Excel 库错误映射为统一业务错误。
fn map_xlsx_error(e: XlsxError) -> AppError {
    AppError::Business {
        code: ResCode::Error.code().to_string(),
        message: format!("生成导出文件失败：{}", e),
    }
}
