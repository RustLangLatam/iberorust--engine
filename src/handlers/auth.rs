use crate::error::AppError;
use crate::state::SharedState;
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn routes() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/google", post(google_login))
        .route("/guest", post(guest_login))
}
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct GoogleLoginRequest {
    #[validate(length(min = 1, message = "Google token is required"))]
    pub google_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/google",
    request_body = GoogleLoginRequest,
    responses(
        (status = 200, description = "Successful login", body = AuthResponse)
    ),
    tag = "Auth"
)]
pub async fn google_login(
    State(state): State<SharedState>,
    Json(payload): Json<GoogleLoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token = state.auth_service.login_with_google(&payload.google_token).await?;
    Ok(Json(AuthResponse { token }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/guest",
    responses(
        (status = 200, description = "Successful guest login", body = AuthResponse)
    ),
    tag = "Auth"
)]
pub async fn guest_login(State(state): State<SharedState>) -> Result<Json<AuthResponse>, AppError> {
    let token = state.auth_service.login_guest().await?;
    Ok(Json(AuthResponse { token }))
}
