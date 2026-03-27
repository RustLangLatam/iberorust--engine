use crate::entities::post as PostEntity;
use crate::error::AppError;
use crate::models::post::{CreatePost, Post, PostSummary, UpdatePost};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use crate::models::common::PaginationAndFilters;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn list_posts(&self, filters: PaginationAndFilters) -> Result<Vec<PostSummary>, AppError>;
    async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, AppError>;
    async fn create_post(&self, author_id: Uuid, req: CreatePost) -> Result<Post, AppError>;
    async fn update_post(&self, id: Uuid, req: UpdatePost) -> Result<Post, AppError>;
    async fn delete_post(&self, id: Uuid) -> Result<(), AppError>;
}

pub struct PostRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<PostEntity::Model> for Post {
    fn from(model: PostEntity::Model) -> Self {
        Self {
            id: model.id,
            title: serde_json::from_str(&model.title).unwrap_or_else(|_| serde_json::Value::String(model.title.clone())),
            content: model.content,
            image_url: model.image_url,
            tags: model.tags,
            author_id: model.author_id,
            published_at: model.published_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn list_posts(&self, filters: PaginationAndFilters) -> Result<Vec<PostSummary>, AppError> {
        let mut query = PostEntity::Entity::find()
            .filter(PostEntity::Column::PublishedAt.is_not_null())
            .order_by_desc(PostEntity::Column::PublishedAt);

        if let Some(search) = filters.search {
            query = query.filter(PostEntity::Column::Title.contains(&search));
        }

        if let Some(tag) = filters.tag {
            query = query.filter(
                sea_query::Expr::val(tag).eq(sea_query::Expr::cust("ANY(tags)"))
            );
        }

        let limit = filters.limit.unwrap_or(50);
        let page = filters.page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;

        let posts = query.limit(limit).offset(offset).all(&self.db).await?;

        Ok(posts
            .into_iter()
            .map(|p| PostSummary {
                id: p.id,
                title: serde_json::from_str(&p.title).unwrap_or_else(|_| serde_json::Value::String(p.title.clone())),
                image_url: p.image_url,
                tags: p.tags,
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

    async fn create_post(&self, author_id: Uuid, req: CreatePost) -> Result<Post, AppError> {
        let new_post = PostEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(req.title.to_string()),
            content: Set(req.content),
            image_url: Set(req.image_url),
            tags: Set(req.tags),
            author_id: Set(Some(author_id)),
            published_at: Set(req.published_at),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = PostEntity::Entity::insert(new_post).exec_with_returning(&self.db).await?;
        Ok(Post::from(result))
    }

    async fn update_post(&self, id: Uuid, req: UpdatePost) -> Result<Post, AppError> {
        let mut p: PostEntity::ActiveModel = PostEntity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?
            .into();

        if let Some(title) = req.title {
            p.title = Set(title.to_string());
        }
        if let Some(content) = req.content {
            p.content = Set(content);
        }
        if let Some(image_url) = req.image_url {
            p.image_url = Set(Some(image_url));
        }
        if let Some(tags) = req.tags {
            p.tags = Set(Some(tags));
        }
        if let Some(published_at) = req.published_at {
            p.published_at = Set(Some(published_at));
        }
        p.updated_at = Set(Utc::now());

        let result = p.update(&self.db).await?;
        Ok(Post::from(result))
    }

    async fn delete_post(&self, id: Uuid) -> Result<(), AppError> {
        let result = PostEntity::Entity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Post not found".to_string()));
        }
        Ok(())
    }
}
