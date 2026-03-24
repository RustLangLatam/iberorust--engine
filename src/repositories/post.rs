use crate::entities::post as PostEntity;
use crate::error::AppError;
use crate::models::post::{Post, PostSummary};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn list_posts(&self) -> Result<Vec<PostSummary>, AppError>;
    async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, AppError>;
}

pub struct PostRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<PostEntity::Model> for Post {
    fn from(model: PostEntity::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            content: model.content,
            author_id: model.author_id,
            published_at: model.published_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn list_posts(&self) -> Result<Vec<PostSummary>, AppError> {
        let posts = PostEntity::Entity::find()
            .filter(PostEntity::Column::PublishedAt.is_not_null())
            .order_by_desc(PostEntity::Column::PublishedAt)
            .all(&self.db)
            .await?;

        Ok(posts
            .into_iter()
            .map(|p| PostSummary {
                id: p.id,
                title: p.title,
                author_id: p.author_id,
                published_at: p.published_at,
            })
            .collect())
    }

    async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, AppError> {
        let post = PostEntity::Entity::find_by_id(post_id)
            .one(&self.db)
            .await?;

        Ok(post.map(Post::from))
    }
}
