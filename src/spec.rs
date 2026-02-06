//! ⛓ Bitcoin Revelation — Consensus Specification v1.0
//!
//! Status: FINAL / FROZEN
//! Consensus Version: v4
//!
//! This file defines ALL consensus-critical rules.
//! Any change to this file constitutes a HARD FORK.
//!
//! Anything not explicitly specified here is NON-CONSENSUS POLICY.

/// ─────────────────────────────────────────────
/// 1. Genesis
/// ─────────────────────────────────────────────
///
/// The genesis block is hard-coded and MUST match exactly.
///
/// Fields (all consensus-critical):
/// - height = 0
/// - timestamp = 1730000000
/// - prev_hash = 32 bytes of 0x00
/// - nonce = 0
/// - target = MAX_TARGET (32 bytes)
/// - merkle_root =
///   a081607fd3b32b29fd4cb46eb5bfe96406aeac0053910e963de67ddd6d10834a
///
/// Genesis block hash:
/// 66753b6e462295ba651389ba0bac73417fa9d3143bbdba908a95b602d16830aa
///
/// Nodes MUST:
/// - hard-code the genesis block
/// - verify its hash
/// - verify its Proof-of-Work
/// - verify its merkle root
///
/// Any deviation is INVALID.

/// ─────────────────────────────────────────────
/// 2. Proof-of-Work
/// ─────────────────────────────────────────────
///
/// Hash function:
/// - double SHA-256
///
/// Hash and target:
/// - exactly 32 bytes
/// - interpreted as unsigned 256-bit BIG-ENDIAN integers
///
/// Validity rule:
///   hash ≤ target
///
/// Any change to:
/// - hash function
/// - comparison rule
/// - endianness
/// - target encoding
///
/// is a HARD FORK.

/// ─────────────────────────────────────────────
/// 3. Difficulty Adjustment
/// ─────────────────────────────────────────────
///
/// Difficulty operates on the FULL 256-bit target.
///
/// Adjustment interval:
/// - fixed
/// - adjustment occurs ONLY at exact interval boundaries
///
/// Formula:
///   new_target = old_target × actual_time / expected_time
///
/// Arithmetic:
/// - integer math only
/// - truncation toward zero
/// - no floating-point
///
/// Clamping:
/// - target is clamped to [MIN_TARGET, MAX_TARGET]
///
/// Any change to interval length, math, rounding,
/// or clamping is a HARD FORK.

/// ─────────────────────────────────────────────
/// 4. Block Validity
/// ─────────────────────────────────────────────
///
/// A block is valid if and only if:
/// - height is correct
/// - prev_hash matches previous block
/// - Proof-of-Work is valid
/// - target equals expected target for the height
/// - timestamp > Median Time Past
/// - timestamp ≤ now + MAX_FUTURE_DRIFT
/// - merkle root matches transactions
/// - block size ≤ MAX_BLOCK_SIZE
///
/// No optional rules exist.

/// ─────────────────────────────────────────────
/// 5. Transactions
/// ─────────────────────────────────────────────
///
/// Model:
/// - UTXO-based
///
/// Rules:
/// - inputs must reference unspent outputs
/// - signatures must verify
/// - input sum ≥ output sum
/// - fees are implicit
///
/// Coinbase:
/// - must be the first transaction in a block
/// - has no inputs
/// - outputs are locked by maturity rules
///
/// Any change is a HARD FORK.

/// ─────────────────────────────────────────────
/// 6. Coinbase Maturity
/// ─────────────────────────────────────────────
///
/// Coinbase outputs are unspendable for a fixed
/// number of blocks after creation.
///
/// Immature coinbase spends are INVALID.
///
/// Changing maturity is a HARD FORK.

/// ─────────────────────────────────────────────
/// 7. Fork Choice
/// ─────────────────────────────────────────────
///
/// Fork choice is based on cumulative Proof-of-Work.
///
/// Cumulative work is defined as:
///   Σ (2^256 / (target + 1))
///
/// Height alone is NOT authoritative.
///
/// Any change is a HARD FORK.

/// ─────────────────────────────────────────────
/// 8. Serialization
/// ─────────────────────────────────────────────
///
/// Consensus serialization is:
/// - manual
/// - deterministic
/// - fixed field order
/// - fixed byte order (little-endian where specified)
///
/// Serialization is part of consensus hashing.
///
/// Any change is a HARD FORK.

/// ─────────────────────────────────────────────
/// 9. Explicit Non-Consensus
/// ─────────────────────────────────────────────
///
/// The following are NOT consensus:
/// - mempool rules
/// - fee policy
/// - mining transaction selection
/// - reorg depth limits
/// - networking transports
/// - APIs / RPCs / UI
///
/// Differences here MUST NOT affect validity.

/// ─────────────────────────────────────────────
/// Finality
/// ─────────────────────────────────────────────
///
/// Consensus v4 is FINAL and FROZEN.
///
/// Any change requires:
/// - a new consensus version
/// - explicit hard fork
/// - independent node adoption
