use crate::error::AppError;
use crate::services::auth::Claims;
use crate::state::SharedState;
use axum::{extract::{FromRequestParts, State}, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AuthUser {
    pub id: uuid::Uuid,
    pub is_guest: bool,
}

impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &SharedState) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| h.trim_start_matches("Bearer "))
            .ok_or_else(|| {
                AppError::AuthError("Missing or invalid Authorization header".to_string())
            })?;

        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(state.config.auth.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

        Ok(AuthUser {
            id: token_data.claims.sub,
            is_guest: token_data.claims.is_guest,
        })
    }
}
