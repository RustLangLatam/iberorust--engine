use sea_orm_migration::{prelude::*, sea_orm::Statement};
use serde_json::Value;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let refs_json = include_str!("../../migration/resources/references/_all_references.json");
        let refs: Vec<Value> = serde_json::from_str(refs_json)
            .map_err(|e| DbErr::Custom(format!("Failed to parse references JSON: {}", e)))?;

        for reference in refs {
            let ref_id = Uuid::new_v4();

            // Serialize bilingual objects
            let title = serde_json::to_string(&reference["title"]).unwrap_or_else(|_| "{}".to_string());
            let description = serde_json::to_string(&reference["description"]).unwrap_or_else(|_| "{}".to_string());

            let url = reference["url"].as_str().unwrap_or("").to_string();
            let ref_type = reference["type"].as_str().unwrap_or("").to_string();

            let insert_ref = format!(
                "INSERT INTO \"references\" (id, title, url, description, type) VALUES ('{}', '{}', '{}', '{}', '{}')",
                ref_id,
                title.replace("'", "''"),
                url.replace("'", "''"),
                description.replace("'", "''"),
                ref_type.replace("'", "''")
            );

            manager
                .get_connection()
                .execute(Statement::from_string(backend, insert_ref))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        manager
            .get_connection()
            .execute(Statement::from_string(backend, "DELETE FROM \"references\";".to_owned()))
            .await?;
        Ok(())
    }
}
