use crate::error::AppError;
use crate::models::post::{Post, PostSummary};
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
}
