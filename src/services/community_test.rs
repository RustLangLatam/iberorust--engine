use crate::models::community::{CreateThreadRequest, Thread};
use crate::repositories::community::MockCommunityRepository;
use crate::services::community::CommunityService;
use crate::state::NotificationMessage;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::test]
async fn test_create_thread_broadcasts_sse_event() {
    let mut mock_repo = MockCommunityRepository::new();
    let author_id = Uuid::new_v4();
    let expected_thread_id = Uuid::new_v4();

    mock_repo
        .expect_create_thread()
        .times(1)
        .returning(move |_author, req| {
            Ok(Thread {
                id: expected_thread_id,
                author_id,
                title: req.title,
                content: req.content,
                tags: req.tags,
                likes_count: Some(0),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        });

    let (sse_sender, mut rx) = broadcast::channel::<NotificationMessage>(10);
    let service = CommunityService::new(Arc::new(mock_repo), sse_sender);

    let req = CreateThreadRequest {
        title: "Test Thread".to_string(),
        content: "Content".to_string(),
        tags: None,
    };

    let thread = service
        .create_thread(author_id, req)
        .await
        .expect("Failed to create thread");

    assert_eq!(thread.id, expected_thread_id);

    // Verify SSE Event was broadcasted
    let msg = rx.try_recv().expect("Expected SSE event to be broadcasted");
    assert!(msg.user_id.is_none(), "Thread creation should broadcast to all");

    match msg.event {
        crate::state::NotificationEvent::NewThread { thread_id } => {
            assert_eq!(thread_id, expected_thread_id)
        }
        _ => panic!("Expected NewThread event"),
    }
}
