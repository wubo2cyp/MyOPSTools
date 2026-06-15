//! Message repository implementation.

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub created_at: String,
}

impl Message {
    pub fn new(session_id: &str, role: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_tool_calls(mut self, tool_calls: &str) -> Self {
        self.tool_calls = Some(tool_calls.to_string());
        self
    }

    pub fn with_tool_call_id(mut self, tool_call_id: &str) -> Self {
        self.tool_call_id = Some(tool_call_id.to_string());
        self
    }
}

#[derive(Debug, Clone)]
pub struct MessageRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> MessageRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, msg: &Message) -> Result<Message, AppError> {
        sqlx::query!(
            "INSERT INTO messages (id, session_id, role, content, tool_calls, tool_call_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            msg.id,
            msg.session_id,
            msg.role,
            msg.content,
            msg.tool_calls,
            msg.tool_call_id,
            msg.created_at,
        )
        .execute(self.pool)
        .await?;
        Ok(msg.clone())
    }

    pub async fn list_by_session(&self, session_id: &str) -> Result<Vec<Message>, AppError> {
        let rows = sqlx::query!(
            "SELECT id, session_id, role, content, tool_calls, tool_call_id, created_at
             FROM messages WHERE session_id = ? ORDER BY created_at ASC",
            session_id,
        )
        .fetch_all(self.pool)
        .await?;
        let messages = rows.into_iter().map(|r| Message {
            id: r.id.expect("id is required"),
            session_id: r.session_id,
            role: r.role,
            content: r.content,
            tool_calls: r.tool_calls,
            tool_call_id: r.tool_call_id,
            created_at: r.created_at,
        }).collect();
        Ok(messages)
    }

    pub async fn delete_by_session(&self, session_id: &str) -> Result<u64, AppError> {
        let result = sqlx::query!("DELETE FROM messages WHERE session_id = ?", session_id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn test_pool() -> SqlitePool {
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn crud() {
        let pool = test_pool().await;
        let msg_repo = MessageRepo::new(&pool);

        sqlx::query(include_str!("../../migrations/20260615000001_init.sql"))
            .execute(&pool)
            .await
            .unwrap();

        let msg = Message::new("session1", "user", "hello");
        let saved = msg_repo.create(&msg).await.unwrap();
        assert_eq!(saved.content, "hello");

        let list = msg_repo.list_by_session("session1").await.unwrap();
        assert_eq!(list.len(), 1);

        let deleted = msg_repo.delete_by_session("session1").await.unwrap();
        assert_eq!(deleted, 1);

        let empty = msg_repo.list_by_session("session1").await.unwrap();
        assert!(empty.is_empty());
    }
}
