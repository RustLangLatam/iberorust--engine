pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260325_095739_add_admin_and_references;
mod m20260325_174618_m20260401_000001_add_i18n_and_visuals;
mod m20260326_095933_m20260501_000001_add_password_hash;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260325_095739_add_admin_and_references::Migration),
            Box::new(m20260325_174618_m20260401_000001_add_i18n_and_visuals::Migration),
            Box::new(m20260326_095933_m20260501_000001_add_password_hash::Migration),
        ]
    }
}
