use crate::error::AppError;
use crate::models::course::{Chapter, Course, CourseDetails, CreateCourse, UpdateCourse};
use crate::state::SharedState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use crate::models::common::PaginationAndFilters;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/courses",
    params(
        crate::models::common::PaginationAndFilters
    ),
    responses(
        (status = 200, description = "List of courses", body = Vec<Course>)
    ),
    tag = "Courses"
)]
pub async fn list_courses(
    State(state): State<SharedState>,
    Query(query): Query<PaginationAndFilters>,
) -> Result<Json<Vec<Course>>, AppError> {
    let courses = state.course_service.list_all_courses(query).await?;
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

#[utoipa::path(
    post,
    path = "/api/v1/courses",
    request_body = CreateCourse,
    responses(
        (status = 200, description = "Course created", body = Course)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn create_course(
    State(state): State<SharedState>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::CreateCourse>,
) -> Result<Json<Course>, AppError> {
    let course = state.course_service.create_course(payload).await?;
    Ok(Json(course))
}

#[utoipa::path(
    put,
    path = "/api/v1/courses/{id}",
    params(
        ("id" = Uuid, Path, description = "Course ID")
    ),
    request_body = UpdateCourse,
    responses(
        (status = 200, description = "Course updated", body = Course)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn update_course(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::UpdateCourse>,
) -> Result<Json<Course>, AppError> {
    let course = state.course_service.update_course(id, payload).await?;
    Ok(Json(course))
}

#[utoipa::path(
    delete,
    path = "/api/v1/courses/{id}",
    params(
        ("id" = Uuid, Path, description = "Course ID")
    ),
    responses(
        (status = 204, description = "Course deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn delete_course(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    _admin: crate::middlewares::auth::AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.course_service.delete_course(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/courses/{course_id}/modules",
    params(
        ("course_id" = Uuid, Path, description = "Course ID")
    ),
    request_body = crate::models::course::CreateModule,
    responses(
        (status = 200, description = "Module created", body = crate::models::course::Module)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn create_module(
    State(state): State<SharedState>,
    Path(course_id): Path<Uuid>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::CreateModule>,
) -> Result<Json<crate::models::course::Module>, AppError> {
    let module = state.course_service.create_module(course_id, payload).await?;
    Ok(Json(module))
}

#[utoipa::path(
    put,
    path = "/api/v1/courses/{course_id}/modules/{module_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("module_id" = Uuid, Path, description = "Module ID")
    ),
    request_body = crate::models::course::UpdateModule,
    responses(
        (status = 200, description = "Module updated", body = crate::models::course::Module)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn update_module(
    State(state): State<SharedState>,
    Path((_course_id, module_id)): Path<(Uuid, Uuid)>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::UpdateModule>,
) -> Result<Json<crate::models::course::Module>, AppError> {
    let module = state.course_service.update_module(module_id, payload).await?;
    Ok(Json(module))
}

#[utoipa::path(
    delete,
    path = "/api/v1/courses/{course_id}/modules/{module_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("module_id" = Uuid, Path, description = "Module ID")
    ),
    responses(
        (status = 204, description = "Module deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn delete_module(
    State(state): State<SharedState>,
    Path((_course_id, module_id)): Path<(Uuid, Uuid)>,
    _admin: crate::middlewares::auth::AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.course_service.delete_module(module_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/courses/{course_id}/modules/{module_id}/chapters",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("module_id" = Uuid, Path, description = "Module ID")
    ),
    request_body = crate::models::course::CreateChapter,
    responses(
        (status = 200, description = "Chapter created", body = crate::models::course::Chapter)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn create_chapter(
    State(state): State<SharedState>,
    Path((_course_id, module_id)): Path<(Uuid, Uuid)>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::CreateChapter>,
) -> Result<Json<crate::models::course::Chapter>, AppError> {
    let chapter = state.course_service.create_chapter(module_id, payload).await?;
    Ok(Json(chapter))
}

#[utoipa::path(
    put,
    path = "/api/v1/courses/{course_id}/modules/{module_id}/chapters/{chapter_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("module_id" = Uuid, Path, description = "Module ID"),
        ("chapter_id" = Uuid, Path, description = "Chapter ID")
    ),
    request_body = crate::models::course::UpdateChapter,
    responses(
        (status = 200, description = "Chapter updated", body = crate::models::course::Chapter)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn update_chapter(
    State(state): State<SharedState>,
    Path((_course_id, _module_id, chapter_id)): Path<(Uuid, Uuid, Uuid)>,
    _admin: crate::middlewares::auth::AdminUser,
    Json(payload): Json<crate::models::course::UpdateChapter>,
) -> Result<Json<crate::models::course::Chapter>, AppError> {
    let chapter = state.course_service.update_chapter(chapter_id, payload).await?;
    Ok(Json(chapter))
}

#[utoipa::path(
    delete,
    path = "/api/v1/courses/{course_id}/modules/{module_id}/chapters/{chapter_id}",
    params(
        ("course_id" = Uuid, Path, description = "Course ID"),
        ("module_id" = Uuid, Path, description = "Module ID"),
        ("chapter_id" = Uuid, Path, description = "Chapter ID")
    ),
    responses(
        (status = 204, description = "Chapter deleted")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Courses"
)]
pub async fn delete_chapter(
    State(state): State<SharedState>,
    Path((_course_id, _module_id, chapter_id)): Path<(Uuid, Uuid, Uuid)>,
    _admin: crate::middlewares::auth::AdminUser,
) -> Result<axum::http::StatusCode, AppError> {
    state.course_service.delete_chapter(chapter_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
