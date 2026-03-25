use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE users ADD COLUMN role VARCHAR(50) DEFAULT 'USER' NOT NULL;

            CREATE TABLE IF NOT EXISTS "references" (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                title VARCHAR(255) NOT NULL,
                url TEXT NOT NULL,
                description TEXT,
                type VARCHAR(100) NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
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
            DROP TABLE IF EXISTS "references";
            ALTER TABLE users DROP COLUMN IF EXISTS role;
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
