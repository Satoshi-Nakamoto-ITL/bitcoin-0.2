//! ─────────────────────────────────────────────
//! ⛓ Bitcoin Revelation — Consensus Specification v5
//!
//! Status: FINAL / FROZEN
//! Activation: Height-based hard fork
//!
//! This file defines ALL consensus rules
//! active from CONSENSUS_V5_HEIGHT onward.
//!
//! Any change to these rules REQUIRES
//! a new consensus version and a hard fork.
//!
//! Anything not explicitly stated here
//! is NON-CONSENSUS policy.
//! ─────────────────────────────────────────────

/* ─────────────────────────────────────────────
   1. Scope
   ─────────────────────────────────────────────

Consensus v5 applies to all blocks with:

    height ≥ CONSENSUS_V5_HEIGHT

Blocks below this height are validated
under Consensus v4 rules.

Consensus v5 does NOT retroactively
invalidate historical blocks.
*/

/* ─────────────────────────────────────────────
   2. Genesis
   ─────────────────────────────────────────────

The genesis block remains unchanged.

Nodes MUST:
- hard-code the genesis block
- verify its hash
- verify its Proof-of-Work
- verify its merkle root

Genesis is immutable forever.
*/

/* ─────────────────────────────────────────────
   3. Block Structure
   ─────────────────────────────────────────────

A block consists of:
- header
- ordered list of transactions

Block header fields:
- height
- timestamp
- prev_hash
- nonce
- target
- merkle_root

The header is serialized manually
and hashed using double SHA-256.
*/

/* ─────────────────────────────────────────────
   4. Height Rules (NEW)
   ─────────────────────────────────────────────

Block height is DERIVED, not trusted.

Rules:
- Genesis height = 0
- height(block) = height(prev_block) + 1

The height field in the header MUST
match the derived height exactly.

Any mismatch is INVALID.
*/

/* ─────────────────────────────────────────────
   5. Proof-of-Work
   ─────────────────────────────────────────────

Hash function:
- double SHA-256

Hash and target:
- exactly 32 bytes
- interpreted as unsigned 256-bit
  BIG-ENDIAN integers

Validity rule:
    hash ≤ target

Endianness is CONSENSUS-CRITICAL.
*/

/* ─────────────────────────────────────────────
   6. Difficulty Adjustment (FIXED)
   ─────────────────────────────────────────────

Difficulty operates on the FULL
256-bit target.

Target for block at height H
is computed using the chain ending
at height H − 1.

Adjustment interval:
- fixed
- adjustments occur ONLY when
  H % INTERVAL == 0

Formula:

    new_target =
        old_target × actual_time
        ───────────────────────
           expected_time

Where:
- actual_time is clamped to
  [expected_time / 4, expected_time × 4]
- all arithmetic is integer-only
- result is clamped to
  [MIN_TARGET, MAX_TARGET]
*/

/* ─────────────────────────────────────────────
   7. Timestamp Rules
   ─────────────────────────────────────────────

For every block:
- timestamp > Median Time Past
- timestamp ≤ now + MAX_FUTURE_DRIFT

Median Time Past is computed using
the last MTP_WINDOW blocks.
*/

/* ─────────────────────────────────────────────
   8. Transactions
   ─────────────────────────────────────────────

Model:
- UTXO-based

Rules:
- inputs must reference unspent outputs
- signatures must verify
- input sum ≥ output sum
- fees are implicit
*/

/* ─────────────────────────────────────────────
   9. Transaction Hashing (FIXED)
   ─────────────────────────────────────────────

Consensus v5 defines TWO hashes:

1. txid
   - excludes signatures and pubkeys
   - used for:
       • UTXO references
       • Merkle root
       • block commitment

2. sighash
   - includes full transaction context
   - used ONLY for signing and verification

Signatures MUST NEVER be included
in txid computation.
*/

/* ─────────────────────────────────────────────
   10. Coinbase Rules (ENFORCED)
   ─────────────────────────────────────────────

- Coinbase MUST be the first transaction
- Coinbase MUST have zero inputs
- Coinbase MUST have ≥ 1 output

Total coinbase output value MUST equal:

    block_reward(height) + total_fees(block)

Any deviation is INVALID.

Coinbase outputs are locked by
COINBASE_MATURITY blocks.
*/

/* ─────────────────────────────────────────────
   11. Fork Choice (HARDENED)
   ─────────────────────────────────────────────

Fork choice is based on
CUMULATIVE PROOF-OF-WORK.

Cumulative work is defined as:

    Σ (2^256 / (target + 1))

Before comparison, a chain MUST:
- be structurally valid
- have consecutive heights
- have valid difficulty at each height
- satisfy PoW for every block

Height alone is NEVER authoritative.
*/

/* ─────────────────────────────────────────────
   12. Serialization
   ─────────────────────────────────────────────

Consensus serialization is:
- manual
- deterministic
- versioned
- fixed field order
- fixed endianness

Serialization is part of consensus.

Any change is a HARD FORK.
*/

/* ─────────────────────────────────────────────
   13. Explicit Non-Consensus
   ─────────────────────────────────────────────

The following are NOT consensus:
- mempool policy
- fee rate policy
- mining transaction selection
- networking transports
- APIs / RPC / UI
- reorg depth limits

Differences here MUST NOT
affect block validity.
*/

/* ─────────────────────────────────────────────
   14. Finality
   ─────────────────────────────────────────────

Consensus v5 is FINAL and FROZEN.

Any change requires:
- new consensus version
- explicit hard fork
- new activation height

There are no silent upgrades.
*/
