use crate::entities::{certification, thread, user as UserEntity};
use crate::error::AppError;
use crate::models::user::{CreateUser, UpdateUser, User, UserStats};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create_user(&self, user: CreateUser, is_guest: bool) -> Result<User, AppError>;
    async fn update_user(&self, id: Uuid, update: UpdateUser) -> Result<User, AppError>;
    async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStats, AppError>;
}

pub struct UserRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<UserEntity::Model> for User {
    fn from(model: UserEntity::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            google_id: model.google_id,
            is_guest: model.is_guest,
            name: model.name,
            avatar_url: model.avatar_url,
            preferred_language: model.preferred_language,
            theme: model.theme,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::Entity::find()
            .filter(UserEntity::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = UserEntity::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(user.map(User::from))
    }

    async fn create_user(&self, user: CreateUser, is_guest: bool) -> Result<User, AppError> {
        let new_user = UserEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(user.email),
            google_id: Set(user.google_id),
            is_guest: Set(is_guest),
            name: Set(user.name),
            avatar_url: Set(user.avatar_url),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let created_user = new_user.insert(&self.db).await?;
        Ok(User::from(created_user))
    }

    async fn update_user(&self, id: Uuid, update: UpdateUser) -> Result<User, AppError> {
        let mut user: UserEntity::ActiveModel = UserEntity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
            .into();

        if let Some(lang) = update.preferred_language {
            user.preferred_language = Set(Some(lang));
        }
        if let Some(theme) = update.theme {
            user.theme = Set(Some(theme));
        }
        user.updated_at = Set(Utc::now());

        let updated_user = user.update(&self.db).await?;
        Ok(User::from(updated_user))
    }

    async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStats, AppError> {
        let completed_courses = certification::Entity::find()
            .filter(certification::Column::UserId.eq(user_id))
            .count(&self.db)
            .await?;

        let community_contributions = thread::Entity::find()
            .filter(thread::Column::AuthorId.eq(user_id))
            .count(&self.db)
            .await?;

        Ok(UserStats {
            completed_courses: completed_courses as i64,
            community_contributions: community_contributions as i64,
        })
    }
}
