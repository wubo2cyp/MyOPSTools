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
use axum::Json;
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
    let session = session.ok_or_else(|| AppError::NotFound(format!("session {} not found", session_id)))?;

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

    // Background: auto-generate a title if the session still has a placeholder
    // title and the feature is enabled. Fire-and-forget so it doesn't block
    // the SSE stream.
    if state.config.agent_auto_title
        && (session.title.is_empty() || session.title == "新会话")
        && agent_messages.is_empty()
    {
        let model_clone = state.model.clone();
        let db_clone = state.db.clone();
        let user_text = req.user_message.clone();
        let session_id_clone = session_id.clone();
        tokio::spawn(async move {
            let new_title =
                crate::agent::title::generate_session_title(model_clone, &user_text).await;
            let repo = SessionRepo::new(&db_clone);
            if let Err(e) = repo.update(&session_id_clone, &new_title).await {
                tracing::warn!(error = %e, session = %session_id_clone, "failed to update session title");
            } else {
                tracing::info!(session = %session_id_clone, title = %new_title, "auto-generated session title");
            }
        });
    }

    // Create channel for runtime events
    let (tx, rx) = mpsc::unbounded_channel::<RunEvent>();

    // Pre-allocate a placeholder run_id and register a cancel slot. The runtime
    // will reuse this run_id when it sends the Started event, so cancellation
    // works without racing the Started emission.
    let run_id = uuid::Uuid::new_v4().to_string();
    let cancel_rx = state.run_registry.register(run_id.clone()).await;

    // Wrap the cancel receiver so its `Result<(), _>` is flattened to `()`.
    let cancel_future = async move {
        let _ = cancel_rx.await;
    };

    // Spawn runtime with cancel support. We pass the pre-allocated `run_id`
    // so the one in the registry and the one emitted in the `Started` SSE
    // event are identical — the client can then cancel by the id it received.
    let runtime = state.runtime.clone();
    let registry = state.run_registry.clone();
    let run_id_for_cleanup = run_id.clone();
    tokio::spawn(async move {
        runtime
            .run_with_cancel(run_id_for_cleanup.clone(), all_messages, tx, cancel_future)
            .await;
        registry.finish(&run_id_for_cleanup).await;
    });

    // Convert mpsc channel to Stream. The Started event from the runtime
    // already carries the correct run_id because we passed it in.
    let stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Some(e) => {
                let event = convert_run_event(e)
                    .unwrap_or_else(|| Event::default());
                Some((Ok::<_, Infallible>(event), rx))
            }
            None => None,
        }
    });

    Ok(Sse::new(stream))
}

pub async fn cancel(
    State(state): State<AppState>,
    Path((_session_id, run_id)): Path<(String, String)>,
) -> AppResult<AxumJson<serde_json::Value>> {
    let cancelled = state.run_registry.cancel(&run_id).await;
    if !cancelled {
        return Err(AppError::NotFound(format!(
            "no active run with id {}",
            run_id
        )));
    }
    Ok(Json(serde_json::json!({ "cancelled": run_id })))
}
