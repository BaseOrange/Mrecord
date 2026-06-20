//! 金额字段文本化迁移
//!
//! 对应 Java BigDecimal 存储语义，将历史 SQLite REAL 金额列重建为 TEXT，避免后续读写继续依赖浮点列。

use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        rebuild_record_amount(db).await?;
        rebuild_fin_month_record_amounts(db).await?;
        rebuild_fin_month_item_record_amount(db).await?;
        rebuild_backup_month_record_amounts(db).await?;
        rebuild_backup_month_item_record_amount(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

/// 执行单条 SQLite DDL/DML 语句。
async fn exec<C>(db: &C, sql: &str) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    db.execute_unprepared(sql).await.map(|_| ())
}

/// 重建 record.amount 为 TEXT。
async fn rebuild_record_amount<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    exec(db, "DROP TABLE IF EXISTS record_decimal_tmp").await?;
    exec(
        db,
        r#"CREATE TABLE record_decimal_tmp (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            amount TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .await?;
    exec(
        db,
        r#"INSERT INTO record_decimal_tmp (id, title, amount, created_at)
           SELECT id, title, printf('%.2f', amount), created_at FROM record"#,
    )
    .await?;
    exec(db, "DROP TABLE record").await?;
    exec(db, "ALTER TABLE record_decimal_tmp RENAME TO record").await
}

/// 重建 FIN_MONTH_RECORD 金额和增长率字段为 TEXT。
async fn rebuild_fin_month_record_amounts<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    exec(db, "DROP TABLE IF EXISTS FIN_MONTH_RECORD_DECIMAL_TMP").await?;
    exec(
        db,
        r#"CREATE TABLE FIN_MONTH_RECORD_DECIMAL_TMP (
            MR_ID TEXT NOT NULL PRIMARY KEY,
            MR_USER_ID TEXT NOT NULL,
            MR_BOOK_ID TEXT NOT NULL,
            MR_YEAR INTEGER NOT NULL,
            MR_MONTH INTEGER NOT NULL,
            MR_TOTAL_ASSET TEXT NOT NULL,
            MR_TOTAL_LIABILITY TEXT NOT NULL,
            MR_NET_ASSET TEXT NOT NULL,
            MR_MONTH_ON_MONTH TEXT NOT NULL,
            MR_YEAR_ON_YEAR TEXT NOT NULL,
            MR_NOTE TEXT,
            MR_CREATE_BY TEXT,
            MR_CREATE_TIME DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            MR_UPDATE_BY TEXT,
            MR_UPDATE_TIME DATETIME,
            MR_IS_DELETED INTEGER NOT NULL DEFAULT 0
        )"#,
    )
    .await?;
    exec(
        db,
        r#"INSERT INTO FIN_MONTH_RECORD_DECIMAL_TMP (
            MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, MR_TOTAL_ASSET,
            MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR,
            MR_NOTE, MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        )
        SELECT
            MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, printf('%.2f', MR_TOTAL_ASSET),
            printf('%.2f', MR_TOTAL_LIABILITY), printf('%.2f', MR_NET_ASSET),
            printf('%.2f', MR_MONTH_ON_MONTH), printf('%.2f', MR_YEAR_ON_YEAR),
            MR_NOTE, MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        FROM FIN_MONTH_RECORD"#,
    )
    .await?;
    exec(db, "DROP TABLE FIN_MONTH_RECORD").await?;
    exec(
        db,
        "ALTER TABLE FIN_MONTH_RECORD_DECIMAL_TMP RENAME TO FIN_MONTH_RECORD",
    )
    .await
}

/// 重建 FIN_MONTH_ITEM_RECORD 明细金额字段为 TEXT。
async fn rebuild_fin_month_item_record_amount<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    exec(db, "DROP TABLE IF EXISTS FIN_MONTH_ITEM_RECORD_DECIMAL_TMP").await?;
    exec(
        db,
        r#"CREATE TABLE FIN_MONTH_ITEM_RECORD_DECIMAL_TMP (
            MR_ID TEXT NOT NULL PRIMARY KEY,
            MR_YEAR INTEGER NOT NULL,
            MR_MONTH INTEGER NOT NULL,
            MR_BOOK_ID TEXT NOT NULL,
            MR_TEMPLATE_ITEM_ID TEXT NOT NULL,
            MR_ITEM_VALUE TEXT NOT NULL,
            MR_CREATE_BY TEXT,
            MR_CREATE_TIME DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            MR_UPDATE_BY TEXT,
            MR_UPDATE_TIME DATETIME,
            MR_IS_DELETED INTEGER NOT NULL DEFAULT 0
        )"#,
    )
    .await?;
    exec(
        db,
        r#"INSERT INTO FIN_MONTH_ITEM_RECORD_DECIMAL_TMP (
            MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE,
            MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        )
        SELECT
            MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, printf('%.2f', MR_ITEM_VALUE),
            MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        FROM FIN_MONTH_ITEM_RECORD"#,
    )
    .await?;
    exec(db, "DROP TABLE FIN_MONTH_ITEM_RECORD").await?;
    exec(
        db,
        "ALTER TABLE FIN_MONTH_ITEM_RECORD_DECIMAL_TMP RENAME TO FIN_MONTH_ITEM_RECORD",
    )
    .await
}

