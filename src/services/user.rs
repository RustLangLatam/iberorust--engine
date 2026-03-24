use crate::error::AppError;
use crate::models::user::{UpdateUser, User, UserStats};
use crate::repositories::user::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, AppError> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    pub async fn update_user_preferences(
        &self,
        user_id: Uuid,
        update: UpdateUser,
    ) -> Result<User, AppError> {
        self.user_repo.update_user(user_id, update).await
    }

    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStats, AppError> {
        self.user_repo.get_user_stats(user_id).await
    }
}
