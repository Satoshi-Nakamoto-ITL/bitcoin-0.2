use std::collections::HashSet;

use secp256k1::PublicKey;

use crate::core::transaction::Transaction;
use crate::core::utxo::UTXOSet;
use crate::crypto::{verify_signature, pubkey_hash};

const COINBASE_MATURITY: u64 = 100;

/// ⚠️ CONSENSUS — v4 / v5
/// Transaction validation rules
pub fn validate_transaction(
    tx: &Transaction,
    utxos: &UTXOSet,
    current_height: u64,
) -> bool {
    // Coinbase tx
    if tx.inputs.is_empty() {
        return true;
    }

    let sighash = tx.sighash();
    let mut input_sum: u64 = 0;
    let mut output_sum: u64 = 0;

    let mut seen_outpoints = HashSet::new();

    for input in &tx.inputs {
        let key = format!(
            "{}:{}",
            hex::encode(&input.txid),
            input.index
        );

        // Prevent same-UTXO double spend inside tx
        if !seen_outpoints.insert(key.clone()) {
            return false;
        }

        let utxo = match utxos.get(&key) {
            Some(u) => u,
            None => return false,
        };

        // Coinbase maturity rule
        if utxo.is_coinbase {
            if current_height < utxo.height + COINBASE_MATURITY {
                return false;
            }
        }

        let pubkey = match PublicKey::from_slice(&input.pubkey) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        if pubkey_hash(&pubkey) != utxo.pubkey_hash {
            return false;
        }

        if !verify_signature(
            &sighash,
            &input.signature,
            &pubkey.serialize(),
        ) {
            return false;
        }

        input_sum = input_sum.saturating_add(utxo.value);
    }

    for output in &tx.outputs {
        output_sum = output_sum.saturating_add(output.value);
    }

    input_sum >= output_sum
}
