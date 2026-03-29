use crate::error::AppError;
use crate::models::course::{Chapter, Course, CourseDetails, CreateCourse, ModuleDetails, UpdateCourse};
use crate::repositories::course::CourseRepository;
use crate::repositories::progress::ProgressRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct CourseService {
    course_repo: Arc<dyn CourseRepository>,
    progress_repo: Arc<dyn ProgressRepository>,
}

impl CourseService {
    pub fn new(course_repo: Arc<dyn CourseRepository>, progress_repo: Arc<dyn ProgressRepository>) -> Self {
        Self { course_repo, progress_repo }
    }

    pub async fn list_all_courses(&self, filters: crate::models::common::PaginationAndFilters) -> Result<Vec<Course>, AppError> {
        self.course_repo.list_courses(filters).await
    }

    pub async fn get_course_structure(&self, course_id: Uuid) -> Result<CourseDetails, AppError> {
        let course = self
            .course_repo
            .get_course_by_id(course_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Course not found".to_string()))?;

        let modules = self.course_repo.get_modules_for_course(course_id).await?;
        let mut module_details = Vec::new();

        for m in modules {
            let chapters = self
                .course_repo
                .get_chapters_summary_for_module(m.id)
                .await?;
            module_details.push(ModuleDetails {
                id: m.id,
                title: m.title,
                description: m.description,
                order: m.order,
                chapters,
            });
        }

        Ok(CourseDetails {
            id: course.id,
            slug: course.slug,
            title: course.title,
            description: course.description,
            level: course.level,
            image_url: course.image_url,
            tags: course.tags,
            prerequisites: course.prerequisites,
            modules: module_details,
        })
    }

    pub async fn get_chapter_details(
        &self,
        user_id: Option<Uuid>,
        course_id: Uuid,
        chapter_id: Uuid,
    ) -> Result<Chapter, AppError> {
        let course = self
            .course_repo
            .get_course_by_id(course_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Course not found".to_string()))?;

        if let Some(prereqs) = &course.prerequisites {
            if !prereqs.is_empty() {
                if let Some(uid) = user_id {
                    let certs = self.progress_repo.get_certifications(uid).await?;
                    let mut completed_course_ids = std::collections::HashSet::new();
                    for cert in certs {
                        completed_course_ids.insert(cert.course_id);
                    }

                    // We need to fetch all courses to check if the user's completed courses match the prereq slugs
                    let all_courses = self.course_repo.list_courses(crate::models::common::PaginationAndFilters { limit: Some(1000), ..Default::default() }).await?;

                    for prereq_slug in prereqs {
                        if let Some(prereq_course) = all_courses.iter().find(|c| &c.slug == prereq_slug) {
                            if !completed_course_ids.contains(&prereq_course.id) {
                                return Err(AppError::Forbidden(format!("Prerequisite course '{}' must be completed first", prereq_slug)));
                            }
                        } else {
                            // If the prerequisite course doesn't exist, we just deny access for safety or log it. Let's deny.
                            return Err(AppError::Forbidden(format!("Prerequisite course '{}' not found", prereq_slug)));
                        }
                    }
                } else {
                    return Err(AppError::Forbidden("You must be logged in to view a course with prerequisites".to_string()));
                }
            }
        }

        let chapter = self
            .course_repo
            .get_chapter(course_id, chapter_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found for this course".to_string()))?;

        Ok(chapter)
    }

    pub async fn create_course(&self, req: CreateCourse) -> Result<Course, AppError> {
        self.course_repo.create_course(req).await
    }

    pub async fn update_course(&self, course_id: Uuid, req: UpdateCourse) -> Result<Course, AppError> {
        self.course_repo.update_course(course_id, req).await
    }

    pub async fn delete_course(&self, course_id: Uuid) -> Result<(), AppError> {
        self.course_repo.delete_course(course_id).await
    }

    pub async fn create_module(&self, course_id: Uuid, req: crate::models::course::CreateModule) -> Result<crate::models::course::Module, AppError> {
        self.course_repo.create_module(course_id, req).await
    }

    pub async fn update_module(&self, module_id: Uuid, req: crate::models::course::UpdateModule) -> Result<crate::models::course::Module, AppError> {
        self.course_repo.update_module(module_id, req).await
    }

    pub async fn delete_module(&self, module_id: Uuid) -> Result<(), AppError> {
        self.course_repo.delete_module(module_id).await
    }

    pub async fn create_chapter(&self, module_id: Uuid, req: crate::models::course::CreateChapter) -> Result<crate::models::course::Chapter, AppError> {
        self.course_repo.create_chapter(module_id, req).await
    }

    pub async fn update_chapter(&self, chapter_id: Uuid, req: crate::models::course::UpdateChapter) -> Result<crate::models::course::Chapter, AppError> {
        self.course_repo.update_chapter(chapter_id, req).await
    }

    pub async fn delete_chapter(&self, chapter_id: Uuid) -> Result<(), AppError> {
        self.course_repo.delete_chapter(chapter_id).await
    }
}
