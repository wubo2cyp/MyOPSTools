//! Shared application state injected into every axum handler.

use crate::agent::{AgentRuntime, ToolRegistry};
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
        
        // Get system prompt from agents table or use default
        let system_prompt = "你是一个轻量级通用 Agent，可以调用工具完成任务。".to_string();
        
        let runtime = Arc::new(AgentRuntime::new(
            model.clone(),
            (*tools).clone(),
            system_prompt,
        ));
        
        Self {
            config: Arc::new(config),
            db,
            tools,
            model,
            runtime,
        }
    }
}
