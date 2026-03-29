use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Reference {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub url: String,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub r#type: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReference {
    #[schema(value_type = Object)]
    pub title: serde_json::Value,
    pub url: String,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub r#type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReference {
    #[schema(value_type = Object)]
    pub title: Option<serde_json::Value>,
    pub url: Option<String>,
    #[schema(value_type = Object)]
    pub description: Option<serde_json::Value>,
    pub r#type: Option<String>,
}
