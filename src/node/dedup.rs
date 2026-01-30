use std::collections::HashMap;
use std::time::{Duration, Instant};

use sha2::{Sha256, Digest};

/// Message de-duplication cache
///
/// Prevents the same raw message bytes from being processed
/// multiple times across different transports.
pub struct MessageDeduplicator {
    seen: HashMap<[u8; 32], Instant>,
    ttl: Duration,
}

impl MessageDeduplicator {
    /// Create a new deduplicator
    ///
    /// ttl = how long to remember message hashes
    pub fn new(ttl: Duration) -> Self {
        Self {
            seen: HashMap::new(),
            ttl,
        }
    }

    /// Returns true if message is NEW
    /// Returns false if message is a DUPLICATE
    pub fn check_and_insert(&mut self, data: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash: [u8; 32] = hasher.finalize().into();

        let now = Instant::now();

        // Cleanup expired entries
        self.seen.retain(|_, t| now.duration_since(*t) < self.ttl);

        if self.seen.contains_key(&hash) {
            return false;
        }

        self.seen.insert(hash, now);
        true
    }
}
