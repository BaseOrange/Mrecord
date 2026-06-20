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
#[sea_orm(table_name = "FIN_MONTH_ITEM_RECORD")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, column_name = "MR_ID")]
    pub id: String,
    /// 统计年份
    #[sea_orm(column_name = "MR_YEAR")]
    pub year: i32,
    /// 统计月份
    #[sea_orm(column_name = "MR_MONTH")]
    pub month: i32,
    /// 关联账簿ID，FIN_BOOK.MR_ID
    #[sea_orm(column_name = "MR_BOOK_ID")]
    pub book_id: String,
    /// 关联模板项ID，FIN_TEMPLATE_ITEM.MR_ID
    #[sea_orm(column_name = "MR_TEMPLATE_ITEM_ID")]
    pub template_item_id: String,
    /// 当月该记账项实际金额
    #[sea_orm(column_name = "MR_ITEM_VALUE")]
    pub item_value: Decimal,
    /// 创建人
    #[sea_orm(column_name = "MR_CREATE_BY")]
    pub create_by: Option<String>,
    /// 创建时间
    #[sea_orm(column_name = "MR_CREATE_TIME")]
    pub create_time: NaiveDateTime,
    /// 更新人
    #[sea_orm(column_name = "MR_UPDATE_BY")]
    pub update_by: Option<String>,
    /// 更新时间
    #[sea_orm(column_name = "MR_UPDATE_TIME")]
    pub update_time: Option<NaiveDateTime>,
    /// 逻辑删除标识（0-正常，1-已删除）
    #[sea_orm(column_name = "MR_IS_DELETED")]
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
