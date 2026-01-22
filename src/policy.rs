/// Policy limits (NOT consensus yet)

pub const MAX_BLOCK_SIZE: usize = 1_000_000; // 1 MB
pub const MAX_BLOCK_TXS: usize = 2_000;

/// Coinbase + headers leave room
pub const MAX_BLOCK_TX_BYTES: usize = MAX_BLOCK_SIZE - 1_000;

/// Mempool policy
pub const MAX_TX_SIZE: usize = 100_000; // 100 KB
