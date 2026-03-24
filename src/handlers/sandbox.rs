use crate::error::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecuteCodeRequest {
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecuteCodeResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[utoipa::path(
    post,
    path = "/api/v1/sandbox/execute",
    request_body = ExecuteCodeRequest,
    responses(
        (status = 200, description = "Execution results from sandbox", body = ExecuteCodeResponse)
    ),
    tag = "Sandbox"
)]
pub async fn execute_code(
    Json(_payload): Json<ExecuteCodeRequest>,
) -> Result<Json<ExecuteCodeResponse>, AppError> {
    let response = ExecuteCodeResponse {
        stdout: "Hello, world!".to_string(),
        stderr: "".to_string(),
        exit_code: 0,
    };

    Ok(Json(response))
}
