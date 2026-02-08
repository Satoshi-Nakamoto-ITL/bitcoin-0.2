// ─────────────────────────────────────────────
// CONSENSUS — VERSION GATING
//
// Defines hard-fork activation heights.
// Any change to this file is a HARD FORK.
// ─────────────────────────────────────────────

/// Consensus versions
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConsensusVersion {
    V4,
    V5,
}

/// 🔒 HARD FORK ACTIVATION HEIGHT
///
/// Blocks at height >= CONSENSUS_V5_HEIGHT
/// MUST be validated under Consensus v5.
///
/// Old nodes WILL fork here.
pub const CONSENSUS_V5_HEIGHT: u64 = 50_000; // ← CHOOSE FINAL VALUE

/// Determine consensus version for a block height
pub fn consensus_version_for_height(height: u64) -> ConsensusVersion {
    if height >= CONSENSUS_V5_HEIGHT {
        ConsensusVersion::V5
    } else {
        ConsensusVersion::V4
    }
}
