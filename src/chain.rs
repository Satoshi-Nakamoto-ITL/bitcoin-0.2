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
};

pub struct Blockchain {
    pub blocks: Vec<Block>,
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
                println!("Loaded chain at height {}", self.blocks.len());
            }
        }

        if utxos_file().exists() {
            let data = fs::read_to_string(utxos_file()).unwrap();
            if !data.trim().is_empty() {
                self.utxos = serde_json::from_str(&data).unwrap();
                return;
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
            self.blocks.push(genesis);
            self.rebuild_utxos();
            self.save_all();
        }
    }

    pub fn mine_block(&mut self, miner_key: &str) {
        let height = self.blocks.len() as u64;
        let reward = block_reward(height);

        let coinbase = Transaction {
            inputs: vec![],
            outputs: vec![TxOutput {
                value: reward,
                pubkey: miner_key.to_string(),
            }],
        };

        let prev = self.blocks.last().unwrap();
        let txs = vec![coinbase];

        let mut block = Block {
            header: BlockHeader {
                height,
                timestamp: OffsetDateTime::now_utc().unix_timestamp(),
                prev_hash: prev.hash.clone(),
                nonce: 0,
                difficulty: self.difficulty,
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

    pub fn validate_and_add_block(&mut self, block: Block) -> bool {
        let expected_height = self.blocks.len() as u64;
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

        if merkle_root(&block.transactions) != block.header.merkle_root {
            return false;
        }

        self.blocks.push(block);
        self.rebuild_utxos();
        self.save_all();
        true
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
                            pubkey: output.pubkey.clone(),
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
