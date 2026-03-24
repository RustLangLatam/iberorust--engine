use crate::error::AppError;
use crate::state::SharedState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GoogleLoginRequest {
    pub google_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn google_login(
    State(state): State<SharedState>,
    Json(payload): Json<GoogleLoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token = state.auth_service.login_with_google(&payload.google_token).await?;
    Ok(Json(AuthResponse { token }))
}

pub async fn guest_login(State(state): State<SharedState>) -> Result<Json<AuthResponse>, AppError> {
    let token = state.auth_service.login_guest().await?;
    Ok(Json(AuthResponse { token }))
}
