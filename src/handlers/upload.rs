use crate::error::AppError;
use crate::middlewares::auth::AuthUser;
use axum::{Json, extract::Multipart};
use serde::Serialize;
use utoipa::ToSchema;
use axum::{routing::post, Router};

pub fn routes() -> Router<crate::state::SharedState> {
    Router::new()
        .route("/image", post(upload_image))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadResponse {
    pub url: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/uploads/image",
    request_body(content_type = "multipart/form-data", description = "Image file to upload"),
    responses(
        (status = 200, description = "Image uploaded successfully", body = UploadResponse)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Uploads"
)]
pub async fn upload_image(
    _auth: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::InternalServerError(anyhow::anyhow!("Failed to read multipart: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" || name == "image" {
            // Read data and possibly perform an S3/Cloud Storage upload
            let _data = field.bytes().await.map_err(|e| {
                AppError::InternalServerError(anyhow::anyhow!("Failed to read file bytes: {}", e))
            })?;
            // Return a mock URL
            return Ok(Json(UploadResponse {
                url: "https://rustedu.com/uploads/mock-image.png".to_string(),
            }));
        }
    }

    Err(AppError::ValidationError("No file uploaded".to_string()))
}
