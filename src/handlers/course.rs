use crate::error::AppError;
use crate::models::course::{Chapter, Course, CourseDetails};
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/courses",
    responses(
        (status = 200, description = "List of courses", body = Vec<Course>)
    ),
    tag = "Courses"
)]
pub async fn list_courses(State(state): State<SharedState>) -> Result<Json<Vec<Course>>, AppError> {
    let courses = state.course_service.list_all_courses().await?;
    Ok(Json(courses))
}

#[utoipa::path(
    get,
    path = "/api/v1/courses/{id}",
    params(
        ("id" = Uuid, Path, description = "Course ID")
    ),
    responses(
        (status = 200, description = "Course structure and details", body = CourseDetails)
    ),
    tag = "Courses"
)]
pub async fn get_course(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CourseDetails>, AppError> {
    let details = state.course_service.get_course_structure(id).await?;
    Ok(Json(details))
}

#[utoipa::path(
    get,
    path = "/api/v1/courses/{course_id}/chapters/{chapter_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("chapter_id" = Uuid, Path, description = "Chapter ID")
    ),
    responses(
        (status = 200, description = "Chapter content details", body = Chapter)
    ),
    tag = "Courses"
)]
pub async fn get_chapter(
    State(state): State<SharedState>,
    Path((course_id, chapter_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Chapter>, AppError> {
    let chapter = state.course_service.get_chapter_details(course_id, chapter_id).await?;
    Ok(Json(chapter))
}
