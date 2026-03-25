use crate::error::AppError;
use crate::models::post::{CreatePost, Post, PostSummary, UpdatePost};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/posts",
    responses(
        (status = 200, description = "List of posts summary", body = Vec<PostSummary>)
    ),
    tag = "Posts"
)]
pub async fn list_posts(State(state): State<SharedState>) -> Result<Json<Vec<PostSummary>>, AppError> {
    let posts = state.post_service.list_posts().await?;
    Ok(Json(posts))
}

#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = Uuid, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Detailed post content", body = Post)
    ),
    tag = "Posts"
)]
pub async fn get_post(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Post>, AppError> {
    let post = state.post_service.get_post(id).await?;

    Ok(Json(post))
}

#[utoipa::path(
    post,
    path = "/api/v1/posts",
    request_body = CreatePost,
    responses(
        (status = 200, description = "Post created", body = Post)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Posts"
)]
pub async fn create_post(
    State(state): State<SharedState>,
    admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::post::CreatePost>,
) -> Result<Json<Post>, AppError> {
    let post = state.post_service.create_post(admin.0.id, payload).await?;
    Ok(Json(post))
}

#[utoipa::path(
    put,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = Uuid, Path, description = "Post ID")
    ),
    request_body = UpdatePost,
    responses(
        (status = 200, description = "Post updated", body = Post)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Posts"
)]
pub async fn update_post(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::post::UpdatePost>,
) -> Result<Json<Post>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;
    Ok(Json(post))
}

#[utoipa::path(
    delete,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = Uuid, Path, description = "Post ID")
    ),
    responses(
        (status = 204, description = "Post deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Posts"
)]
pub async fn delete_post(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: crate::middlewares::auth::AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.post_service.delete_post(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
