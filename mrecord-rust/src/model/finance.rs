//! 财务记账相关请求/响应 DTO
//!
//! 对应 Java DTO:
//! - `com.dcz.mrecord.dto.MonthRecordDTO`
//! - `com.dcz.mrecord.dto.MonthItemDTO`
//! - `com.dcz.mrecord.dto.FinBookRecordDTO`
//! - `com.dcz.mrecord.dto.FinTempItemDTO`
//! - `com.dcz.mrecord.dto.QueryFinBookDTO`
//! - `com.dcz.mrecord.dto.ExportBookDTO`
//! - `com.dcz.mrecord.dto.DataStatisticsDTO`

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::page_info::PageInfo;

// ==================== 请求 DTO ====================

/// 月度汇总查询 / 保存请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthRecordDto {
    /// 账簿 ID
    pub book_id: String,
    /// 年份
    pub year: Option<i32>,
    /// 月份
    pub month: Option<i32>,
    /// 备注
    pub note: Option<String>,
}

/// 月度明细保存请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthItemDto {
    /// 账簿 ID
    pub book_id: String,
    /// 年份
    pub year: Option<i32>,
    /// 月份
    pub month: Option<i32>,
    /// 明细列表
    pub item_list: Option<Vec<MonthItemEntry>>,
    /// 备注
    pub note: Option<String>,
}

/// 月度明细条目（用于 MonthItemDto 内嵌列表）
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthItemEntry {
    /// 主键 ID（更新时传）
    pub id: Option<String>,
    /// 关联模板项 ID，FIN_TEMPLATE_ITEM.MR_ID
    pub template_item_id: String,
    /// 当月该记账项实际金额
    #[serde(with = "rust_decimal::serde::str")]
    pub item_value: Decimal,
}

/// 创建 / 复制账本模板项请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinTempItemDto {
    /// 账本 ID
    pub book_id: String,
    /// 旧账簿 ID（复制场景）
    pub old_book_id: Option<String>,
    /// 账本模板项列表
    pub item_list: Option<Vec<TemplateItemEntry>>,
}

/// 模板项条目（用于 FinTempItemDto 内嵌列表）
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateItemEntry {
    /// 主键 ID（更新时传）
    pub id: Option<String>,
    /// 记账项名称，如"招行储蓄卡"、"花呗"
    pub item_name: String,
    /// 账簿类型（-1:负债，0:不统计仅记录，1:资产）
    pub item_type: i32,
    /// 图标标识；兼容前端旧请求未传 icon 的模板项，等同 Java 端空字符串默认值。
    #[serde(default)]
    pub icon: String,
    /// 展示排序号
    pub sort: String,
}

/// 创建 / 更新账簿请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBookDto {
    /// 主键 ID（更新时传）
    pub id: Option<String>,
    /// 账簿名称
    pub book_name: String,
}

/// 查询账簿请求参数（继承分页参数）
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFinBookDto {
    /// 分页参数
    #[serde(flatten)]
    pub page: PageInfo,
    /// 账簿名称（模糊匹配）
    pub name: Option<String>,
    /// 账簿类型
    #[serde(rename = "type")]
    pub book_type: Option<String>,
    /// 账簿年份
    pub year: Option<String>,
}

/// 账簿导出请求
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportBookDto {
    /// 账簿 ID，不传则导出全部账簿
    pub book_id: Option<String>,
    /// 导出起始年月，格式 yyyyMM，不传则从最早数据开始
    pub start_year_month: Option<String>,
    /// 导出结束年月，格式 yyyyMM，不传则到最新数据结束
    pub end_year_month: Option<String>,
}

// ==================== 响应 DTO ====================

/// 账簿列表响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinBookResponse {
    pub id: String,
    pub book_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
}

impl From<crate::entity::fin_book::Model> for FinBookResponse {
    fn from(m: crate::entity::fin_book::Model) -> Self {
        Self {
            id: m.id,
            book_name: m.book_name,
            create_time: m.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: m
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 账簿最新月度统计响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinBookRecordResponse {
    pub book_id: String,
    pub book_name: String,
    pub id: String,
    pub year: i32,
    pub month: i32,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_asset: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_liability: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub net_asset: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub month_on_month: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub year_on_year: Decimal,
    pub note: Option<String>,
    pub create_time: String,
    pub update_time: Option<String>,
}

impl FinBookRecordResponse {
    pub fn new(
        book_id: String,
        book_name: String,
        record: crate::entity::fin_month_record::Model,
    ) -> Self {
        Self {
            book_id,
            book_name,
            id: record.id,
            year: record.year,
            month: record.month,
            total_asset: record.total_asset,
            total_liability: record.total_liability,
            net_asset: record.net_asset,
            month_on_month: record.month_on_month,
            year_on_year: record.year_on_year,
            note: record.note,
            create_time: record.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: record
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 模板项列表响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateItemResponse {
    pub id: String,
    pub book_id: String,
    pub item_name: String,
    pub item_type: i32,
    pub icon: String,
    pub sort: String,
}

impl From<crate::entity::fin_template_item::Model> for TemplateItemResponse {
    fn from(m: crate::entity::fin_template_item::Model) -> Self {
        Self {
            id: m.id,
            book_id: m.book_id,
            item_name: m.item_name,
            item_type: m.item_type,
            icon: m.icon,
            sort: m.sort,
        }
    }
}

/// 月度汇总响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthRecordResponse {
    pub id: String,
    pub book_id: String,
    pub year: i32,
    pub month: i32,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_asset: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_liability: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub net_asset: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub month_on_month: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub year_on_year: Decimal,
    pub note: Option<String>,
    pub create_time: String,
    pub update_time: Option<String>,
}

impl From<crate::entity::fin_month_record::Model> for MonthRecordResponse {
    fn from(m: crate::entity::fin_month_record::Model) -> Self {
        Self {
            id: m.id,
            book_id: m.book_id,
            year: m.year,
            month: m.month,
            total_asset: m.total_asset,
            total_liability: m.total_liability,
            net_asset: m.net_asset,
            month_on_month: m.month_on_month,
            year_on_year: m.year_on_year,
            note: m.note,
            create_time: m.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: m
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 月度明细项响应
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthItemRecordResponse {
    pub id: String,
    pub book_id: String,
    pub year: i32,
    pub month: i32,
    pub template_item_id: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub item_value: Decimal,
}

impl From<crate::entity::fin_month_item_record::Model> for MonthItemRecordResponse {
    fn from(m: crate::entity::fin_month_item_record::Model) -> Self {
        Self {
            id: m.id,
            book_id: m.book_id,
            year: m.year,
            month: m.month,
            template_item_id: m.template_item_id,
            item_value: m.item_value,
        }
    }
}

/// 数据统计响应（泛型，支持普通记录和备份记录）
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStatisticsResponse<T>
where
    T: Serialize,
{
    /// 开始年月，格式 yyyyMM
    pub start_year_month: String,
    /// 结束年月，格式 yyyyMM
    pub end_year_month: String,
    /// 各账户最新月份统计数据
    pub record_list: Vec<T>,
}
