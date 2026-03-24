use crate::entities::{certification, chapter, module, progress as ProgressEntity};
use crate::error::AppError;
use crate::models::progress::{Certification, Progress};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use sea_orm::{
    sea_query::{Expr, OnConflict},
    *,
};
use uuid::Uuid;
use chrono::Utc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProgressRepository: Send + Sync {
    async fn get_user_progress(&self, user_id: Uuid) -> Result<Vec<Progress>, AppError>;
    async fn save_chapter_progress(
        &self,
        user_id: Uuid,
        chapter_id: Uuid,
        completed: bool,
        score: Option<i32>,
    ) -> Result<Progress, AppError>;
    async fn check_course_completion(&self, user_id: Uuid, course_id: Uuid) -> Result<bool, AppError>;
    async fn get_certifications(&self, user_id: Uuid) -> Result<Vec<Certification>, AppError>;
    async fn save_certification(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        pdf_url: Option<String>,
    ) -> Result<Certification, AppError>;
}

pub struct ProgressRepositoryImpl {
    pub db: DatabaseConnection,
}

impl From<ProgressEntity::Model> for Progress {
    fn from(model: ProgressEntity::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            chapter_id: model.chapter_id,
            completed: model.completed,
            score: model.score,
            completed_at: model.completed_at,
        }
    }
}

impl From<certification::Model> for Certification {
    fn from(model: certification::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            course_id: model.course_id,
            issued_at: model.issued_at,
            pdf_url: model.pdf_url,
        }
    }
}

#[async_trait]
impl ProgressRepository for ProgressRepositoryImpl {
    async fn get_user_progress(&self, user_id: Uuid) -> Result<Vec<Progress>, AppError> {
        let progress = ProgressEntity::Entity::find()
            .filter(ProgressEntity::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        Ok(progress.into_iter().map(Progress::from).collect())
    }

    async fn save_chapter_progress(
        &self,
        user_id: Uuid,
        chapter_id: Uuid,
        completed: bool,
        score: Option<i32>,
    ) -> Result<Progress, AppError> {
        let completed_at = if completed { Some(Utc::now()) } else { None };

        let progress = ProgressEntity::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            chapter_id: Set(chapter_id),
            completed: Set(Some(completed)),
            score: Set(score),
            completed_at: Set(completed_at),
        };

        let result = ProgressEntity::Entity::insert(progress)
            .on_conflict(
                OnConflict::columns(vec![
                    ProgressEntity::Column::UserId,
                    ProgressEntity::Column::ChapterId,
                ])
                .update_columns(vec![
                    ProgressEntity::Column::Completed,
                    ProgressEntity::Column::Score,
                    ProgressEntity::Column::CompletedAt,
                ])
                .to_owned(),
            )
            .exec(&self.db)
            .await?;

        // Fetch inserted or updated row
        let saved_progress = ProgressEntity::Entity::find()
            .filter(ProgressEntity::Column::UserId.eq(user_id))
            .filter(ProgressEntity::Column::ChapterId.eq(chapter_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::InternalServerError(anyhow::anyhow!("Failed to retrieve saved progress")))?;

        Ok(Progress::from(saved_progress))
    }

    async fn check_course_completion(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<bool, AppError> {
        // Query total chapters in course
        let total_chapters = chapter::Entity::find()
            .inner_join(module::Entity)
            .filter(module::Column::CourseId.eq(course_id))
            .count(&self.db)
            .await?;

        if total_chapters == 0 {
            return Ok(false);
        }

        // Query completed chapters by user in course
        let completed_chapters = ProgressEntity::Entity::find()
            .inner_join(chapter::Entity)
            .join(
                JoinType::InnerJoin,
                chapter::Relation::Module.def(),
            )
            .filter(module::Column::CourseId.eq(course_id))
            .filter(ProgressEntity::Column::UserId.eq(user_id))
            .filter(ProgressEntity::Column::Completed.eq(true))
            .count(&self.db)
            .await?;

        Ok(total_chapters == completed_chapters)
    }

    async fn get_certifications(&self, user_id: Uuid) -> Result<Vec<Certification>, AppError> {
        let certs = certification::Entity::find()
            .filter(certification::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        Ok(certs.into_iter().map(Certification::from).collect())
    }

    async fn save_certification(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        pdf_url: Option<String>,
    ) -> Result<Certification, AppError> {
        let cert = certification::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            course_id: Set(course_id),
            issued_at: Set(Utc::now()),
            pdf_url: Set(pdf_url),
        };

        let result = certification::Entity::insert(cert)
            .on_conflict(
                OnConflict::columns(vec![
                    certification::Column::UserId,
                    certification::Column::CourseId,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec(&self.db)
            .await?;

        let saved_cert = certification::Entity::find()
            .filter(certification::Column::UserId.eq(user_id))
            .filter(certification::Column::CourseId.eq(course_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::InternalServerError(anyhow::anyhow!("Failed to retrieve saved certification")))?;

        Ok(Certification::from(saved_cert))
    }
}
