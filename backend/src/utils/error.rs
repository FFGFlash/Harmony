use std::fmt;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
  DatabaseError(sqlx::Error),
  NotFound(String),
  Unauthorized(String),
  BadRequest(String),
  InternalServerError(String),
  ValidationError(String),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
      AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
      AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
      AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
      AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
      AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
    }
  }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
  fn from(error: sqlx::Error) -> Self {
    AppError::DatabaseError(error)
  }
}

#[derive(Serialize)]
struct ErrorResponse {
  error: String,
  message: String,
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    let (status, error_message) = match self {
      AppError::DatabaseError(e) => {
        tracing::error!("Database error: {:?}", e);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Database error occurred".to_string(),
        )
      }
      AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
      AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
      AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
      AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
      AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
    };

    let body = Json(ErrorResponse {
      error: status
        .canonical_reason()
        .unwrap_or("Unknown error")
        .to_string(),
      message: error_message,
    });

    (status, body).into_response()
  }
}

pub type AppResult<T> = Result<T, AppError>;
