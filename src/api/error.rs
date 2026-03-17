use rocket::{
    Request,
    http::Status,
    response::{self, Responder, status},
    serde::json::Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Database(String),
    #[error("{0}")]
    Internal(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl ApiError {
    fn code(&self) -> &'static str {
        match self {
            Self::Config(_) => "config_error",
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::Validation(_) => "validation_error",
            Self::NotFound(_) => "not_found",
            Self::Conflict(_) => "conflict",
            Self::Database(_) => "database_error",
            Self::Internal(_) => "internal_error",
        }
    }

    fn status(&self) -> Status {
        match self {
            Self::Config(_) | Self::Internal(_) | Self::Database(_) => Status::InternalServerError,
            Self::Unauthorized(_) => Status::Unauthorized,
            Self::Forbidden(_) => Status::Forbidden,
            Self::Validation(_) => Status::BadRequest,
            Self::NotFound(_) => Status::NotFound,
            Self::Conflict(_) => Status::Conflict,
        }
    }
}

impl From<mysql_async::Error> for ApiError {
    fn from(error: mysql_async::Error) -> Self {
        Self::Database(error.to_string())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::Unauthorized(error.to_string())
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let status_code = self.status();
        let body = Json(ErrorResponse {
            error: self.code(),
            message: self.to_string(),
        });

        status::Custom(status_code, body).respond_to(request)
    }
}
