//! Database initialization and migration runner.

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // For sqlite://./data/agent.db style URLs, ensure parent dir exists.
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;
    Ok(pool)
}

/// Run all SQL files under `migrations/` in lexical order. Each migration is
/// recorded in `_migrations` to guarantee idempotency.
pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS _migrations (
            name TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"#,
    )
    .execute(pool)
    .await?;

    let mut entries = tokio::fs::read_dir("migrations").await?;
    let mut files: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".sql") {
                    files.push(name.to_string());
                }
            }
        }
    }
    files.sort();

    for name in files {
        let already: Option<(String,)> =
            sqlx::query_as("SELECT name FROM _migrations WHERE name = ?")
                .bind(&name)
                .fetch_optional(pool)
                .await?;
        if already.is_some() {
            continue;
        }
        let sql = tokio::fs::read_to_string(format!("migrations/{}", name)).await?;
        // SQLx SQLite does not support multi-statement execute by default; split on ';'.
        for stmt in sql.split(';') {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            sqlx::query(stmt).execute(pool).await.map_err(|e| {
                anyhow::anyhow!("migration {} failed at statement: {}\nerr: {}", name, stmt, e)
            })?;
        }
        sqlx::query("INSERT INTO _migrations(name) VALUES (?)")
            .bind(&name)
            .execute(pool)
            .await?;
        tracing::info!(migration = %name, "applied");
    }
    Ok(())
}

#[allow(dead_code)]
fn _unused_fromstr(_: &str) -> Result<(), <String as FromStr>::Err> {
    Ok(())
}