/// 重建 SYS_BACKUP_MONTH_RECORD 金额和增长率字段为 TEXT。
async fn rebuild_backup_month_record_amounts<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    exec(
        db,
        "DROP TABLE IF EXISTS SYS_BACKUP_MONTH_RECORD_DECIMAL_TMP",
    )
    .await?;
    exec(
        db,
        r#"CREATE TABLE SYS_BACKUP_MONTH_RECORD_DECIMAL_TMP (
            MR_ID TEXT NOT NULL PRIMARY KEY,
            MR_USER_ID TEXT NOT NULL,
            MR_BOOK_ID TEXT NOT NULL,
            MR_YEAR INTEGER NOT NULL,
            MR_MONTH INTEGER NOT NULL,
            MR_TOTAL_ASSET TEXT NOT NULL,
            MR_TOTAL_LIABILITY TEXT NOT NULL,
            MR_NET_ASSET TEXT NOT NULL,
            MR_MONTH_ON_MONTH TEXT NOT NULL,
            MR_YEAR_ON_YEAR TEXT NOT NULL,
            MR_NOTE TEXT,
            MR_CREATE_BY TEXT,
            MR_CREATE_TIME DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            MR_UPDATE_BY TEXT,
            MR_UPDATE_TIME DATETIME,
            MR_IS_DELETED INTEGER NOT NULL DEFAULT 0
        )"#,
    )
    .await?;
    exec(
        db,
        r#"INSERT INTO SYS_BACKUP_MONTH_RECORD_DECIMAL_TMP (
            MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, MR_TOTAL_ASSET,
            MR_TOTAL_LIABILITY, MR_NET_ASSET, MR_MONTH_ON_MONTH, MR_YEAR_ON_YEAR,
            MR_NOTE, MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        )
        SELECT
            MR_ID, MR_USER_ID, MR_BOOK_ID, MR_YEAR, MR_MONTH, printf('%.2f', MR_TOTAL_ASSET),
            printf('%.2f', MR_TOTAL_LIABILITY), printf('%.2f', MR_NET_ASSET),
            printf('%.2f', MR_MONTH_ON_MONTH), printf('%.2f', MR_YEAR_ON_YEAR),
            MR_NOTE, MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        FROM SYS_BACKUP_MONTH_RECORD"#,
    )
    .await?;
    exec(db, "DROP TABLE SYS_BACKUP_MONTH_RECORD").await?;
    exec(
        db,
        "ALTER TABLE SYS_BACKUP_MONTH_RECORD_DECIMAL_TMP RENAME TO SYS_BACKUP_MONTH_RECORD",
    )
    .await
}

/// 重建 SYS_BACKUP_MONTH_ITEM_RECORD 明细金额字段为 TEXT。
async fn rebuild_backup_month_item_record_amount<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    exec(
        db,
        "DROP TABLE IF EXISTS SYS_BACKUP_MONTH_ITEM_RECORD_DECIMAL_TMP",
    )
    .await?;
    exec(
        db,
        r#"CREATE TABLE SYS_BACKUP_MONTH_ITEM_RECORD_DECIMAL_TMP (
            MR_ID TEXT NOT NULL PRIMARY KEY,
            MR_YEAR INTEGER NOT NULL,
            MR_MONTH INTEGER NOT NULL,
            MR_BOOK_ID TEXT NOT NULL,
            MR_TEMPLATE_ITEM_ID TEXT NOT NULL,
            MR_ITEM_VALUE TEXT NOT NULL,
            MR_CREATE_BY TEXT,
            MR_CREATE_TIME DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            MR_UPDATE_BY TEXT,
            MR_UPDATE_TIME DATETIME,
            MR_IS_DELETED INTEGER NOT NULL DEFAULT 0
        )"#,
    )
    .await?;
    exec(
        db,
        r#"INSERT INTO SYS_BACKUP_MONTH_ITEM_RECORD_DECIMAL_TMP (
            MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, MR_ITEM_VALUE,
            MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        )
        SELECT
            MR_ID, MR_YEAR, MR_MONTH, MR_BOOK_ID, MR_TEMPLATE_ITEM_ID, printf('%.2f', MR_ITEM_VALUE),
            MR_CREATE_BY, MR_CREATE_TIME, MR_UPDATE_BY, MR_UPDATE_TIME, MR_IS_DELETED
        FROM SYS_BACKUP_MONTH_ITEM_RECORD"#,
    )
    .await?;
    exec(db, "DROP TABLE SYS_BACKUP_MONTH_ITEM_RECORD").await?;
    exec(
        db,
        "ALTER TABLE SYS_BACKUP_MONTH_ITEM_RECORD_DECIMAL_TMP RENAME TO SYS_BACKUP_MONTH_ITEM_RECORD",
    )
    .await
}
