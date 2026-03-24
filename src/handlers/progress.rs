use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::progress::{Certification, Progress, QuizResult, QuizSubmission};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/progress",
    responses(
        (status = 200, description = "Current user progress", body = Vec<Progress>)
    ),
    tag = "Progress",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_progress(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Progress>>, AppError> {
    let progress = state.progress_service.list_user_progress(auth_user.id).await?;
    Ok(Json(progress))
}

#[utoipa::path(
    post,
    path = "/api/v1/progress/chapters/{chapter_id}",
    params(
        ("chapter_id" = Uuid, Path, description = "Chapter ID")
    ),
    request_body(content = Option<QuizSubmission>, description = "Optional quiz submission answers"),
    responses(
        (status = 200, description = "Chapter progress saved. Returns quiz result if applicable.", body = Option<QuizResult>)
    ),
    tag = "Progress",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn save_chapter_progress(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(chapter_id): Path<Uuid>,
    payload: Option<Json<QuizSubmission>>,
) -> Result<Json<Option<QuizResult>>, AppError> {
    let result = state
        .progress_service
        .evaluate_quiz_and_save_progress(auth_user.id, chapter_id, payload.map(|j| j.0))
        .await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/api/v1/certifications",
    responses(
        (status = 200, description = "List of user certifications", body = Vec<Certification>)
    ),
    tag = "Progress",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_certifications(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Certification>>, AppError> {
    let certs = state.progress_service.get_user_certifications(auth_user.id).await?;
    Ok(Json(certs))
}

#[utoipa::path(
    post,
    path = "/api/v1/certifications/generate/{course_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID")
    ),
    responses(
        (status = 200, description = "Certification generated successfully", body = Certification),
        (status = 403, description = "Course is not fully completed")
    ),
    tag = "Progress",
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn generate_cert(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Certification>, AppError> {
    let cert = state.progress_service.generate_certification(auth_user.id, course_id).await?;
    Ok(Json(cert))
}
