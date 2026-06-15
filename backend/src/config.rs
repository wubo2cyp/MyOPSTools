//! Application configuration loaded from environment variables.

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub cors_allow_origin: String,
    pub openai_api_key: Option<String>,
    pub openai_base_url: String,
    pub openai_model: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/agent.db".to_string()),
            cors_allow_origin: env::var("CORS_ALLOW_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            openai_api_key: env::var("OPENAI_API_KEY").ok().filter(|v| !v.is_empty()),
            openai_base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
        })
    }
}
