//! 用户注销清理定时任务
//!
//! 对应 Java: `com.dcz.mrecord.task.CancelCleanupTask`（Java 端同样未实现，由 Rust 端补齐）。
//!
//! Java `SysUserServiceImpl.canceledMyUser` 仅把用户置为「注销待生效」并记录 `cancelTime`，
//! 方法注释中预留了「后续会有单独的定时任务，定时扫描待注销状态的用户」。本任务每日扫描
//! 冷静期已过的待注销用户，复用账簿删除流程（[`crate::handler::fin_book::purge_book`] 的
//! 备份 + 逻辑删除）清理其全部账簿 / 月度汇总 / 明细 / 模板项，最后物理删除用户本体。

use std::sync::Arc;

use chrono::Local;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};

use crate::{
    constant::user_status::UserStatus,
    entity::sys_user::{Column as UserCol, Entity as UserEntity, Model as UserModel},
    error::AppError,
    handler::fin_book,
    util::schedule::duration_until_daily,
};

/// 注销冷静期（天）。
///
/// 用户申请注销后保留该天数「反悔」窗口，`cancel_time` 超过本天数才真正清理数据。
/// 与 Java `SysUser` 实体注释（「用于计算 15 天冷静期」）保持一致；如需调整改这一处常量即可。
const COOLING_PERIOD_DAYS: i64 = 15;

/// 每日执行时刻（本地时间 03:00，避开月度提醒与导出高峰）。
const RUN_HOUR: u32 = 3;

/// 用户注销清理任务服务。
///
/// 负责：
/// 1. 每日扫描 `status = CANCELED_WAIT` 且 `cancel_time` 早于冷静期截止时间的用户；
/// 2. 对每位命中用户备份并逻辑删除其全部账簿数据（`is_deleted = 1`），再物理删除用户本体。
pub struct CancelCleanupTask;

impl CancelCleanupTask {
    /// 创建注销清理任务服务实例。
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    /// 启动后台定时循环。
    ///
    /// 对应 Java 中由 `@EnableScheduling` 自动注册、`@Scheduled(cron = "0 0 3 * * ?")` 触发的任务。
    pub fn start(self: Arc<Self>, db: DatabaseConnection) {
        tokio::spawn(async move {
            loop {
                let wait = duration_until_daily(RUN_HOUR, 0);
                tracing::info!("用户注销清理任务将在 {} 秒后执行", wait.as_secs());
                tokio::time::sleep(wait).await;

                if let Err(e) = self.run_once(&db).await {
                    tracing::error!("用户注销清理任务执行失败: {:?}", e);
                }
            }
        });
    }

    /// 执行一次注销清理，返回实际清理的用户数量。
    ///
    /// 单用户失败不影响其它用户：出错时记录日志并跳过，整体不返回错误。
    pub async fn run_once(&self, db: &DatabaseConnection) -> Result<usize, AppError> {
        let users = load_expired_cancel_users(db).await?;
        if users.is_empty() {
            tracing::info!("用户注销清理任务：无可清理用户");
            return Ok(0);
        }

        tracing::info!("用户注销清理任务匹配到 {} 位待注销用户", users.len());
        let mut cleaned = 0usize;
        for user in &users {
            match cleanup_user(db, user).await {
                Ok(books) => {
                    cleaned += 1;
                    tracing::info!(
                        user_id = %user.id,
                        books,
                        "用户注销清理完成，已删除 {} 个账簿及其关联数据",
                        books
                    );
                }
                Err(e) => {
                    tracing::error!(user_id = %user.id, "用户注销清理失败，已跳过: {:?}", e);
                }
            }
        }

        tracing::info!("用户注销清理任务执行完成，共清理 {} 位用户", cleaned);
        Ok(cleaned)
    }
}

/// 查询冷静期已过、可被清理的待注销用户。
///
/// 条件对齐 `canceledMyUser` 写入的字段：`status = CANCELED_WAIT`、`is_deleted = 0`，
/// 且 `cancel_time <= 冷静期截止时间`（`NULL` 的 `cancel_time` 不参与比较，不会被清理）。
async fn load_expired_cancel_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, AppError> {
    let deadline = Local::now().naive_local() - chrono::Duration::days(COOLING_PERIOD_DAYS);
    Ok(UserEntity::find()
        .filter(UserCol::Status.eq(UserStatus::CanceledWait as i32))
        .filter(UserCol::IsDeleted.eq(0))
        .filter(UserCol::CancelTime.lte(deadline))
        .all(db)
        .await?)
}

/// 物理删除单个用户及其全部账簿 / 月度汇总 / 明细 / 模板项。
///
/// 整个流程在单个事务内完成：先复用 [`crate::handler::fin_book::delete_user_books`] 备份并
/// 删除账簿数据，再物理删除用户本体。返回删除的账簿数量。
async fn cleanup_user(db: &DatabaseConnection, user: &UserModel) -> Result<usize, AppError> {
    let txn = db.begin().await?;
    let books = fin_book::delete_user_books(&txn, &user.id).await?;
    UserEntity::delete_by_id(user.id.clone()).exec(&txn).await?;
    txn.commit().await?;
    Ok(books)
}

