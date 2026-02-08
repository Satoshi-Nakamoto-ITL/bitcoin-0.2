// ─────────────────────────────────────────────
// CONSENSUS v5 — MINER
//
// Produces blocks valid under v4 (pre-fork)
// and v5 (post-fork).
//
// Any change affecting block construction
// after activation is a HARD FORK.
// ─────────────────────────────────────────────

use time::OffsetDateTime;

use crate::core::block::{Block, BlockHeader};
use crate::core::transaction::{Transaction, TxOutput};
use crate::core::utxo::UTXOSet;

use crate::reward::block_reward;
use crate::consensus::difficulty::{
    calculate_next_target,
    calculate_next_target_v5,
};
use crate::consensus::version::{
    consensus_version_for_height,
    ConsensusVersion,
};
use crate::merkle::merkle_root;
use crate::pow::mine;
use crate::validation::validate_transaction;
use crate::policy::{MAX_BLOCK_TXS, MAX_BLOCK_TX_BYTES};

const MIN_FEE_PER_BYTE: i64 = 1; // POLICY ONLY

pub fn mine_block(
    prev_block: &Block,
    utxos: &UTXOSet,
    mempool_txs: Vec<Transaction>,
    miner_pubkey_hash: Vec<u8>,
    chain: &[Block],
) -> Block {
    let height = prev_block.header.height + 1;
    let version = consensus_version_for_height(height);

    let mut selected: Vec<Transaction> = Vec::new();
    let mut total_fees: u64 = 0;
    let mut total_bytes: usize = 0;

    // ───────── Select transactions ─────────
    for tx in mempool_txs {
        if selected.len() >= MAX_BLOCK_TXS {
            break;
        }

        let size = tx.serialized_size();
        if total_bytes + size > MAX_BLOCK_TX_BYTES {
            break;
        }

        if !validate_transaction(&tx, utxos, height) {
            continue;
        }

        let mut input_sum = 0i64;
        let mut output_sum = 0i64;

        for i in &tx.inputs {
            let key = format!(
                "{}:{}",
                hex::encode(&i.txid),
                i.index
            );
            if let Some(u) = utxos.get(&key) {
                input_sum += u.value as i64;
            }
        }

        for o in &tx.outputs {
            output_sum += o.value as i64;
        }

        let fee = input_sum - output_sum;
        if fee <= 0 {
            continue;
        }

        let fee_rate = fee / size as i64;
        if fee_rate < MIN_FEE_PER_BYTE {
            continue;
        }

        total_fees += fee as u64;
        total_bytes += size;
        selected.push(tx);
    }

    // ───────── Coinbase transaction ─────────
    let coinbase_value = block_reward(height) + total_fees;

    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![TxOutput {
            value: coinbase_value,
            pubkey_hash: miner_pubkey_hash,
        }],
    };

    let mut txs = Vec::with_capacity(selected.len() + 1);
    txs.push(coinbase);
    txs.extend(selected);

    // ───────── Difficulty target ─────────
    let target = match version {
        ConsensusVersion::V4 => calculate_next_target(chain),
        ConsensusVersion::V5 => calculate_next_target_v5(chain, height),
    };

    let mut block = Block {
        header: BlockHeader {
            height,
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            prev_hash: prev_block.hash.clone(),
            nonce: 0,
            target,
            merkle_root: merkle_root(&txs),
        },
        transactions: txs,
        hash: vec![],
    };

    mine(&mut block);
    block
}
