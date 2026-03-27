use crate::error::AppError;
use crate::models::user::{CreateUser, User};
use crate::repositories::user::{MockUserRepository, UserRepository};
use crate::services::auth::AuthService;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_login_guest_creates_new_user_and_returns_jwt() {
    let mut mock_repo = MockUserRepository::new();

    let expected_user_id = Uuid::new_v4();

    mock_repo
        .expect_create_user()
        .withf(|user, is_guest| {
            user.name == "Guest User" && user.email.starts_with("guest_") && *is_guest == true
        })
        .times(1)
        .returning(move |req, is_guest| {
            Ok(User {
                id: expected_user_id,
                email: req.email,
                google_id: None,
                is_guest,
                name: req.name,
                avatar_url: None,
                password_hash: None,
                preferred_language: Some("EN".to_string()),
                theme: Some("system".to_string()),
                role: "USER".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        });

    unsafe {
        std::env::set_var("JWT_SECRET", "test_secret_key");
    }

    let auth_service = AuthService::new(Arc::new(mock_repo), "test_secret_key".to_string());

    let jwt = auth_service.login_guest().await.expect("Failed to login guest");

    assert!(!jwt.is_empty(), "JWT should not be empty");

    unsafe {
        std::env::remove_var("JWT_SECRET");
    }
}

#[tokio::test]
async fn test_verify_google_token_invalid_format() {
    let mock_repo = MockUserRepository::new();
    let auth_service = AuthService::new(Arc::new(mock_repo), "test_secret_key".to_string());

    let result = auth_service.verify_google_token("invalid_token").await;

    assert!(result.is_err(), "Should fail with invalid token");
    match result {
        Err(AppError::AuthError(msg)) => assert_eq!(msg, "Invalid Google token"),
        _ => panic!("Expected AuthError"),
    }
}
