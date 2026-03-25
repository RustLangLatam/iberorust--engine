use crate::entities::reference as ReferenceEntity;
use crate::error::AppError;
use crate::models::reference::{CreateReference, Reference, UpdateReference};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReferenceRepository: Send + Sync {
    async fn list_references(&self) -> Result<Vec<Reference>, AppError>;
    async fn create_reference(&self, req: CreateReference) -> Result<Reference, AppError>;
    async fn update_reference(&self, id: Uuid, req: UpdateReference) -> Result<Reference, AppError>;
    async fn delete_reference(&self, id: Uuid) -> Result<(), AppError>;
}

pub struct ReferenceRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<ReferenceEntity::Model> for Reference {
    fn from(model: ReferenceEntity::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            url: model.url,
            description: model.description,
            r#type: model.r#type,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl ReferenceRepository for ReferenceRepositoryImpl {
    async fn list_references(&self) -> Result<Vec<Reference>, AppError> {
        let refs: Vec<ReferenceEntity::Model> = ReferenceEntity::Entity::find()
            .order_by_desc(ReferenceEntity::Column::CreatedAt)
            .all(&self.db)
            .await?;
        Ok(refs.into_iter().map(Reference::from).collect())
    }

    async fn create_reference(&self, req: CreateReference) -> Result<Reference, AppError> {
        let new_ref = ReferenceEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(req.title),
            url: Set(req.url),
            description: Set(req.description),
            r#type: Set(req.r#type),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = ReferenceEntity::Entity::insert(new_ref).exec_with_returning(&self.db).await?;
        Ok(Reference::from(result))
    }

    async fn update_reference(&self, id: Uuid, req: UpdateReference) -> Result<Reference, AppError> {
        let model: Option<ReferenceEntity::Model> = ReferenceEntity::Entity::find_by_id(id).one(&self.db).await?;
        let mut r: ReferenceEntity::ActiveModel = model
            .ok_or_else(|| AppError::NotFound("Reference not found".to_string()))?
            .into();

        if let Some(title) = req.title { r.title = Set(title); }
        if let Some(url) = req.url { r.url = Set(url); }
        if let Some(desc) = req.description { r.description = Set(Some(desc)); }
        if let Some(t) = req.r#type { r.r#type = Set(t); }

        r.updated_at = Set(Utc::now());
        let result = r.update(&self.db).await?;
        Ok(Reference::from(result))
    }

    async fn delete_reference(&self, id: Uuid) -> Result<(), AppError> {
        let result = ReferenceEntity::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Reference not found".to_string()));
        }
        Ok(())
    }
}
