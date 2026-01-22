use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::PathBuf;

use time::OffsetDateTime;

use crate::consensus::{
    fork_choice::cumulative_work,
    difficulty::calculate_next_difficulty,
};

use crate::{
    block::{Block, BlockHeader},
    pow::mine,
    utxo::{UTXOSet, UTXO},
    transaction::{Transaction, TxOutput},
    reward::block_reward,
    revelation::revelation_tx,
    merkle::merkle_root,
    crypto::sha256,
    crypto::signature::verify_signature,
};

const COINBASE_MATURITY: u64 = 100;

// ─────────────────────────────────────────────────────────────
// BLOCKCHAIN
// ─────────────────────────────────────────────────────────────

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: UTXOSet,
}

// ─────────────────────────────────────────────────────────────
// DATA FILES
// ─────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────
// IMPLEMENTATION
// ─────────────────────────────────────────────────────────────

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            utxos: HashMap::new(),
        }
    }

    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn utxos(&self) -> &UTXOSet {
        &self.utxos
    }

    // ─────────────────────────────────────────────────────────
    // 🔀 REORG / FORK CHOICE
    // ─────────────────────────────────────────────────────────

    pub fn maybe_reorg(&mut self, candidate: Vec<Block>) -> bool {
        if !self.validate_chain(&candidate) {
            return false;
        }

        let current_work = cumulative_work(&self.blocks);
        let candidate_work = cumulative_work(&candidate);

        if candidate_work <= current_work {
            return false;
        }

        self.blocks = candidate;
        self.rebuild_utxos();
        self.save_all();
        true
    }

    // ─────────────────────────────────────────────────────────
    // 🧱 INITIALIZATION
    // ─────────────────────────────────────────────────────────

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
            let difficulty = calculate_next_difficulty(&self.blocks);

            let mut genesis = Block {
                header: BlockHeader {
                    height: 0,
                    timestamp: 1730000000,
                    prev_hash: vec![0u8; 32],
                    nonce: 0,
                    difficulty,
                    merkle_root: merkle_root(&txs),
                },
                transactions: txs,
                hash: vec![],
            };

            mine(&mut genesis);
            self.blocks.push(genesis);
        }

        self.rebuild_utxos();
        self.save_all();
    }

    // ─────────────────────────────────────────────────────────
    // ⛏️ MINING
    // ─────────────────────────────────────────────────────────

    pub fn mine_block(
        &mut self,
        miner_key: &str,
        mempool_txs: Vec<Transaction>,
    ) {
        let height = self.height();
        let prev = self.blocks.last().unwrap();
        let reward = block_reward(height);

        let coinbase = Transaction {
            inputs: vec![],
            outputs: vec![TxOutput {
                value: reward,
                pubkey_hash: sha256(miner_key.as_bytes()),
            }],
        };

        let mut txs = vec![coinbase];

        for tx in mempool_txs {
            if self.validate_transaction(&tx) {
                txs.push(tx);
            }
        }

        let difficulty = calculate_next_difficulty(&self.blocks);

        let mut block = Block {
            header: BlockHeader {
                height,
                timestamp: OffsetDateTime::now_utc().unix_timestamp(),
                prev_hash: prev.hash.clone(),
                nonce: 0,
                difficulty,
                merkle_root: merkle_root(&txs),
            },
            transactions: txs,
            hash: vec![],
        };

        mine(&mut block);

        self.blocks.push(block);
        self.rebuild_utxos();
        self.save_all();
    }

    // ─────────────────────────────────────────────────────────
    // 🔐 TRANSACTION VALIDATION
    // ─────────────────────────────────────────────────────────

    pub fn validate_transaction(&self, tx: &Transaction) -> bool {
        if tx.inputs.is_empty() {
            return true;
        }

        let sighash = tx.sighash();
        let current_height = self.height();
        let mut input_sum = 0u64;
        let mut output_sum = 0u64;

        for input in &tx.inputs {
            let key = format!("{}:{}", hex::encode(&input.txid), input.index);

            let utxo = match self.utxos.get(&key) {
                Some(u) => u,
                None => return false,
            };

            if utxo.is_coinbase &&
                current_height < utxo.height + COINBASE_MATURITY {
                return false;
            }

            if sha256(&input.pubkey) != utxo.pubkey_hash {
                return false;
            }

            if !verify_signature(&sighash, &input.signature, &input.pubkey) {
                return false;
            }

            input_sum += utxo.value;
        }

        for output in &tx.outputs {
            output_sum += output.value;
        }

        input_sum >= output_sum
    }

    // ─────────────────────────────────────────────────────────
    // 🔗 BLOCK VALIDATION
    // ─────────────────────────────────────────────────────────

    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        let expected_height = self.height();
        let prev = self.blocks.last().unwrap();

        if block.header.height != expected_height {
            return false;
        }

        if block.header.prev_hash != prev.hash {
            return false;
        }

        if !block.verify_pow() {
            return false;
        }

        if block.hash != block.hash_header() {
            return false;
        }

        if merkle_root(&block.transactions) != block.header.merkle_root {
            return false;
        }

        if let Some(cb) = block.transactions.first() {
            if !cb.inputs.is_empty() {
                return false;
            }

            let expected = block_reward(block.header.height);
            let actual: u64 = cb.outputs.iter().map(|o| o.value).sum();

            if actual > expected {
                return false;
            }
        }

        for tx in &block.transactions {
            if !self.validate_transaction(tx) {
                return false;
            }
        }

        self.blocks.push(block);
        self.rebuild_utxos();
        self.save_all();
        true
    }

    // ─────────────────────────────────────────────────────────
    // 📦 UTXO REBUILD
    // ─────────────────────────────────────────────────────────

    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear();

        for block in &self.blocks {
            for (tx_index, tx) in block.transactions.iter().enumerate() {
                let txid = hex::encode(tx.txid());

                for input in &tx.inputs {
                    let key = format!("{}:{}", hex::encode(&input.txid), input.index);
                    self.utxos.remove(&key);
                }

                let is_coinbase = tx_index == 0 && tx.inputs.is_empty();

                for (i, output) in tx.outputs.iter().enumerate() {
                    let key = format!("{}:{}", txid, i);
                    self.utxos.insert(
                        key,
                        UTXO {
                            value: output.value,
                            pubkey_hash: output.pubkey_hash.clone(),
                            height: block.header.height,
                            is_coinbase,
                        },
                    );
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────
    // 💾 PERSISTENCE
    // ─────────────────────────────────────────────────────────

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

    // ─────────────────────────────────────────────────────────
    // ✅ FULL CHAIN VALIDATION (FOR REORG)
    // ─────────────────────────────────────────────────────────

    pub fn validate_chain(&self, chain: &[Block]) -> bool {
        if chain.is_empty() {
            return false;
        }

        for i in 1..chain.len() {
            let prev = &chain[i - 1];
            let block = &chain[i];

            if block.header.height != prev.header.height + 1 {
                return false;
            }

            if block.header.prev_hash != prev.hash {
                return false;
            }

            if !block.verify_pow() {
                return false;
            }

            if block.hash != block.hash_header() {
                return false;
            }

            if merkle_root(&block.transactions) != block.header.merkle_root {
                return false;
            }
        }

        true
    }
}
