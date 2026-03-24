use crate::entities::inquiry as InquiryEntity;
use crate::error::AppError;
use crate::models::contact::{Inquiry, SubmitInquiryRequest};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn save_inquiry(&self, req: SubmitInquiryRequest) -> Result<Inquiry, AppError>;
}

pub struct ContactRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<InquiryEntity::Model> for Inquiry {
    fn from(model: InquiryEntity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            r#type: model.r#type,
            message: model.message,
            created_at: model.created_at,
        }
    }
}

#[async_trait]
impl ContactRepository for ContactRepositoryImpl {
    async fn save_inquiry(&self, req: SubmitInquiryRequest) -> Result<Inquiry, AppError> {
        let i = InquiryEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            email: Set(req.email),
            r#type: Set(req.r#type),
            message: Set(req.message),
            created_at: Set(Utc::now()),
        };

        let result = i.insert(&self.db).await?;
        Ok(Inquiry::from(result))
    }
}
