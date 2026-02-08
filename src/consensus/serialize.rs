// ─────────────────────────────────────────────
// CONSENSUS v5 — SERIALIZATION
//
// Defines explicit serializers for:
// - txid (NO signatures)
// - sighash (FULL context)
//
// Any change to this file is a HARD FORK.
// ─────────────────────────────────────────────

use crate::core::transaction::{Transaction, TxInput, TxOutput};
use crate::core::block::BlockHeader;

// ───────── Primitive writers ─────────

fn write_u64_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_i64_le(v: i64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_u32_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_bytes(bytes: &[u8], out: &mut Vec<u8>) {
    write_u32_le(bytes.len() as u32, out);
    out.extend_from_slice(bytes);
}

// ───────── Block header (UNCHANGED) ─────────

/// Serialize block header EXACTLY for hashing (CONSENSUS)
pub fn serialize_block_header(header: &BlockHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);

    write_u64_le(header.height, &mut out);
    write_i64_le(header.timestamp, &mut out);
    write_bytes(&header.prev_hash, &mut out);
    write_u64_le(header.nonce, &mut out);
    out.extend_from_slice(&header.target);
    write_bytes(&header.merkle_root, &mut out);

    out
}

// ───────── Transaction serializers ─────────

/// Serialize transaction for txid (CONSENSUS v5)
///
/// EXCLUDES:
/// - input.pubkey
/// - input.signature
pub fn serialize_transaction_for_txid(tx: &Transaction) -> Vec<u8> {
    let mut out = Vec::new();

    // Inputs
    write_u32_le(tx.inputs.len() as u32, &mut out);
    for i in &tx.inputs {
        serialize_input_for_txid(i, &mut out);
    }

    // Outputs
    write_u32_le(tx.outputs.len() as u32, &mut out);
    for o in &tx.outputs {
        serialize_output(o, &mut out);
    }

    out
}

/// Serialize transaction for sighash (CONSENSUS v5)
///
/// INCLUDES full input context
pub fn serialize_transaction_for_sighash(tx: &Transaction) -> Vec<u8> {
    let mut out = Vec::new();

    write_u32_le(tx.inputs.len() as u32, &mut out);
    for i in &tx.inputs {
        serialize_input_full(i, &mut out);
    }

    write_u32_le(tx.outputs.len() as u32, &mut out);
    for o in &tx.outputs {
        serialize_output(o, &mut out);
    }

    out
}

// ───────── Input variants ─────────

fn serialize_input_for_txid(i: &TxInput, out: &mut Vec<u8>) {
    write_bytes(&i.txid, out);
    write_u32_le(i.index, out);
}

fn serialize_input_full(i: &TxInput, out: &mut Vec<u8>) {
    write_bytes(&i.txid, out);
    write_u32_le(i.index, out);
    write_bytes(&i.pubkey, out);
    write_bytes(&i.signature, out);
    write_u32_le(i.address_index, out);
}

// ───────── Output ─────────

fn serialize_output(o: &TxOutput, out: &mut Vec<u8>) {
    write_u64_le(o.value, out);
    write_bytes(&o.pubkey_hash, out);
}
