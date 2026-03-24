use crate::error::AppError;
use crate::models::post::{Post, PostSummary};
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
