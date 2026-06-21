//! 数据库连接与初始化
//!
//! 对应 Java: Spring Boot 启动时执行 `src/main/resources/schema.sql` 初始化 SQLite 数据库。

use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

const SCHEMA_SQL: &str = include_str!("../schema.sql");

/// 建立 SQLite 连接并执行与 Java 项目一致的 schema.sql 初始化脚本。
pub async fn connect() -> DatabaseConnection {
    let db_url = "sqlite://data.db?mode=rwc";
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    conn.execute_unprepared(SCHEMA_SQL)
        .await
        .expect("Failed to initialize database schema");

    conn
}
