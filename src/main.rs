use bitcoin_v0_2_revelation::chain::Blockchain;
use bitcoin_v0_2_revelation::network::P2PNetwork;
use std::net::SocketAddr;
use std::time::Duration;
use std::thread::sleep;

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

    let mut chain = Blockchain::new();
    chain.initialize();

    let miner_key = "REVELATION_MINER_001";

    let listen_addr = "0.0.0.0:8333".parse::<SocketAddr>().unwrap();
    let seed_nodes: Vec<SocketAddr> = vec![
        // add real public peers here
    ];

    let p2p = P2PNetwork::with_seeds(listen_addr, seed_nodes);

    println!("🌐 P2P listening on {}", listen_addr);
    println!("📡 Peers connected: {}", p2p.peer_count());

    let mut mode = NodeMode::Syncing;
    let mut block_counter = 0u64;

    println!("🔄 Starting initial sync");
    p2p.request_chain_state();

    loop {
        if let Some(block) = p2p.receive_block() {
            if chain.validate_and_add_block(block) {
                println!("📥 Block accepted | height {}", chain.height());
            }
        }

        match mode {
            NodeMode::Syncing => {
                if !p2p.is_syncing() {
                    println!("✅ Sync complete at height {}", chain.height());
                    mode = NodeMode::Normal;
                }
                sleep(Duration::from_millis(100));
            }

            NodeMode::Normal => {
                chain.mine_block(miner_key);
                block_counter += 1;

                if let Some(latest) = chain.blocks.last() {
                    p2p.broadcast_block(latest);
                }

                if block_counter % 5 == 0 {
                    println!("\n📊 Blockchain Status:");
                    println!("Height: {}", chain.blocks.len());
                    println!("Difficulty: {}", chain.difficulty);
                    println!("UTXO Set Size: {}", chain.utxos.len());
                    println!("Connected Peers: {}", p2p.peer_count());

                    if let Some(latest) = chain.blocks.last() {
                        println!("Latest Block Height: {}", latest.header.height);
                        println!("Latest Block Transactions: {}", latest.transactions.len());
                    }
                }

                if block_counter % 10 == 0 {
                    let stats = p2p.get_stats();
                    println!("\n🌐 Network Stats:");
                    println!("  Total Peers: {}", stats.total_peers);
                    println!("  Validated Peers: {}", stats.validated_peers);
                    println!("  Known Nodes: {}", stats.known_nodes);
                    println!("  Max Peers: {}", stats.max_peers);
                }

                if block_counter % 20 == 0 {
                    p2p.cleanup_stale_peers();
                    print_chain(&chain);
                }
            }
        }
    }
}
