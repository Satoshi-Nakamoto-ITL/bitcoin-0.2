use bitcoin_v0_2_revelation::chain::Blockchain;
use bitcoin_v0_2_revelation::network::P2PNetwork;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

enum NodeMode {
    Syncing,
    Normal,
}

fn print_chain(chain: &Blockchain) {
    println!("\n🔗 Full Blockchain:");
    println!("═══════════════════════════════════════");
    for block in &chain.blocks {
        println!("Block #{}", block.header.height);
        println!("  Hash: {}", hex::encode(&block.hash));
        println!("  Transactions: {}", block.transactions.len());
        println!("  Timestamp: {}", block.header.timestamp);
        println!("  Difficulty: {}", block.header.difficulty);
        println!("─────────────────────────────────────");
    }
}

fn main() {
    println!("⛓ Bitcoin v0.2 — Revelation Edition");

    let mut local_chain = Blockchain::new();
    local_chain.initialize();

    let chain = Arc::new(Mutex::new(local_chain));

    let miner_key = "REVELATION_MINER_001";

    let listen_addr = "0.0.0.0:8333".parse::<SocketAddr>().unwrap();
    let p2p = P2PNetwork::new(listen_addr, Arc::clone(&chain));

    println!("🌐 P2P listening on {}", listen_addr);

    let mut mode = NodeMode::Syncing;
    let mut last_height = chain.lock().unwrap().height();
    let mut last_change = Instant::now();
    let mut mined_blocks = 0u64;

    println!("🔄 Requesting sync from peers");
    p2p.request_sync();

    loop {
        match mode {
            NodeMode::Syncing => {
                let height = chain.lock().unwrap().height();

                if height != last_height {
                    println!("📥 Sync progress | height {}", height);
                    last_height = height;
                    last_change = Instant::now();
                }

                // If no new blocks for 3 seconds, assume sync complete
                if last_change.elapsed() > Duration::from_secs(3) && height > 0 {
                    println!("✅ Sync complete at height {}", height);
                    mode = NodeMode::Normal;
                }

                sleep(Duration::from_millis(300));
            }

            NodeMode::Normal => {
                {
                    let mut chain = chain.lock().unwrap();
                    chain.mine_block(miner_key);
                    mined_blocks += 1;

                    if let Some(latest) = chain.blocks.last() {
                        p2p.broadcast_block(latest);
                    }
                }

                if mined_blocks % 5 == 0 {
                    let chain = chain.lock().unwrap();
                    println!("\n📊 Blockchain Status:");
                    println!("Height: {}", chain.blocks.len());
                    println!("Difficulty: {}", chain.difficulty);
                    println!("UTXO Set Size: {}", chain.utxos.len());
                    println!("Connected Peers: {}", p2p.peer_count());

                    if let Some(latest) = chain.blocks.last() {
                        println!("Latest Block Height: {}", latest.header.height);
                        println!(
                            "Latest Block Transactions: {}",
                            latest.transactions.len()
                        );
                    }
                }

                if mined_blocks % 20 == 0 {
                    let chain = chain.lock().unwrap();
                    print_chain(&chain);
                }

                // Yield CPU to networking
                sleep(Duration::from_millis(100));
            }
        }
    }
}
