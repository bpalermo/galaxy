pub use sea_orm_migration::prelude::*;

mod m20221218_000001_create_account_table;
mod m20221218_000002_create_txn_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221218_000001_create_account_table::Migration),
            Box::new(m20221218_000002_create_txn_table::Migration),
        ]
    }
}
