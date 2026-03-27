use crate::error::AppError;
use crate::models::post::{Post, PostSummary};
use crate::repositories::post::MockPostRepository;
use crate::services::post::PostService;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_list_posts_returns_summaries() {
    let mut mock_repo = MockPostRepository::new();
    let post_id = Uuid::new_v4();
    let author_id = Uuid::new_v4();

    mock_repo
        .expect_list_posts()
        .times(1)
        .returning(move |_| {
            Ok(vec![PostSummary {
                id: post_id,
                title: serde_json::Value::String("First Post".to_string()),
                image_url: None,
                tags: None,
                author_id: Some(author_id),
                published_at: Some(Utc::now()),
            }])
        });

    let service = PostService::new(Arc::new(mock_repo));

    let list = service.list_posts(crate::models::common::PaginationAndFilters::default()).await.expect("Failed to list posts");

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, post_id);
    assert_eq!(list[0].title, serde_json::Value::String("First Post".to_string()));
}

#[tokio::test]
async fn test_get_post_not_found() {
    let mut mock_repo = MockPostRepository::new();
    let post_id = Uuid::new_v4();

    mock_repo
        .expect_get_post()
        .with(mockall::predicate::eq(post_id))
        .times(1)
        .returning(move |_id| Ok(None));

    let service = PostService::new(Arc::new(mock_repo));

    let result = service.get_post(post_id).await;

    assert!(result.is_err(), "Should return error when post not found");
    match result {
        Err(AppError::NotFound(msg)) => assert_eq!(msg, "Post not found"),
        _ => panic!("Expected NotFound error"),
    }
}
