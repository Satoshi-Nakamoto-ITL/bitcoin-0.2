// ─────────────────────────────────────────────
// CONSENSUS — DIFFICULTY
//
// v4: legacy, frozen
// v5: height-correct, hardened
//
// Any modification is a HARD FORK.
// ─────────────────────────────────────────────

use crate::core::block::Block;
use crate::consensus::params::*;

use num_bigint::BigUint;

/// Clamp target to consensus bounds
fn clamp_target_big(target: BigUint) -> BigUint {
    let max = BigUint::from_bytes_be(&MAX_TARGET);
    let min = BigUint::from_bytes_be(&MIN_TARGET);

    if target > max {
        max
    } else if target < min {
        min
    } else {
        target
    }
}

/// ─────────────────────────────────────────────
/// CONSENSUS v4 — LEGACY (DO NOT MODIFY)
/// ─────────────────────────────────────────────
///
/// Target calculation for blocks mined
/// before CONSENSUS_V5_HEIGHT.
pub fn calculate_next_target(chain: &[Block]) -> [u8; 32] {
    // Genesis / empty chain
    if chain.is_empty() {
        return MAX_TARGET;
    }

    let height = chain.len();
    let last = chain.last().unwrap();

    // Not enough blocks yet
    if height < DIFFICULTY_ADJUSTMENT_INTERVAL + 1 {
        return last.header.target;
    }

    // Only adjust on interval
    if height % DIFFICULTY_ADJUSTMENT_INTERVAL != 0 {
        return last.header.target;
    }

    let first =
        &chain[height - DIFFICULTY_ADJUSTMENT_INTERVAL - 1];

    let mut actual_time =
        last.header.timestamp - first.header.timestamp;

    let expected_time =
        TARGET_BLOCK_TIME * DIFFICULTY_ADJUSTMENT_INTERVAL as i64;

    if actual_time <= 0 {
        return last.header.target;
    }

    let min_time = expected_time / 4;
    let max_time = expected_time * 4;

    if actual_time < min_time {
        actual_time = min_time;
    } else if actual_time > max_time {
        actual_time = max_time;
    }

    let old_target =
        BigUint::from_bytes_be(&last.header.target);

    let scaled =
        (&old_target * BigUint::from(actual_time as u64))
            / BigUint::from(expected_time as u64);

    let new_target = clamp_target_big(scaled);

    let mut out = [0u8; 32];
    let bytes = new_target.to_bytes_be();

    let start = 32usize.saturating_sub(bytes.len());
    out[start..].copy_from_slice(&bytes);

    out
}

/// ─────────────────────────────────────────────
/// CONSENSUS v5 — HEIGHT-CORRECT
/// ─────────────────────────────────────────────
///
/// Target for block at height `next_height`
/// is computed using chain ending at height-1.
pub fn calculate_next_target_v5(
    chain: &[Block],
    next_height: u64,
) -> [u8; 32] {
    if next_height == 0 {
        return MAX_TARGET;
    }

    let last = match chain.last() {
        Some(b) => b,
        None => return MAX_TARGET,
    };

    if last.header.height + 1 != next_height {
        return last.header.target;
    }

    if next_height < (DIFFICULTY_ADJUSTMENT_INTERVAL as u64) + 1 {
        return last.header.target;
    }

    if next_height % DIFFICULTY_ADJUSTMENT_INTERVAL as u64 != 0 {
        return last.header.target;
    }

    let interval = DIFFICULTY_ADJUSTMENT_INTERVAL as usize;
    let first = &chain[chain.len() - interval - 1];

    let mut actual_time =
        last.header.timestamp - first.header.timestamp;

    let expected_time =
        TARGET_BLOCK_TIME * interval as i64;

    if actual_time <= 0 {
        return last.header.target;
    }

    let min_time = expected_time / 4;
    let max_time = expected_time * 4;

    if actual_time < min_time {
        actual_time = min_time;
    } else if actual_time > max_time {
        actual_time = max_time;
    }

    let old_target =
        BigUint::from_bytes_be(&last.header.target);

    let scaled =
        (&old_target * BigUint::from(actual_time as u64))
            / BigUint::from(expected_time as u64);

    let new_target = clamp_target_big(scaled);

    let mut out = [0u8; 32];
    let bytes = new_target.to_bytes_be();

    let start = 32usize.saturating_sub(bytes.len());
    out[start..].copy_from_slice(&bytes);

    out
}
