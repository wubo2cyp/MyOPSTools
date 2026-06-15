//! Offline Mock provider. Returns a deterministic reply that mentions the
//! available tool names, so the UI can be exercised without any API key.

use crate::agent::Message;
use crate::model::{ChatRequest, ChatResponse, ModelProvider, ProviderError};
use async_trait::async_trait;

#[derive(Debug, Default, Clone)]
pub struct MockProvider;

#[async_trait]
impl ModelProvider for MockProvider {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let last_user = req
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, crate::agent::message::Role::User))
            .map(|m| m.content.clone())
            .unwrap_or_default();
        let tool_names: Vec<String> = req.tools.iter().map(|t| t.name.clone()).collect();
        let reply = format!(
            "[Mock] 收到消息: {last_user}\n可用工具: {tools}",
            last_user = last_user,
            tools = if tool_names.is_empty() {
                "<none>".to_string()
            } else {
                tool_names.join(", ")
            }
        );
        Ok(ChatResponse::Text(reply))
    }
}

// Keep imports tidy.
#[allow(unused_imports)]
use crate::agent::message::Role;
#[allow(unused_imports)]
use crate::agent::Message as _Message;
