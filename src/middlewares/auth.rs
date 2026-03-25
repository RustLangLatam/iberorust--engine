use crate::error::AppError;
use crate::services::auth::Claims;
use crate::state::SharedState;
use axum::{extract::{FromRequestParts, State}, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AuthUser {
    pub id: uuid::Uuid,
    pub is_guest: bool,
    pub role: String,
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
            role: token_data.claims.role,
        })
    }
}

pub struct AdminUser(pub AuthUser);

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if user.role != "ADMIN" {
            return Err(AppError::Forbidden("Requires ADMIN role".to_string()));
        }
        Ok(AdminUser(user))
    }
}

pub struct ModeratorUser(pub AuthUser);

impl<S> FromRequestParts<S> for ModeratorUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if user.role != "ADMIN" && user.role != "MODERATOR" {
            return Err(AppError::Forbidden("Requires MODERATOR or ADMIN role".to_string()));
        }
        Ok(ModeratorUser(user))
    }
}
