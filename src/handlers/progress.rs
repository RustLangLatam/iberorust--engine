use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::progress::{Certification, Progress, QuizResult, QuizSubmission};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn get_progress(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Progress>>, AppError> {
    let progress = state.progress_service.list_user_progress(auth_user.id).await?;
    Ok(Json(progress))
}

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

pub async fn get_certifications(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Certification>>, AppError> {
    let certs = state.progress_service.get_user_certifications(auth_user.id).await?;
    Ok(Json(certs))
}

pub async fn generate_cert(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Certification>, AppError> {
    let cert = state.progress_service.generate_certification(auth_user.id, course_id).await?;
    Ok(Json(cert))
}
