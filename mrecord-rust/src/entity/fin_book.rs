//! 财务账簿实体
//!
//! 存储用户创建的年度/分类账簿，实现多账簿独立管理。
//!
//! 对应 Java 实体: `com.dcz.mrecord.entity.FinBook`
//! 数据库表: `FIN_BOOK`

use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "FIN_BOOK")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = false, column_name = "MR_ID")]
    pub id: String,
    /// 操作用户ID，关联 SYS_USER.MR_ID
    #[sea_orm(column_name = "MR_USER_ID")]
    pub user_id: String,
    /// 账簿名称，用户自定义
    #[sea_orm(column_name = "MR_BOOK_NAME")]
    pub book_name: String,
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
