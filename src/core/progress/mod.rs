use std::sync::atomic::{AtomicU64, Ordering};

/// Shared transfer progress, updated atomically by worker threads and read by
/// the UI. One instance lives in AppState behind an Arc.
pub struct TransferProgress {
    /// Bytes transferred so far.
    pub bytes_done: AtomicU64,
    /// Total bytes expected (0 = unknown).
    pub bytes_total: AtomicU64,
    /// Items processed (e.g. files/dirs deleted); displayed when bytes are 0.
    pub items_done: AtomicU64,
}

impl TransferProgress {
    pub fn new() -> Self {
        Self {
            bytes_done: AtomicU64::new(0),
            bytes_total: AtomicU64::new(0),
            items_done: AtomicU64::new(0),
        }
    }

    pub fn reset(&self, total: u64) {
        self.bytes_done.store(0, Ordering::Relaxed);
        self.bytes_total.store(total, Ordering::Relaxed);
        self.items_done.store(0, Ordering::Relaxed);
    }

    pub fn add(&self, n: u64) {
        self.bytes_done.fetch_add(n, Ordering::Relaxed);
    }

    pub fn add_item(&self) {
        self.items_done.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> (u64, u64) {
        (
            self.bytes_done.load(Ordering::Relaxed),
            self.bytes_total.load(Ordering::Relaxed),
        )
    }
}
