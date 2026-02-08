// ─────────────────────────────────────────────
// CONSENSUS v5 — TRANSACTION HASHING
//
// txid  ≠ sighash
//
// txid:
//   - excludes signatures & pubkeys
//   - used for UTXO / Merkle / blocks
//
// sighash:
//   - includes full transaction context
//   - used ONLY for signing & verification
//
// Any change to this file is a HARD FORK.
// ─────────────────────────────────────────────

use serde::{Serialize, Deserialize};
use crate::crypto::sha256;
use crate::consensus::serialize::{
    serialize_transaction_for_txid,
    serialize_transaction_for_sighash,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxInput {
    pub txid: Vec<u8>,
    pub index: u32,

    // NOT part of txid
    pub pubkey: Vec<u8>,
    pub signature: Vec<u8>,

    // Wallet metadata (non-consensus but serialized)
    pub address_index: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: u64,
    pub pubkey_hash: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    /// Transaction ID (CONSENSUS v5)
    ///
    /// Used for:
    /// - UTXO references
    /// - Merkle root
    /// - Block commitment
    ///
    /// Signatures and pubkeys are EXCLUDED.
    pub fn txid(&self) -> Vec<u8> {
        sha256(&serialize_transaction_for_txid(self))
    }

    /// Signature hash (CONSENSUS v5)
    ///
    /// Used ONLY for:
    /// - signing
    /// - signature verification
    ///
    /// Includes full transaction context.
    pub fn sighash(&self) -> Vec<u8> {
        sha256(&serialize_transaction_for_sighash(self))
    }

    /// Estimated serialized size (POLICY ONLY)
    pub fn serialized_size(&self) -> usize {
        self.inputs.len() * 148 + self.outputs.len() * 34 + 10
    }
}
