use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TxInput {
    pub txid: Vec<u8>,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

use crate::crypto::sha256;

impl Transaction {
    pub fn txid(&self) -> Vec<u8> {
    sha256(&sha256(&bincode::serialize(self).unwrap()))
}
}
