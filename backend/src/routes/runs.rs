//! Run endpoints (SSE streaming). Stub for M1; real implementation in M3+M4.

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
    Json as AxumJson,
};
use futures::stream::{self, Stream};
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct CreateRunReq {
    #[serde(default)]
    pub user_message: String,
}

pub async fn create(
    State(_state): State<AppState>,
    Path(_session_id): Path<String>,
    AxumJson(_req): AxumJson<CreateRunReq>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let run_id = uuid::Uuid::new_v4().to_string();
    let stream = stream::unfold(0u32, move |i| {
        let run_id = run_id.clone();
        async move {
            if i >= 3 {
                None
            } else {
                let event = match i {
                    0 => Event::default().event("run.started").data(serde_json::json!({ "run_id": run_id }).to_string()),
                    1 => Event::default().event("message.delta").data("{\"delta\":\"[M1 skeleton] hello \"}"),
                    _ => Event::default().event("run.finished").data(serde_json::json!({ "run_id": run_id, "status": "ok" }).to_string()),
                };
                tokio::time::sleep(Duration::from_millis(150)).await;
                Some((Ok::<_, Infallible>(event), i + 1))
            }
        }
    });
    Ok(Sse::new(stream))
}

pub async fn cancel(
    State(_state): State<AppState>,
    Path((_session_id, _run_id)): Path<(String, String)>,
) -> AppResult<AxumJson<serde_json::Value>> {
    // TODO(M4)
    Err(AppError::NotFound("cancel not implemented yet".to_string()))
}
