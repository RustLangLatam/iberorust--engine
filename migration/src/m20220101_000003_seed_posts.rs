use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("../../migration/resources/seed_posts.sql");
        let backend = manager.get_database_backend();
        for statement in sql.split(";\n") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                manager
                    .get_connection()
                    .execute(Statement::from_string(backend, stmt.to_owned()))
                    .await?;
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        manager
            .get_connection()
            .execute(Statement::from_string(backend, "DELETE FROM posts;".to_owned()))
            .await?;
        Ok(())
    }
}
