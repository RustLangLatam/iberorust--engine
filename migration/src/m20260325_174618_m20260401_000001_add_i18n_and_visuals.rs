use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE courses ADD COLUMN IF NOT EXISTS image_url TEXT;
            ALTER TABLE posts ADD COLUMN IF NOT EXISTS image_url TEXT;
            ALTER TABLE posts ADD COLUMN IF NOT EXISTS tags TEXT[];
            ALTER TABLE chapters ADD COLUMN IF NOT EXISTS video_url TEXT;
        "#;

        let backend = manager.get_database_backend();
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                manager
                    .get_connection()
                    .execute(sea_orm_migration::sea_orm::Statement::from_string(backend, stmt.to_owned()))
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE chapters DROP COLUMN IF EXISTS video_url;
            ALTER TABLE posts DROP COLUMN IF EXISTS tags;
            ALTER TABLE posts DROP COLUMN IF EXISTS image_url;
            ALTER TABLE courses DROP COLUMN IF EXISTS image_url;
        "#;

        let backend = manager.get_database_backend();
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                manager
                    .get_connection()
                    .execute(sea_orm_migration::sea_orm::Statement::from_string(backend, stmt.to_owned()))
                    .await?;
            }
        }

        Ok(())
    }
}
