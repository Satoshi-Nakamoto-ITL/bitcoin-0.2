pub mod signature;

use sha2::{Sha256, Digest};

/// sha256 helper used across the codebase
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// sha256(pubkey) → pubkey_hash
pub fn pubkey_hash(pubkey: &[u8]) -> Vec<u8> {
    sha256(pubkey)
}

// Convenience re-exports
pub use signature::verify_signature;
