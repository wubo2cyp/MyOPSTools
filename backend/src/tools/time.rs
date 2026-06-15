//! `get_current_time` tool.

use crate::agent::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct TimeTool;

#[async_trait]
impl Tool for TimeTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "get_current_time".to_string(),
            description: "Return the current server time in RFC3339 format and a Unix timestamp.".to_string(),
            input_schema: json!({ "type": "object", "properties": {} }),
        }
    }

    async fn invoke(&self, _arguments: Value) -> anyhow::Result<String> {
        let now = chrono::Utc::now();
        let payload = json!({
            "rfc3339": now.to_rfc3339(),
            "unix": now.timestamp(),
        });
        Ok(payload.to_string())
    }
}
