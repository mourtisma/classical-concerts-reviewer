pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240302_153402_add_updated_at;
mod m20240302_171341_timestamps;
mod m20240302_175801_relations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240302_153402_add_updated_at::Migration),
            Box::new(m20240302_171341_timestamps::Migration),
            Box::new(m20240302_175801_relations::Migration),
        ]
    }
}
