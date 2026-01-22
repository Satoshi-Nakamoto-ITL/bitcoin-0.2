use crate::block::Block;
use crate::consensus::difficulty::calculate_next_difficulty;

pub fn mine_block(
    chain: &[Block],
    transactions: Vec<Vec<u8>>,
    timestamp: i64,
) -> Block {
    let prev = chain.last().unwrap();
    let difficulty = calculate_next_difficulty(chain);

    let mut nonce = 0u64;

    loop {
        let block = Block {
            index: prev.index + 1,
            timestamp,
            prev_hash: prev.hash(),
            nonce,
            difficulty,
            transactions: transactions.clone(),
        };

        if block.meets_difficulty() {
            return block;
        }

        nonce += 1;
    }
}
