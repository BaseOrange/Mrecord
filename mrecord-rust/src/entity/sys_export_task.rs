//! 异步导出任务实体
//!
//! 存储用户 Excel 导出异步任务，实现非阻塞导出，任务完成后邮件通知用户。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.SysExportTask`
//! 数据库表: `SYS_EXPORT_TASK`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sys_export_task")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 操作用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "mr_user_id")]
    pub user_id: String,
    /// 导出账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "mr_book_id")]
    pub book_id: String,
    /// 导出开始年月，格式 yyyyMM
    #[sea_orm(column_name = "mr_start_year_month")]
    pub start_year_month: String,
    /// 导出结束年月，格式 yyyyMM
    #[sea_orm(column_name = "mr_end_year_month")]
    pub end_year_month: String,
    /// 任务状态（WAIT-待执行，RUN-执行中，SUCCESS-成功，FAIL-失败）
    #[sea_orm(column_name = "mr_status")]
    pub status: String,
    /// 生成的 Excel 文件名
    #[sea_orm(column_name = "mr_file_name")]
    pub file_name: Option<String>,
    /// 任务失败原因，失败时填充
    #[sea_orm(column_name = "mr_fail_reason")]
    pub fail_reason: Option<String>,
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
