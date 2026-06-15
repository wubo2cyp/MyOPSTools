//! Message endpoints.

use crate::error::{AppError, AppResult};
use crate::repo::message::{Message, MessageRepo};
use crate::state::AppState;
use axum::{extract::State, Json};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

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

impl From<crate::repo::message::Message> for MessageDto {
    fn from(m: crate::repo::message::Message) -> Self {
        let tool_calls = m.tool_calls.as_ref().and_then(|s| serde_json::from_str(s).ok());
        Self {
            id: m.id,
            session_id: m.session_id,
            role: m.role,
            content: m.content,
            tool_calls,
            tool_call_id: m.tool_call_id,
            created_at: m.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageReq {
    pub role: String,
    pub content: String,
}

pub async fn list(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> AppResult<Json<Vec<MessageDto>>> {
    let repo = MessageRepo::new(&state.db);
    let messages = repo.list_by_session(&session_id).await?;
    Ok(Json(messages.into_iter().map(MessageDto::from).collect()))
}

pub async fn create(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(req): Json<CreateMessageReq>,
) -> AppResult<Json<MessageDto>> {
    let repo = MessageRepo::new(&state.db);
    let msg = Message::new(&session_id, &req.role, &req.content);
    let saved = repo.create(&msg).await?;
    Ok(Json(saved.into()))
}
