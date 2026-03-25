use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdminConfig {
    pub default_email: String,
    pub default_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub auth: AuthConfig,
    pub admin: AdminConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("APP_").split("_"))
            .extract()
    }
}
