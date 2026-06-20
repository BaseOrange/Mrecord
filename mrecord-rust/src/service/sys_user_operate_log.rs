//! 用户操作审计日志服务
//!
//! 对应 Java: `com.dcz.mrecord.service.impl.SysUserOperateLogServiceImpl`。

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Local;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    common::page::PageResult,
    entity::{
        sys_user::{Column as UserCol, Entity as UserEntity},
        sys_user_operate_log::{
            ActiveModel as OperateLogActive, Column as OperateLogCol, Entity as OperateLogEntity,
        },
    },
    error::AppError,
    model::sys_user_operate_log::OperateLogResponse,
};

const MAX_LOG_CONTENT_LEN: usize = 1000;

/// 用户操作审计日志服务。
///
/// 对应 Java: `SysUserOperateLogService`，负责保存请求日志和管理员分页查询日志。
pub struct SysUserOperateLogService;

impl SysUserOperateLogService {
    /// 创建操作日志服务实例。
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    /// 保存操作日志。
    ///
    /// 对应 Java: `SysUserOperateLogServiceImpl.saveLog`。
    pub async fn save_log(
        &self,
        db: &DatabaseConnection,
        user_id: Option<String>,
        operate_type: String,
        content: String,
        ip: String,
    ) -> Result<(), AppError> {
        let user_id = user_id.unwrap_or_default();
        let content = truncate_content(content);
        let now = Local::now().naive_local();
        OperateLogEntity::insert(OperateLogActive {
            id: Set(Uuid::new_v4().simple().to_string()),
            user_id: Set(user_id.clone()),
            operate_type: Set(operate_type),
            content: Set(content),
            ip: Set(ip),
            create_by: Set(if user_id.is_empty() {
                None
            } else {
                Some(user_id)
            }),
            create_time: Set(now),
            update_by: Set(None),
            update_time: Set(None),
            is_deleted: Set(0),
        })
        .exec(db)
        .await?;
        Ok(())
    }

    /// 分页查询操作日志。
    ///
    /// 对应 Java: `SysUserOperateLogServiceImpl.queryList`。
    pub async fn query_list(
        &self,
        db: &DatabaseConnection,
        page_num: u64,
        page_size: u64,
    ) -> Result<PageResult<OperateLogResponse>, AppError> {
        let q = OperateLogEntity::find()
            .filter(OperateLogCol::IsDeleted.eq(0))
            .order_by_desc(OperateLogCol::CreateTime);
        let paginator = q.paginate(db, page_size);
        let total = paginator.num_items().await?;
        let logs = paginator.fetch_page(page_num - 1).await?;
        let user_ids: Vec<String> = logs
            .iter()
            .flat_map(|log| [log.create_by.clone(), log.update_by.clone()])
            .flatten()
            .collect();
        let users = if user_ids.is_empty() {
            Vec::new()
        } else {
            UserEntity::find()
                .filter(UserCol::Id.is_in(user_ids))
                .all(db)
                .await?
        };
        let user_map: HashMap<String, String> = users
            .into_iter()
            .map(|user| (user.id, user.nickname))
            .collect();
        let records = logs
            .into_iter()
            .map(|log| {
                let create_by_name = log
                    .create_by
                    .as_ref()
                    .and_then(|id| user_map.get(id).cloned());
                let update_by_name = log
                    .update_by
                    .as_ref()
                    .and_then(|id| user_map.get(id).cloned());
                OperateLogResponse::from_log(log, create_by_name, update_by_name)
            })
            .collect();
        Ok(PageResult::new(records, total, page_num, page_size))
    }
}

/// 截断超长请求体，避免日志表存储过大的 JSON。
fn truncate_content(content: String) -> String {
    if content.chars().count() <= MAX_LOG_CONTENT_LEN {
        return content;
    }
    content.chars().take(MAX_LOG_CONTENT_LEN).collect()
}
