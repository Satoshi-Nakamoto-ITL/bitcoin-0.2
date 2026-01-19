use crate::crypto::sha256;
use crate::transaction::Transaction;

pub fn merkle_root(
    txs: &[Transaction],
    height: u64,
    prev_hash: &[u8],
) -> Vec<u8> {
    if txs.is_empty() {
        return sha256(&[prev_hash, &height.to_le_bytes()].concat());
    }

    let mut hashes: Vec<Vec<u8>> =
        txs.iter().map(|t| t.txid()).collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            hashes.push(hashes.last().unwrap().clone());
        }

        hashes = hashes
            .chunks(2)
            .map(|pair| {
                sha256(
                    &[
                        pair[0].clone(),
                        pair[1].clone(),
                        height.to_le_bytes().to_vec(),
                    ]
                    .concat(),
                )
            })
            .collect();
    }
    sha256(&[hashes[0].clone(), prev_hash.to_vec()].concat())
}
