use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Course {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Module {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Chapter {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub module_id: Uuid,
    pub title: String,
    pub content: String,
    pub is_quiz: Option<bool>,
    pub order: i32,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CourseDetails {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
    pub modules: Vec<ModuleDetails>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ModuleDetails {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    pub chapters: Vec<ChapterSummary>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChapterSummary {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub is_quiz: Option<bool>,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCourse {
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCourse {
    pub title: Option<String>,
    pub description: Option<String>,
    pub level: Option<String>,
}
