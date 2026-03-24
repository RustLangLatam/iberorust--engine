use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub chapter_id: Uuid,
    pub completed: Option<bool>,
    pub score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuizSubmission {
    pub answers: Vec<String>, // Mocking answers for now
}

#[derive(Debug, Serialize)]
pub struct QuizResult {
    pub score: i32,
    pub passed: bool,
}
