//! 月度财务汇总实体
//!
//! 存储每月财务数据汇总指标，自动计算总资产、总负债、净资产及同比环比。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.FinMonthRecord`
//! 数据库表: `FIN_MONTH_RECORD`

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "fin_month_record")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 所属用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "mr_user_id")]
    pub user_id: String,
    /// 所属账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "mr_book_id")]
    pub book_id: String,
    /// 统计年份
    #[sea_orm(column_name = "mr_year")]
    pub year: i32,
    /// 统计月份
    #[sea_orm(column_name = "mr_month")]
    pub month: i32,
    /// 当月总资产，所有资产项之和
    #[sea_orm(column_name = "mr_total_asset")]
    pub total_asset: Decimal,
    /// 当月总负债，所有负债项之和
    #[sea_orm(column_name = "mr_total_liability")]
    pub total_liability: Decimal,
    /// 当月净资产，总资产 - 总负债
    #[sea_orm(column_name = "mr_net_asset")]
    pub net_asset: Decimal,
    /// 环比增长/下跌金额，对比上月
    #[sea_orm(column_name = "mr_month_on_month")]
    pub month_on_month: Decimal,
    /// 同比增长/下跌金额，对比去年同月
    #[sea_orm(column_name = "mr_year_on_year")]
    pub year_on_year: Decimal,
    /// 用户本月汇总备注
    #[sea_orm(column_name = "mr_note")]
    pub note: Option<String>,
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
