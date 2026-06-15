//! Shared application state injected into every axum handler.

use crate::agent::{AgentRuntime, RunRegistry, ToolRegistry};
use crate::config::Config;
use crate::model::{ModelProvider, OpenAIProvider};
use crate::tools::builtin_tools;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: SqlitePool,
    pub tools: Arc<ToolRegistry>,
    pub model: Arc<dyn ModelProvider>,
    pub runtime: Arc<AgentRuntime>,
    pub run_registry: RunRegistry,
}

impl AppState {
    pub fn new(config: Config, db: SqlitePool) -> Self {
        let tools = Arc::new(builtin_tools());
        let model: Arc<dyn ModelProvider> = match config.openai_api_key.as_deref() {
            Some(key) => Arc::new(OpenAIProvider::new(
                key.to_string(),
                config.openai_base_url.clone(),
                config.openai_model.clone(),
            )),
            None => Arc::new(crate::model::MockProvider::default()),
        };
        
        let runtime = Arc::new(
            AgentRuntime::new(
                model.clone(),
                (*tools).clone(),
                config.agent_system_prompt.clone(),
            )
            .with_limits(
                config.agent_max_tool_calls,
                std::time::Duration::from_millis(config.agent_tool_timeout_ms),
            )
            .with_temperature(config.openai_temperature),
        );
        
        Self {
            config: Arc::new(config),
            db,
            tools,
            model,
            runtime,
            run_registry: RunRegistry::new(),
        }
    }
}
