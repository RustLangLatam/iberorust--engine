use crate::error::AppError;
use crate::models::post::{CreatePost, Post, PostSummary, UpdatePost};
use crate::repositories::post::PostRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostService {
    post_repo: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(post_repo: Arc<dyn PostRepository>) -> Self {
        Self { post_repo }
    }

    pub async fn list_posts(&self) -> Result<Vec<PostSummary>, AppError> {
        self.post_repo.list_posts().await
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post, AppError> {
        let post = self
            .post_repo
            .get_post(post_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

        Ok(post)
    }

    pub async fn create_post(&self, author_id: Uuid, req: CreatePost) -> Result<Post, AppError> {
        self.post_repo.create_post(author_id, req).await
    }

    pub async fn update_post(&self, id: Uuid, req: UpdatePost) -> Result<Post, AppError> {
        self.post_repo.update_post(id, req).await
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<(), AppError> {
        self.post_repo.delete_post(id).await
    }
}
