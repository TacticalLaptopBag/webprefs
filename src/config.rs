use std::env;

use crate::error::{AppError, AppResult};

#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    /// Access token lifetime in seconds (default 15 min)
    pub jwt_expiry_secs: i64,
    /// Refresh token lifetime in seconds (default 7 days)
    pub jwt_refresh_expiry_secs: i64,
    pub use_secure_cookies: bool,
    pub host: String,
    pub port: u16,

    pub database_url: String,
    pub init_user_name: Option<String>,
    pub init_user_pass: Option<String>,

    pub app_serve_path: Option<String>,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            jwt_secret: env::var("JWT_SECRET").map_err(|_| AppError::EnvVarUnset("JWT_SECRET"))?,
            jwt_expiry_secs: env::var("JWT_EXPIRY_SECONDS")
                .unwrap_or("900".into())
                .parse()
                .map_err(|_| AppError::EnvVarNotANumber("JWT_EXPIRY_SECONDS"))?,
            jwt_refresh_expiry_secs: env::var("JWT_REFRESH_EXPIRY_SECONDS")
                .unwrap_or("604800".into())
                .parse()
                .map_err(|_| AppError::EnvVarNotANumber("JWT_REFRESH_EXPIRY_SECONDS"))?,
            use_secure_cookies: env::var("USE_SECURE_COOKIES")
                .unwrap_or("true".into())
                .to_lowercase()
                .parse()
                .map_err(|_| AppError::EnvVarNotABoolean("USE_SECURE_COOKIES"))?,
            host: env::var("HOST").unwrap_or("127.0.0.1".into()),
            port: env::var("PORT")
                .unwrap_or("8080".into())
                .parse()
                .map_err(|_| AppError::EnvVarNotANumber("PORT"))?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| AppError::EnvVarUnset("DATABASE_URL"))?,
            init_user_name: env::var("INIT_USER_NAME").ok(),
            init_user_pass: env::var("INIT_USER_PASS").ok(),
            app_serve_path: env::var("APP_SERVE_PATH").ok(),
        })
    }
}
