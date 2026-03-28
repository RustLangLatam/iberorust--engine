use crate::error::AppError;
use crate::models::course::{ChapterSummary, Course, Module};
use crate::repositories::course::MockCourseRepository;
use crate::services::course::CourseService;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_get_course_structure_with_modules() {
    let mut mock_repo = MockCourseRepository::new();

    let course_id = Uuid::new_v4();
    let module_id = Uuid::new_v4();

    mock_repo
        .expect_get_course_by_id()
        .with(mockall::predicate::eq(course_id))
        .times(1)
        .returning(move |id| {
            Ok(Some(Course {
                id,
                slug: "rust-for-beginners".to_string(),
                title: serde_json::Value::String("Rust for Beginners".to_string()),
                description: Some(serde_json::Value::String("Learn Rust".to_string())),
                level: Some("Beginner".to_string()),
                image_url: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        });

    mock_repo
        .expect_get_modules_for_course()
        .with(mockall::predicate::eq(course_id))
        .times(1)
        .returning(move |c_id| {
            Ok(vec![Module {
                id: module_id,
                course_id: c_id,
                title: serde_json::Value::String("Intro to Rust".to_string()),
                description: None,
                order: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }])
        });

    mock_repo
        .expect_get_chapters_summary_for_module()
        .with(mockall::predicate::eq(module_id))
        .times(1)
        .returning(move |_m_id| {
            Ok(vec![ChapterSummary {
                id: Uuid::new_v4(),
                title: serde_json::Value::String("Hello World".to_string()),
                is_quiz: Some(false),
                video_url: None,
                order: 1,
            }])
        });

    let service = CourseService::new(Arc::new(mock_repo));

    let structure = service
        .get_course_structure(course_id)
        .await
        .expect("Failed to get course structure");

    assert_eq!(structure.id, course_id);
    assert_eq!(structure.title, serde_json::Value::String("Rust for Beginners".to_string()));
    assert_eq!(structure.modules.len(), 1);
    assert_eq!(structure.modules[0].id, module_id);
    assert_eq!(structure.modules[0].chapters.len(), 1);
}

#[tokio::test]
async fn test_get_course_structure_not_found() {
    let mut mock_repo = MockCourseRepository::new();

    let course_id = Uuid::new_v4();

    mock_repo
        .expect_get_course_by_id()
        .with(mockall::predicate::eq(course_id))
        .times(1)
        .returning(move |_id| Ok(None)); // Not found

    let service = CourseService::new(Arc::new(mock_repo));

    let result = service.get_course_structure(course_id).await;

    assert!(result.is_err(), "Should return an error when not found");
    match result {
        Err(AppError::NotFound(msg)) => assert_eq!(msg, "Course not found"),
        _ => panic!("Expected NotFound error"),
    }
}
