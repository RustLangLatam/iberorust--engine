use crate::error::AppError;
use crate::models::user::CreateUser;
use crate::repositories::user::UserRepository;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub is_guest: bool,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenInfo {
    pub email: String,
    pub name: String,
    pub sub: String, // Google ID
    pub picture: Option<String>,
}

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub fn generate_jwt(&self, user_id: Uuid, email: &str, is_guest: bool, role: &str) -> Result<String, AppError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalServerError(anyhow::anyhow!("Time went backwards")))?
            .as_secs() as usize
            + (60 * 60 * 24 * 7); // 7 days

        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            is_guest,
            role: role.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!("JWT encoding error: {}", e)))
    }

    pub async fn verify_google_token(&self, token: &str) -> Result<CreateUser, AppError> {
        let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", token);
        let client = reqwest::Client::new();

        let response = client.get(&url).send().await.map_err(|_| {
            AppError::AuthError("Failed to verify Google token".to_string())
        })?;

        if !response.status().is_success() {
            return Err(AppError::AuthError("Invalid Google token".to_string()));
        }

        let token_info = response.json::<GoogleTokenInfo>().await.map_err(|_| {
            AppError::AuthError("Failed to parse Google token info".to_string())
        })?;

        Ok(CreateUser {
            email: token_info.email,
            name: token_info.name,
            google_id: Some(token_info.sub),
            avatar_url: token_info.picture,
            password_hash: None,
        })
    }

    pub async fn login_with_google(&self, google_token: &str) -> Result<String, AppError> {
        let google_user = self.verify_google_token(google_token).await?;

        let user = match self.user_repo.find_by_email(&google_user.email).await? {
            Some(user) => user,
            None => self.user_repo.create_user(google_user, false).await?,
        };

        self.generate_jwt(user.id, &user.email, user.is_guest, &user.role)
    }

    pub async fn login_guest(&self) -> Result<String, AppError> {
        let guest_id = uuid::Uuid::new_v4();
        let guest_email = format!("guest_{}@rustedu.com", guest_id);

        let create_req = CreateUser {
            email: guest_email.clone(),
            name: "Guest User".to_string(),
            google_id: None,
            avatar_url: None,
            password_hash: None,
        };

        let user = self.user_repo.create_user(create_req, true).await?;

        self.generate_jwt(user.id, &user.email, user.is_guest, &user.role)
    }

    pub async fn login_standard(&self, email: &str, password: &str) -> Result<String, AppError> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::AuthError("Invalid email or password".to_string()))?;

        if let Some(hash) = &user.password_hash {
            let parsed_hash = PasswordHash::new(hash)
                .map_err(|_| AppError::InternalServerError(anyhow::anyhow!("Invalid password hash format")))?;

            let argon2 = Argon2::default();
            if argon2.verify_password(password.as_bytes(), &parsed_hash).is_err() {
                return Err(AppError::AuthError("Invalid email or password".to_string()));
            }

            self.generate_jwt(user.id, &user.email, user.is_guest, &user.role)
        } else {
            Err(AppError::AuthError("Account requires external login (e.g. Google)".to_string()))
        }
    }
}
