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
