use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::user::{UpdateUser, User, UserStats};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "Current user profile", body = User)
    ),
    tag = "Users",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_me(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.get_user_by_id(auth_user.id).await?;

    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/me",
    request_body = UpdateUser,
    responses(
        (status = 200, description = "User profile updated", body = User)
    ),
    tag = "Users",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_me(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = state.user_service.update_user_preferences(auth_user.id, payload).await?;

    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/stats",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User stats", body = UserStats)
    ),
    tag = "Users"
)]
pub async fn get_stats(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserStats>, AppError> {
    let stats = state.user_service.get_user_stats(id).await?;

    Ok(Json(stats))
}
