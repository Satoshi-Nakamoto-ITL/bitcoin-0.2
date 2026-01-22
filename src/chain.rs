use crate::difficulty;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::PathBuf;
use time::OffsetDateTime;

use crate::{
    block::{Block, BlockHeader},
    pow::mine,
    utxo::{UTXOSet, UTXO},
    transaction::{Transaction, TxOutput},
    reward::block_reward,
    revelation::revelation_tx,
    merkle::merkle_root,
    crypto::{sha256, verify_signature},
};

pub const COINBASE_MATURITY: u64 = 100;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub all_blocks: HashMap<Vec<u8>, Block>,
    pub utxos: UTXOSet,
    pub difficulty: u32,
}

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

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            all_blocks: HashMap::new(),
            utxos: HashMap::new(),
            difficulty: 2,
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
                for b in &self.blocks {
                    self.all_blocks.insert(b.hash.clone(), b.clone());
                }
            }
        }

        if utxos_file().exists() {
            let data = fs::read_to_string(utxos_file()).unwrap();
            if !data.trim().is_empty() {
                self.utxos = serde_json::from_str(&data).unwrap();
            }
        }

        if self.blocks.is_empty() {
            let txs = vec![revelation_tx()];

            let mut genesis = Block {
                header: BlockHeader {
                    height: 0,
                    timestamp: 1730000000,
                    prev_hash: vec![0u8; 32],
                    nonce: 0,
                    difficulty: self.difficulty,
                    merkle_root: merkle_root(&txs),
                },
                transactions: txs,
                hash: vec![],
            };

            mine(&mut genesis);
            self.blocks.push(genesis.clone());
            self.all_blocks.insert(genesis.hash.clone(), genesis);
            self.rebuild_utxos();
            self.save_all();
        }
    }

    /// 🔍 Find a matured coinbase belonging to this miner
    pub fn find_matured_coinbase(
        &self,
        miner_pubkey_hash: &[u8],
    ) -> Option<(String, UTXO)> {
        let height = self.height();

        self.utxos.iter()
            .find(|(_, u)| {
                u.pubkey_hash == miner_pubkey_hash
                    && u.height
                        .map(|h| height >= h + COINBASE_MATURITY)
                        .unwrap_or(false)
            })
            .map(|(k, v)| (k.clone(), v.clone()))
    }

    pub fn mine_block(&mut self, miner_key: &str) {
        let height = self.height();
        let reward = block_reward(height);
        let miner_hash = sha256(miner_key.as_bytes());

        let mut txs = Vec::new();

        // ⛏ Coinbase
        txs.push(Transaction {
            inputs: vec![],
            outputs: vec![TxOutput {
                value: reward,
                pubkey_hash: miner_hash.clone(),
            }],
        });

        // 💸 Spend matured coinbase (if any)
        if let Some((key, utxo)) = self.find_matured_coinbase(&miner_hash) {
            let spend_tx =
                Transaction::spend_coinbase(&key, &utxo, miner_key);
            txs.push(spend_tx);
        }

        let prev = self.blocks.last().unwrap();

        let mut block = Block {
            header: BlockHeader {
                height,
                timestamp: 0,
                prev_hash: prev.hash.clone(),
                nonce: 0,
                difficulty: self.difficulty,
                merkle_root: merkle_root(&txs),
            },
            transactions: txs,
            hash: vec![],
        };

        mine(&mut block);
        block.header.timestamp = OffsetDateTime::now_utc().unix_timestamp();

        let actual_time = block.header.timestamp - prev.header.timestamp;
        self.difficulty =
            difficulty::retarget(self.difficulty, actual_time, 10);

        self.validate_and_add_block(block);
    }

    pub fn validate_transaction(&self, tx: &Transaction) -> bool {
        if tx.inputs.is_empty() {
            return true;
        }

        let current_height = self.height();
        let sighash = tx.sighash();

        for input in &tx.inputs {
            let key = format!("{}:{}", hex::encode(&input.txid), input.index);
            let utxo = match self.utxos.get(&key) {
                Some(u) => u,
                None => return false,
            };

            if let Some(origin_height) = utxo.height {
                if current_height < origin_height + COINBASE_MATURITY {
                    return false;
                }
            }

            if sha256(&input.pubkey) != utxo.pubkey_hash {
                return false;
            }

            if !verify_signature(&input.pubkey, &sighash, &input.signature) {
                return false;
            }
        }

        true
    }

    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        if !block.verify_pow() {
            return false;
        }

        if merkle_root(&block.transactions) != block.header.merkle_root {
            return false;
        }

        for tx in &block.transactions {
            if !self.validate_transaction(tx) {
                return false;
            }
        }

        let hash = block.hash.clone();
        self.all_blocks.insert(hash.clone(), block.clone());

        let candidate = match self.build_chain_from_tip(hash) {
            Some(c) => c,
            None => return false,
        };

        if Self::total_work(&candidate) > Self::total_work(&self.blocks) {
            self.blocks = candidate;
            self.rebuild_utxos();
            self.save_all();
        }

        true
    }

    fn total_work(chain: &[Block]) -> u128 {
        chain.iter().map(|b| b.header.difficulty as u128).sum()
    }

    fn build_chain_from_tip(&self, tip: Vec<u8>) -> Option<Vec<Block>> {
        let mut chain = Vec::new();
        let mut current = tip;

        loop {
            let block = self.all_blocks.get(&current)?.clone();
            let prev = block.header.prev_hash.clone();
            chain.push(block);
            if prev == vec![0u8; 32] { break; }
            current = prev;
        }

        chain.reverse();
        Some(chain)
    }

    pub fn rebuild_utxos(&mut self) {
        self.utxos.clear();

        for block in &self.blocks {
            for tx in &block.transactions {
                let txid = hex::encode(tx.txid());

                for input in &tx.inputs {
                    let key = format!("{}:{}", hex::encode(&input.txid), input.index);
                    self.utxos.remove(&key);
                }

                for (i, output) in tx.outputs.iter().enumerate() {
                    let key = format!("{}:{}", txid, i);
                    self.utxos.insert(
                        key,
                        UTXO {
                            value: output.value,
                            pubkey_hash: output.pubkey_hash.clone(),
                            height: if block.header.height == 0 {
                                None // genesis stays immature forever
                            } else {
                                Some(block.header.height)
                            },
                        },
                    );
                }
            }
        }
    }

    pub fn save_all(&self) {
        fs::create_dir_all(data_dir()).unwrap();
        fs::write(blocks_file(), serde_json::to_string_pretty(&self.blocks).unwrap()).unwrap();
        fs::write(utxos_file(), serde_json::to_string_pretty(&self.utxos).unwrap()).unwrap();
    }
}
