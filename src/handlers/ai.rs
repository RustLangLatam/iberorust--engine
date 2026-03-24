use crate::error::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TtsRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct TtsResponse {
    pub audio_url: String, // Mock response
}

#[derive(Debug, Deserialize)]
pub struct ImageEditRequest {
    pub image_base64: String,
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ImageEditResponse {
    pub edited_image_url: String, // Mock response
}

pub async fn tts_proxy(Json(_payload): Json<TtsRequest>) -> Result<Json<TtsResponse>, AppError> {
    let response = TtsResponse {
        audio_url: "https://rustedu.com/ai/tts/mock.mp3".to_string(),
    };
    Ok(Json(response))
}

pub async fn image_edit_proxy(
    Json(_payload): Json<ImageEditRequest>,
) -> Result<Json<ImageEditResponse>, AppError> {
    let response = ImageEditResponse {
        edited_image_url: "https://rustedu.com/ai/images/mock-edited.png".to_string(),
    };
    Ok(Json(response))
}
