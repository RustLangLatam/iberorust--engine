use crate::error::AppError;
use crate::models::contact::{Inquiry, SubmitInquiryRequest};
use crate::repositories::contact::ContactRepository;
use std::sync::Arc;

pub struct ContactService {
    contact_repo: Arc<dyn ContactRepository>,
}

impl ContactService {
    pub fn new(contact_repo: Arc<dyn ContactRepository>) -> Self {
        Self { contact_repo }
    }

    pub async fn submit_inquiry(&self, req: SubmitInquiryRequest) -> Result<Inquiry, AppError> {
        let inquiry = self.contact_repo.save_inquiry(req).await?;

        // Here we could trigger async email notification processes

        Ok(inquiry)
    }
}
