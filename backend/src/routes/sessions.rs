//! Session CRUD endpoints.
//!
//! Stub implementations for M1 skeleton. M2 will fill in real persistence.

use crate::error::AppResult;
use crate::state::AppState;
use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SessionDto {
    pub id: String,
    pub title: String,
    pub agent_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionReq {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default = "default_agent")]
    pub agent_id: String,
}

fn default_agent() -> String {
    "default".to_string()
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionReq {
    pub title: String,
}

pub async fn list(_state: State<AppState>) -> AppResult<Json<Vec<SessionDto>>> {
    // TODO(M2): query sessions from DB
    Ok(Json(vec![]))
}

pub async fn create(
    State(_state): State<AppState>,
    Json(_req): Json<CreateSessionReq>,
) -> AppResult<Json<SessionDto>> {
    // TODO(M2): persist and return
    Ok(Json(SessionDto {
        id: uuid::Uuid::new_v4().to_string(),
        title: _req.title.unwrap_or_else(|| "新会话".to_string()),
        agent_id: _req.agent_id,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}

pub async fn get(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> AppResult<Json<SessionDto>> {
    // TODO(M2)
    Err(crate::error::AppError::NotFound(format!("session {} not implemented yet", _id)))
}

pub async fn update(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
    Json(_req): Json<UpdateSessionReq>,
) -> AppResult<Json<SessionDto>> {
    // TODO(M2)
    Err(crate::error::AppError::NotFound(format!("session {} not implemented yet", _id)))
}

pub async fn delete(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    // TODO(M2)
    Ok(Json(serde_json::json!({ "deleted": _id })))
}
