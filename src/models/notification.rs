use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Notification {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
    pub is_read: Option<bool>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNotification {
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
}
