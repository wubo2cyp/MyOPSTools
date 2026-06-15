//! Agent runtime: drives the `think -> tool -> think` loop.

use crate::agent::message::Role;
use crate::agent::{Message, ToolDefinition, ToolRegistry};
use crate::model::{ChatRequest, ChatResponse, ModelProvider, ProviderError};
use futures::StreamExt;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

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
    pub max_tool_calls: usize,
    pub tool_timeout: Duration,
    pub temperature: Option<f32>,
}

impl AgentRuntime {
    pub fn new(model: Arc<dyn ModelProvider>, tools: ToolRegistry, system_prompt: impl Into<String>) -> Self {
        Self {
            model,
            tools,
            system_prompt: system_prompt.into(),
            max_tool_calls: 10,
            tool_timeout: Duration::from_secs(30),
            temperature: None,
        }
    }

    pub fn with_limits(mut self, max_tool_calls: usize, tool_timeout: Duration) -> Self {
        self.max_tool_calls = max_tool_calls;
        self.tool_timeout = tool_timeout;
        self
    }

    pub fn with_temperature(mut self, temperature: Option<f32>) -> Self {
        self.temperature = temperature;
        self
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

    /// Run the agent: think -> tool -> think loop. Never cancellable.
    pub async fn run(
        &self,
        run_id: String,
        history: Vec<Message>,
        tx: mpsc::UnboundedSender<RunEvent>,
    ) {
        self.run_with_cancel(run_id, history, tx, std::future::pending::<()>())
            .await;
    }

    /// Same as [`run`] but with an additional cancellation future. When the
    /// future resolves, the loop terminates gracefully with status `cancelled`.
    pub async fn run_with_cancel(
        &self,
        run_id: String,
        history: Vec<Message>,
        tx: mpsc::UnboundedSender<RunEvent>,
        cancel: impl std::future::Future<Output = ()> + Send + 'static,
    ) {
        let _ = tx.send(RunEvent::Started { run_id: run_id.clone() });

        let tool_defs: Vec<ToolDefinition> = self.tools.definitions();
        let mut messages = self.build_messages(&history);
        let mut tool_call_count = 0;
        let mut cancelled = false;

        // Bridge the cancel future into a stream-like pollable future so we
        // can check it from many places without re-consuming it. We pin the
        // future first so the resulting stream is Unpin and can be polled
        // repeatedly.
        let cancel = Box::pin(cancel);
        let cancel_stream = futures::stream::once(cancel).into_future();
        let mut cancel_stream = cancel_stream;

        loop {
            if cancelled {
                let _ = tx.send(RunEvent::Finished {
                    run_id: run_id.clone(),
                    status: "cancelled".to_string(),
                });
                return;
            }

            if tool_call_count >= self.max_tool_calls {
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
                temperature: self.temperature,
            };

            // Race the model call against cancellation.
            let model_call = self.model.chat(req);
            let response: Result<ChatResponse, ProviderError> = tokio::select! {
                biased;
                _ = &mut cancel_stream => {
                    cancelled = true;
                    continue;
                }
                r = model_call => r,
            };

            match response {
                Ok(ChatResponse::Text(text)) => {
                    let _ = tx.send(RunEvent::MessageDelta { delta: text.clone() });
                    messages.push(Message {
                        role: Role::Assistant,
                        content: text,
                        tool_calls: None,
                        tool_call_id: None,
                    });
                    break;
                }
                Ok(ChatResponse::ToolCalls { calls, text_delta }) => {
                    if let Some(delta) = text_delta {
                        let _ = tx.send(RunEvent::MessageDelta { delta });
                    }

                    for call in calls {
                        // Check cancellation between tool calls
                        let was_cancelled = match futures::poll!(&mut cancel_stream) {
                            std::task::Poll::Ready(_) => true,
                            std::task::Poll::Pending => false,
                        };
                        if was_cancelled {
                            cancelled = true;
                            break;
                        }

                        let call_id = call.id.clone();
                        let tool_name = call.name.clone();
                        let arguments = call.arguments.clone();

                        let _ = tx.send(RunEvent::ToolCall {
                            id: call_id.clone(),
                            name: tool_name.clone(),
                            arguments: arguments.clone(),
                        });

                        match self.execute_tool(&tool_name, arguments).await {
                            Ok(output) => {
                                let _ = tx.send(RunEvent::ToolResult {
                                    call_id: call_id.clone(),
                                    output: output.clone(),
                                });
                                messages.push(Message {
                                    role: Role::Assistant,
                                    content: String::new(),
                                    tool_calls: Some(vec![call]),
                                    tool_call_id: None,
                                });
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

                        if tool_call_count >= self.max_tool_calls {
                            break;
                        }
                    }

                    if cancelled {
                        continue;
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
        tokio::time::timeout(self.tool_timeout, tool.invoke(arguments))
            .await
            .map_err(|_| anyhow::anyhow!("tool '{}' timed out after {:?}", name, self.tool_timeout))?
    }

    /// M1 stub: emits a single delta and finishes.
    #[allow(dead_code)]
    pub async fn run_stub(
        &self,
        run_id: String,
        _history: Vec<Message>,
        tx: mpsc::UnboundedSender<RunEvent>,
    ) {
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

        runtime.run("test-run-1".to_string(), history, tx).await;

        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        assert!(!events.is_empty());
        assert!(matches!(events[0], RunEvent::Started { .. }));
        assert!(matches!(events.last(), Some(RunEvent::Finished { .. })));
    }

    #[tokio::test]
    async fn test_runtime_cancellation() {
        let model = Arc::new(MockProvider::default());
        let tools = ToolRegistry::new();
        let runtime = AgentRuntime::new(
            model,
            tools,
            "You are a helpful assistant.",
        );

        let (tx, mut rx) = mpsc::unbounded_channel();

        let cancel = async {
            tokio::time::sleep(Duration::from_millis(10)).await;
        };

        let history = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
            tool_calls: None,
            tool_call_id: None,
        }];

        runtime
            .run_with_cancel("test-run-2".to_string(), history, tx, cancel)
            .await;

        let mut events = Vec::new();
        while let Some(event) = rx.recv().await {
            events.push(event);
        }

        assert!(!events.is_empty());
        assert!(matches!(events[0], RunEvent::Started { .. }));
    }

    #[test]
    fn test_run_event_serialization() {
        let event = RunEvent::Started { run_id: "test-123".to_string() };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("test-123"));
    }
}
