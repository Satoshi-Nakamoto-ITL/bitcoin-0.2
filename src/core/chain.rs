// ─────────────────────────────────────────────
// CONSENSUS v3 — FROZEN
//
// The rules in this module define L1 consensus.
// Any modification requires a version-gated fork.
// Do NOT refactor, optimize, or "clean up" casually.
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
    reward::block_reward,
    revelation::revelation_tx,
    merkle::merkle_root,
    validation::validate_transaction,
};

#[allow(dead_code)]
const COINBASE_MATURITY: u64 = 100;
const CONSENSUS_V2_HEIGHT: u64 = 1000;

// ─────────────────────────────────────────────
// 🔒 HARD-CODED GENESIS (CONSENSUS LAW)
// ─────────────────────────────────────────────

const GENESIS_TIMESTAMP: i64 = 1730000000;
const GENESIS_NONCE: u64 = 0;

const GENESIS_TARGET: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

const GENESIS_MERKLE: &str =
    "a081607fd3b32b29fd4cb46eb5bfe96406aeac0053910e963de67ddd6d10834a";

const GENESIS_HASH: &str =
    "66753b6e462295ba651389ba0bac73417fa9d3143bbdba908a95b602d16830aa";

/// 🔥 NON-CONSENSUS ADDITIONS: mempool
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: UTXOSet,
    pub mempool: Vec<Transaction>,
}

/* ───────────── Data files ───────────── */

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

/* ───────────── Helpers ───────────── */

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

/* ───────────── Implementation ───────────── */

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

    // ─────────────────────────────────────────
    // 🔥 WALLET LAYER — NON-CONSENSUS
    // ─────────────────────────────────────────

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

        let mut outputs = vec![
            TxOutput {
                value: amount,
                pubkey_hash: to,
            }
        ];

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

    /* ───────── BLOCK VALIDATION ───────── */

    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        let expected_height = self.height();

        if expected_height == 0 {
            if block.header.height != 0 {
                return false;
            }
        } else {
            let prev = self.blocks.last().unwrap();

            if block.header.height != expected_height {
                return false;
            }

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
                OffsetDateTime::now_utc().unix_timestamp()
                    + MAX_FUTURE_DRIFT
            {
                return false;
            }
        }

        let expected_target =
            calculate_next_target(&self.blocks);

        if block.header.target != expected_target {
            return false;
        }

        if !block.verify_pow() {
            return false;
        }

        if merkle_root(&block.transactions)
            != block.header.merkle_root
        {
            return false;
        }

        let block_size =
            bincode::serialize(&block).unwrap().len();

        if block_size > MAX_BLOCK_SIZE {
            return false;
        }

        if let Some(cb) = block.transactions.first() {
            if !cb.inputs.is_empty() {
                return false;
            }

            let expected_reward =
                block_reward(block.header.height);

            let actual_reward: u64 =
                cb.outputs.iter().map(|o| o.value).sum();

            if actual_reward > expected_reward {
                return false;
            }
        }

        for tx in &block.transactions {
            if !validate_transaction(tx, &self.utxos, block.header.height) {
                return false;
            }

            if block.header.height >= CONSENSUS_V2_HEIGHT {
                if !self.enforce_coinbase_maturity(
                    tx,
                    block.header.height,
                ) {
                    return false;
                }
            }
        }

        self.blocks.push(block);
        self.rebuild_utxos();
        self.save_all();

        true
    }

    fn enforce_coinbase_maturity(
        &self,
        tx: &Transaction,
        current_height: u64,
    ) -> bool {
        if tx.inputs.is_empty() {
            return true;
        }

        for input in &tx.inputs {
            let key = format!(
                "{}:{}",
                hex::encode(&input.txid),
                input.index
            );

            let utxo = match self.utxos.get(&key) {
                Some(u) => u,
                None => return false,
            };

            if utxo.is_coinbase {
                if current_height
                    < utxo.height + COINBASE_MATURITY
                {
                    return false;
                }
            }
        }

        true
    }

    /* ───────── INIT ───────── */

    pub fn initialize(&mut self) {
        fs::create_dir_all(data_dir()).unwrap();

        if blocks_file().exists() {
            let data = fs::read_to_string(blocks_file()).unwrap();
            if !data.trim().is_empty() {
                self.blocks = serde_json::from_str(&data).unwrap();
            }
        }

        if self.blocks.is_empty() {
            let txs = vec![revelation_tx()];

            let genesis = Block {
                header: BlockHeader {
                    height: 0,
                    timestamp: GENESIS_TIMESTAMP,
                    prev_hash: vec![0u8; 32],
                    nonce: GENESIS_NONCE,
                    target: GENESIS_TARGET,
                    merkle_root: hex::decode(GENESIS_MERKLE).unwrap(),
                },
                transactions: txs,
                hash: hex::decode(GENESIS_HASH).unwrap(),
            };

            self.blocks.push(genesis);
        }

        self.rebuild_utxos();
        self.save_all();
    }

    /* ───────── UTXO REBUILD ───────── */

    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear();

        for block in &self.blocks {
            for (tx_index, tx) in block.transactions.iter().enumerate() {
                let txid = hex::encode(tx.txid());

                for input in &tx.inputs {
                    let key = format!(
                        "{}:{}",
                        hex::encode(&input.txid),
                        input.index
                    );
                    self.utxos.remove(&key);
                }

                let is_coinbase =
                    tx_index == 0 && tx.inputs.is_empty();

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

    /* ───────── SAVE ───────── */

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

    /* ───────── FULL CHAIN CHECK ───────── */

    pub fn validate_chain(&self, chain: &[Block]) -> bool {
        if chain.is_empty() {
            return false;
        }

        for i in 1..chain.len() {
            let prev = &chain[i - 1];
            let b = &chain[i];

            if b.header.height != prev.header.height + 1 {
                return false;
            }

            if b.header.prev_hash != prev.hash {
                return false;
            }

            let expected =
                calculate_next_target(&chain[..i]);

            if b.header.target != expected {
                return false;
            }

            if !b.verify_pow() {
                return false;
            }

            if merkle_root(&b.transactions)
                != b.header.merkle_root
            {
                return false;
            }
        }

        true
    }
}
