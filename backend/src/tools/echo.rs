//! `echo` tool — echoes the input back. Useful for smoke tests.

use crate::agent::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "echo".to_string(),
            description: "Echo the input back to the caller.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "Text to echo" }
                },
                "required": ["text"]
            }),
        }
    }

    async fn invoke(&self, arguments: Value) -> anyhow::Result<String> {
        let text = arguments
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("echo: missing 'text'"))?;
        Ok(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_echo_invoke() {
        let tool = EchoTool;
        let result = tool.invoke(json!({ "text": "hello world" })).await.unwrap();
        assert_eq!(result, "hello world");
    }

    #[tokio::test]
    async fn test_echo_invoke_missing_text() {
        let tool = EchoTool;
        let result = tool.invoke(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_echo_definition() {
        let tool = EchoTool;
        let def = tool.definition();
        assert_eq!(def.name, "echo");
        assert!(!def.description.is_empty());
    }
}
