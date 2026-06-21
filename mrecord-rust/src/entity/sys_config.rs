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
#[sea_orm(table_name = "SYS_CONFIG")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "MR_ID")]
    pub id: String,
    /// 配置项 key（代码读取唯一标识）
    #[sea_orm(column_name = "MR_CONFIG_KEY")]
    pub key: String,
    /// 配置项 value
    #[sea_orm(column_name = "MR_CONFIG_VALUE")]
    pub value: String,
    /// 配置项描述
    #[sea_orm(column_name = "MR_REMARK")]
    pub remark: Option<String>,
    /// 创建人
    #[sea_orm(column_name = "MR_CREATE_BY")]
    pub create_by: Option<String>,
    /// 创建时间；Java schema.sql 允许为空，初始化配置项不会写入该字段。
    #[sea_orm(column_name = "MR_CREATE_TIME")]
    pub create_time: Option<NaiveDateTime>,
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
