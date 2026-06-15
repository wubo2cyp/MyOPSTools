//! Agent runtime: drives the `think -> tool -> think` loop.
//!
//! Stub for M1. M3 will implement the full loop with a real `ModelProvider`.

use crate::agent::{Message, ToolRegistry};
use crate::model::ModelProvider;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Streamed event emitted by the runtime, consumed by the HTTP layer to produce SSE.
#[derive(Debug, Clone)]
pub enum RunEvent {
    Started { run_id: String },
    MessageDelta { delta: String },
    ToolCall { id: String, name: String, arguments: serde_json::Value },
    ToolResult { call_id: String, output: String },
    Finished { run_id: String, status: String },
    Error { code: String, message: String },
}

pub struct AgentRuntime {
    pub model: Arc<dyn ModelProvider>,
    pub tools: ToolRegistry,
    pub system_prompt: String,
}

impl AgentRuntime {
    pub fn new(model: Arc<dyn ModelProvider>, tools: ToolRegistry, system_prompt: impl Into<String>) -> Self {
        Self { model, tools, system_prompt: system_prompt.into() }
    }

    /// M1 placeholder: emits a single delta and finishes.
    /// Real implementation in M3.
    pub async fn run_stub(&self, _history: Vec<Message>, tx: mpsc::UnboundedSender<RunEvent>) {
        let run_id = uuid::Uuid::new_v4().to_string();
        let _ = tx.send(RunEvent::Started { run_id: run_id.clone() });
        let _ = tx.send(RunEvent::MessageDelta { delta: "[M1 skeleton] runtime not implemented yet".to_string() });
        let _ = tx.send(RunEvent::Finished { run_id, status: "ok".to_string() });
    }
}
