use axum::{
    Json,
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use thiserror::Error;

/// An error response returned by the REST API.
#[derive(Debug, Clone)]
pub struct ErrResponse {
    status: StatusCode,
    error: String,
}

impl ErrResponse {
    pub fn new(status: StatusCode, error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            status,
        }
    }
}

impl ErrResponse {
    /// The HTTP status code of the error.
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// The error message.
    pub fn error(&self) -> &str {
        &self.error
    }

    /// The hash of the error message.
    /// This is used to anonymize internal error messages in production.
    pub fn error_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.error.hash(&mut hasher);
        hasher.finish()
    }
}

// Depending on the build profile, we either return the full error message
// or a generic one in the case of an internal server error.
impl IntoResponse for ErrResponse {
    #[cfg(debug_assertions)]
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!(
                {
                    "error": self.error
                }
            )),
        )
            .into_response()
    }

    #[cfg(not(debug_assertions))]
    fn into_response(self) -> Response {
        let reason = if self.status == StatusCode::INTERNAL_SERVER_ERROR {
            format!("Internal Server Error - Ref: #{}", self.error_hash())
        } else {
            self.error
        };

        (
            self.status,
            Json(json!(
                {
                    "error": reason
                }
            )),
        )
            .into_response()
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    #[error("Failed to serialize/deserialize JSON: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Failed to parse multipart/form-data: {0}")]
    Multipart(#[from] MultipartError),
    #[error("Internal Server Error: {0}")]
    Axum(#[from] axum::Error),
    #[error("Not Found: {message}")]
    NotFound { message: String },
    #[error("Bad Request: {message}")]
    IllegalArgument { message: String },
    #[error("Internal Server Error: {message}")]
    Unhandled { message: String },
}

impl AppError {
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Multipart(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::JSON(_) | Self::IllegalArgument { .. } => StatusCode::BAD_REQUEST,
            Self::Axum(_) | Self::Unhandled { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(error = %self);
        }
        ErrResponse::new(status, self.to_string()).into_response()
    }
}

/// Errors that can occur during the REST API execution.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RESTError {
    #[error(transparent)]
    App(AppError),
    #[error("Internal server error: {message}")]
    InternalServerError { message: String },
    #[error("Not Found: {message}")]
    NotFound { message: String },
    #[error("Forbidden: {message}")]
    Forbidden { message: String },
    #[error("Bad Request: {message}")]
    BadRequest { message: String },
    #[error("Payload Too Large: {message}")]
    PayloadTooLarge { message: String },
    #[error("Conflict: {message}")]
    Conflict { message: String },
}

impl RESTError {
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::App(e) => e.status_code(),
            Self::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::Forbidden { .. } => StatusCode::FORBIDDEN,
            Self::PayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}

// Anything that can be converted into an AppError can be converted into a RESTError
impl<T: Into<AppError>> From<T> for RESTError {
    fn from(e: T) -> Self {
        Self::App(e.into())
    }
}

impl IntoResponse for RESTError {
    fn into_response(self) -> Response {
        ErrResponse::new(self.status_code(), self.to_string()).into_response()
    }
}
