//! OpenAI-compatible chat provider. Stub for M1; full streaming in M5.

use crate::model::{ChatRequest, ChatResponse, ModelProvider, ProviderError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self { api_key, base_url, model }
    }
}

#[derive(Debug, Serialize)]
struct OaiRequest<'a> {
    model: &'a str,
    messages: &'a [crate::agent::Message],
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OaiTool<'a>>,
}

#[derive(Debug, Serialize)]
struct OaiTool<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
    function: OaiFunction<'a>,
}

#[derive(Debug, Serialize)]
struct OaiFunction<'a> {
    name: &'a str,
    description: &'a str,
    parameters: &'a serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OaiResponse {
    choices: Vec<OaiChoice>,
}

#[derive(Debug, Deserialize)]
struct OaiChoice {
    message: OaiMessage,
}

#[derive(Debug, Deserialize)]
struct OaiMessage {
    #[serde(default)]
    content: String,
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, ProviderError> {
        // M1 stub: real HTTP call implemented in M5.
        Err(ProviderError::Upstream(
            "OpenAIProvider is a stub in M1; full implementation in M5".to_string(),
        ))
    }
}

#[allow(dead_code)]
fn _typecheck_serialize() {
    let _ = OaiRequest {
        model: "",
        messages: &[],
        tools: vec![],
    };
}
