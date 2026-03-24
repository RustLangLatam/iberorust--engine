pub mod error;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod services;
pub mod state;
pub mod entities;

use crate::state::{AppState, SharedState};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{ConnectOptions, Database};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/rustedu".to_string());

    tracing::info!("Connecting to database...");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    tracing::info!("Running migrations...");
    Migrator::up(&db, None).await?;

    // Create the SSE broadcast channel
    let (sse_sender, _rx) = broadcast::channel(100);

    let user_repo = Arc::new(repositories::user::UserRepositoryImpl { db: db.clone() });
    let course_repo = Arc::new(repositories::course::CourseRepositoryImpl { db: db.clone() });
    let progress_repo = Arc::new(repositories::progress::ProgressRepositoryImpl { db: db.clone() });

    let community_repo = Arc::new(repositories::community::CommunityRepositoryImpl { db: db.clone() });
    let notification_repo = Arc::new(repositories::notification::NotificationRepositoryImpl { db: db.clone() });
    let post_repo = Arc::new(repositories::post::PostRepositoryImpl { db: db.clone() });
    let contact_repo = Arc::new(repositories::contact::ContactRepositoryImpl { db: db.clone() });

    let contact_repo = Arc::new(repositories::contact::ContactRepositoryImpl { db: db.clone() });

    let auth_service = Arc::new(services::auth::AuthService::new(user_repo.clone()));
    let user_service = Arc::new(services::user::UserService::new(user_repo.clone()));
    let course_service = Arc::new(services::course::CourseService::new(course_repo.clone()));
    let progress_service = Arc::new(services::progress::ProgressService::new(
        progress_repo.clone(),
        course_repo.clone(),
        sse_sender.clone(),
    ));
    let community_service = Arc::new(services::community::CommunityService::new(
        community_repo.clone(),
        sse_sender.clone(),
    ));
    let notification_service = Arc::new(services::notification::NotificationService::new(notification_repo.clone()));
    let post_service = Arc::new(services::post::PostService::new(post_repo.clone()));
    let contact_service = Arc::new(services::contact::ContactService::new(contact_repo.clone()));

    let state: SharedState = Arc::new(AppState {
        sse_sender,
        user_repo,
        course_repo,
        progress_repo,
        community_repo,
        notification_repo,
        post_repo,
        contact_repo,
        auth_service,
        user_service,
        course_service,
        progress_service,
        community_service,
        notification_service,
        post_service,
        contact_service,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let auth_routes = Router::new()
        .route("/google", post(handlers::auth::google_login))
        .route("/guest", post(handlers::auth::guest_login));

    let user_routes = Router::new()
        .route("/me", get(handlers::user::get_me))
        .route("/me", put(handlers::user::update_me))
        .route("/:id/stats", get(handlers::user::get_stats));

    let course_routes = Router::new()
        .route("/", get(handlers::course::list_courses))
        .route("/:id", get(handlers::course::get_course))
        .route("/:course_id/chapters/:chapter_id", get(handlers::course::get_chapter));

    let progress_routes = Router::new()
        .route("/", get(handlers::progress::get_progress))
        .route("/chapters/:chapter_id", post(handlers::progress::save_chapter_progress));

    let cert_routes = Router::new()
        .route("/", get(handlers::progress::get_certifications))
        .route("/generate/:course_id", post(handlers::progress::generate_cert));

    let community_routes = Router::new()
        .route("/", get(handlers::community::list_threads))
        .route("/", post(handlers::community::create_thread))
        .route("/:id", get(handlers::community::get_thread))
        .route("/:id", put(handlers::community::update_thread))
        .route("/:id", delete(handlers::community::delete_thread))
        .route("/:id/comments", post(handlers::community::add_thread_comment))
        .route("/:id/like", post(handlers::community::toggle_like_thread));

    let comments_routes = Router::new()
        .route("/:id/like", post(handlers::community::toggle_like_comment));

    let notification_routes = Router::new()
        .route("/", get(handlers::notification::list_notifications))
        .route("/:id/read", put(handlers::notification::mark_as_read))
        .route("/read-all", put(handlers::notification::mark_all_as_read));

    let stream_routes = Router::new()
        .route("/notifications", get(handlers::notification::sse_stream));

    let sandbox_routes = Router::new()
        .route("/execute", post(handlers::sandbox::execute_code));

    let post_routes = Router::new()
        .route("/", get(handlers::post::list_posts))
        .route("/:id", get(handlers::post::get_post));

    let ai_routes = Router::new()
        .route("/tts", post(handlers::ai::tts_proxy))
        .route("/image-edit", post(handlers::ai::image_edit_proxy));

    let contact_routes = Router::new()
        .route("/inquiry", post(handlers::contact::submit_inquiry));

    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .nest("/courses", course_routes)
        .nest("/progress", progress_routes)
        .nest("/certifications", cert_routes)
        .nest("/threads", community_routes)
        .nest("/comments", comments_routes)
        .nest("/notifications", notification_routes)
        .nest("/stream", stream_routes)
        .nest("/sandbox", sandbox_routes)
        .nest("/posts", post_routes)
        .nest("/ai", ai_routes)
        .nest("/contact", contact_routes);

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
