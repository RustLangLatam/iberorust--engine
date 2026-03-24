use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Inquiry {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub r#type: String, // Arquitectura, Entrenamiento
    pub message: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitInquiryRequest {
    #[validate(length(min = 2, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Valid email is required"))]
    pub email: String,
    #[validate(length(min = 2, message = "Type is required"))]
    pub r#type: String,
    #[validate(length(min = 10, message = "Message must be at least 10 characters long"))]
    pub message: String,
}
