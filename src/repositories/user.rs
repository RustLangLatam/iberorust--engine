use crate::entities::{certification, course, post, thread, user as UserEntity};
use crate::error::AppError;
use crate::models::user::{AdminStats, CreateUser, UpdateUser, User, UserRoleUpdate, UserStats};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use crate::models::common::PaginationAndFilters;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create_user(&self, user: CreateUser, is_guest: bool) -> Result<User, AppError>;
    async fn update_user(&self, id: Uuid, update: UpdateUser) -> Result<User, AppError>;
    async fn get_user_stats(&self, user_id: Uuid) -> Result<UserStats, AppError>;

    async fn list_users(&self, filters: PaginationAndFilters) -> Result<Vec<User>, AppError>;
    async fn update_user_role(&self, id: Uuid, update: UserRoleUpdate) -> Result<User, AppError>;
    async fn delete_user(&self, id: Uuid) -> Result<(), AppError>;

    async fn get_admin_stats(&self) -> Result<AdminStats, AppError>;
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
            password_hash: model.password_hash,
            preferred_language: model.preferred_language,
            theme: model.theme,
            role: model.role,
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
            password_hash: Set(user.password_hash),
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

    async fn list_users(&self, filters: PaginationAndFilters) -> Result<Vec<User>, AppError> {
        let mut query = UserEntity::Entity::find().order_by_desc(UserEntity::Column::CreatedAt);

        if let Some(search) = filters.search {
            query = query.filter(UserEntity::Column::Name.contains(&search));
        }
        if let Some(role) = filters.role {
            query = query.filter(UserEntity::Column::Role.eq(role));
        }

        let limit = filters.limit.unwrap_or(50);
        let page = filters.page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;

        let users = query.limit(limit).offset(offset).all(&self.db).await?;
        Ok(users.into_iter().map(User::from).collect())
    }

    async fn update_user_role(&self, id: Uuid, update: UserRoleUpdate) -> Result<User, AppError> {
        let mut user: UserEntity::ActiveModel = UserEntity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
            .into();

        user.role = Set(update.role);
        user.updated_at = Set(Utc::now());

        let updated_user = user.update(&self.db).await?;
        Ok(User::from(updated_user))
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let result = UserEntity::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }
        Ok(())
    }

    async fn get_admin_stats(&self) -> Result<AdminStats, AppError> {
        let total_users = UserEntity::Entity::find().count(&self.db).await?;

        let active_students = UserEntity::Entity::find()
            .filter(UserEntity::Column::IsGuest.eq(false))
            .count(&self.db)
            .await?;

        let total_courses = course::Entity::find().count(&self.db).await?;
        let total_posts = post::Entity::find().count(&self.db).await?;
        let total_threads = thread::Entity::find().count(&self.db).await?;

        Ok(AdminStats {
            total_users: total_users as i64,
            active_students: active_students as i64,
            total_courses: total_courses as i64,
            total_posts: total_posts as i64,
            total_threads: total_threads as i64,
        })
    }
}
