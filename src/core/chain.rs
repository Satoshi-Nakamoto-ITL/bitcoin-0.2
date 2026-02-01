// ─────────────────────────────────────────────
// CONSENSUS v3 — FROZEN
// ─────────────────────────────────────────────

use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::PathBuf;

use time::OffsetDateTime;

use crate::consensus::{
    difficulty::calculate_next_target,
    params::*,
};

use crate::{
    block::{Block, BlockHeader},
    utxo::{UTXOSet, UTXO},
    transaction::{Transaction, TxInput, TxOutput},
    revelation::revelation_tx,
    merkle::merkle_root,
};

// ─────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────

#[allow(dead_code)]
const COINBASE_MATURITY: u64 = 100;
const _CONSENSUS_V2_HEIGHT: u64 = 1000;

// 🔒 POLICY — NON-CONSENSUS
const MAX_REORG_DEPTH: usize = 100;

// ─────────────────────────────────────────────
// 🔒 HARD-CODED GENESIS (CONSENSUS LAW)
// ─────────────────────────────────────────────

const GENESIS_TIMESTAMP: i64 = 1730000000;
const GENESIS_NONCE: u64 = 0;
const GENESIS_TARGET: [u8; 32] = [0xff; 32];

const GENESIS_MERKLE: &str =
    "a081607fd3b32b29fd4cb46eb5bfe96406aeac0053910e963de67ddd6d10834a";

// ⚠️ THIS VALUE IS CURRENTLY WRONG — WILL BE REPLACED
const GENESIS_HASH: &str =
    "66753b6e462295ba651389ba0bac73417fa9d3143bbdba908a95b602d16830aa";

// ─────────────────────────────────────────────
// Blockchain struct
// ─────────────────────────────────────────────

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: UTXOSet,
    pub mempool: Vec<Transaction>,
}

// ─────────────────────────────────────────────
// Wallet layer (NON-CONSENSUS)
// ─────────────────────────────────────────────

impl Blockchain {
    pub fn create_transaction(
        &self,
        from: Vec<u8>,
        to: Vec<u8>,
        amount: u64,
    ) -> Result<Transaction, String> {
        let mut accumulated = 0;
        let mut inputs = Vec::new();

        for (key, utxo) in &self.utxos {
            if utxo.pubkey_hash == from {
                let parts: Vec<&str> = key.split(':').collect();
                let txid = hex::decode(parts[0]).map_err(|_| "Bad txid")?;
                let index: u32 = parts[1].parse().map_err(|_| "Bad index")?;

                accumulated += utxo.value;

                inputs.push(TxInput {
                    txid,
                    index,
                    pubkey: vec![],
                    signature: vec![],
                    address_index: 0,
                });

                if accumulated >= amount {
                    break;
                }
            }
        }

        if accumulated < amount {
            return Err("Not enough balance".into());
        }

        let mut outputs = vec![TxOutput {
            value: amount,
            pubkey_hash: to,
        }];

        if accumulated > amount {
            outputs.push(TxOutput {
                value: accumulated - amount,
                pubkey_hash: from,
            });
        }

        Ok(Transaction { inputs, outputs })
    }

    pub fn add_to_mempool(&mut self, tx: Transaction) {
        self.mempool.push(tx);
    }

    pub fn drain_mempool(&mut self) -> Vec<Transaction> {
        std::mem::take(&mut self.mempool)
    }
}

// ─────────────────────────────────────────────
// Consensus logic
// ─────────────────────────────────────────────

fn data_dir() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("data");
    path
}

fn blocks_file() -> PathBuf {
    let mut path = data_dir();
    path.push("blocks.json");
    path
}

fn utxos_file() -> PathBuf {
    let mut path = data_dir();
    path.push("utxos.json");
    path
}

fn median_time_past(chain: &[Block]) -> i64 {
    let mut times: Vec<i64> = chain
        .iter()
        .rev()
        .take(MTP_WINDOW)
        .map(|b| b.header.timestamp)
        .collect();
    times.sort();
    times[times.len() / 2]
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            utxos: HashMap::new(),
            mempool: Vec::new(),
        }
    }

    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        let expected_height = self.height();

        if block.header.height < expected_height {
            let depth = expected_height - block.header.height;
            if depth as usize > MAX_REORG_DEPTH {
                return false;
            }
        }

        if expected_height == 0 {
            if block.header.height != 0 {
                return false;
            }
        } else {
            let prev = self.blocks.last().unwrap();
            if block.header.prev_hash != prev.hash {
                return false;
            }
        }

        if !self.blocks.is_empty() {
            let mtp = median_time_past(&self.blocks);
            if block.header.timestamp <= mtp {
                return false;
            }
            if block.header.timestamp >
                OffsetDateTime::now_utc().unix_timestamp() + MAX_FUTURE_DRIFT {
                return false;
            }
        }

        if block.header.target != calculate_next_target(&self.blocks) {
            return false;
        }

        if !block.verify_pow() {
            return false;
        }

        if merkle_root(&block.transactions) != block.header.merkle_root {
            return false;
        }

        self.blocks.push(block);
        self.rebuild_utxos();
        self.save_all();
        true
    }

    pub fn initialize(&mut self) {
        fs::create_dir_all(data_dir()).unwrap();

        if self.blocks.is_empty() {
            let genesis = Block {
                header: BlockHeader {
                    height: 0,
                    timestamp: GENESIS_TIMESTAMP,
                    prev_hash: vec![0u8; 32],
                    nonce: GENESIS_NONCE,
                    target: GENESIS_TARGET,
                    merkle_root: hex::decode(GENESIS_MERKLE).unwrap(),
                },
                transactions: vec![revelation_tx()],
                hash: hex::decode(GENESIS_HASH).unwrap(),
            };

            let computed = genesis.hash_header();

            println!("🔴 GENESIS MISMATCH DETECTED");
            println!("Hardcoded genesis hash : {}", hex::encode(&genesis.hash));
            println!("Computed genesis hash  : {}", hex::encode(&computed));

            assert!(
                genesis.hash == computed,
                "Genesis hash constant does not match computed header hash"
            );

            assert!(genesis.verify_pow());

            self.blocks.push(genesis);
        }

        self.rebuild_utxos();
        self.save_all();
    }

    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear();

        for block in &self.blocks {
            for (tx_index, tx) in block.transactions.iter().enumerate() {
                let txid = hex::encode(tx.txid());

                for input in &tx.inputs {
                    self.utxos.remove(&format!(
                        "{}:{}",
                        hex::encode(&input.txid),
                        input.index
                    ));
                }

                let is_coinbase = tx_index == 0 && tx.inputs.is_empty();

                for (i, o) in tx.outputs.iter().enumerate() {
                    self.utxos.insert(
                        format!("{}:{}", txid, i),
                        UTXO {
                            value: o.value,
                            pubkey_hash: o.pubkey_hash.clone(),
                            height: block.header.height,
                            is_coinbase,
                        },
                    );
                }
            }
        }
    }

    pub fn save_all(&self) {
        fs::create_dir_all(data_dir()).unwrap();

        fs::write(
            blocks_file(),
            serde_json::to_string_pretty(&self.blocks).unwrap(),
        )
        .unwrap();

        fs::write(
            utxos_file(),
            serde_json::to_string_pretty(&self.utxos).unwrap(),
        )
        .unwrap();
    }
}
