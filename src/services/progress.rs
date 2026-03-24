use crate::error::AppError;
use crate::models::progress::{Certification, Progress, QuizResult, QuizSubmission};
use crate::repositories::course::CourseRepository;
use crate::repositories::progress::ProgressRepository;
use crate::state::{NotificationEvent, NotificationMessage};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct ProgressService {
    progress_repo: Arc<dyn ProgressRepository>,
    course_repo: Arc<dyn CourseRepository>,
    sse_sender: broadcast::Sender<NotificationMessage>,
}

impl ProgressService {
    pub fn new(
        progress_repo: Arc<dyn ProgressRepository>,
        course_repo: Arc<dyn CourseRepository>,
        sse_sender: broadcast::Sender<NotificationMessage>,
    ) -> Self {
        Self {
            progress_repo,
            course_repo,
            sse_sender,
        }
    }

    pub async fn list_user_progress(&self, user_id: Uuid) -> Result<Vec<Progress>, AppError> {
        self.progress_repo.get_user_progress(user_id).await
    }

    pub async fn evaluate_quiz_and_save_progress(
        &self,
        user_id: Uuid,
        chapter_id: Uuid,
        submission: Option<QuizSubmission>,
    ) -> Result<Option<QuizResult>, AppError> {
        let chapter = self
            .course_repo
            .get_chapter_by_id(chapter_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Chapter not found".to_string()))?;

        let is_quiz = chapter.is_quiz.unwrap_or(false);

        if is_quiz {
            let _sub = submission.ok_or_else(|| {
                AppError::ValidationError("Quiz answers are required for this chapter".to_string())
            })?;
            // Mocking evaluation logic: score = 80 out of 100
            let score = 80;
            let passed = score >= 70;

            self.progress_repo
                .save_chapter_progress(user_id, chapter_id, passed, Some(score))
                .await?;

            Ok(Some(QuizResult { score, passed }))
        } else {
            // Just reading a chapter, mark as completed
            self.progress_repo
                .save_chapter_progress(user_id, chapter_id, true, None)
                .await?;
            Ok(None)
        }
    }

    pub async fn get_user_certifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Certification>, AppError> {
        self.progress_repo.get_certifications(user_id).await
    }

    pub async fn generate_certification(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<Certification, AppError> {
        let is_completed = self
            .progress_repo
            .check_course_completion(user_id, course_id)
            .await?;

        if !is_completed {
            return Err(AppError::Forbidden(
                "Course is not fully completed. Cannot generate certification.".to_string(),
            ));
        }

        // Generate dummy PDF URL
        let pdf_url = format!("https://rustedu.com/certifications/{}_{}.pdf", user_id, course_id);
        let cert = self
            .progress_repo
            .save_certification(user_id, course_id, Some(pdf_url))
            .await?;

        // Broadcast SSE Achievement Unlocked event
        let _ = self.sse_sender.send(NotificationMessage {
            user_id: Some(user_id),
            event: NotificationEvent::AchievementUnlocked { course_id },
        });

        Ok(cert)
    }
}
