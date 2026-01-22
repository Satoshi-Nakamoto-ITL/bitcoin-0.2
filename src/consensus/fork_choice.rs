use crate::block::Block;

/// Sum of difficulty = total chain work
pub fn cumulative_work(chain: &[Block]) -> u128 {
    chain
        .iter()
        .map(|b| b.header.difficulty as u128)
        .sum()
}
