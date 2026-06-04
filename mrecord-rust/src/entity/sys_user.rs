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
#[sea_orm(table_name = "SYS_USER")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, column_name = "MR_ID")]
    pub id: String,
    /// 邮箱
    #[sea_orm(column_name = "MR_EMAIL")]
    pub email: String,
    /// 密码
    #[sea_orm(column_name = "MR_PASSWORD")]
    pub password: String,
    /// 昵称
    #[sea_orm(column_name = "MR_NICKNAME")]
    pub nickname: String,
    /// 是否管理员（0-正常用户，1-管理员）
    #[sea_orm(column_name = "MR_ADMIN")]
    pub admin: i32,
    /// 状态（0-正常，1-停用，2-注销待生效，3-已注销）
    #[sea_orm(column_name = "MR_STATUS")]
    pub status: i32,
    /// 账号注销申请时间，用于计算15天冷静期
    #[sea_orm(column_name = "MR_CANCEL_TIME")]
    pub cancel_time: Option<NaiveDateTime>,
    /// 邮件提醒功能是否启用（0-关闭，1-开启）
    #[sea_orm(column_name = "MR_REMIND_ENABLED")]
    pub remind_enabled: i32,
    /// 月度提醒日期（取值1-31，无对应日期取月末）
    #[sea_orm(column_name = "MR_REMIND_DAY")]
    pub remind_day: Option<i32>,
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
