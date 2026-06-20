//! 用户操作审计日志实体
//!
//! 记录用户所有关键操作，用于操作追溯、安全审计，不可修改删除。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.SysUserOperateLog`
//! 数据库表: `SYS_USER_OPERATE_LOG`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sys_user_operate_log")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 操作用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "mr_user_id")]
    pub user_id: String,
    /// 操作类型（LOGIN-登录，LOGOUT-登出，UPDATE-数据修改，EXPORT-导出，CANCEL-注销/撤销注销，RESET_PWD-密码重置）
    #[sea_orm(column_name = "mr_operate_type")]
    pub operate_type: String,
    /// 操作内容详细描述
    #[sea_orm(column_name = "mr_content")]
    pub content: String,
    /// 操作IP地址
    #[sea_orm(column_name = "mr_ip")]
    pub ip: String,
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
