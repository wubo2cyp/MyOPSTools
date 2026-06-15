//! Database initialization and migration runner.

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // For sqlite://./data/agent.db style URLs, ensure the parent directory
    // and the file itself exist. SQLite/sqlx doesn't always create the
    // file on first open, which trips up the "open" syscall with ENOENT
    // (sqlx error code 14) in some environments.
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        ensure_sqlite_file(path)?;
    } else if let Some(path) = database_url.strip_prefix("sqlite:") {
        ensure_sqlite_file(path)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;
    Ok(pool)
}

fn ensure_sqlite_file(path: &str) -> anyhow::Result<()> {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    // Touch the file so it exists. `OpenOptions::create(true)` won't
    // overwrite an existing file.
    if !p.exists() {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(p)?;
    }
    Ok(())
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
        let stmts = parse_sql_statements(&sql);
        for (i, stmt) in stmts.iter().enumerate() {
            let stmt = stmt.trim();
            if stmt.is_empty() {
                continue;
            }
            sqlx::query(stmt).execute(pool).await.map_err(|e| {
                anyhow::anyhow!("migration {} failed at statement {}: {}\nerr: {}", name, i, stmt, e)
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

fn parse_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_comment = false;
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if in_comment {
            if c == '\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }
        
        if c == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
            in_comment = true;
            i += 2;
            continue;
        }
        
        if c == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
            current.push(c);
            i += 1;
            continue;
        }
        
        if c == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
            current.push(c);
            i += 1;
            continue;
        }
        
        if c == ';' && !in_single_quote && !in_double_quote {
            statements.push(current.trim().to_string());
            current.clear();
            i += 1;
            continue;
        }
        
        current.push(c);
        i += 1;
    }
    
    if !current.trim().is_empty() {
        statements.push(current.trim().to_string());
    }
    
    statements
}

#[allow(dead_code)]
fn _unused_fromstr(_: &str) -> Result<(), <String as FromStr>::Err> {
    Ok(())
}
