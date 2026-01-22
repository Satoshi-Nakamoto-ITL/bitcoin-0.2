use crate::transaction::Transaction;
use crate::utxo::UTXOSet;
use crate::crypto::{verify_signature, pubkey_hash};


pub fn validate_transaction(tx: &Transaction, utxos: &UTXOSet) -> bool {
    let sighash = tx.sighash();

    for input in &tx.inputs {
        let key = format!("{}:{}", hex::encode(&input.txid), input.index);

        let utxo = match utxos.get(&key) {
            Some(u) => u,
            None => return false,
        };

        // Ownership: sha256(pubkey) must match UTXO lock
        if pubkey_hash(&input.pubkey) != utxo.pubkey_hash {
            return false;
        }

        // Signature verification
        if !verify_signature(
            &sighash,
            &input.signature,
            &input.pubkey,
        ) {
            return false;
        }
    }

    true
}
