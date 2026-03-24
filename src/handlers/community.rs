use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::community::{
    Comment, CreateCommentRequest, CreateThreadRequest, Thread, ThreadWithComments, UpdateThreadRequest,
};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn list_threads(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Thread>>, AppError> {
    let threads = state.community_service.list_threads().await?;
    Ok(Json(threads))
}

pub async fn get_thread(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ThreadWithComments>, AppError> {
    let thread = state.community_service.get_thread_with_comments(id).await?;
    Ok(Json(thread))
}

use validator::Validate;

pub async fn create_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateThreadRequest>,
) -> Result<(StatusCode, Json<Thread>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let thread = state.community_service.create_thread(auth_user.id, payload).await?;
    Ok((StatusCode::CREATED, Json(thread)))
}

pub async fn update_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateThreadRequest>,
) -> Result<Json<Thread>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let thread = state.community_service.update_thread(id, auth_user.id, payload).await?;
    Ok(Json(thread))
}

pub async fn delete_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.delete_thread(id, auth_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_thread_comment(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let comment = state.community_service.add_comment(id, auth_user.id, payload).await?;
    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn toggle_like_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.like_thread(id, auth_user.id).await?;
    Ok(StatusCode::OK)
}

pub async fn toggle_like_comment(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.like_comment(id, auth_user.id).await?;
    Ok(StatusCode::OK)
}
