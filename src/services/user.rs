use crate::error::AppError;
use crate::models::user::{AdminStats, UpdateUser, User, UserRoleUpdate, UserStats};
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

    pub async fn get_admin_stats(&self) -> Result<AdminStats, AppError> {
        self.user_repo.get_admin_stats().await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, AppError> {
        self.user_repo.list_users().await
    }

    pub async fn update_user_role(&self, user_id: Uuid, req: UserRoleUpdate) -> Result<User, AppError> {
        self.user_repo.update_user_role(user_id, req).await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
        self.user_repo.delete_user(user_id).await
    }
}
