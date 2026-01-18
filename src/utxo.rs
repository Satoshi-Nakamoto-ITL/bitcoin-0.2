use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UTXO {
    pub value: u64,
    pub pubkey: String,
}

pub type UTXOSet = HashMap<String, UTXO>;
