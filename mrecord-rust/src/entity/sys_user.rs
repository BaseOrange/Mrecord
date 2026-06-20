//! 用户实体
//!
//! 存储系统用户信息，包括登录凭证、角色、提醒设置等。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.SysUser`
//! 数据库表: `SYS_USER`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 邮箱
    #[sea_orm(column_name = "mr_email")]
    pub email: String,
    /// 密码
    #[sea_orm(column_name = "mr_password")]
    pub password: String,
    /// 昵称
    #[sea_orm(column_name = "mr_nickname")]
    pub nickname: String,
    /// 是否管理员（0-正常用户，1-管理员）
    #[sea_orm(column_name = "mr_admin")]
    pub admin: i32,
    /// 状态（0-正常，1-停用，2-注销待生效，3-已注销）
    #[sea_orm(column_name = "mr_status")]
    pub status: i32,
    /// 账号注销申请时间，用于计算15天冷静期
    #[sea_orm(column_name = "mr_cancel_time")]
    pub cancel_time: Option<NaiveDateTime>,
    /// 邮件提醒功能是否启用（0-关闭，1-开启）
    #[sea_orm(column_name = "mr_remind_enabled")]
    pub remind_enabled: i32,
    /// 月度提醒日期（取值1-31，无对应日期取月末）
    #[sea_orm(column_name = "mr_remind_day")]
    pub remind_day: Option<i32>,
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
