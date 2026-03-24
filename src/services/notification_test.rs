use crate::models::notification::Notification;
use crate::repositories::notification::MockNotificationRepository;
use crate::services::notification::NotificationService;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_read_all_notifications_calls_repo() {
    let mut mock_repo = MockNotificationRepository::new();
    let user_id = Uuid::new_v4();

    mock_repo
        .expect_mark_all_as_read()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(()));

    let service = NotificationService::new(Arc::new(mock_repo));

    let result = service.read_all_notifications(user_id).await;

    assert!(result.is_ok(), "Failed to read all notifications");
}

#[tokio::test]
async fn test_get_user_notifications_returns_list() {
    let mut mock_repo = MockNotificationRepository::new();
    let user_id = Uuid::new_v4();
    let notif_id = Uuid::new_v4();

    mock_repo
        .expect_list_notifications()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |u_id| {
            Ok(vec![Notification {
                id: notif_id,
                user_id: u_id,
                r#type: "SYSTEM_ALERT".to_string(),
                title: "Welcome".to_string(),
                content: None,
                is_read: Some(false),
                created_at: Utc::now(),
            }])
        });

    let service = NotificationService::new(Arc::new(mock_repo));

    let list = service
        .get_user_notifications(user_id)
        .await
        .expect("Failed to list notifications");

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, notif_id);
    assert_eq!(list[0].r#type, "SYSTEM_ALERT");
}
