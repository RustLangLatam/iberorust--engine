use crate::error::AppError;
use crate::models::reference::{CreateReference, Reference, UpdateReference};
use crate::middlewares::auth::AdminUser;
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/references",
    responses(
        (status = 200, description = "List of references", body = Vec<Reference>)
    ),
    tag = "References"
)]
pub async fn list_references(State(state): State<SharedState>) -> Result<Json<Vec<Reference>>, AppError> {
    let refs = state.reference_service.list_references().await?;
    Ok(Json(refs))
}

#[utoipa::path(
    post,
    path = "/api/v1/references",
    request_body = CreateReference,
    responses(
        (status = 200, description = "Reference created", body = Reference)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "References"
)]
pub async fn create_reference(
    State(state): State<SharedState>,
    _admin: AdminUser,
    Json(payload): Json<CreateReference>,
) -> Result<Json<Reference>, AppError> {
    let reference = state.reference_service.create_reference(payload).await?;
    Ok(Json(reference))
}

#[utoipa::path(
    put,
    path = "/api/v1/references/{id}",
    params(
        ("id" = Uuid, Path, description = "Reference ID")
    ),
    request_body = UpdateReference,
    responses(
        (status = 200, description = "Reference updated", body = Reference)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "References"
)]
pub async fn update_reference(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: AdminUser,
    Json(payload): Json<UpdateReference>,
) -> Result<Json<Reference>, AppError> {
    let reference = state.reference_service.update_reference(id, payload).await?;
    Ok(Json(reference))
}

#[utoipa::path(
    delete,
    path = "/api/v1/references/{id}",
    params(
        ("id" = Uuid, Path, description = "Reference ID")
    ),
    responses(
        (status = 204, description = "Reference deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "References"
)]
pub async fn delete_reference(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.reference_service.delete_reference(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
