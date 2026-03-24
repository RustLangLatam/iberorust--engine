use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inquiry {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub r#type: String, // Arquitectura, Entrenamiento
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
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
