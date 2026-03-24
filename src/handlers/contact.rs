use crate::error::AppError;
use crate::models::contact::{Inquiry, SubmitInquiryRequest};
use crate::state::SharedState;
use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/contact/inquiry",
    request_body = SubmitInquiryRequest,
    responses(
        (status = 201, description = "Inquiry submitted successfully", body = Inquiry)
    ),
    tag = "Contact"
)]
pub async fn submit_inquiry(
    State(state): State<SharedState>,
    Json(payload): Json<SubmitInquiryRequest>,
) -> Result<(StatusCode, Json<Inquiry>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let inquiry = state.contact_service.submit_inquiry(payload).await?;

    // In a real application, this is where you would trigger an email send

    Ok((StatusCode::CREATED, Json(inquiry)))
}
