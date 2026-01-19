use crate::crypto::sha256;
use crate::transaction::Transaction;

pub fn merkle_root(txs: &[Transaction]) -> Vec<u8> {
    if txs.is_empty() {
        return vec![0u8; 32];
    }

    let mut hashes: Vec<Vec<u8>> =
        txs.iter().map(|t| t.txid()).collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            hashes.push(hashes.last().unwrap().clone());
        }

        hashes = hashes
            .chunks(2)
            .map(|pair| sha256(&[pair[0].clone(), pair[1].clone()].concat()))
            .collect();
    }

    hashes[0].clone()
}