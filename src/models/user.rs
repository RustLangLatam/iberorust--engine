use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub email: String,
    pub google_id: Option<String>,
    pub is_guest: bool,
    pub name: String,
    pub avatar_url: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub preferred_language: Option<String>,
    pub theme: Option<String>,
    pub role: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 255))]
    pub name: String,
    pub google_id: Option<String>,
    pub avatar_url: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUser {
    pub preferred_language: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserRoleUpdate {
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserStats {
    pub completed_courses: i64,
    pub community_contributions: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminStats {
    pub total_users: i64,
    pub active_students: i64,
    pub total_courses: i64,
    pub total_posts: i64,
    pub total_threads: i64,
}
