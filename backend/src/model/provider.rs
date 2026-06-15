//! `ModelProvider` abstraction.

use crate::agent::{Message, ToolDefinition};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    #[serde(default)]
    pub tools: Vec<ToolDefinition>,
    #[serde(default)]
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ChatResponse {
    Text(String),
    ToolCalls { calls: Vec<crate::agent::ToolCall>, text_delta: Option<String> },
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("upstream error: {0}")]
    Upstream(String),
    #[error("config error: {0}")]
    Config(String),
}

#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, ProviderError>;
}
