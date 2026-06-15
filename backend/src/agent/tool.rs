//! Tool trait + registry.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    async fn invoke(&self, arguments: Value) -> anyhow::Result<String>;
}

#[derive(Default, Clone)]
pub struct ToolRegistry {
    inner: Arc<Mutex<HashMap<String, Arc<dyn Tool>>>>,
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.lock().unwrap();
        f.debug_struct("ToolRegistry")
            .field("tools", &inner.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: Tool + 'static>(&self, tool: T) {
        let def = tool.definition();
        self.inner.lock().unwrap().insert(def.name, Arc::new(tool));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.inner.lock().unwrap().get(name).cloned()
    }

    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.inner.lock().unwrap().values().map(|t| t.definition()).collect()
    }
}
