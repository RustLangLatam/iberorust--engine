use sea_orm_migration::{prelude::*, sea_orm::Statement};
use serde_json::Value;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let posts_json = include_str!("../../migration/resources/posts/_all_posts.json");
        let posts: Vec<Value> = serde_json::from_str(posts_json)
            .map_err(|e| DbErr::Custom(format!("Failed to parse posts JSON: {}", e)))?;

        for post in posts {
            let post_id = Uuid::new_v4();

            // Serialize bilingual objects
            let title = serde_json::to_string(&post["title"]).unwrap_or_else(|_| "{}".to_string());
            let content = post["content"].as_str().unwrap_or("{}").to_string(); // content in posts is already a JSON string in the _all_posts.json file

            let image_url = post["image_url"].as_str().unwrap_or("");
            let published_at = post["published_at"].as_str().unwrap_or("NOW()");

            let tags: Vec<String> = post["tags"]
                .as_array()
                .map(|a| a.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect())
                .unwrap_or_default();

            let tags_sql = if tags.is_empty() {
                "ARRAY[]::TEXT[]".to_string()
            } else {
                format!("ARRAY[{}]::TEXT[]", tags.iter().map(|t| format!("'{}'", t.replace("'", "''"))).collect::<Vec<_>>().join(", "))
            };

            let insert_post = format!(
                "INSERT INTO posts (id, title, content, image_url, published_at, tags) VALUES ('{}', '{}', '{}', '{}', '{}', {})",
                post_id,
                title.replace("'", "''"),
                content.replace("'", "''"),
                image_url.replace("'", "''"),
                published_at,
                tags_sql
            );

            manager
                .get_connection()
                .execute(Statement::from_string(backend, insert_post))
                .await?;
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
