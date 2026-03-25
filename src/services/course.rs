use crate::error::AppError;
use crate::models::course::{Chapter, Course, CourseDetails, CreateCourse, ModuleDetails, UpdateCourse};
use crate::repositories::course::CourseRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct CourseService {
    course_repo: Arc<dyn CourseRepository>,
}

impl CourseService {
    pub fn new(course_repo: Arc<dyn CourseRepository>) -> Self {
        Self { course_repo }
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
            title: course.title,
            description: course.description,
            level: course.level,
            image_url: course.image_url,
            modules: module_details,
        })
    }

    pub async fn get_chapter_details(
        &self,
        course_id: Uuid,
        chapter_id: Uuid,
    ) -> Result<Chapter, AppError> {
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
