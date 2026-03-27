use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);";

        let backend = manager.get_database_backend();
        manager
            .get_connection()
            .execute(sea_orm_migration::sea_orm::Statement::from_string(backend, sql.to_owned()))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "ALTER TABLE users DROP COLUMN IF EXISTS password_hash;";

        let backend = manager.get_database_backend();
        manager
            .get_connection()
            .execute(sea_orm_migration::sea_orm::Statement::from_string(backend, sql.to_owned()))
            .await?;

        Ok(())
    }
}
