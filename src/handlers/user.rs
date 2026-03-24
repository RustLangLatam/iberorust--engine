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

pub async fn get_me(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.get_user_by_id(auth_user.id).await?;

    Ok(Json(user))
}

pub async fn update_me(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = state.user_service.update_user_preferences(auth_user.id, payload).await?;

    Ok(Json(user))
}

pub async fn get_stats(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserStats>, AppError> {
    let stats = state.user_service.get_user_stats(id).await?;

    Ok(Json(stats))
}
