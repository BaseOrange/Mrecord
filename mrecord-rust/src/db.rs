use migration::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub async fn connect() -> DatabaseConnection {
    let db_url = "sqlite://data.db?mode=rwc";
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&conn, None)
        .await
        .expect("Failed to run migrations");

    conn
}
