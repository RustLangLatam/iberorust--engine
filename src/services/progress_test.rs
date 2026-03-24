use crate::error::AppError;
use crate::models::progress::{Certification, QuizSubmission};
use crate::repositories::course::MockCourseRepository;
use crate::repositories::progress::MockProgressRepository;
use crate::services::progress::ProgressService;
use crate::state::NotificationMessage;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::test]
async fn test_generate_certification_course_completed() {
    let mut mock_progress_repo = MockProgressRepository::new();
    let mock_course_repo = MockCourseRepository::new(); // Not used in this particular test but required for service init

    let user_id = Uuid::new_v4();
    let course_id = Uuid::new_v4();

    mock_progress_repo
        .expect_check_course_completion()
        .with(mockall::predicate::eq(user_id), mockall::predicate::eq(course_id))
        .times(1)
        .returning(|_, _| Ok(true)); // User completed all chapters

    let cert_id = Uuid::new_v4();
    mock_progress_repo
        .expect_save_certification()
        .times(1)
        .returning(move |u_id, c_id, pdf| {
            Ok(Certification {
                id: cert_id,
                user_id: u_id,
                course_id: c_id,
                issued_at: Utc::now(),
                pdf_url: pdf,
            })
        });

    let (sse_sender, mut rx) = broadcast::channel::<NotificationMessage>(10);
    let service = ProgressService::new(
        Arc::new(mock_progress_repo),
        Arc::new(mock_course_repo),
        sse_sender,
    );

    let cert = service
        .generate_certification(user_id, course_id)
        .await
        .expect("Failed to generate cert");

    assert_eq!(cert.user_id, user_id);
    assert!(cert.pdf_url.is_some());

    // Verify SSE achievement broadcast
    let msg = rx.try_recv().expect("Expected SSE event");
    assert_eq!(msg.user_id, Some(user_id), "Notification must be sent specifically to the user");

    match msg.event {
        crate::state::NotificationEvent::AchievementUnlocked { course_id: c_id } => {
            assert_eq!(c_id, course_id)
        }
        _ => panic!("Expected AchievementUnlocked event"),
    }
}

#[tokio::test]
async fn test_generate_certification_forbidden_when_not_completed() {
    let mut mock_progress_repo = MockProgressRepository::new();
    let mock_course_repo = MockCourseRepository::new();

    let user_id = Uuid::new_v4();
    let course_id = Uuid::new_v4();

    mock_progress_repo
        .expect_check_course_completion()
        .with(mockall::predicate::eq(user_id), mockall::predicate::eq(course_id))
        .times(1)
        .returning(|_, _| Ok(false)); // Not completed yet

    let (sse_sender, _rx) = broadcast::channel::<NotificationMessage>(10);
    let service = ProgressService::new(
        Arc::new(mock_progress_repo),
        Arc::new(mock_course_repo),
        sse_sender,
    );

    let result = service.generate_certification(user_id, course_id).await;

    assert!(result.is_err());
    match result {
        Err(AppError::Forbidden(msg)) => assert_eq!(
            msg,
            "Course is not fully completed. Cannot generate certification."
        ),
        _ => panic!("Expected Forbidden error"),
    }
}
