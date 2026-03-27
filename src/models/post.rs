use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub content: String,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    #[schema(value_type = String)]
    pub author_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub published_at: Option<DateTime<Utc>>,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostSummary {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    #[schema(value_type = String)]
    pub author_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePost {
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub content: String,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePost {
    #[schema(value_type = Object)]
    pub title: Option<serde_json::Value>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published_at: Option<DateTime<Utc>>,
}
