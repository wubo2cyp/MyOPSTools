//! Session repository stub. Real implementation in M2.

use sqlx::SqlitePool;

#[allow(dead_code)]
pub struct SessionRepo<'a> {
    pool: &'a SqlitePool,
}
