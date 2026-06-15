//! Per-process in-memory registry of active runs.
//!
//! Used by `POST /sessions/:id/runs/:run_id/cancel` to interrupt a running
//! agent loop. Entries are inserted when a run starts and removed when it
//! finishes (or when the cancel signal has been delivered).

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

#[derive(Default, Clone)]
pub struct RunRegistry {
    inner: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

impl RunRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new run. The returned `oneshot::Receiver` is held by the
    /// runtime loop; when it fires, the run terminates gracefully.
    pub async fn register(&self, run_id: String) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.inner.lock().await.insert(run_id, tx);
        rx
    }

    /// Remove the entry for `run_id` (call this when a run finishes normally).
    pub async fn finish(&self, run_id: &str) {
        self.inner.lock().await.remove(run_id);
    }

    /// Fire the cancel signal for `run_id`. Returns `true` if a run was found.
    pub async fn cancel(&self, run_id: &str) -> bool {
        if let Some(tx) = self.inner.lock().await.remove(run_id) {
            let _ = tx.send(());
            true
        } else {
            false
        }
    }

    /// Current number of active runs (for observability/tests).
    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn register_and_cancel() {
        let reg = RunRegistry::new();
        let mut rx = reg.register("r1".to_string()).await;
        assert_eq!(reg.len().await, 1);
        assert!(reg.cancel("r1").await);
        // Receiver should fire
        assert!(rx.try_recv().is_ok());
        assert_eq!(reg.len().await, 0);
    }

    #[tokio::test]
    async fn cancel_unknown_returns_false() {
        let reg = RunRegistry::new();
        assert!(!reg.cancel("missing").await);
    }

    #[tokio::test]
    async fn finish_removes_entry() {
        let reg = RunRegistry::new();
        let _rx = reg.register("r1".to_string()).await;
        assert_eq!(reg.len().await, 1);
        reg.finish("r1").await;
        assert_eq!(reg.len().await, 0);
        assert!(!reg.cancel("r1").await);
    }
}
