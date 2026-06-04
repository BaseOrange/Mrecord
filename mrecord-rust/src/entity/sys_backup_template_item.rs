//! 记账模板明细备份实体
//!
//! 存储模板项的备份数据，结构同 FIN_TEMPLATE_ITEM。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.backup.SysBackupTemplateItem`
//! 数据库表: `SYS_BACKUP_TEMPLATE_ITEM`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "SYS_BACKUP_TEMPLATE_ITEM")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, column_name = "MR_ID")]
    pub id: String,
    /// 所属账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "MR_BOOK_ID")]
    pub book_id: String,
    /// 记账项名称，如"招行储蓄卡"、"花呗"
    #[sea_orm(column_name = "MR_ITEM_NAME")]
    pub item_name: String,
    /// 账簿类型（-1:负债，0:不统计仅记录，1:资产）
    #[sea_orm(column_name = "MR_ITEM_TYPE")]
    pub item_type: i32,
    /// 图标标识，对应系统内置图标库
    #[sea_orm(column_name = "MR_ICON")]
    pub icon: String,
    /// 展示排序号，数值越小越靠前
    #[sea_orm(column_name = "MR_SORT")]
    pub sort: String,
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
