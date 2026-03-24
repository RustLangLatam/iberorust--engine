use crate::entities::notification as NotificationEntity;
use crate::error::AppError;
use crate::models::notification::{CreateNotification, Notification};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{sea_query::Expr, *};
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn list_notifications(&self, user_id: Uuid) -> Result<Vec<Notification>, AppError>;
    async fn create_notification(&self, req: CreateNotification) -> Result<Notification, AppError>;
    async fn mark_as_read(&self, notification_id: Uuid, user_id: Uuid) -> Result<Notification, AppError>;
    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), AppError>;
}

pub struct NotificationRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<NotificationEntity::Model> for Notification {
    fn from(model: NotificationEntity::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            r#type: model.r#type,
            title: model.title,
            content: model.content,
            is_read: model.is_read,
            created_at: model.created_at,
        }
    }
}

#[async_trait]
impl NotificationRepository for NotificationRepositoryImpl {
    async fn list_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, AppError> {
        let notifications = NotificationEntity::Entity::find()
            .filter(NotificationEntity::Column::UserId.eq(user_id))
            .order_by_desc(NotificationEntity::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(notifications.into_iter().map(Notification::from).collect())
    }

    async fn create_notification(
        &self,
        req: CreateNotification,
    ) -> Result<Notification, AppError> {
        let n = NotificationEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(req.user_id),
            r#type: Set(req.r#type),
            title: Set(req.title),
            content: Set(req.content),
            is_read: Set(Some(false)),
            created_at: Set(Utc::now()),
        };

        let result = n.insert(&self.db).await?;
        Ok(Notification::from(result))
    }

    async fn mark_as_read(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, AppError> {
        let mut n: NotificationEntity::ActiveModel = NotificationEntity::Entity::find()
            .filter(NotificationEntity::Column::Id.eq(notification_id))
            .filter(NotificationEntity::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification not found".to_string()))?
            .into();

        n.is_read = Set(Some(true));
        let result = n.update(&self.db).await?;

        Ok(Notification::from(result))
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), AppError> {
        NotificationEntity::Entity::update_many()
            .col_expr(NotificationEntity::Column::IsRead, Expr::value(true))
            .filter(NotificationEntity::Column::UserId.eq(user_id))
            .filter(NotificationEntity::Column::IsRead.eq(false))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
