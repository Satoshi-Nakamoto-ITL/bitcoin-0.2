use crate::transaction::Transaction;
use crate::utxo::UTXOSet;
use crate::policy::MAX_TX_SIZE;
use crate::validation::validate_transaction;

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct MempoolEntry {
    pub tx: Transaction,
    pub fee: i64,
    pub size: usize,
    pub timestamp: i64,
}

pub struct Mempool {
    entries: Vec<MempoolEntry>,

    // (txid, vout index) — must match TxInput types exactly
    spent_outpoints: HashSet<(Vec<u8>, usize)>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            spent_outpoints: HashSet::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Policy-only admission (NO consensus logic here)
    pub fn add_transaction(
        &mut self,
        tx: Transaction,
        utxos: &UTXOSet,
    ) -> bool {
        let size = tx.serialized_size();
        if size > MAX_TX_SIZE {
            return false;
        }

        // Consensus validation (signatures, maturity, inflation, etc.)
        if !validate_transaction(&tx, utxos) {
            return false;
        }

        // Prevent mempool double-spends
        for input in &tx.inputs {
            let key = (input.txid.clone(), input.index);
            if self.spent_outpoints.contains(&key) {
                return false;
            }
        }

        let fee = calculate_fee(&tx, utxos);

        // Policy: reject zero or negative fee txs
        if fee <= 0 {
            return false;
        }

        // Accept transaction
        for input in &tx.inputs {
            self.spent_outpoints
                .insert((input.txid.clone(), input.index));
        }

        self.entries.push(MempoolEntry {
            tx,
            fee,
            size,
            timestamp: now(),
        });

        true
    }

    /// Fee-rate sorted (sat/byte), integer-only, deterministic
    pub fn sorted_for_mining(&self) -> Vec<Transaction> {
        let mut entries = self.entries.clone();

        entries.sort_by(|a, b| {
            // Compare a.fee / a.size vs b.fee / b.size without floats
            let lhs = a.fee * b.size as i64;
            let rhs = b.fee * a.size as i64;
            rhs.cmp(&lhs)
        });

        entries.into_iter().map(|e| e.tx).collect()
    }

    /// Remove confirmed transactions after block acceptance
    pub fn remove_confirmed(&mut self, confirmed: &[Transaction]) {
        for tx in confirmed {
            for input in &tx.inputs {
                self.spent_outpoints
                    .remove(&(input.txid.clone(), input.index));
            }
        }

        self.entries.retain(|entry| {
            !confirmed.iter().any(|tx| tx.txid() == entry.tx.txid())
        });
    }
}

/// Deterministic fee calculation (policy-only)
fn calculate_fee(tx: &Transaction, utxos: &UTXOSet) -> i64 {
    let mut input_sum = 0i64;
    let mut output_sum = 0i64;

    for input in &tx.inputs {
        let key = format!("{}:{}", hex::encode(&input.txid), input.index);
        let utxo = utxos
            .get(&key)
            .expect("validated transaction must reference existing UTXO");

        input_sum += utxo.value as i64;
    }

    for output in &tx.outputs {
        output_sum += output.value as i64;
    }

    input_sum - output_sum
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
