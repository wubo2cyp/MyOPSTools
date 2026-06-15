//! LLM-driven helpers for non-agent tasks (titles, summaries, etc).

use crate::model::{ChatRequest, ChatResponse, ModelProvider};
use std::sync::Arc;

/// Generate a short title for a conversation from its first user message.
/// Falls back to a trimmed prefix of the user message if the model call fails
/// or the response is empty.
pub async fn generate_session_title(
    model: Arc<dyn ModelProvider>,
    user_message: &str,
) -> String {
    let trimmed = user_message.trim();
    if trimmed.is_empty() {
        return "新会话".to_string();
    }

    let prompt = format!(
        "请根据用户的第一条消息，用 6–18 个中文字符总结一个会话标题。\n\
         规则：\n\
         - 不要使用引号、句号或前缀（如\"标题：\"）。\n\
         - 直接输出标题文字本身。\n\
         - 如果用户消息是英文，输出对应中文翻译或保留 2–6 个英文单词。\n\n\
         用户消息：\n\
         {trimmed}\n\n\
         标题："
    );

    let messages = vec![crate::agent::Message {
        role: crate::agent::message::Role::User,
        content: prompt,
        tool_calls: None,
        tool_call_id: None,
    }];

    let req = ChatRequest {
        messages,
        tools: vec![],
        temperature: Some(0.3),
    };

    let raw = match model.chat(req).await {
        Ok(ChatResponse::Text(s)) => s,
        Ok(ChatResponse::ToolCalls { text_delta, .. }) => text_delta.unwrap_or_default(),
        Err(_) => return fallback_title(trimmed),
    };

    let cleaned = raw
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim()
        .trim_matches(|c: char| matches!(c, '"' | '\'' | '「' | '」' | ':' | '：' | ','))
        .to_string();

    if cleaned.is_empty() {
        fallback_title(trimmed)
    } else if cleaned.chars().count() > 30 {
        cleaned.chars().take(30).collect()
    } else {
        cleaned
    }
}

fn fallback_title(s: &str) -> String {
    let cleaned: String = s.chars().take(20).collect();
    if cleaned.is_empty() {
        "新会话".to_string()
    } else {
        cleaned
    }
}
