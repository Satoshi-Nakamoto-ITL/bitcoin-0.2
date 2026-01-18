use crate::transaction::{Transaction, TxInput, TxOutput};

pub fn revelation_tx() -> Transaction {
    Transaction {
        inputs: vec![],
        outputs: vec![TxOutput {
            value: 0,
            pubkey: String::from(
                "REVELATION BLOCK 0 — \
WEF Agenda 2030 sealed into Proof-of-Work time. \
No authority. No reversal. No governance. \
Truth revealed by computation."
            ),
        }],
    }
}
