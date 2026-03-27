use sea_orm_migration::{prelude::*, sea_orm::Statement};
use serde_json::Value;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let courses_json = include_str!("../../migration/resources/courses/_all_courses.json");
        let courses: Vec<Value> = serde_json::from_str(courses_json)
            .map_err(|e| DbErr::Custom(format!("Failed to parse courses JSON: {}", e)))?;

        for course in courses {
            let course_string_id = course["id"].as_str().unwrap_or("unknown");
            let course_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, course_string_id.as_bytes());

            // Serialize bilingual objects back to JSON strings for DB insertion
            let title = serde_json::to_string(&course["title"]).unwrap_or_else(|_| "{}".to_string());
            let description = serde_json::to_string(&course["description"]).unwrap_or_else(|_| "{}".to_string());

            let level = course["level"].as_str().unwrap_or("");
            let image_url = course["thumbnail"].as_str().unwrap_or("");

            let insert_course = format!(
                "INSERT INTO courses (id, title, description, level, image_url) VALUES ('{}', '{}', '{}', '{}', '{}')",
                course_id,
                title.replace("'", "''"),
                description.replace("'", "''"),
                level.replace("'", "''"),
                image_url.replace("'", "''")
            );

            manager
                .get_connection()
                .execute(Statement::from_string(backend, insert_course))
                .await?;

            if let Some(modules) = course["modules"].as_array() {
                for (module_idx, module) in modules.iter().enumerate() {
                    let module_string_id = module["id"].as_str().unwrap_or_else(|| "");
                    let unique_mod_str = if module_string_id.is_empty() {
                        format!("{}-{}", course_string_id, module_idx)
                    } else {
                        format!("{}-{}", course_string_id, module_string_id)
                    };

                    let module_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, unique_mod_str.as_bytes());

                    let module_title = serde_json::to_string(&module["title"]).unwrap_or_else(|_| "{}".to_string());
                    let module_order = module_idx as i32 + 1;

                    let insert_module = format!(
                        "INSERT INTO modules (id, course_id, title, \"order\") VALUES ('{}', '{}', '{}', {})",
                        module_id,
                        course_id,
                        module_title.replace("'", "''"),
                        module_order
                    );

                    manager
                        .get_connection()
                        .execute(Statement::from_string(backend, insert_module))
                        .await?;

                    if let Some(chapters) = module["chapters"].as_array() {
                        for (chapter_idx, chapter) in chapters.iter().enumerate() {
                            let chapter_string_id = chapter["id"].as_str().unwrap_or_else(|| "");
                            let unique_chap_str = if chapter_string_id.is_empty() {
                                format!("{}-{}", unique_mod_str, chapter_idx)
                            } else {
                                format!("{}-{}", unique_mod_str, chapter_string_id)
                            };

                            let chapter_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, unique_chap_str.as_bytes());

                            let chapter_title = serde_json::to_string(&chapter["title"]).unwrap_or_else(|_| "{}".to_string());
                            let content = serde_json::to_string(&chapter["content"]).unwrap_or_else(|_| "{}".to_string());
                            let is_quiz = chapter["type"].as_str().unwrap_or("") == "quiz";
                            let chapter_order = chapter["order"].as_i64().unwrap_or((chapter_idx as i64) + 1) as i32;

                            let insert_chapter = format!(
                                "INSERT INTO chapters (id, module_id, title, content, is_quiz, \"order\") VALUES ('{}', '{}', '{}', '{}', {}, {})",
                                chapter_id,
                                module_id,
                                chapter_title.replace("'", "''"),
                                content.replace("'", "''"),
                                is_quiz,
                                chapter_order
                            );

                            manager
                                .get_connection()
                                .execute(Statement::from_string(backend, insert_chapter))
                                .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        manager
            .get_connection()
            .execute(Statement::from_string(backend, "DELETE FROM courses;".to_owned()))
            .await?;
        Ok(())
    }
}
