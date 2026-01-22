use serde::{Serialize, Deserialize};

use crate::transaction::Transaction;
use crate::crypto::sha256;
use crate::policy::{MAX_BLOCK_TX_BYTES, MAX_BLOCK_TXS};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: i64,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
    pub difficulty: u32,
    pub merkle_root: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Block {
    pub fn mine_block(
        prev_hash: Vec<u8>,
        transactions: Vec<Transaction>,
        difficulty: u32,
        height: u64,
    ) -> Block {
        let mut selected = Vec::new();
        let mut total_bytes = 0usize;

        for tx in transactions {
            let tx_size = tx.serialized_size();

            if selected.len() >= MAX_BLOCK_TXS {
                break;
            }

            if total_bytes + tx_size > MAX_BLOCK_TX_BYTES {
                break;
            }

            total_bytes += tx_size;
            selected.push(tx);
        }

        let mut block = Block {
            header: BlockHeader {
                height,
                timestamp: now(),
                prev_hash,
                nonce: 0,
                difficulty,
                merkle_root: vec![],
            },
            transactions: selected,
            hash: vec![],
        };

        block.header.merkle_root = block.calculate_merkle_root();
        block.mine();

        block
    }

    pub fn mine(&mut self) {
        loop {
            let hash = self.hash_header();
            if leading_zero_bits(&hash) >= self.header.difficulty {
                self.hash = hash;
                break;
            }
            self.header.nonce += 1;
        }
    }

    pub fn hash_header(&self) -> Vec<u8> {
        sha256(&sha256(&bincode::serialize(&self.header).unwrap()))
    }

    pub fn verify_pow(&self) -> bool {
        self.hash == self.hash_header()
            && leading_zero_bits(&self.hash) >= self.header.difficulty
    }

    pub fn calculate_merkle_root(&self) -> Vec<u8> {
        if self.transactions.is_empty() {
            return vec![0u8; 32];
        }

        let mut hashes: Vec<Vec<u8>> =
            self.transactions.iter().map(|tx| tx.txid()).collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                hashes.push(hashes.last().unwrap().clone());
            }

            hashes = hashes
                .chunks(2)
                .map(|pair| sha256(&[pair[0].as_slice(), pair[1].as_slice()].concat()))
                .collect();
        }

        hashes[0].clone()
    }
}

fn leading_zero_bits(hash: &[u8]) -> u32 {
    let mut count = 0;
    for byte in hash {
        let z = byte.leading_zeros();
        count += z;
        if z < 8 {
            break;
        }
    }
    count
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
