//! Run endpoints (SSE streaming). Full implementation in M3.

use crate::agent::message::Role;
use crate::agent::{Message, RunEvent};
use crate::error::{AppError, AppResult};
use crate::repo::message::MessageRepo;
use crate::repo::session::SessionRepo;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
    Json as AxumJson,
};
use futures::stream::{self, Stream};
use serde::Deserialize;
use std::convert::Infallible;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
pub struct CreateRunReq {
    pub user_message: String,
}

fn convert_run_event(e: RunEvent) -> Option<Event> {
    match e {
        RunEvent::Started { run_id } => {
            Some(Event::default().event("run.started").data(serde_json::json!({ "run_id": run_id }).to_string()))
        }
        RunEvent::MessageDelta { delta } => {
            Some(Event::default().event("message.delta").data(serde_json::json!({ "delta": delta }).to_string()))
        }
        RunEvent::ToolCall { id, name, arguments } => {
            Some(Event::default().event("tool.call").data(serde_json::json!({
                "id": id,
                "name": name,
                "arguments": arguments
            }).to_string()))
        }
        RunEvent::ToolResult { call_id, output } => {
            Some(Event::default().event("tool.result").data(serde_json::json!({
                "call_id": call_id,
                "output": output
            }).to_string()))
        }
        RunEvent::Finished { run_id, status } => {
            Some(Event::default().event("run.finished").data(serde_json::json!({
                "run_id": run_id,
                "status": status
            }).to_string()))
        }
        RunEvent::Error { code, message } => {
            Some(Event::default().event("run.error").data(serde_json::json!({
                "code": code,
                "message": message
            }).to_string()))
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    AxumJson(req): AxumJson<CreateRunReq>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    // Validate session exists
    let session_repo = SessionRepo::new(&state.db);
    let session = session_repo.get(&session_id).await?;
    if session.is_none() {
        return Err(AppError::NotFound(format!("session {} not found", session_id)));
    }

    // Get conversation history
    let msg_repo = MessageRepo::new(&state.db);
    let history = msg_repo.list_by_session(&session_id).await?;

    // Convert DB messages to agent messages
    let agent_messages: Vec<Message> = history
        .into_iter()
        .map(|m| Message {
            role: match m.role.as_str() {
                "system" => Role::System,
                "assistant" => Role::Assistant,
                "tool" => Role::Tool,
                _ => Role::User,
            },
            content: m.content,
            tool_calls: m.tool_calls.and_then(|tc| serde_json::from_str(&tc).ok()),
            tool_call_id: m.tool_call_id,
        })
        .collect();

    // Add user's new message
    let user_message = Message {
        role: Role::User,
        content: req.user_message.clone(),
        tool_calls: None,
        tool_call_id: None,
    };
    let mut all_messages = agent_messages.clone();
    all_messages.push(user_message);

    // Save user message to DB
    let db_msg = crate::repo::message::Message::new(&session_id, "user", &req.user_message);
    msg_repo.create(&db_msg).await?;

    // Create channel for runtime events
    let (tx, rx) = mpsc::unbounded_channel::<RunEvent>();

    // Spawn runtime
    let runtime = state.runtime.clone();
    tokio::spawn(async move {
        runtime.run(all_messages, tx).await;
    });

    // Convert mpsc channel to Stream
    let stream = stream::unfold(rx, |mut rx| async {
        match rx.recv().await {
            Some(e) => {
                if let Some(event) = convert_run_event(e) {
                    Some((Ok::<_, Infallible>(event), rx))
                } else {
                    Some((Ok::<_, Infallible>(Event::default()), rx))
                }
            }
            None => None,
        }
    });

    Ok(Sse::new(stream))
}

pub async fn cancel(
    State(_state): State<AppState>,
    Path((_session_id, _run_id)): Path<(String, String)>,
) -> AppResult<AxumJson<serde_json::Value>> {
    // TODO(M4): Implement cancellation via cancellation token
    Err(AppError::NotFound("cancel not implemented yet".to_string()))
}
