use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use crate::models::notification::Notification;
use crate::state::SharedState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
    Json,
};
use futures::stream::{Stream, StreamExt};
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

pub async fn sse_stream(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse_sender.subscribe();
    let stream = BroadcastStream::new(rx);

    let user_id = auth_user.id;

    let event_stream = stream.filter_map(move |msg| async move {
        match msg {
            Ok(notification_message) => {
                // Only send to specific user or all if None
                if let Some(target_user) = notification_message.user_id {
                    if target_user != user_id {
                        return None;
                    }
                }

                let event_json = serde_json::to_string(&notification_message.event).unwrap_or_default();
                Some(Ok(Event::default().data(event_json)))
            }
            Err(_) => None, // Broadcast stream lagged, drop for now
        }
    });

    Sse::new(event_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive-text"),
    )
}

pub async fn list_notifications(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Notification>>, AppError> {
    let notifications = state.notification_service.get_user_notifications(auth_user.id).await?;
    Ok(Json(notifications))
}

pub async fn mark_as_read(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Notification>, AppError> {
    let notification = state.notification_service.read_notification(id, auth_user.id).await?;
    Ok(Json(notification))
}

pub async fn mark_all_as_read(
    State(state): State<SharedState>,
    auth_user: AuthUser,
) -> Result<Json<()>, AppError> {
    state.notification_service.read_all_notifications(auth_user.id).await?;
    Ok(Json(()))
}
