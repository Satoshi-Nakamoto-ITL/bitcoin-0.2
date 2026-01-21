use crate::chain::Blockchain;

/// Debug helpers (not used in runtime yet)
#[allow(dead_code)]
fn display_chain(blockchain: &Blockchain) {
    println!("Chain height: {}", blockchain.height());
}

#[allow(dead_code)]
fn display_block_info(blockchain: &Blockchain) {
    if let Some(block) = blockchain.blocks.last() {
        println!("Latest block hash: {:?}", block.hash);
    }
}

#[allow(dead_code)]
pub fn display_full_chain(blockchain: &Blockchain) {
    for block in &blockchain.blocks {
        println!(
            "Block {} | txs={}",
            block.header.height,
            block.transactions.len()
        );
    }
}
