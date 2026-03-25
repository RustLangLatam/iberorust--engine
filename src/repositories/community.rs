use crate::entities::{comment, comment_like, thread as ThreadEntity, thread_like};
use crate::error::AppError;
use crate::models::community::{Comment, CreateThreadRequest, Thread, UpdateComment, UpdateThreadRequest};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn list_threads(&self) -> Result<Vec<Thread>, AppError>;
    async fn get_thread(&self, thread_id: Uuid) -> Result<Option<Thread>, AppError>;
    async fn create_thread(&self, author_id: Uuid, req: CreateThreadRequest) -> Result<Thread, AppError>;
    async fn update_thread(&self, thread_id: Uuid, req: UpdateThreadRequest) -> Result<Thread, AppError>;
    async fn delete_thread(&self, thread_id: Uuid) -> Result<(), AppError>;
    async fn get_comments_for_thread(&self, thread_id: Uuid) -> Result<Vec<Comment>, AppError>;
    async fn add_comment(&self, thread_id: Uuid, author_id: Uuid, content: String) -> Result<Comment, AppError>;
    async fn toggle_thread_like(&self, thread_id: Uuid, user_id: Uuid) -> Result<(), AppError>;
    async fn toggle_comment_like(&self, comment_id: Uuid, user_id: Uuid) -> Result<(), AppError>;

    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, AppError>;
    async fn update_comment(&self, comment_id: Uuid, req: UpdateComment) -> Result<Comment, AppError>;
    async fn delete_comment(&self, comment_id: Uuid) -> Result<(), AppError>;
}

