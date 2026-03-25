use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Reference {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub r#type: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReference {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReference {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
}
