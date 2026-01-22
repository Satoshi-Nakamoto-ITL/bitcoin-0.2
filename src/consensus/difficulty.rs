use crate::block::Block;
use crate::consensus::params::*;

pub fn calculate_next_difficulty(chain: &[Block]) -> u32 {
    let height = chain.len();

    if height < DIFFICULTY_ADJUSTMENT_INTERVAL + 1 {
        return chain
            .last()
            .map(|b| b.header.difficulty)
            .unwrap_or(MIN_DIFFICULTY);
    }

    if height % DIFFICULTY_ADJUSTMENT_INTERVAL != 0 {
        return chain.last().unwrap().header.difficulty;
    }

    let last = chain.last().unwrap();
    let first = &chain[height - DIFFICULTY_ADJUSTMENT_INTERVAL - 1];

    let actual_time =
        last.header.timestamp - first.header.timestamp;

    let expected_time =
        TARGET_BLOCK_TIME * DIFFICULTY_ADJUSTMENT_INTERVAL as i64;

    let mut diff = last.header.difficulty;

    if actual_time < expected_time / 2 {
        diff += 1;
    } else if actual_time > expected_time * 2 {
        diff = diff.saturating_sub(1);
    }

    diff.clamp(MIN_DIFFICULTY, MAX_DIFFICULTY)
}
