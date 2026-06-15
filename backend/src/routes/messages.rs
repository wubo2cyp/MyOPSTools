//! Message endpoints.

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use axum::{extract::{Path, State}, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageDto {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
    pub tool_call_id: Option<String>,
    pub created_at: String,
}

pub async fn list(
    State(_state): State<AppState>,
    Path(_session_id): Path<String>,
) -> AppResult<Json<Vec<MessageDto>>> {
    // TODO(M2)
    Err(AppError::NotFound("messages not implemented yet".to_string()))
}
