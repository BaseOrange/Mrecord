mod m20240604_000001_create_record;
mod m20240604_000002_create_core_tables;
mod m20240604_000003_create_sys_tables;
mod m20240604_000004_create_backup_tables;
mod m20260620_000001_money_decimal_text;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240604_000001_create_record::Migration),
            Box::new(m20240604_000002_create_core_tables::Migration),
            Box::new(m20240604_000003_create_sys_tables::Migration),
            Box::new(m20240604_000004_create_backup_tables::Migration),
            Box::new(m20260620_000001_money_decimal_text::Migration),
        ]
    }
}
