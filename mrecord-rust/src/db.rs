//! 数据库连接与初始化
//!
//! 对应 Java: Spring Boot 启动时执行 `src/main/resources/schema.sql` 初始化 SQLite 数据库。

use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, FromQueryResult, Statement,
};

const SCHEMA_SQL: &str = include_str!("../schema.sql");
const MONEY_SCHEMA_PATCH_SQL: &str = r#"
BEGIN;

ALTER TABLE FIN_MONTH_RECORD RENAME TO FIN_MONTH_RECORD_OLD;
CREATE TABLE FIN_MONTH_RECORD (
    MR_ID              TEXT PRIMARY KEY,
    MR_USER_ID         TEXT,
    MR_BOOK_ID         TEXT,
    MR_YEAR            INTEGER,
    MR_MONTH           INTEGER,
    MR_TOTAL_ASSET     REAL,
    MR_TOTAL_LIABILITY REAL,
    MR_NET_ASSET       REAL,
    MR_MONTH_ON_MONTH  REAL,
    MR_YEAR_ON_YEAR    REAL,
    MR_NOTE            TEXT,
    MR_CREATE_BY       TEXT,
    MR_CREATE_TIME     TEXT DEFAULT CURRENT_TIMESTAMP,
    MR_UPDATE_BY       TEXT,
    MR_UPDATE_TIME     TEXT,
    MR_IS_DELETED      INTEGER DEFAULT 0
);
INSERT INTO FIN_MONTH_RECORD SELECT
    MR_ID,
    MR_USER_ID,
    MR_BOOK_ID,
    MR_YEAR,
    MR_MONTH,
    CAST(MR_TOTAL_ASSET AS REAL),
    CAST(MR_TOTAL_LIABILITY AS REAL),
    CAST(MR_NET_ASSET AS REAL),
    CAST(MR_MONTH_ON_MONTH AS REAL),
    CAST(MR_YEAR_ON_YEAR AS REAL),
    MR_NOTE,
    MR_CREATE_BY,
    MR_CREATE_TIME,
    MR_UPDATE_BY,
    MR_UPDATE_TIME,
    MR_IS_DELETED
FROM FIN_MONTH_RECORD_OLD;
DROP TABLE FIN_MONTH_RECORD_OLD;

ALTER TABLE FIN_MONTH_ITEM_RECORD RENAME TO FIN_MONTH_ITEM_RECORD_OLD;
CREATE TABLE FIN_MONTH_ITEM_RECORD (
    MR_ID               TEXT PRIMARY KEY,
    MR_YEAR             INTEGER,
    MR_MONTH            INTEGER,
    MR_BOOK_ID          TEXT,
    MR_TEMPLATE_ITEM_ID TEXT,
    MR_ITEM_VALUE       REAL,
    MR_CREATE_BY        TEXT,
    MR_CREATE_TIME      TEXT DEFAULT CURRENT_TIMESTAMP,
    MR_UPDATE_BY        TEXT,
    MR_UPDATE_TIME      TEXT,
    MR_IS_DELETED       INTEGER DEFAULT 0
);
INSERT INTO FIN_MONTH_ITEM_RECORD SELECT
    MR_ID,
    MR_YEAR,
    MR_MONTH,
    MR_BOOK_ID,
    MR_TEMPLATE_ITEM_ID,
    CAST(MR_ITEM_VALUE AS REAL),
    MR_CREATE_BY,
    MR_CREATE_TIME,
    MR_UPDATE_BY,
    MR_UPDATE_TIME,
    MR_IS_DELETED
FROM FIN_MONTH_ITEM_RECORD_OLD;
DROP TABLE FIN_MONTH_ITEM_RECORD_OLD;

ALTER TABLE SYS_BACKUP_MONTH_RECORD RENAME TO SYS_BACKUP_MONTH_RECORD_OLD;
CREATE TABLE SYS_BACKUP_MONTH_RECORD (
    MR_ID              TEXT PRIMARY KEY,
    MR_USER_ID         TEXT,
    MR_BOOK_ID         TEXT,
    MR_YEAR            INTEGER,
    MR_MONTH           INTEGER,
    MR_TOTAL_ASSET     REAL,
    MR_TOTAL_LIABILITY REAL,
    MR_NET_ASSET       REAL,
    MR_MONTH_ON_MONTH  REAL,
    MR_YEAR_ON_YEAR    REAL,
    MR_NOTE            TEXT,
    MR_CREATE_BY       TEXT,
    MR_CREATE_TIME     TEXT DEFAULT CURRENT_TIMESTAMP,
    MR_UPDATE_BY       TEXT,
    MR_UPDATE_TIME     TEXT,
    MR_IS_DELETED      INTEGER DEFAULT 0
);
INSERT INTO SYS_BACKUP_MONTH_RECORD SELECT
    MR_ID,
    MR_USER_ID,
    MR_BOOK_ID,
    MR_YEAR,
    MR_MONTH,
    CAST(MR_TOTAL_ASSET AS REAL),
    CAST(MR_TOTAL_LIABILITY AS REAL),
    CAST(MR_NET_ASSET AS REAL),
    CAST(MR_MONTH_ON_MONTH AS REAL),
    CAST(MR_YEAR_ON_YEAR AS REAL),
    MR_NOTE,
    MR_CREATE_BY,
    MR_CREATE_TIME,
    MR_UPDATE_BY,
    MR_UPDATE_TIME,
    MR_IS_DELETED
