pub fn block_reward(height: u64) -> u64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        0
    } else {
        50 * 100_000_000 >> halvings
    }
}
