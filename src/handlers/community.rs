use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::community::{
    Comment, CreateCommentRequest, CreateThreadRequest, Thread, ThreadWithComments, UpdateComment, UpdateThreadRequest,
};
use crate::state::SharedState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use crate::models::common::PaginationAndFilters;
use uuid::Uuid;
use axum::{routing::{get, post, put, delete}, Router};

pub fn thread_routes() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/", get(list_threads).post(create_thread))
        .route("/{id}", get(get_thread).put(update_thread).delete(delete_thread))
        .route("/{id}/comments", post(add_thread_comment))
        .route("/{id}/like", post(toggle_like_thread))
}

pub fn comment_routes() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/{id}/like", post(toggle_like_comment))
        .route("/{id}", put(update_comment).delete(delete_comment))
}

#[utoipa::path(
    get,
    path = "/api/v1/threads",
    params(
        crate::models::common::PaginationAndFilters
    ),
    responses(
        (status = 200, description = "List of discussion threads", body = Vec<Thread>)
    ),
    tag = "Community"
)]
pub async fn list_threads(
    State(state): State<SharedState>,
    Query(query): Query<PaginationAndFilters>,
) -> Result<Json<Vec<Thread>>, AppError> {
    let threads = state.community_service.list_threads(query).await?;
    Ok(Json(threads))
}

#[utoipa::path(
    get,
    path = "/api/v1/threads/{id}",
    params(
        ("id" = Uuid, Path, description = "Thread ID")
    ),
    responses(
        (status = 200, description = "Thread details and nested comments", body = ThreadWithComments)
    ),
    tag = "Community"
)]
pub async fn get_thread(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ThreadWithComments>, AppError> {
    let thread = state.community_service.get_thread_with_comments(id).await?;
    Ok(Json(thread))
}

use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/threads",
    request_body = CreateThreadRequest,
    responses(
        (status = 201, description = "Thread created successfully", body = Thread)
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateThreadRequest>,
) -> Result<(StatusCode, Json<Thread>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let thread = state.community_service.create_thread(auth_user.id, payload).await?;
    Ok((StatusCode::CREATED, Json(thread)))
}

#[utoipa::path(
    put,
    path = "/api/v1/threads/{id}",
    params(
        ("id" = Uuid, Path, description = "Thread ID")
    ),
    request_body = UpdateThreadRequest,
    responses(
        (status = 200, description = "Thread updated successfully", body = Thread)
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/v1/threads/{id}",
    params(
        ("id" = Uuid, Path, description = "Thread ID")
    ),
    responses(
        (status = 204, description = "Thread deleted successfully")
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.delete_thread(id, auth_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/threads/{id}/comments",
    params(
        ("id" = Uuid, Path, description = "Thread ID")
    ),
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment added successfully", body = Comment)
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
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

#[utoipa::path(
    post,
    path = "/api/v1/threads/{id}/like",
    params(
        ("id" = Uuid, Path, description = "Thread ID")
    ),
    responses(
        (status = 200, description = "Thread like toggled")
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn toggle_like_thread(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.like_thread(id, auth_user.id).await?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/api/v1/comments/{id}/like",
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    responses(
        (status = 200, description = "Comment like toggled")
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn toggle_like_comment(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.like_comment(id, auth_user.id).await?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    put,
    path = "/api/v1/comments/{id}",
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    request_body = UpdateComment,
    responses(
        (status = 200, description = "Comment updated", body = Comment)
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_comment(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateComment>,
) -> Result<Json<Comment>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let comment = state.community_service.update_comment(id, auth_user.id, &auth_user.role, payload).await?;
    Ok(Json(comment))
}

#[utoipa::path(
    delete,
    path = "/api/v1/comments/{id}",
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    responses(
        (status = 204, description = "Comment deleted")
    ),
    tag = "Community",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_comment(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.community_service.delete_comment(id, auth_user.id, &auth_user.role).await?;
    Ok(StatusCode::NO_CONTENT)
}
