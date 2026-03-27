pub mod auth;
pub mod user;
pub mod course;
pub mod progress;
pub mod community;
pub mod notification;
pub mod sandbox;
pub mod post;
pub mod ai;
pub mod contact;
pub mod reference;
pub mod upload;

pub fn api_router() -> axum::Router<crate::state::SharedState> {
    axum::Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", user::routes())
        .nest("/courses", course::routes())
        .nest("/progress", progress::routes())
        .nest("/certifications", progress::cert_routes())
        .nest("/threads", community::thread_routes())
        .nest("/comments", community::comment_routes())
        .nest("/notifications", notification::routes())
        .nest("/stream", notification::stream_routes())
        .nest("/sandbox", sandbox::routes())
        .nest("/posts", post::routes())
        .nest("/ai", ai::routes())
        .nest("/contact", contact::routes())
        .nest("/references", reference::routes())
        .nest("/admin", user::admin_routes())
        .nest("/uploads", upload::routes())
}