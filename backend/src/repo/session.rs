//! Session repository implementation.

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub agent_id: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Session {
    pub fn new(user_id: &str, title: &str, agent_id: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            title: title.to_string(),
            agent_id: agent_id.to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionRepo<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepo<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: &str, title: &str, agent_id: &str) -> Result<Session, AppError> {
        let s = Session::new(user_id, title, agent_id);
        sqlx::query!(
            "INSERT INTO sessions (id, user_id, title, agent_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            s.id,
            s.user_id,
            s.title,
            s.agent_id,
            s.created_at,
            s.updated_at,
        )
        .execute(self.pool)
        .await?;
        Ok(s)
    }

    pub async fn get(&self, id: &str) -> Result<Option<Session>, AppError> {
        let row = sqlx::query!(
            "SELECT id, user_id, title, agent_id, created_at, updated_at FROM sessions WHERE id = ?",
            id,
        )
        .fetch_optional(self.pool)
        .await?;
        let s = row.map(|r| Session {
            id: r.id.expect("id is required"),
            user_id: r.user_id,
            title: r.title,
            agent_id: r.agent_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        });
        Ok(s)
    }

    pub async fn list_by_user(&self, user_id: &str) -> Result<Vec<Session>, AppError> {
        let rows = sqlx::query!(
            "SELECT id, user_id, title, agent_id, created_at, updated_at
             FROM sessions WHERE user_id = ? ORDER BY updated_at DESC",
            user_id,
        )
        .fetch_all(self.pool)
        .await?;
        let sessions = rows.into_iter().map(|r| Session {
            id: r.id.expect("id is required"),
            user_id: r.user_id,
            title: r.title,
            agent_id: r.agent_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect();
        Ok(sessions)
    }

    pub async fn update(&self, id: &str, title: &str) -> Result<Option<Session>, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = sqlx::query!(
            "UPDATE sessions SET title = ?, updated_at = ? WHERE id = ?",
            title,
            now,
            id,
        )
        .execute(self.pool)
        .await?;
        if result.rows_affected() == 0 {
            Ok(None)
        } else {
            self.get(id).await
        }
    }

    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        let result = sqlx::query!("DELETE FROM sessions WHERE id = ?", id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::time::Duration;

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
        let repo = SessionRepo::new(&pool);

        sqlx::query(include_str!("../../migrations/20260615000001_init.sql"))
            .execute(&pool)
            .await
            .unwrap();

        let s = repo.create("user1", "test title", "default").await.unwrap();
        assert_eq!(s.title, "test title");

        let found = repo.get(&s.id).await.unwrap().unwrap();
        assert_eq!(found.id, s.id);

        let updated = repo.update(&s.id, "new title").await.unwrap().unwrap();
        assert_eq!(updated.title, "new title");

        let list = repo.list_by_user("user1").await.unwrap();
        assert_eq!(list.len(), 1);

        let deleted = repo.delete(&s.id).await.unwrap();
        assert!(deleted);

        let not_found = repo.get(&s.id).await.unwrap();
        assert!(not_found.is_none());
    }
}
