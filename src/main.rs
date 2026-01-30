use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::env;
use std::net::SocketAddr;

use tokio::runtime::Runtime;
use rpassword::read_password;

// ───────── Imports from the LIB crate ─────────
use bitcoin_v0_2_revelation::core::chain::Blockchain;
use bitcoin_v0_2_revelation::node::p2p::P2PNetwork;
use bitcoin_v0_2_revelation::node::transport::tcp::TcpTransport;
use bitcoin_v0_2_revelation::node::transport::satellite::SatelliteTransport;
use bitcoin_v0_2_revelation::node::transport::bluetooth::BluetoothTransport;
use bitcoin_v0_2_revelation::node::transport::geo::GeoTransport;
use bitcoin_v0_2_revelation::node::dedup::MessageDeduplicator;
use bitcoin_v0_2_revelation::interface::{api::start_api, cli};
use bitcoin_v0_2_revelation::node::mempool::Mempool;
use bitcoin_v0_2_revelation::wallet::Wallet;
use bitcoin_v0_2_revelation::wallet_store::load_wallet_store;
use bitcoin_v0_2_revelation::config::load_miner_config;
use bitcoin_v0_2_revelation::node::miner;

enum NodeMode {
    Syncing,
    Normal,
}

/// 🔒 Secure password input
fn prompt_secret(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    read_password().unwrap()
}

/// 🌱 Bootstrap seeds (non-consensus)
const BOOTSTRAP_SEEDS: &[&str] = &[
    "bitcoin-revelation-node.fly.dev:8333",
];

fn main() {
    println!("⛓ Bitcoin v0.3.3 — Revelation Edition (Consensus v3)");

    // ───────── Wallet & Miner Config ─────────
    let wallet_store = load_wallet_store();
    let miner_config = load_miner_config();

    if wallet_store.get_path(&miner_config.coinbase_wallet).is_none() {
        panic!("Configured wallet '{}' not found", miner_config.coinbase_wallet);
    }

    let _passphrase = prompt_secret("🔐 Enter wallet passphrase: ");
    let password = prompt_secret("🔑 Enter wallet password: ");

    let mut wallet = Wallet::load_or_create(&password);
    let miner_pubkey_hash = wallet.address().expect("wallet locked");

    println!(
        "👛 Miner pubkey hash ({}): {}",
        miner_config.coinbase_wallet,
        hex::encode(&miner_pubkey_hash)
    );

    // ───────── Blockchain ─────────
    let mut local_chain = Blockchain::new();
    local_chain.initialize();

    let chain = Arc::new(Mutex::new(local_chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    // ───────── CLI MODE ─────────
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "wallet" {
        cli::handle_command(
            args,
            &mut wallet,
            Arc::clone(&chain),
            Arc::clone(&mempool),
        );
        return;
    }

    // ───────── API Server ─────────
    let api_chain = Arc::clone(&chain);
    thread::spawn(move || {
        let rt = Runtime::new().expect("Tokio runtime failed");
        rt.block_on(start_api(api_chain, 8080));
    });

    println!("🌐 Explorer running at http://127.0.0.1:8080");

    // ───────── P2P BOOTSTRAP ─────────

    // Break circular ownership
    let p2p_holder: Arc<Mutex<Option<Arc<P2PNetwork>>>> =
        Arc::new(Mutex::new(None));

    // Message deduplication
    let dedup = Arc::new(Mutex::new(
        MessageDeduplicator::new(Duration::from_secs(60))
    ));

    // Unified receive callback for ALL transports
    let on_receive = Arc::new({
        let p2p_holder = Arc::clone(&p2p_holder);
        let dedup = Arc::clone(&dedup);

        move |addr: SocketAddr, data: Vec<u8>| {
            let is_new = {
                let mut d = dedup.lock().unwrap();
                d.check_and_insert(&data)
            };

            if !is_new {
                return; // drop duplicate silently
            }

            if let Some(p2p) = &*p2p_holder.lock().unwrap() {
                p2p.on_receive(addr, data);
            }
        }
    });

    // ───────── TCP Transport ─────────
    let transport = TcpTransport::new("0.0.0.0:0", on_receive.clone());

    let p2p = Arc::new(
        P2PNetwork::new(transport.clone(), Arc::clone(&chain))
    );

    *p2p_holder.lock().unwrap() = Some(Arc::clone(&p2p));

    println!("🔗 P2P TCP transport initialized");

    // ───────── Satellite Transport (receive-only) ─────────
    SatelliteTransport::listen_udp("0.0.0.0:9999", on_receive.clone());
    // SatelliteTransport::listen_file("satellite.dat", on_receive.clone());

    // ───────── GEO / LAN Mesh ─────────
    GeoTransport::start("0.0.0.0:9333", on_receive.clone());

    // ───────── Bluetooth BLE (async) ─────────
    {
        let on_receive = on_receive.clone();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(BluetoothTransport::start(on_receive));
        });
    }

    // ───────── Connect bootstrap seeds ─────────
    for seed in BOOTSTRAP_SEEDS {
        if let Ok(addr) = seed.parse::<SocketAddr>() {
            println!("🌱 Connecting to seed {}", seed);
            transport.connect(addr);
        }
    }

    println!("🔄 Requesting sync from peers");

    // ───────── Node Loop ─────────
    let mut mode = NodeMode::Syncing;
    let mut last_height = chain.lock().unwrap().height();
    let mut last_change = Instant::now();
    let mut last_balance: u64 = 0;

    loop {
        match mode {
            NodeMode::Syncing => {
                let height = chain.lock().unwrap().height();

                if height != last_height {
                    last_height = height;
                    last_change = Instant::now();
                }

                if last_change.elapsed() > Duration::from_secs(3) && height > 0 {
                    println!("✅ Sync complete at height {}", height);
                    mode = NodeMode::Normal;
                }

                sleep(Duration::from_millis(300));
            }

            NodeMode::Normal => {
                let txs = mempool.lock().unwrap().sorted_for_mining();

                let candidate_block = {
                    let c = chain.lock().unwrap();
                    let prev = c.blocks.last().unwrap();
                    miner::mine_block(
                        prev,
                        &c.utxos,
                        txs,
                        miner_pubkey_hash.clone(),
                        &c.blocks,
                    )
                };

                let accepted = {
                    let mut c = chain.lock().unwrap();
                    c.validate_and_add_block(candidate_block.clone())
                };

                if accepted {
                    p2p.broadcast_block(&candidate_block);

                    mempool
                        .lock()
                        .unwrap()
                        .remove_confirmed(&candidate_block.transactions);

                    let c = chain.lock().unwrap();
                    let balance: u64 = c.utxos
                        .values()
                        .filter(|u| u.pubkey_hash == miner_pubkey_hash)
                        .map(|u| u.value)
                        .sum();

                    let height = c.height();
                    if balance != last_balance {
                        println!("💰 Wallet balance: {} (height {})", balance, height);
                        last_balance = balance;
                    }
                }

                sleep(Duration::from_millis(100));
            }
        }
    }
}
