use crate::entities::{chapter, course, module};
use crate::error::AppError;
use crate::models::course::{Chapter, ChapterSummary, Course, CreateCourse, Module, UpdateCourse};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn list_courses(&self) -> Result<Vec<Course>, AppError>;
    async fn get_course_by_id(&self, course_id: Uuid) -> Result<Option<Course>, AppError>;
    async fn get_modules_for_course(&self, course_id: Uuid) -> Result<Vec<Module>, AppError>;
    async fn get_chapters_summary_for_module(&self, module_id: Uuid) -> Result<Vec<ChapterSummary>, AppError>;
    async fn get_chapter(&self, course_id: Uuid, chapter_id: Uuid) -> Result<Option<Chapter>, AppError>;
    async fn get_chapter_by_id(&self, chapter_id: Uuid) -> Result<Option<Chapter>, AppError>;

    async fn create_course(&self, req: CreateCourse) -> Result<Course, AppError>;
    async fn update_course(&self, course_id: Uuid, req: UpdateCourse) -> Result<Course, AppError>;
    async fn delete_course(&self, course_id: Uuid) -> Result<(), AppError>;
}

pub struct CourseRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<course::Model> for Course {
    fn from(model: course::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            level: model.level,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<module::Model> for Module {
    fn from(model: module::Model) -> Self {
        Self {
            id: model.id,
            course_id: model.course_id,
            title: model.title,
            description: model.description,
            order: model.order,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<chapter::Model> for Chapter {
    fn from(model: chapter::Model) -> Self {
        Self {
            id: model.id,
            module_id: model.module_id,
            title: model.title,
            content: model.content,
            is_quiz: model.is_quiz,
            order: model.order,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn list_courses(&self) -> Result<Vec<Course>, AppError> {
        let courses = course::Entity::find()
            .order_by_desc(course::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok(courses.into_iter().map(Course::from).collect())
    }

    async fn get_course_by_id(&self, course_id: Uuid) -> Result<Option<Course>, AppError> {
        let c = course::Entity::find_by_id(course_id)
            .one(&self.db)
            .await?;

        Ok(c.map(Course::from))
    }

    async fn get_modules_for_course(&self, course_id: Uuid) -> Result<Vec<Module>, AppError> {
        let modules = module::Entity::find()
            .filter(module::Column::CourseId.eq(course_id))
            .order_by_asc(module::Column::Order)
            .all(&self.db)
            .await?;

        Ok(modules.into_iter().map(Module::from).collect())
    }

    async fn get_chapters_summary_for_module(
        &self,
        module_id: Uuid,
    ) -> Result<Vec<ChapterSummary>, AppError> {
        let chapters = chapter::Entity::find()
            .filter(chapter::Column::ModuleId.eq(module_id))
            .order_by_asc(chapter::Column::Order)
            .all(&self.db)
            .await?;

        Ok(chapters
            .into_iter()
            .map(|c| ChapterSummary {
                id: c.id,
                title: c.title,
                is_quiz: c.is_quiz,
                order: c.order,
            })
            .collect())
    }

    async fn get_chapter(
        &self,
        course_id: Uuid,
        chapter_id: Uuid,
    ) -> Result<Option<Chapter>, AppError> {
        // Find the chapter and its module
        let result = chapter::Entity::find_by_id(chapter_id)
            .find_also_related(module::Entity)
            .one(&self.db)
            .await?;

        if let Some((ch, Some(m))) = result {
            if m.course_id == course_id {
                return Ok(Some(Chapter::from(ch)));
            }
        }

        Ok(None)
    }

    async fn get_chapter_by_id(&self, chapter_id: Uuid) -> Result<Option<Chapter>, AppError> {
        let ch = chapter::Entity::find_by_id(chapter_id)
            .one(&self.db)
            .await?;

        Ok(ch.map(Chapter::from))
    }

    async fn create_course(&self, req: CreateCourse) -> Result<Course, AppError> {
        let new_course = course::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(req.title),
            description: Set(req.description),
            level: Set(req.level),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = course::Entity::insert(new_course).exec_with_returning(&self.db).await?;
        Ok(Course::from(result))
    }

    async fn update_course(&self, course_id: Uuid, req: UpdateCourse) -> Result<Course, AppError> {
        let mut c: course::ActiveModel = course::Entity::find_by_id(course_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Course not found".to_string()))?
            .into();

        if let Some(title) = req.title {
            c.title = Set(title);
        }
        if let Some(desc) = req.description {
            c.description = Set(Some(desc));
        }
        if let Some(level) = req.level {
            c.level = Set(Some(level));
        }
        c.updated_at = Set(Utc::now());

        let result = c.update(&self.db).await?;
        Ok(Course::from(result))
    }

    async fn delete_course(&self, course_id: Uuid) -> Result<(), AppError> {
        let result = course::Entity::delete_by_id(course_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Course not found".to_string()));
        }
        Ok(())
    }
}
