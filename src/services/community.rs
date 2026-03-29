use crate::error::AppError;
use crate::models::community::{
    Comment, CreateCommentRequest, CreateThreadRequest, Thread, ThreadWithComments, UpdateComment, UpdateThreadRequest,
};
use crate::repositories::community::CommunityRepository;
use crate::state::{NotificationEvent, NotificationMessage};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct CommunityService {
    community_repo: Arc<dyn CommunityRepository>,
    sse_sender: broadcast::Sender<NotificationMessage>,
}

impl CommunityService {
    pub fn new(
        community_repo: Arc<dyn CommunityRepository>,
        sse_sender: broadcast::Sender<NotificationMessage>,
    ) -> Self {
        Self {
            community_repo,
            sse_sender,
        }
    }

    pub async fn list_threads(&self, filters: crate::models::common::PaginationAndFilters) -> Result<Vec<Thread>, AppError> {
        self.community_repo.list_threads(filters).await
    }

    pub async fn get_thread_with_comments(
        &self,
        thread_id: Uuid,
    ) -> Result<ThreadWithComments, AppError> {
        let thread = self
            .community_repo
            .get_thread(thread_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?;

        let comments = self.community_repo.get_comments_for_thread(thread_id).await?;

        Ok(ThreadWithComments { thread, comments })
    }

    pub async fn create_thread(
        &self,
        author_id: Uuid,
        req: CreateThreadRequest,
    ) -> Result<Thread, AppError> {
        let thread = self.community_repo.create_thread(author_id, req).await?;

        // Broadcast SSE
        let _ = self.sse_sender.send(NotificationMessage {
            user_id: None, // Broadcast to all
            event: NotificationEvent::NewThread { thread_id: thread.id },
        });

        Ok(thread)
    }

    pub async fn update_thread(
        &self,
        thread_id: Uuid,
        author_id: Uuid,
        req: UpdateThreadRequest,
    ) -> Result<Thread, AppError> {
        let thread = self
            .community_repo
            .get_thread(thread_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?;

        if thread.author_id != author_id {
            return Err(AppError::Forbidden(
                "Only the author can update this thread".to_string(),
            ));
        }

        let updated_thread = self.community_repo.update_thread(thread_id, req).await?;
        Ok(updated_thread)
    }

    pub async fn delete_thread(
        &self,
        thread_id: Uuid,
        author_id: Uuid,
    ) -> Result<(), AppError> {
        let thread = self
            .community_repo
            .get_thread(thread_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?;

        if thread.author_id != author_id {
            return Err(AppError::Forbidden(
                "Only the author can delete this thread".to_string(),
            ));
        }

        self.community_repo.delete_thread(thread_id).await?;
        Ok(())
    }

    pub async fn add_comment(
        &self,
        thread_id: Uuid,
        author_id: Uuid,
        req: CreateCommentRequest,
    ) -> Result<Comment, AppError> {
        let thread = self
            .community_repo
            .get_thread(thread_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Thread not found".to_string()))?;

        let comment = self
            .community_repo
            .add_comment(thread_id, author_id, req.content)
            .await?;

        // Send notification to thread author if it's not their own comment
        if thread.author_id != author_id {
            let _ = self.sse_sender.send(NotificationMessage {
                user_id: Some(thread.author_id),
                event: NotificationEvent::NewReply {
                    thread_id,
                    comment_id: comment.id,
                },
            });
        }

        Ok(comment)
    }

    pub async fn like_thread(
        &self,
        thread_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.community_repo.toggle_thread_like(thread_id, user_id).await
    }

    pub async fn like_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.community_repo.toggle_comment_like(comment_id, user_id).await
    }

    pub async fn update_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        role: &str,
        req: UpdateComment,
    ) -> Result<Comment, AppError> {
        let comment = self.community_repo.get_comment(comment_id).await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

        if comment.author_id != user_id && role != "MODERATOR" && role != "ADMIN" {
            return Err(AppError::Forbidden("Not authorized to edit this comment".to_string()));
        }

        self.community_repo.update_comment(comment_id, req).await
    }

    pub async fn delete_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
        role: &str,
    ) -> Result<(), AppError> {
        let comment = self.community_repo.get_comment(comment_id).await?
            .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

        if comment.author_id != user_id && role != "MODERATOR" && role != "ADMIN" {
            return Err(AppError::Forbidden("Not authorized to delete this comment".to_string()));
        }

        self.community_repo.delete_comment(comment_id).await
    }
}
