//! Unified application error type. Maps to HTTP responses via `IntoResponse`.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match &self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, "not_found", m.clone()),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, "bad_request", m.clone()),
            AppError::Upstream(m) => (StatusCode::BAD_GATEWAY, "upstream_error", m.clone()),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, "db_error", e.to_string()),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, "internal", e.to_string()),
        };
        tracing::warn!(error = %self, "request failed");
        (status, Json(json!({ "error": { "code": code, "message": message } }))).into_response()
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
