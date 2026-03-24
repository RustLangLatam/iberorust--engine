use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
}
