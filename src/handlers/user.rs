use crate::error::AppError;
use crate::middlewares::auth::{AdminUser, AuthUser};
use crate::models::user::{AdminStats, UpdateUser, User, UserRoleUpdate, UserStats};
use crate::state::SharedState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use crate::models::common::PaginationAndFilters;
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

#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(
        crate::models::common::PaginationAndFilters
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Users"
)]
pub async fn list_users(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Query(query): Query<PaginationAndFilters>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = state.user_service.list_users(query).await?;
    Ok(Json(users))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/role",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UserRoleUpdate,
    responses(
        (status = 200, description = "User role updated", body = User)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Users"
)]
pub async fn update_user_role(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: AdminUser,
    Json(payload): Json<UserRoleUpdate>,
) -> Result<Json<User>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let user = state.user_service.update_user_role(id, payload).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Users"
)]
pub async fn delete_user(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.user_service.delete_user(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/stats",
    responses(
        (status = 200, description = "Global Admin Stats", body = AdminStats)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Users"
)]
pub async fn get_admin_stats(
    State(state): State<SharedState>,
    _admin: AdminUser,
) -> Result<Json<AdminStats>, AppError> {
    let stats = state.user_service.get_admin_stats().await?;
    Ok(Json(stats))
}
