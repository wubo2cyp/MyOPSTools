//! Application configuration loaded from environment variables / .env.

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub run_migrations: bool,
    pub cors_allow_origin: String,
    pub openai_api_key: Option<String>,
    pub openai_base_url: String,
    pub openai_model: String,
    pub openai_temperature: Option<f32>,
    pub agent_system_prompt: String,
    pub agent_max_tool_calls: usize,
    pub agent_tool_timeout_ms: u64,
    pub agent_auto_title: bool,
}

impl Config {
    /// Load config from process env, falling back to `.env` if present.
    /// Errors out only if a typed cast fails; unknown keys are ignored.
    pub fn from_env() -> anyhow::Result<Self> {
        // Best-effort .env load — missing file is not an error.
        let _ = dotenvy::dotenv();

        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://data/agent.db".to_string()),
            run_migrations: parse_bool(&env::var("RUN_MIGRATIONS").ok()).unwrap_or(true),
            cors_allow_origin: env::var("CORS_ALLOW_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            openai_api_key: env::var("OPENAI_API_KEY").ok().filter(|v| !v.is_empty()),
            openai_base_url: env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
            openai_temperature: env::var("OPENAI_TEMPERATURE")
                .ok()
                .filter(|v| !v.is_empty())
                .and_then(|v| v.parse().ok()),
            agent_system_prompt: env::var("AGENT_SYSTEM_PROMPT")
                .unwrap_or_else(|_| "你是一个轻量级通用 Agent，可以调用工具完成任务。".to_string()),
            agent_max_tool_calls: env::var("AGENT_MAX_TOOL_CALLS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            agent_tool_timeout_ms: env::var("AGENT_TOOL_TIMEOUT_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30_000),
            agent_auto_title: parse_bool(&env::var("AGENT_AUTO_TITLE").ok()).unwrap_or(true),
        })
    }
}

fn parse_bool(v: &Option<String>) -> Option<bool> {
    match v.as_deref()?.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" | "" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_variants() {
        assert_eq!(parse_bool(&Some("true".to_string())), Some(true));
        assert_eq!(parse_bool(&Some("FALSE".to_string())), Some(false));
        assert_eq!(parse_bool(&Some("yes".to_string())), Some(true));
        assert_eq!(parse_bool(&Some("0".to_string())), Some(false));
        assert_eq!(parse_bool(&None), None);
        assert_eq!(parse_bool(&Some("garbage".to_string())), None);
    }

    #[test]
    fn config_loads_with_defaults() {
        // Note: relies on `dotenvy` not having side effects in tests.
        // We can't easily override env, so this just exercises the code path.
        let cfg = Config::from_env().expect("config should load");
        assert!(!cfg.bind_addr.is_empty());
        assert!(!cfg.database_url.is_empty());
    }
}
