//! OpenAI-compatible chat provider.

use crate::agent::message::ToolCall;
use crate::model::{ChatRequest, ChatResponse, ModelProvider, ProviderError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("failed to build reqwest client");
        Self {
            client,
            api_key,
            base_url,
            model,
        }
    }

    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok()?;
        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = std::env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());
        Some(Self::new(api_key, base_url, model))
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
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OaiMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OaiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OaiToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: OaiFunctionCall,
}

#[derive(Debug, Deserialize)]
struct OaiFunctionCall {
    name: String,
    arguments: String,
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let oai_req = OaiRequest {
            model: &self.model,
            messages: &req.messages,
            tools: req.tools.iter().map(|t| OaiTool {
                kind: "function",
                function: OaiFunction {
                    name: &t.name,
                    description: &t.description,
                    parameters: &t.input_schema,
                },
            }).collect(),
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&oai_req)
            .send()
            .await
            .map_err(|e| ProviderError::Upstream(format!("request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Upstream(format!(
                "API error {}: {}",
                status, body
            )));
        }

        let oai_resp: OaiResponse = resp
            .json()
            .await
            .map_err(|e| ProviderError::Upstream(format!("parse failed: {}", e)))?;

        let choice = oai_resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ProviderError::Upstream("no choices in response".to_string()))?;

        if let Some(tool_calls) = choice.message.tool_calls {
            let calls: Vec<ToolCall> = tool_calls
                .into_iter()
                .map(|tc| {
                    let arguments: serde_json::Value = serde_json::from_str(&tc.function.arguments)
                        .unwrap_or(serde_json::Value::Object(Default::default()));
                    ToolCall {
                        id: tc.id,
                        name: tc.function.name,
                        arguments,
                    }
                })
                .collect();
            let text = choice.message.content.unwrap_or_default();
            Ok(ChatResponse::ToolCalls {
                calls,
                text_delta: if text.is_empty() { None } else { Some(text) },
            })
        } else {
            let content = choice.message.content.unwrap_or_default();
            Ok(ChatResponse::Text(content))
        }
    }
}
