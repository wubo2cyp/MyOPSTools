//! Message repository stub. Real implementation in M2.

use sqlx::SqlitePool;

#[allow(dead_code)]
pub struct MessageRepo<'a> {
    pool: &'a SqlitePool,
}
