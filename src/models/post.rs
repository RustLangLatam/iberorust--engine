use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
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
    pub title: String,
    #[schema(value_type = String)]
    pub author_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}
