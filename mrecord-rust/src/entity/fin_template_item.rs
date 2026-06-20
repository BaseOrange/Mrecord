//! 记账模板明细实体
//!
//! 存储每个账簿的自定义资产/负债记账项，支持图标、排序设置。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.FinTemplateItem`
//! 数据库表: `FIN_TEMPLATE_ITEM`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "fin_template_item")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 所属账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "mr_book_id")]
    pub book_id: String,
    /// 记账项名称，如"招行储蓄卡"、"花呗"
    #[sea_orm(column_name = "mr_item_name")]
    pub item_name: String,
    /// 账簿类型（-1:负债，0:不统计仅记录，1:资产）
    #[sea_orm(column_name = "mr_item_type")]
    pub item_type: i32,
    /// 图标标识，对应系统内置图标库
    #[sea_orm(column_name = "mr_icon")]
    pub icon: String,
    /// 展示排序号，数值越小越靠前
    #[sea_orm(column_name = "mr_sort")]
    pub sort: String,
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