pub struct CommunityRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<ThreadEntity::Model> for Thread {
    fn from(model: ThreadEntity::Model) -> Self {
        Self {
            id: model.id,
            author_id: model.author_id,
            title: model.title,
            content: model.content,
            tags: model.tags,
            likes_count: model.likes_count,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<comment::Model> for Comment {
    fn from(model: comment::Model) -> Self {
        Self {
            id: model.id,
            thread_id: model.thread_id,
            author_id: model.author_id,
            content: model.content,
            likes_count: model.likes_count,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl CommunityRepository for CommunityRepositoryImpl {
    async fn list_threads(&self) -> Result<Vec<Thread>, AppError> {
        let threads = ThreadEntity::Entity::find()
            .order_by_desc(ThreadEntity::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(threads.into_iter().map(Thread::from).collect())
    }

    async fn get_thread(&self, thread_id: Uuid) -> Result<Option<Thread>, AppError> {
        let t = ThreadEntity::Entity::find_by_id(thread_id)
            .one(&self.db)
            .await?;

        Ok(t.map(Thread::from))
    }

    async fn create_thread(
        &self,
        author_id: Uuid,
        req: CreateThreadRequest,
    ) -> Result<Thread, AppError> {
        let t = ThreadEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            author_id: Set(author_id),
            title: Set(req.title),
            content: Set(req.content),
            tags: Set(req.tags),
            likes_count: Set(Some(0)),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let result = t.insert(&self.db).await?;
        Ok(Thread::from(result))
    }

    async fn update_thread(
        &self,
        thread_id: Uuid,
        req: UpdateThreadRequest,
    ) -> Result<Thread, AppError> {
        let mut t: ThreadEntity::ActiveModel = ThreadEntity::Entity::find_by_id(thread_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?
            .into();

        if let Some(title) = req.title {
            t.title = Set(title);
        }
        if let Some(content) = req.content {
            t.content = Set(content);
        }
        if let Some(tags) = req.tags {
            t.tags = Set(Some(tags));
        }
        t.updated_at = Set(Utc::now());

        let result = t.update(&self.db).await?;
        Ok(Thread::from(result))
    }

    async fn delete_thread(&self, thread_id: Uuid) -> Result<(), AppError> {
        ThreadEntity::Entity::delete_by_id(thread_id)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn get_comments_for_thread(&self, thread_id: Uuid) -> Result<Vec<Comment>, AppError> {
        let comments = comment::Entity::find()
            .filter(comment::Column::ThreadId.eq(thread_id))
            .order_by_asc(comment::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(comments.into_iter().map(Comment::from).collect())
    }

    async fn add_comment(
        &self,
        thread_id: Uuid,
        author_id: Uuid,
        content: String,
    ) -> Result<Comment, AppError> {
        let c = comment::ActiveModel {
            id: Set(Uuid::new_v4()),
            thread_id: Set(thread_id),
            author_id: Set(author_id),
            content: Set(content),
            likes_count: Set(Some(0)),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let result = c.insert(&self.db).await?;
        Ok(Comment::from(result))
    }

    async fn toggle_thread_like(
        &self,
        thread_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let txn = self.db.begin().await?;

        let existing = thread_like::Entity::find()
            .filter(thread_like::Column::ThreadId.eq(thread_id))
            .filter(thread_like::Column::UserId.eq(user_id))
            .one(&txn)
            .await?;

        if let Some(like) = existing {
            // Remove like
            thread_like::Entity::delete_by_id((like.thread_id, like.user_id))
                .exec(&txn)
                .await?;

            // Decrement
            let mut thread: ThreadEntity::ActiveModel = ThreadEntity::Entity::find_by_id(thread_id)
                .one(&txn)
                .await?
                .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?
                .into();

            let count = match thread.likes_count {
                sea_orm::ActiveValue::Set(Some(v)) => v,
                sea_orm::ActiveValue::Unchanged(Some(v)) => v,
                _ => 0,
            };

            thread.likes_count = Set(Some(count - 1));
            thread.update(&txn).await?;
        } else {
            // Add like
            let new_like = thread_like::ActiveModel {
                thread_id: Set(thread_id),
                user_id: Set(user_id),
            };
            thread_like::Entity::insert(new_like).exec(&txn).await?;

            // Increment
            let mut thread: ThreadEntity::ActiveModel = ThreadEntity::Entity::find_by_id(thread_id)
                .one(&txn)
                .await?
                .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?
                .into();

            let count = match thread.likes_count {
                sea_orm::ActiveValue::Set(Some(v)) => v,
                sea_orm::ActiveValue::Unchanged(Some(v)) => v,
                _ => 0,
            };

            thread.likes_count = Set(Some(count + 1));
            thread.update(&txn).await?;
        }

        txn.commit().await?;
        Ok(())
    }

    async fn toggle_comment_like(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let txn = self.db.begin().await?;

        let existing = comment_like::Entity::find()
            .filter(comment_like::Column::CommentId.eq(comment_id))
            .filter(comment_like::Column::UserId.eq(user_id))
            .one(&txn)
            .await?;

        if let Some(like) = existing {
            // Remove
            comment_like::Entity::delete_by_id((like.comment_id, like.user_id))
                .exec(&txn)
                .await?;

            // Decrement
            let mut comment: comment::ActiveModel = comment::Entity::find_by_id(comment_id)
                .one(&txn)
                .await?
                .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?
                .into();

            let count = match comment.likes_count {
                sea_orm::ActiveValue::Set(Some(v)) => v,
                sea_orm::ActiveValue::Unchanged(Some(v)) => v,
                _ => 0,
            };

            comment.likes_count = Set(Some(count - 1));
            comment.update(&txn).await?;
        } else {
            // Add
            let new_like = comment_like::ActiveModel {
                comment_id: Set(comment_id),
                user_id: Set(user_id),
            };
            comment_like::Entity::insert(new_like).exec(&txn).await?;

            // Increment
            let mut comment: comment::ActiveModel = comment::Entity::find_by_id(comment_id)
                .one(&txn)
                .await?
                .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?
                .into();

            let count = match comment.likes_count {
                sea_orm::ActiveValue::Set(Some(v)) => v,
                sea_orm::ActiveValue::Unchanged(Some(v)) => v,
                _ => 0,
            };

            comment.likes_count = Set(Some(count + 1));
            comment.update(&txn).await?;
        }

        txn.commit().await?;
        Ok(())
    }

    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, AppError> {
        let c = comment::Entity::find_by_id(comment_id)
            .one(&self.db)
            .await?;
        Ok(c.map(Comment::from))
    }

    async fn update_comment(&self, comment_id: Uuid, req: UpdateComment) -> Result<Comment, AppError> {
        let mut c: comment::ActiveModel = comment::Entity::find_by_id(comment_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?
            .into();

        c.content = Set(req.content);
        c.updated_at = Set(Utc::now());

        let updated = c.update(&self.db).await?;
        Ok(Comment::from(updated))
    }

    async fn delete_comment(&self, comment_id: Uuid) -> Result<(), AppError> {
        let result = comment::Entity::delete_by_id(comment_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Comment not found".to_string()));
        }
        Ok(())
    }
}
