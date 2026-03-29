use crate::error::AppError;
use crate::models::reference::{CreateReference, Reference, UpdateReference};
use crate::repositories::reference::ReferenceRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct ReferenceService {
    reference_repo: Arc<dyn ReferenceRepository>,
}

impl ReferenceService {
    pub fn new(reference_repo: Arc<dyn ReferenceRepository>) -> Self {
        Self { reference_repo }
    }

    pub async fn list_references(&self, filters: crate::models::common::PaginationAndFilters) -> Result<Vec<Reference>, AppError> {
        self.reference_repo.list_references(filters).await
    }

    pub async fn create_reference(&self, req: CreateReference) -> Result<Reference, AppError> {
        self.reference_repo.create_reference(req).await
    }

    pub async fn update_reference(&self, id: Uuid, req: UpdateReference) -> Result<Reference, AppError> {
        self.reference_repo.update_reference(id, req).await
    }

    pub async fn delete_reference(&self, id: Uuid) -> Result<(), AppError> {
        self.reference_repo.delete_reference(id).await
    }
}
