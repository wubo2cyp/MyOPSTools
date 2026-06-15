//! Session CRUD endpoints.

use crate::error::{AppError, AppResult};
use crate::repo::session::SessionRepo;
use crate::state::AppState;
use axum::{extract::State, Json};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SessionDto {
    pub id: String,
    pub title: String,
    pub agent_id: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::repo::session::Session> for SessionDto {
    fn from(s: crate::repo::session::Session) -> Self {
        Self {
            id: s.id,
            title: s.title,
            agent_id: s.agent_id,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
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

pub async fn list(State(state): State<AppState>) -> AppResult<Json<Vec<SessionDto>>> {
    let repo = SessionRepo::new(&state.db);
    let sessions = repo.list_by_user("default-user").await?;
    Ok(Json(sessions.into_iter().map(SessionDto::from).collect()))
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionReq>,
) -> AppResult<Json<SessionDto>> {
    let repo = SessionRepo::new(&state.db);
    let title = req.title.unwrap_or_else(|| "新会话".to_string());
    let session = repo.create("default-user", &title, &req.agent_id).await?;
    Ok(Json(session.into()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<SessionDto>> {
    let repo = SessionRepo::new(&state.db);
    let session = repo.get(&id).await?.ok_or_else(|| AppError::NotFound(format!("session {id} not found")))?;
    Ok(Json(session.into()))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSessionReq>,
) -> AppResult<Json<SessionDto>> {
    let repo = SessionRepo::new(&state.db);
    let session = repo.update(&id, &req.title).await?.ok_or_else(|| AppError::NotFound(format!("session {id} not found")))?;
    Ok(Json(session.into()))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let repo = SessionRepo::new(&state.db);
    let deleted = repo.delete(&id).await?;
    if !deleted {
        return Err(AppError::NotFound(format!("session {id} not found")));
    }
    Ok(Json(serde_json::json!({ "deleted": id })))
}
