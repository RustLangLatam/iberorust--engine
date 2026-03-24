use crate::error::AppError;
use crate::models::notification::Notification;
use crate::repositories::notification::NotificationRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct NotificationService {
    notification_repo: Arc<dyn NotificationRepository>,
}

impl NotificationService {
    pub fn new(notification_repo: Arc<dyn NotificationRepository>) -> Self {
        Self { notification_repo }
    }

    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, AppError> {
        self.notification_repo.list_notifications(user_id).await
    }

    pub async fn read_notification(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, AppError> {
        self.notification_repo.mark_as_read(notification_id, user_id).await
    }

    pub async fn read_all_notifications(&self, user_id: Uuid) -> Result<(), AppError> {
        self.notification_repo.mark_all_as_read(user_id).await
    }
}
