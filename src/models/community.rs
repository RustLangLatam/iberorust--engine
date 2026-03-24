use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub likes_count: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub likes_count: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateThreadRequest {
    #[validate(length(min = 5, message = "Title must be at least 5 characters"))]
    pub title: String,
    #[validate(length(min = 10, message = "Content must be at least 10 characters"))]
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateThreadRequest {
    #[validate(length(min = 5, message = "Title must be at least 5 characters"))]
    pub title: Option<String>,
    #[validate(length(min = 10, message = "Content must be at least 10 characters"))]
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ThreadWithComments {
    #[serde(flatten)]
    pub thread: Thread,
    pub comments: Vec<Comment>,
}
