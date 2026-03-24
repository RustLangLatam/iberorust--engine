use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
    pub is_read: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
}
