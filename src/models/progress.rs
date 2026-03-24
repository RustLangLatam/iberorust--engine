use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Progress {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub chapter_id: Uuid,
    pub completed: Option<bool>,
    pub score: Option<i32>,
    #[schema(value_type = String)]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Certification {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub course_id: Uuid,
    #[schema(value_type = String)]
    pub issued_at: DateTime<Utc>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuizSubmission {
    pub answers: Vec<String>, // Mocking answers for now
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuizResult {
    pub score: i32,
    pub passed: bool,
}
