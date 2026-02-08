use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::PathBuf;

use time::OffsetDateTime;

use crate::consensus::{
    difficulty::calculate_next_target,
    fork_choice,
    params::*,
};

use crate::core::{
    block::{Block, BlockHeader},
    utxo::{UTXOSet, UTXO},
    transaction::Transaction,
};
use crate::revelation::revelation_tx;
use crate::merkle::merkle_root;


/* ───────── Persistence helpers ───────── */

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

/* ───────── Blockchain ───────── */

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: UTXOSet,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            utxos: HashMap::new(),
            mempool: Vec::new(),
        }
    }

    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn initialize(&mut self) {
        fs::create_dir_all(data_dir()).unwrap();

        if blocks_file().exists() {
            let data = fs::read_to_string(blocks_file()).unwrap();
            if !data.trim().is_empty() {
                self.blocks = serde_json::from_str(&data).unwrap();
            }
        }

        if self.blocks.is_empty() {
            let genesis = Block {
                header: BlockHeader {
                    height: 0,
                    timestamp: 1730000000,
                    prev_hash: vec![0u8; 32],
                    nonce: 0,
                    target: MAX_TARGET,
                    merkle_root: hex::decode(
                        "a081607fd3b32b29fd4cb46eb5bfe96406aeac0053910e963de67ddd6d10834a"
                    ).unwrap(),
                },
                transactions: vec![revelation_tx()],
                hash: hex::decode(
                    "8bdfff36f8f80e042e85770768df64f95b61f9e5f5128f4e49955bce3e902a1d"
                ).unwrap(),
            };

            assert!(genesis.hash == genesis.hash_header());
            assert!(genesis.verify_pow());

            self.blocks.push(genesis);
        }

        self.rebuild_utxos();
        self.save_all();
    }

    /// ⛓ CONSENSUS v4 — ORIGINAL LOGIC (UNCHANGED)
    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        // Height sanity
        if block.header.height > self.height() + 1 {
            return false;
        }

        // Timestamp rules
        if !self.blocks.is_empty() {
            let mtp = median_time_past(&self.blocks);
            if block.header.timestamp <= mtp {
                return false;
            }

            if block.header.timestamp >
                OffsetDateTime::now_utc().unix_timestamp() + MAX_FUTURE_DRIFT
            {
                return false;
            }
        }

        // Difficulty check
        if block.header.target != calculate_next_target(&self.blocks) {
            return false;
        }

        // PoW check
        if !block.verify_pow() {
            return false;
        }

        // Merkle root
        if merkle_root(&block.transactions) != block.header.merkle_root {
            return false;
        }

        // Accept block
        self.blocks.push(block);

        // Fork choice (v4+)
        if let Some(best_hash) = fork_choice::best_tip(&self.blocks) {
            let mut chain = Vec::new();
            let mut current = best_hash;

            while let Some(b) = self.blocks.iter().find(|x| x.hash == current) {
                chain.push(b.clone());
                if b.header.height == 0 {
                    break;
                }
                current = b.header.prev_hash.clone();
            }

            chain.reverse();
            self.blocks = chain;
        }

        self.rebuild_utxos();
        self.save_all();
        true
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
        ).unwrap();

        fs::write(
            utxos_file(),
            serde_json::to_string_pretty(&self.utxos).unwrap(),
        ).unwrap();
    }
}