FROM SYS_BACKUP_MONTH_RECORD_OLD;
DROP TABLE SYS_BACKUP_MONTH_RECORD_OLD;

ALTER TABLE SYS_BACKUP_MONTH_ITEM_RECORD RENAME TO SYS_BACKUP_MONTH_ITEM_RECORD_OLD;
CREATE TABLE SYS_BACKUP_MONTH_ITEM_RECORD (
    MR_ID               TEXT PRIMARY KEY,
    MR_YEAR             INTEGER,
    MR_MONTH            INTEGER,
    MR_BOOK_ID          TEXT,
    MR_TEMPLATE_ITEM_ID TEXT,
    MR_ITEM_VALUE       REAL,
    MR_CREATE_BY        TEXT,
    MR_CREATE_TIME      TEXT DEFAULT CURRENT_TIMESTAMP,
    MR_UPDATE_BY        TEXT,
    MR_UPDATE_TIME      TEXT,
    MR_IS_DELETED       INTEGER DEFAULT 0
);
INSERT INTO SYS_BACKUP_MONTH_ITEM_RECORD SELECT
    MR_ID,
    MR_YEAR,
    MR_MONTH,
    MR_BOOK_ID,
    MR_TEMPLATE_ITEM_ID,
    CAST(MR_ITEM_VALUE AS REAL),
    MR_CREATE_BY,
    MR_CREATE_TIME,
    MR_UPDATE_BY,
    MR_UPDATE_TIME,
    MR_IS_DELETED
FROM SYS_BACKUP_MONTH_ITEM_RECORD_OLD;
DROP TABLE SYS_BACKUP_MONTH_ITEM_RECORD_OLD;

COMMIT;
"#;

/// 建立 SQLite 连接并执行与 Java 项目一致的 schema.sql 初始化脚本。
pub async fn connect() -> DatabaseConnection {
    let db_url = "sqlite://data.db?mode=rwc";
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    conn.execute_unprepared(SCHEMA_SQL)
        .await
        .expect("Failed to initialize database schema");

    patch_money_column_types(&conn).await;

    conn
}

/// 修复早期 schema.sql 将金额字段建成 TEXT，导致 SeaORM Decimal 按 REAL 解码失败的问题。
async fn patch_money_column_types(conn: &DatabaseConnection) {
    if !is_money_schema_patch_required(conn).await {
        return;
    }

    conn.execute_unprepared(MONEY_SCHEMA_PATCH_SQL)
        .await
        .expect("Failed to patch money column types");
}

/// PRAGMA table_info 查询结果行（Sea-ORM 2.0 的 `query_all` 只接受 `StatementBuilder`，
/// 原生 SQL 查询需经 `FromQueryResult` 结构体承载）。
///
/// 注意：`FromQueryResult` 派生宏不识别实体模型用的 `column_name` 属性（那是
/// `DeriveEntityModel` 的），只识别 `alias`/`from_alias`。PRAGMA 返回的列名是
/// 小写的 `type`，与 Rust 关键字冲突无法直接作字段名，故此处用 `alias = "type"`。
#[derive(Clone, Debug, PartialEq, FromQueryResult)]
struct PragmaColumnRow {
    name: String,
    #[sea_orm(alias = "type")]
    column_type: String,
}

/// 检查 FIN_MONTH_ITEM_RECORD.MR_ITEM_VALUE 是否仍为旧版 TEXT 类型。
async fn is_money_schema_patch_required(conn: &DatabaseConnection) -> bool {
    let rows = PragmaColumnRow::find_by_statement(Statement::from_string(
        DbBackend::Sqlite,
        "PRAGMA table_info(FIN_MONTH_ITEM_RECORD)",
    ))
    .all(conn)
    .await
    .expect("Failed to inspect FIN_MONTH_ITEM_RECORD schema");

    rows.into_iter().any(|row| {
        row.name.eq_ignore_ascii_case("MR_ITEM_VALUE")
            && row.column_type.eq_ignore_ascii_case("TEXT")
    })
}
