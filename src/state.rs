use crate::repositories::{
    community::CommunityRepository,
    contact::ContactRepository,
    course::CourseRepository,
    notification::NotificationRepository,
    post::PostRepository,
    progress::ProgressRepository,
    reference::ReferenceRepository,
    user::UserRepository,
};
use crate::services::auth::AuthService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationEvent {
    NewReply { thread_id: Uuid, comment_id: Uuid },
    NewThread { thread_id: Uuid },
    AchievementUnlocked { course_id: Uuid },
    SystemAlert { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub user_id: Option<Uuid>, // None means broadcast to all
    pub event: NotificationEvent,
}

pub struct AppState {
    pub sse_sender: broadcast::Sender<NotificationMessage>,

    // Repositories
    pub user_repo: Arc<dyn UserRepository>,
    pub course_repo: Arc<dyn CourseRepository>,
    pub progress_repo: Arc<dyn ProgressRepository>,
    pub community_repo: Arc<dyn CommunityRepository>,
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub post_repo: Arc<dyn PostRepository>,
    pub contact_repo: Arc<dyn ContactRepository>,
    pub reference_repo: Arc<dyn ReferenceRepository>,

    // Services
    pub auth_service: Arc<AuthService>,
    pub course_service: Arc<crate::services::course::CourseService>,
    pub progress_service: Arc<crate::services::progress::ProgressService>,
    pub community_service: Arc<crate::services::community::CommunityService>,
    pub notification_service: Arc<crate::services::notification::NotificationService>,
    pub post_service: Arc<crate::services::post::PostService>,
    pub user_service: Arc<crate::services::user::UserService>,
    pub contact_service: Arc<crate::services::contact::ContactService>,
    pub reference_service: Arc<crate::services::reference::ReferenceService>,
}

pub type SharedState = Arc<AppState>;