#[cfg(test)]
mod tests {

    /// 原生 COUNT 查询行（Sea-ORM 2.0 的 `query_one` 只接受 `StatementBuilder`，原生 SQL 走 `find_by_statement`）。
    #[derive(Clone, Debug, PartialEq, FromQueryResult)]
    struct CountRow {
        c: i64,
    }

    use super::*;
    use chrono::NaiveDateTime;
    use sea_orm::{ConnectionTrait, Database, DbBackend, EntityTrait, FromQueryResult, Statement};

    /// 建好任务涉及的全部表（主表 + 备份表），返回内存库连接。
    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:?cache=shared")
            .await
            .expect("连接内存库失败");
        for ddl in [
            "CREATE TABLE IF NOT EXISTS SYS_USER (
                MR_ID TEXT PRIMARY KEY, MR_EMAIL TEXT, MR_PASSWORD TEXT, MR_NICKNAME TEXT,
                MR_ADMIN INTEGER DEFAULT 0, MR_STATUS INTEGER DEFAULT 0, MR_CANCEL_TIME TEXT,
                MR_REMIND_ENABLED INTEGER DEFAULT 0, MR_REMIND_DAY INTEGER,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_BOOK (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_NAME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_TEMPLATE_ITEM (
                MR_ID TEXT PRIMARY KEY, MR_BOOK_ID TEXT, MR_ITEM_NAME TEXT, MR_ITEM_TYPE INTEGER,
                MR_ICON TEXT, MR_SORT TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS FIN_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_BOOK (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_NAME TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_MONTH_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_USER_ID TEXT, MR_BOOK_ID TEXT,
                MR_YEAR INTEGER, MR_MONTH INTEGER,
                MR_TOTAL_ASSET REAL, MR_TOTAL_LIABILITY REAL, MR_NET_ASSET REAL,
                MR_MONTH_ON_MONTH REAL, MR_YEAR_ON_YEAR REAL, MR_NOTE TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_MONTH_ITEM_RECORD (
                MR_ID TEXT PRIMARY KEY, MR_YEAR INTEGER, MR_MONTH INTEGER, MR_BOOK_ID TEXT,
                MR_TEMPLATE_ITEM_ID TEXT, MR_ITEM_VALUE REAL,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
            "CREATE TABLE IF NOT EXISTS SYS_BACKUP_TEMPLATE_ITEM (
                MR_ID TEXT PRIMARY KEY, MR_BOOK_ID TEXT, MR_ITEM_NAME TEXT, MR_ITEM_TYPE INTEGER,
                MR_ICON TEXT, MR_SORT TEXT,
                MR_CREATE_BY TEXT, MR_CREATE_TIME TEXT DEFAULT CURRENT_TIMESTAMP,
                MR_UPDATE_BY TEXT, MR_UPDATE_TIME TEXT, MR_IS_DELETED INTEGER DEFAULT 0
            )",
        ] {
            db.execute_unprepared(ddl).await.expect("建表失败");
        }
        db
    }

    /// 插入一位用户，`cancel_time` 按给定时间回拨。
    async fn insert_user(
        db: &DatabaseConnection,
        id: &str,
        status: i32,
        cancel_time: Option<NaiveDateTime>,
    ) {
        let cancel = cancel_time
            .map(|t| format!("'{}'", t.format("%Y-%m-%d %H:%M:%S")))
            .unwrap_or_else(|| "NULL".to_string());
        db.execute_unprepared(&format!(
            "INSERT INTO SYS_USER (MR_ID, MR_EMAIL, MR_PASSWORD, MR_NICKNAME, MR_STATUS, MR_CANCEL_TIME)
             VALUES ('{id}', '{id}@test.com', 'hashed', '{id}', {status}, {cancel})"
        ))
        .await
        .unwrap();
    }

    /// 为用户写入两个账簿及其模板项 / 月度汇总 / 明细。
    async fn insert_user_finance(db: &DatabaseConnection, user_id: &str) {
        for book in ["bk1", "bk2"] {
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_BOOK (MR_ID, MR_USER_ID, MR_BOOK_NAME) VALUES ('{book}', '{user_id}', '{book}')"
            ))
            .await
            .unwrap();
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_TEMPLATE_ITEM (MR_ID, MR_BOOK_ID, MR_ITEM_NAME, MR_ITEM_TYPE, MR_ICON, MR_SORT)
                 VALUES ('tpl-{book}', '{book}', '现金', 1, '', '1')"
            ))
            .await
            .unwrap();
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_MONTH_RECORD (MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, MR_TOTAL_ASSET, MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR)
                 VALUES ('mr-{book}', '{user_id}', '{book}', 2026, 4, 100, 40, 60, 5, 12)"
            ))
            .await
            .unwrap();
            db.execute_unprepared(&format!(
                "INSERT INTO FIN_MONTH_ITEM_RECORD (MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE)
                 VALUES ('mir-{book}', 2026, 4, '{book}', 'tpl-{book}', 100)"
            ))
            .await
            .unwrap();
        }
    }

    async fn count(db: &DatabaseConnection, table: &str) -> i64 {
        let row = CountRow::find_by_statement(Statement::from_string(
            DbBackend::Sqlite,
            format!("SELECT COUNT(*) AS c FROM {table}"),
        ))
        .one(db)
        .await
        .unwrap()
        .unwrap();
        row.c
    }

    /// 统计业务表中「在册」（`is_deleted = 0`）的行数——账簿链路为逻辑删除，
    /// 删除后行仍在库中但置为 `is_deleted = 1`，对用户不可见。
    async fn count_active(db: &DatabaseConnection, table: &str) -> i64 {
        let row = CountRow::find_by_statement(Statement::from_string(
            DbBackend::Sqlite,
            format!("SELECT COUNT(*) AS c FROM {table} WHERE MR_IS_DELETED = 0"),
        ))
        .one(db)
        .await
        .unwrap()
        .unwrap();
        row.c
    }

    /// 验收：待注销且 `cancel_time` 超过冷静期（回拨 16 天）的用户，任务执行后用户被物理
    /// 删除、账簿链路被逻辑删除（`is_deleted = 1`，对用户不可见），备份表留下快照。
    #[tokio::test]
    async fn run_once_cleans_expired_canceled_user() {
        let db = setup_db().await;
        insert_user(
            &db,
            "expired",
            UserStatus::CanceledWait as i32,
            Some(Local::now().naive_local() - chrono::Duration::days(16)),
        )
        .await;
        insert_user_finance(&db, "expired").await;

        let task = CancelCleanupTask::new();
        let cleaned = task.run_once(&db).await.expect("任务执行失败");

        assert_eq!(cleaned, 1);
        // 用户本体物理删除
        assert_eq!(count(&db, "SYS_USER").await, 0);
        // 账簿链路逻辑删除：行保留但 is_deleted = 1（业务查询过滤 is_deleted = 0 后不可见）
        assert_eq!(count_active(&db, "FIN_BOOK").await, 0);
        assert_eq!(count_active(&db, "FIN_TEMPLATE_ITEM").await, 0);
        assert_eq!(count_active(&db, "FIN_MONTH_RECORD").await, 0);
        assert_eq!(count_active(&db, "FIN_MONTH_ITEM_RECORD").await, 0);
        assert_eq!(count(&db, "FIN_BOOK").await, 2);
        assert_eq!(count(&db, "FIN_MONTH_RECORD").await, 2);
        // 备份表保留快照（2 账簿 / 2 模板项 / 2 汇总 / 2 明细）
        assert_eq!(count(&db, "SYS_BACKUP_BOOK").await, 2);
        assert_eq!(count(&db, "SYS_BACKUP_TEMPLATE_ITEM").await, 2);
        assert_eq!(count(&db, "SYS_BACKUP_MONTH_RECORD").await, 2);
        assert_eq!(count(&db, "SYS_BACKUP_MONTH_ITEM_RECORD").await, 2);
    }

    /// 冷静期内（回拨 3 天）的用户与正常用户都不应被清理。
    #[tokio::test]
    async fn run_once_skips_user_within_cooling_period() {
        let db = setup_db().await;
        insert_user(
            &db,
            "waiting",
            UserStatus::CanceledWait as i32,
            Some(Local::now().naive_local() - chrono::Duration::days(3)),
        )
        .await;
        insert_user_finance(&db, "waiting").await;
        // 正常用户与待注销但未记录 cancel_time 的用户均保留
        insert_user(&db, "normal", UserStatus::Normal as i32, None).await;
        insert_user(&db, "no-cancel-time", UserStatus::CanceledWait as i32, None).await;

        let task = CancelCleanupTask::new();
        let cleaned = task.run_once(&db).await.expect("任务执行失败");

        assert_eq!(cleaned, 0);
        assert_eq!(count(&db, "SYS_USER").await, 3);
        assert_eq!(count(&db, "FIN_BOOK").await, 2);
        // 备份表不应有任何写入
        assert_eq!(count(&db, "SYS_BACKUP_BOOK").await, 0);
    }

    /// 冷静期边界：恰好 15 天前注销的用户应被清理（`lte` 语义）。
    #[tokio::test]
    async fn run_once_cleans_user_at_cooling_boundary() {
        let db = setup_db().await;
        insert_user(
            &db,
            "boundary",
            UserStatus::CanceledWait as i32,
            Some(Local::now().naive_local() - chrono::Duration::days(COOLING_PERIOD_DAYS)),
        )
        .await;
        insert_user_finance(&db, "boundary").await;

        let task = CancelCleanupTask::new();
        let cleaned = task.run_once(&db).await.expect("任务执行失败");

        assert_eq!(cleaned, 1);
        assert_eq!(count(&db, "SYS_USER").await, 0);
    }
}
