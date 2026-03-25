use crate::entities::{chapter, course, module};
use crate::error::AppError;
use crate::models::course::{Chapter, ChapterSummary, Course, CreateCourse, Module, UpdateCourse};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

use crate::models::common::PaginationAndFilters;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn list_courses(&self, filters: PaginationAndFilters) -> Result<Vec<Course>, AppError>;
    async fn get_course_by_id(&self, course_id: Uuid) -> Result<Option<Course>, AppError>;
    async fn get_modules_for_course(&self, course_id: Uuid) -> Result<Vec<Module>, AppError>;
    async fn get_chapters_summary_for_module(&self, module_id: Uuid) -> Result<Vec<ChapterSummary>, AppError>;
    async fn get_chapter(&self, course_id: Uuid, chapter_id: Uuid) -> Result<Option<Chapter>, AppError>;
    async fn get_chapter_by_id(&self, chapter_id: Uuid) -> Result<Option<Chapter>, AppError>;

    async fn create_course(&self, req: CreateCourse) -> Result<Course, AppError>;
    async fn update_course(&self, course_id: Uuid, req: UpdateCourse) -> Result<Course, AppError>;
    async fn delete_course(&self, course_id: Uuid) -> Result<(), AppError>;

    async fn create_module(&self, course_id: Uuid, req: crate::models::course::CreateModule) -> Result<Module, AppError>;
    async fn update_module(&self, module_id: Uuid, req: crate::models::course::UpdateModule) -> Result<Module, AppError>;
    async fn delete_module(&self, module_id: Uuid) -> Result<(), AppError>;

    async fn create_chapter(&self, module_id: Uuid, req: crate::models::course::CreateChapter) -> Result<Chapter, AppError>;
    async fn update_chapter(&self, chapter_id: Uuid, req: crate::models::course::UpdateChapter) -> Result<Chapter, AppError>;
    async fn delete_chapter(&self, chapter_id: Uuid) -> Result<(), AppError>;
}

pub struct CourseRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<course::Model> for Course {
    fn from(model: course::Model) -> Self {
        Self {
            id: model.id,
            title: serde_json::from_str(&model.title).unwrap_or_else(|_| serde_json::Value::String(model.title.clone())),
            description: model.description.map(|d| serde_json::from_str(&d).unwrap_or_else(|_| serde_json::Value::String(d))),
            level: model.level,
            image_url: model.image_url,
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
            title: serde_json::from_str(&model.title).unwrap_or_else(|_| serde_json::Value::String(model.title.clone())),
            description: model.description.map(|d| serde_json::from_str(&d).unwrap_or_else(|_| serde_json::Value::String(d))),
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
            title: serde_json::from_str(&model.title).unwrap_or_else(|_| serde_json::Value::String(model.title.clone())),
            content: model.content,
            is_quiz: model.is_quiz,
            video_url: model.video_url,
            order: model.order,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn list_courses(&self, filters: PaginationAndFilters) -> Result<Vec<Course>, AppError> {
        let mut query = course::Entity::find().order_by_desc(course::Column::CreatedAt);

        if let Some(search) = filters.search {
            query = query.filter(course::Column::Title.contains(&search));
        }

        let limit = filters.limit.unwrap_or(50);
        let page = filters.page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;

        let courses = query.limit(limit).offset(offset).all(&self.db).await?;
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
                title: serde_json::from_str(&c.title).unwrap_or_else(|_| serde_json::Value::String(c.title.clone())),
                is_quiz: c.is_quiz,
                video_url: c.video_url,
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
            title: Set(req.title.to_string()),
            description: Set(req.description.map(|d| d.to_string())),
            level: Set(req.level),
            image_url: Set(req.image_url),
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
            c.title = Set(title.to_string());
        }
        if let Some(desc) = req.description {
            c.description = Set(Some(desc.to_string()));
        }
        if let Some(level) = req.level {
            c.level = Set(Some(level));
        }
        if let Some(image_url) = req.image_url {
            c.image_url = Set(Some(image_url));
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

    async fn create_module(&self, course_id: Uuid, req: crate::models::course::CreateModule) -> Result<Module, AppError> {
        let new_mod = module::ActiveModel {
            id: Set(Uuid::new_v4()),
            course_id: Set(course_id),
            title: Set(req.title.to_string()),
            description: Set(req.description.map(|d| d.to_string())),
            order: Set(req.order),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = module::Entity::insert(new_mod).exec_with_returning(&self.db).await?;
        Ok(Module::from(result))
    }

    async fn update_module(&self, module_id: Uuid, req: crate::models::course::UpdateModule) -> Result<Module, AppError> {
        let mut m: module::ActiveModel = module::Entity::find_by_id(module_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Module not found".to_string()))?
            .into();

        if let Some(title) = req.title {
            m.title = Set(title.to_string());
        }
        if let Some(desc) = req.description {
            m.description = Set(Some(desc.to_string()));
        }
        if let Some(order) = req.order {
            m.order = Set(order);
        }
        m.updated_at = Set(Utc::now());

        let result = m.update(&self.db).await?;
        Ok(Module::from(result))
    }

    async fn delete_module(&self, module_id: Uuid) -> Result<(), AppError> {
        let result = module::Entity::delete_by_id(module_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Module not found".to_string()));
        }
        Ok(())
    }

    async fn create_chapter(&self, module_id: Uuid, req: crate::models::course::CreateChapter) -> Result<Chapter, AppError> {
        let new_chapter = chapter::ActiveModel {
            id: Set(Uuid::new_v4()),
            module_id: Set(module_id),
            title: Set(req.title.to_string()),
            content: Set(req.content),
            is_quiz: Set(req.is_quiz),
            video_url: Set(req.video_url),
            order: Set(req.order),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let result = chapter::Entity::insert(new_chapter).exec_with_returning(&self.db).await?;
        Ok(Chapter::from(result))
    }

    async fn update_chapter(&self, chapter_id: Uuid, req: crate::models::course::UpdateChapter) -> Result<Chapter, AppError> {
        let mut ch: chapter::ActiveModel = chapter::Entity::find_by_id(chapter_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found".to_string()))?
            .into();

        if let Some(title) = req.title {
            ch.title = Set(title.to_string());
        }
        if let Some(content) = req.content {
            ch.content = Set(content);
        }
        if let Some(is_quiz) = req.is_quiz {
            ch.is_quiz = Set(Some(is_quiz));
        }
        if let Some(video_url) = req.video_url {
            ch.video_url = Set(Some(video_url));
        }
        if let Some(order) = req.order {
            ch.order = Set(order);
        }
        ch.updated_at = Set(Utc::now());

        let result = ch.update(&self.db).await?;
        Ok(Chapter::from(result))
    }

    async fn delete_chapter(&self, chapter_id: Uuid) -> Result<(), AppError> {
        let result = chapter::Entity::delete_by_id(chapter_id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Chapter not found".to_string()));
        }
        Ok(())
    }
}
