use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Config errors
    #[error("Environment variable {0} must be set")]
    EnvVarUnset(&'static str),
    #[error("Environment variable {0} must be a number")]
    EnvVarNotANumber(&'static str),
    #[error("Environment variable {0} must be a boolean")]
    EnvVarNotABoolean(&'static str),

    // Authentication errors
    #[error("Invalid username or password")]
    AuthInvalidCredentials,
    #[error("Invalid login token")]
    AuthInvalidToken,
    #[error("Login expired")]
    AuthExpiredToken,
    #[error("You are not logged in")]
    AuthBlacklistedToken,
    #[error("You are not logged in")]
    AuthMissingToken,
    #[error("You aren't allowed to do that!")]
    AuthUnauthorized,

    // Database errors
    #[error(transparent)]
    R2d2Error(#[from] r2d2::Error),
    #[error(transparent)]
    DbQueryError(#[from] diesel::result::Error),
    #[error("Object not found")]
    DbObjectNotFound,

    // Misc.
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error(transparent)]
    Blocking(#[from] actix_web::error::BlockingError),

    // Third-party errors
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
}

impl From<AppError> for std::io::Error {
    fn from(e: AppError) -> Self {
        std::io::Error::other(e)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let body = ErrorBody {
            error: self.to_string(),
        };
        log::error!("Responding with error: {}", body.error);
        match self {
            Self::AuthInvalidCredentials
            | Self::AuthInvalidToken
            | Self::AuthExpiredToken
            | Self::AuthBlacklistedToken
            | Self::AuthMissingToken
            | Self::AuthUnauthorized => HttpResponse::Unauthorized().json(body),
            Self::DbObjectNotFound => HttpResponse::NotFound().json(body),
            _ => HttpResponse::InternalServerError().json(body),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
pub struct ErrorBody {
    error: String,
}
