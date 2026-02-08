use std::sync::{Arc, Mutex};

use crate::core::chain::Blockchain;
use crate::node::mempool::Mempool;
use crate::wallet::Wallet;
use crate::core::validation::validate_transaction;

const COINBASE_MATURITY: u64 = 100;

pub fn handle_command(
    args: Vec<String>,
    wallet: &mut Wallet,
    chain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
) {
    if args.len() < 3 {
        println!("Usage:");
        println!("  wallet balance");
        println!("  wallet send <to_pubkey_hash_hex> <amount>");
        return;
    }

    match args[2].as_str() {
        "balance" => {
            let chain_guard = chain.lock().unwrap();
            let my_hash = wallet.address().expect("wallet locked");
            let current_height = chain_guard.height();

            let mut total = 0u64;
            let mut spendable = 0u64;
            let mut locked = 0u64;

            for u in chain_guard.utxos.values() {
                if u.pubkey_hash != my_hash {
                    continue;
                }

                total += u.value;

                if !u.is_coinbase {
                    spendable += u.value;
                } else if current_height >= u.height + COINBASE_MATURITY {
                    spendable += u.value;
                } else {
                    locked += u.value;
                }
            }

            println!("💰 Total balance:     {}", total);
            println!("💸 Spendable balance: {}", spendable);
            println!("🔒 Locked balance:    {}", locked);
        }

        "send" => {
            if args.len() != 5 {
                println!("Usage: wallet send <to_pubkey_hash_hex> <amount>");
                return;
            }

            let to = hex::decode(&args[3]).expect("invalid pubkey hash");
            let amount: u64 = args[4].parse().expect("invalid amount");

            let chain_guard = chain.lock().unwrap();
            let current_height = chain_guard.height();

            let tx = match wallet.create_transaction(
                &chain_guard.utxos,
                to,
                amount,
                current_height,
            ) {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Wallet error: {}", e);
                    return;
                }
            };

            if !validate_transaction(&tx, &chain_guard.utxos, current_height) {
                println!("❌ Transaction failed consensus validation");
                return;
            }

            drop(chain_guard);

            let mut mempool_guard = mempool.lock().unwrap();
            let chain_guard = chain.lock().unwrap();

            if mempool_guard.add_transaction(tx, &chain_guard.utxos, current_height) {
                println!("✅ Transaction added to mempool");
            } else {
                println!("❌ Transaction rejected by mempool policy");
            }
        }

        _ => println!("Unknown wallet command"),
    }
}
