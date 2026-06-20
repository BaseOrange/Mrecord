//! 月度财务明细项实体
//!
//! 存储每月各记账项的具体金额，与月度汇总表一一对应。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.FinMonthItemRecord`
//! 数据库表: `FIN_MONTH_ITEM_RECORD`

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "fin_month_item_record")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 统计年份
    #[sea_orm(column_name = "mr_year")]
    pub year: i32,
    /// 统计月份
    #[sea_orm(column_name = "mr_month")]
    pub month: i32,
    /// 关联账簿ID，FIN_BOOK.MR_ID
    #[sea_orm(column_name = "mr_book_id")]
    pub book_id: String,
    /// 关联模板项ID，FIN_TEMPLATE_ITEM.MR_ID
    #[sea_orm(column_name = "mr_template_item_id")]
    pub template_item_id: String,
    /// 当月该记账项实际金额
    #[sea_orm(column_name = "mr_item_value")]
    pub item_value: Decimal,
    /// 创建人
    #[sea_orm(column_name = "mr_create_by")]
    pub create_by: Option<String>,
    /// 创建时间
    #[sea_orm(column_name = "mr_create_time")]
    pub create_time: NaiveDateTime,
    /// 更新人
    #[sea_orm(column_name = "mr_update_by")]
    pub update_by: Option<String>,
    /// 更新时间
    #[sea_orm(column_name = "mr_update_time")]
    pub update_time: Option<NaiveDateTime>,
    /// 逻辑删除标识（0-正常，1-已删除）
    #[sea_orm(column_name = "mr_is_deleted")]
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
