use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::crypto::sha256;

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
    pub fn hash_header(&self) -> Vec<u8> {
        sha256(&sha256(&bincode::serialize(&self.header).unwrap()))
    }
}
