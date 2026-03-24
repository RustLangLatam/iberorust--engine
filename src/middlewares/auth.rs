use crate::error::AppError;
use crate::services::auth::Claims;
use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AuthUser {
    pub id: uuid::Uuid,
    pub is_guest: bool,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| h.trim_start_matches("Bearer "))
            .ok_or_else(|| {
                AppError::AuthError("Missing or invalid Authorization header".to_string())
            })?;

        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            AppError::InternalServerError(anyhow::anyhow!("JWT_SECRET not set in environment"))
        })?;

        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

        Ok(AuthUser {
            id: token_data.claims.sub,
            is_guest: token_data.claims.is_guest,
        })
    }
}
