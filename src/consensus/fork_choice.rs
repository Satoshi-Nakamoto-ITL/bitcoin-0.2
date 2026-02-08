use crate::core::block::Block;
use crate::consensus::difficulty::calculate_next_target_v5;

use num_bigint::BigUint;
use num_traits::{Zero, One};

use std::collections::HashMap;

/* ─────────────────────────────────────────────
   CONSENSUS v4 — Legacy Fork Choice
   (Longest cumulative work chain tip)
   ───────────────────────────────────────────── */

/// Compute work from a target (Bitcoin-style)
fn block_work_from_target(target: &[u8; 32]) -> BigUint {
    let t = BigUint::from_bytes_be(target);
    if t.is_zero() {
        return BigUint::zero();
    }
    (BigUint::one() << 256u32) / (t + BigUint::one())
}

/// Return the best tip hash using cumulative work (v4)
pub fn best_tip(blocks: &[Block]) -> Option<Vec<u8>> {
    if blocks.is_empty() {
        return None;
    }

    // Map: block_hash -> cumulative_work
    let mut work_map: HashMap<Vec<u8>, BigUint> = HashMap::new();

    for block in blocks {
        let work = block_work_from_target(&block.header.target);

        let cumulative = if block.header.height == 0 {
            work
        } else {
            work_map
                .get(&block.header.prev_hash)
                .cloned()
                .unwrap_or_else(BigUint::zero)
                + work
        };

        work_map.insert(block.hash.clone(), cumulative);
    }

    // Choose tip with max cumulative work
    work_map
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(hash, _)| hash)
}

/* ─────────────────────────────────────────────
   CONSENSUS v5 — Hardened Fork Choice
   (Structural + cumulative PoW)
   ───────────────────────────────────────────── */

fn block_work(block: &Block) -> BigUint {
    block_work_from_target(&block.header.target)
}

fn cumulative_work(chain: &[Block]) -> BigUint {
    let mut total = BigUint::zero();
    for b in chain {
        total += block_work(b);
    }
    total
}

/// Structural validation for v5 chains
fn validate_chain_v5(chain: &[Block]) -> bool {
    if chain.is_empty() {
        return false;
    }

    if chain[0].header.height != 0 {
        return false;
    }

    for i in 1..chain.len() {
        let prev = &chain[i - 1];
        let curr = &chain[i];

        if curr.header.height != prev.header.height + 1 {
            return false;
        }

        if curr.header.prev_hash != prev.hash {
            return false;
        }

        let expected_target =
            calculate_next_target_v5(&chain[..i], curr.header.height);

        if curr.header.target != expected_target {
            return false;
        }

        if !curr.verify_pow() {
            return false;
        }
    }

    true
}

/// Select best chain under CONSENSUS v5
pub fn select_best_chain_v5(
    candidates: Vec<Vec<Block>>,
) -> Option<Vec<Block>> {
    let mut best_chain: Option<Vec<Block>> = None;
    let mut best_work = BigUint::zero();

    for chain in candidates {
        if !validate_chain_v5(&chain) {
            continue;
        }

        let work = cumulative_work(&chain);

        if best_chain.is_none() || work > best_work {
            best_work = work;
            best_chain = Some(chain);
        }
    }

    best_chain
}
