use bitcoin_v0_2_revelation::chain::Blockchain;

fn main() {
    println!("⛓ Bitcoin v0.2 — Revelation Edition");

    let mut chain = Blockchain::new();
    chain.initialize();

    let miner_key = "REVELATION_MINER_001";

    loop {
        chain.mine_block(miner_key);
    }
}
