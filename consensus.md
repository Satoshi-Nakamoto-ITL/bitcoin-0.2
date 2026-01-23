# Consensus Rules

This document defines the rules by which nodes determine whether a block and a chain are valid.

Consensus is achieved independently by each node through verification of these rules.

Nodes do not trust peers.
All validation is local.

---

## Block Validity

A block is considered valid if all of the following conditions are met:

* The block header references the hash of the previous block
* The block height is exactly one greater than the previous block
* The block contains at least one transaction
* The Merkle root matches the transactions included
* The proof-of-work hash satisfies the stated difficulty
* The difficulty value matches the expected difficulty derived from prior blocks

Blocks failing any of these checks are rejected.

---

## Proof-of-Work

Proof-of-work is performed by repeatedly hashing the block header with a changing nonce.

A block is valid only if its hash contains a sufficient number of leading zero bits as defined by the difficulty parameter.

Difficulty is adjusted deterministically based on prior blocks.

---

## Coinbase Maturity (Consensus v2)

Outputs created by coinbase transactions are subject to a mandatory maturity period.

A coinbase output may only be spent after a fixed number of blocks have been mined after its creation.

This rule is enforced during block validation.

Coinbase maturity enforcement is activated by block height and does not invalidate historical blocks.

---

## Chain Validity

A chain is valid if:

* All blocks are individually valid
* Blocks are correctly linked in sequence
* The Revelation Block matches the expected initial state

---

## Chain Selection

When multiple valid chains are observed, the node selects the chain with the greatest accumulated proof-of-work.

If chains have equal work, the first received chain is retained.

---

## Finality

Blocks are considered increasingly irreversible as more blocks are added after them.

Reorganizations may occur but are expected to become less frequent as depth increases.

---

## Summary

Consensus emerges from independent verification.

There are no trusted parties, checkpoints, or coordinators.

If all nodes apply these rules consistently, the system converges on a single history.

---

### Status

This document reflects **Consensus v2**, activated at block height **1000**.

---
