use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub content: String,
    pub is_quiz: Option<bool>,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CourseDetails {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
    pub modules: Vec<ModuleDetails>,
}

#[derive(Debug, Serialize)]
pub struct ModuleDetails {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    pub chapters: Vec<ChapterSummary>,
}

#[derive(Debug, Serialize)]
pub struct ChapterSummary {
    pub id: Uuid,
    pub title: String,
    pub is_quiz: Option<bool>,
    pub order: i32,
}
