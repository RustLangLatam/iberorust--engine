use crate::error::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TtsRequest {
    pub text: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TtsResponse {
    pub audio_url: String, // Mock response
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImageEditRequest {
    pub image_base64: String,
    pub prompt: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImageEditResponse {
    pub edited_image_url: String, // Mock response
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatResponse {
    pub reply: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/ai/tts",
    request_body = TtsRequest,
    responses(
        (status = 200, description = "TTS audio stream generated", body = TtsResponse)
    ),
    tag = "AI"
)]
pub async fn tts_proxy(Json(_payload): Json<TtsRequest>) -> Result<Json<TtsResponse>, AppError> {
    let response = TtsResponse {
        audio_url: "https://rustedu.com/ai/tts/mock.mp3".to_string(),
    };
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/ai/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "AI Chat Response", body = ChatResponse)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "AI"
)]
pub async fn chat_proxy(
    _user: crate::middlewares::auth::AuthUser,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let reply = format!("Mock AI response to: '{}'", payload.message);
    Ok(Json(ChatResponse { reply }))
}

#[utoipa::path(
    post,
    path = "/api/v1/ai/image-edit",
    request_body = ImageEditRequest,
    responses(
        (status = 200, description = "Image edited successfully", body = ImageEditResponse)
    ),
    tag = "AI"
)]
pub async fn image_edit_proxy(
    Json(_payload): Json<ImageEditRequest>,
) -> Result<Json<ImageEditResponse>, AppError> {
    let response = ImageEditResponse {
        edited_image_url: "https://rustedu.com/ai/images/mock-edited.png".to_string(),
    };
    Ok(Json(response))
}
