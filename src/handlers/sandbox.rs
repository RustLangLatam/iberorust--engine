use crate::error::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExecuteCodeRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct ExecuteCodeResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

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
