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
    ///
    /// `content` 存**全量**请求体，不做长度截断——对齐 Java
    /// （`LogInterceptor.preHandle` 直接存 `cachedRequest.getCachedBody()` 全文，
    /// `CachedBodyHttpServletRequest` 也不做任何截断）。
    ///
    /// SQLite 单行体积由中间件的 `MAX_BODY_BYTES`（1 MB）读取上限兜底：
    /// SQLite 的 `SQLITE_MAX_LENGTH` 默认上限为 1 GB，1 MB 远低于该值，
    /// 既避免日志表行体积失控，又不影响业务请求体（登录 / 注册 / 导出参数都很小）。
    pub async fn save_log(
        &self,
        db: &DatabaseConnection,
        user_id: Option<String>,
        operate_type: String,
        content: String,
        ip: String,
    ) -> Result<(), AppError> {
        let user_id = user_id.unwrap_or_default();
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

#[cfg(test)]
mod tests {
    //! 操作日志服务测试。
    //!
    //! 对应 REFACTOR_TODO 3.10：
    //! - `content` 存全量不截断（对齐 Java `LogInterceptor` 存 `getCachedBody()` 全文）；
    //! - 响应体带 `isDeleted`（对齐 Java `BaseEntity`）。

    use super::*;
    use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("连接内存库失败");
        for ddl in [
            "CREATE TABLE IF NOT EXISTS SYS_USER_OPERATE_LOG (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_OPERATE_TYPE TEXT,
                MR_CONTENT TEXT, MR_IP TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
             )",
            // query_list 会 LEFT JOIN SYS_USER 取昵称；字段对齐 entity::sys_user
            "CREATE TABLE IF NOT EXISTS SYS_USER (
                MR_ID TEXT PRIMARY KEY, MR_EMAIL TEXT, MR_PASSWORD TEXT, MR_NICKNAME TEXT,
                MR_ADMIN INTEGER DEFAULT 0, MR_STATUS INTEGER DEFAULT 1, MR_AVATAR TEXT,
                MR_REMIND_ENABLED INTEGER, MR_REMIND_DAY INTEGER, MR_CANCEL_TIME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
             )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db
    }

    /// 超过原 `MAX_LOG_CONTENT_LEN`(1000) 的请求体也能全文落库。
    #[tokio::test]
    async fn save_log_stores_full_content_without_truncation() {
        let db = setup_db().await;
        let svc = SysUserOperateLogService::new();

        // 5000 字符的请求体，远超原先的 1000 截断阈值
        let long_content = "x".repeat(5000);
        svc.save_log(
            &db,
            Some("u1".into()),
            "/book/create".into(),
            long_content.clone(),
            "127.0.0.1".into(),
        )
        .await
        .expect("保存日志失败");

        let row = db
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT MR_CONTENT AS c, MR_USER_ID AS u FROM SYS_USER_OPERATE_LOG",
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            row.try_get::<String>("", "c").unwrap(),
            long_content,
            "content 应为全文，不再截断到 1000 字符"
        );
        assert_eq!(row.try_get::<String>("", "u").unwrap(), "u1");
    }

    /// 分页查询返回的响应体携带 `isDeleted` 字段。
    #[tokio::test]
    async fn query_list_response_exposes_is_deleted() {
        let db = setup_db().await;
        let svc = SysUserOperateLogService::new();
        svc.save_log(
            &db,
            Some("u1".into()),
            "/user/login".into(),
            "{}".into(),
            "127.0.0.1".into(),
        )
        .await
        .expect("保存日志失败");

        let result = svc.query_list(&db, 1, 10).await.expect("查询日志失败");
        assert_eq!(result.records.len(), 1);
        assert_eq!(
            result.records[0].is_deleted, 0,
            "响应体应带 isDeleted 且在册时为 0"
        );
    }

    /// 未登录请求的 `user_id` 落空串、`create_by` 落 NULL，对齐 Java（UserContext 为 null）。
    #[tokio::test]
    async fn save_log_anonymous_user_writes_empty_id() {
        let db = setup_db().await;
        let svc = SysUserOperateLogService::new();
        svc.save_log(
            &db,
            None,
            "/config/initialized".into(),
            "".into(),
            "".into(),
        )
        .await
        .expect("保存日志失败");

        let row = db
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT MR_USER_ID AS u, MR_CREATE_BY AS cb FROM SYS_USER_OPERATE_LOG",
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(row.try_get::<String>("", "u").unwrap(), "");
        assert!(row.try_get::<Option<String>>("", "cb").unwrap().is_none());
    }
}
