use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Course {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub slug: String,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub level: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub prerequisites: Option<Vec<String>>,
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
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
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
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub content: String,
    pub is_quiz: Option<bool>,
    #[schema(value_type = Object)]
    pub quiz_data: Option<serde_json::Value>,
    pub code_snippet: Option<String>,
    pub video_url: Option<String>,
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
    pub slug: String,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub level: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub prerequisites: Option<Vec<String>>,
    pub modules: Vec<ModuleDetails>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ModuleDetails {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub order: i32,
    pub chapters: Vec<ChapterSummary>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChapterSummary {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub is_quiz: Option<bool>,
    #[schema(value_type = Object)]
    pub quiz_data: Option<serde_json::Value>,
    pub code_snippet: Option<String>,
    pub video_url: Option<String>,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCourse {
    pub slug: String,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub level: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub prerequisites: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateModule {
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateModule {
    #[schema(value_type = Object)]
    pub title: Option<serde_json::Value>,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChapter {
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub content: String,
    pub is_quiz: Option<bool>,
    #[schema(value_type = Object)]
    pub quiz_data: Option<serde_json::Value>,
    pub code_snippet: Option<String>,
    pub video_url: Option<String>,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateChapter {
    #[schema(value_type = Object)]
    pub title: Option<serde_json::Value>,
    pub content: Option<String>,
    pub is_quiz: Option<bool>,
    #[schema(value_type = Object)]
    pub quiz_data: Option<serde_json::Value>,
    pub code_snippet: Option<String>,
    pub video_url: Option<String>,
    pub order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCourse {
    pub slug: Option<String>,
    #[schema(value_type = Object)]
    pub title: Option<serde_json::Value>,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub level: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub prerequisites: Option<Vec<String>>,
}
