//! 配置项实体
//!
//! 存储系统全局配置参数，如邮件SMTP配置、注册开关等，仅管理员可修改。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.SysConfig`
//! 数据库表: `SYS_CONFIG`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "mr_id")]
    pub id: String,
    /// 配置项 key（代码读取唯一标识）
    #[sea_orm(column_name = "mr_config_key")]
    pub key: String,
    /// 配置项 value
    #[sea_orm(column_name = "mr_config_value")]
    pub value: String,
    /// 配置项描述
    #[sea_orm(column_name = "mr_remark")]
    pub remark: Option<String>,
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
