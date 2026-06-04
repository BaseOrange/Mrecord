//! 月度财务汇总备份实体
//!
//! 存储每月汇总数据的备份，结构同 FIN_MONTH_RECORD。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.backup.SysBackupMonthRecord`
//! 数据库表: `SYS_BACKUP_MONTH_RECORD`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "SYS_BACKUP_MONTH_RECORD")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, column_name = "MR_ID")]
    pub id: String,
    /// 所属用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "MR_USER_ID")]
    pub user_id: String,
    /// 所属账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "MR_BOOK_ID")]
    pub book_id: String,
    /// 统计年份
    #[sea_orm(column_name = "MR_YEAR")]
    pub year: i32,
    /// 统计月份
    #[sea_orm(column_name = "MR_MONTH")]
    pub month: i32,
    /// 当月总资产，所有资产项之和
    #[sea_orm(column_name = "MR_TOTAL_ASSET")]
    pub total_asset: f64,
    /// 当月总负债，所有负债项之和
    #[sea_orm(column_name = "MR_TOTAL_LIABILITY")]
    pub total_liability: f64,
    /// 当月净资产，总资产 - 总负债
    #[sea_orm(column_name = "MR_NET_ASSET")]
    pub net_asset: f64,
    /// 环比增长/下跌金额，对比上月
    #[sea_orm(column_name = "MR_MONTH_ON_MONTH")]
    pub month_on_month: f64,
    /// 同比增长/下跌金额，对比去年同月
    #[sea_orm(column_name = "MR_YEAR_ON_YEAR")]
    pub year_on_year: f64,
    /// 用户本月汇总备注
    #[sea_orm(column_name = "MR_NOTE")]
    pub note: Option<String>,
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
