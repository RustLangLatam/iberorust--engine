use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub google_id: Option<String>,
    pub is_guest: bool,
    pub name: String,
    pub avatar_url: Option<String>,
    pub preferred_language: Option<String>,
    pub theme: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 255))]
    pub name: String,
    pub google_id: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    pub preferred_language: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserStats {
    pub completed_courses: i64,
    pub community_contributions: i64,
}
