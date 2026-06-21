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
#[sea_orm(table_name = "SYS_EXPORT_TASK")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "MR_ID")]
    pub id: String,
    /// 操作用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "MR_USER_ID")]
    pub user_id: String,
    /// 导出账簿ID，关联 FIN_BOOK.MR_ID
    #[sea_orm(column_name = "MR_BOOK_ID")]
    pub book_id: String,
    /// 导出开始年月，格式 yyyyMM
    #[sea_orm(column_name = "MR_START_YEAR_MONTH")]
    pub start_year_month: String,
    /// 导出结束年月，格式 yyyyMM
    #[sea_orm(column_name = "MR_END_YEAR_MONTH")]
    pub end_year_month: String,
    /// 任务状态（WAIT-待执行，RUN-执行中，SUCCESS-成功，FAIL-失败）
    #[sea_orm(column_name = "MR_STATUS")]
    pub status: String,
    /// 生成的 Excel 文件名
    #[sea_orm(column_name = "MR_FILE_NAME")]
    pub file_name: Option<String>,
    /// 任务失败原因，失败时填充
    #[sea_orm(column_name = "MR_FAIL_REASON")]
    pub fail_reason: Option<String>,
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
