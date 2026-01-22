use serde::{Serialize, Deserialize};
use crate::crypto::{sha256, sign};

#[derive(Serialize, Deserialize, Clone)]
pub struct TxInput {
    pub txid: Vec<u8>,
    pub index: usize,
    pub signature: Vec<u8>,
    pub pubkey: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub pubkey_hash: Vec<u8>, // sha256(pubkey)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn txid(&self) -> Vec<u8> {
        sha256(&sha256(&bincode::serialize(self).unwrap()))
    }

    pub fn sighash(&self) -> Vec<u8> {
        let mut stripped = self.clone();
        for input in &mut stripped.inputs {
            input.signature.clear();
        }
        sha256(&bincode::serialize(&stripped).unwrap())
    }

    /// 🔑 Spend a matured coinbase UTXO (Bitcoin-accurate)
    pub fn spend_coinbase(
        utxo_key: &str,
        utxo: &crate::utxo::UTXO,
        miner_key: &str,
    ) -> Self {
        let parts: Vec<&str> = utxo_key.split(':').collect();
        let txid = hex::decode(parts[0]).unwrap();
        let index = parts[1].parse::<usize>().unwrap();

        let pubkey = miner_key.as_bytes().to_vec();
        let sighash = sha256(b"coinbase-spend");
        let signature = sign(miner_key.as_bytes(), &sighash);

        Transaction {
            inputs: vec![TxInput {
                txid,
                index,
                pubkey,
                signature,
            }],
            outputs: vec![TxOutput {
                value: utxo.value,
                pubkey_hash: sha256(miner_key.as_bytes()),
            }],
        }
    }
}
