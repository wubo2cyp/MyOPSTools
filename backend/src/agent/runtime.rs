//! Agent runtime: drives the `think -> tool -> think` loop.

use crate::agent::message::Role;
use crate::agent::{Message, ToolDefinition, ToolRegistry};
use crate::model::{ChatRequest, ChatResponse, ModelProvider};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Maximum number of tool call loops to prevent infinite recursion.
const MAX_TOOL_CALLS: usize = 10;

/// Streamed event emitted by the runtime, consumed by the HTTP layer to produce SSE.
#[derive(Debug, Clone, Serialize)]
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
        Self {
            model,
            tools,
            system_prompt: system_prompt.into(),
        }
    }

    /// Build the list of messages for the model, including system prompt.
    fn build_messages(&self, history: &[Message]) -> Vec<Message> {
        let mut msgs = vec![Message {
            role: Role::System,
            content: self.system_prompt.clone(),
            tool_calls: None,
            tool_call_id: None,
        }];
        msgs.extend(history.iter().cloned());
        msgs
    }

    /// Run the agent: think -> tool -> think loop.
    pub async fn run(
        &self,
        history: Vec<Message>,
        tx: mpsc::UnboundedSender<RunEvent>,
    ) {
        let run_id = uuid::Uuid::new_v4().to_string();
        let _ = tx.send(RunEvent::Started { run_id: run_id.clone() });

        let tool_defs: Vec<ToolDefinition> = self.tools.definitions();
        let mut messages = self.build_messages(&history);
        let mut tool_call_count = 0;

        loop {
            if tool_call_count >= MAX_TOOL_CALLS {
                let _ = tx.send(RunEvent::Error {
                    code: "max_iterations".to_string(),
                    message: "Maximum tool call iterations reached".to_string(),
                });
                let _ = tx.send(RunEvent::Finished {
                    run_id: run_id.clone(),
                    status: "max_iterations".to_string(),
                });
                return;
            }

            let req = ChatRequest {
                messages: messages.clone(),
                tools: tool_defs.clone(),
                temperature: None,
            };

            match self.model.chat(req).await {
                Ok(response) => {
                    match response {
                        ChatResponse::Text(text) => {
                            let _ = tx.send(RunEvent::MessageDelta { delta: text.clone() });
                            messages.push(Message {
                                role: Role::Assistant,
                                content: text,
                                tool_calls: None,
                                tool_call_id: None,
                            });
                            break;
                        }
                        ChatResponse::ToolCalls { calls, text_delta } => {
                            // Send text delta if any
                            if let Some(delta) = text_delta {
                                let _ = tx.send(RunEvent::MessageDelta { delta });
                            }

                            // Process each tool call
                            for call in calls {
                                let call_id = call.id.clone();
                                let tool_name = call.name.clone();
                                let arguments = call.arguments.clone();

                                // Send tool call event
                                let _ = tx.send(RunEvent::ToolCall {
                                    id: call_id.clone(),
                                    name: tool_name.clone(),
                                    arguments: arguments.clone(),
                                });

                                // Execute the tool
                                match self.execute_tool(&tool_name, arguments).await {
                                    Ok(output) => {
                                        let _ = tx.send(RunEvent::ToolResult {
                                            call_id: call_id.clone(),
                                            output: output.clone(),
                                        });
                                        // Add assistant's tool call message
                                        messages.push(Message {
                                            role: Role::Assistant,
                                            content: String::new(),
                                            tool_calls: Some(vec![call]),
                                            tool_call_id: None,
                                        });
                                        // Add tool result as a message
                                        messages.push(Message {
                                            role: Role::Tool,
                                            content: output,
                                            tool_calls: None,
                                            tool_call_id: Some(call_id),
                                        });
                                    }
                                    Err(e) => {
                                        let error_msg = format!("Tool execution failed: {}", e);
                                        let _ = tx.send(RunEvent::ToolResult {
                                            call_id: call_id.clone(),
                                            output: error_msg.clone(),
                                        });
                                        // Still add the messages to avoid confusing the model
                                        messages.push(Message {
                                            role: Role::Assistant,
                                            content: String::new(),
                                            tool_calls: Some(vec![call]),
                                            tool_call_id: None,
                                        });
                                        messages.push(Message {
                                            role: Role::Tool,
                                            content: error_msg,
                                            tool_calls: None,
                                            tool_call_id: Some(call_id),
                                        });
                                    }
                                }
                                tool_call_count += 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(RunEvent::Error {
                        code: "model_error".to_string(),
                        message: e.to_string(),
                    });
                    let _ = tx.send(RunEvent::Finished {
                        run_id: run_id.clone(),
                        status: "error".to_string(),
                    });
                    return;
                }
            }
        }

        let _ = tx.send(RunEvent::Finished {
            run_id,
            status: "ok".to_string(),
        });
    }

    /// Execute a tool by name with the given arguments.
    async fn execute_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> anyhow::Result<String> {
        let tool = self.tools.get(name).ok_or_else(|| {
            anyhow::anyhow!("tool '{}' not found in registry", name)
        })?;

        // Execute with timeout
        tokio::time::timeout(Duration::from_secs(30), tool.invoke(arguments))
            .await
            .map_err(|_| anyhow::anyhow!("tool '{}' timed out after 30 seconds", name))?
    }

    /// M1 stub: emits a single delta and finishes.
    #[allow(dead_code)]
    pub async fn run_stub(&self, _history: Vec<Message>, tx: mpsc::UnboundedSender<RunEvent>) {
        let run_id = uuid::Uuid::new_v4().to_string();
        let _ = tx.send(RunEvent::Started { run_id: run_id.clone() });
        let _ = tx.send(RunEvent::MessageDelta { delta: "[M1 skeleton] runtime not implemented yet".to_string() });
        let _ = tx.send(RunEvent::Finished { run_id, status: "ok".to_string() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::message::Role;
    use crate::model::MockProvider;

    #[tokio::test]
    async fn test_runtime_with_mock_provider() {
        let model = Arc::new(MockProvider::default());
        let tools = ToolRegistry::new();
        let runtime = AgentRuntime::new(
            model,
            tools,
            "You are a helpful assistant.",
        );

        let (tx, mut rx) = mpsc::unbounded_channel();
        
        let history = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
            tool_calls: None,
            tool_call_id: None,
        }];

        runtime.run(history, tx).await;

        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        assert!(!events.is_empty());
        assert!(matches!(events[0], RunEvent::Started { .. }));
        assert!(matches!(events.last(), Some(RunEvent::Finished { .. })));
    }

    #[test]
    fn test_run_event_serialization() {
        let event = RunEvent::Started { run_id: "test-123".to_string() };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("test-123"));
    }
}
