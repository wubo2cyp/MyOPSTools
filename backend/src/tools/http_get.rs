//! `http_get` tool — fetches a URL and returns the body (capped).
//!
//! Note: M1 skeleton. M3 will add timeout, size limit, and content-type detection.

use crate::agent::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Default)]
pub struct HttpGetTool {
    client: reqwest::Client,
}

#[async_trait]
impl Tool for HttpGetTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "http_get".to_string(),
            description: "Perform an HTTP GET to a URL and return the response body (up to 64KB).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "Absolute URL to fetch" }
                },
                "required": ["url"]
            }),
        }
    }

    async fn invoke(&self, arguments: Value) -> anyhow::Result<String> {
        let url = arguments
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("http_get: missing 'url'"))?;
        let resp = self
            .client
            .get(url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let truncated = body.chars().take(64 * 1024).collect::<String>();
        Ok(json!({ "status": status.as_u16(), "body": truncated }).to_string())
    }
}
