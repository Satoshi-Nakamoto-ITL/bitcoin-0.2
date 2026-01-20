This document describes the technical design, consensus rules, and known deviations of **Bitcoin v0.2 — Revelation Edition** from Bitcoin Core.

The goal is clarity and explicitness, not protocol compatibility.

## Design Goals

* Minimal and readable implementation
* Explicit consensus rules defined in code
* Deterministic behavior across identical nodes
* Bitcoin-inspired mechanics without historical or network compatibility requirements

Non-goals:

* Bitcoin Core compatibility
* Production hardening
* Economic or security guarantees
* Full historical accuracy

## Architecture Overview

The system is implemented as a single Rust binary with the following core components:

* **Block**: header + transactions
* **BlockHeader**: metadata required for Proof-of-Work
* **Blockchain**: local chain state and validation logic
* **UTXO Set**: tracks spendable outputs
* **Miner**: performs Proof-of-Work locally

Persistent state is stored on disk in the `data/` directory.

## Block Structure

Each block consists of:

* Header

  * Block height
  * Timestamp (Unix epoch)
  * Previous block hash
  * Difficulty parameter
  * Nonce
* Transactions

  * Coinbase transaction
  * Zero or more user transactions

The block hash is computed from the serialized header only.

## Revelation Block

The revelation block is:

* Deterministic
* Reproducible
* Generated locally on first run

All nodes running identical code will produce the same revelation block without coordination.

A static message is embedded in the revelation block and committed to Proof-of-Work time.

## Proof-of-Work

### Hash Function

* Double SHA-256 is used:

  ```
  SHA256(SHA256(block_header))
  ```

### Difficulty Model

Difficulty is expressed as a numeric value representing the required number of leading zero bits in the block hash.

This differs from Bitcoin Core, which compares the block hash against a compact target (`nBits`) using big-integer arithmetic.

The simplified model improves readability and ease of experimentation at the cost of precision and compatibility.

## Mining Algorithm

Mining proceeds by:

1. Serializing the block header
2. Computing the double SHA-256 hash
3. Evaluating the hash against the current difficulty rule
4. Incrementing the nonce until a valid hash is found

Mining is CPU-based and single-threaded by default.

## Chain Selection Rule

The active chain is selected based on:

* Highest accumulated Proof-of-Work

Accumulated work is derived from the difficulty values of blocks in the chain.
This is a simplified approximation and does not use Bitcoin’s exact work calculation.

## Difficulty Adjustment

Difficulty retargeting is implemented using a simplified algorithm:

* Adjustment is based on observed block production time
* The algorithm does not replicate Bitcoin’s 2016-block retargeting window
* Upper and lower bounds may be applied to limit abrupt difficulty changes

The purpose is experimentation rather than precise economic tuning.

## Transaction Model

The system uses a **UTXO-based ledger model**:

* Transactions consume unspent outputs
* Transactions create new outputs
* Double-spends are prevented by UTXO validation

Script execution, signature verification, and Bitcoin Script compatibility are not implemented.

## Block Subsidy

* Initial block subsidy: 50 coins
* Subsidy halving interval: 210,000 blocks
* Subsidy calculation is deterministic and enforced by consensus rules

Transaction fees, if present, are optional and simplified.

## Validation Rules

A block is considered valid if:

* Its hash satisfies the current difficulty requirement
* It references a known previous block revelation
* All transactions are valid under UTXO rules
* The coinbase subsidy matches the expected value
* Consensus rules are enforced entirely by code

No manual overrides or administrative controls exist.

## Networking

At present:

* No peer-to-peer networking layer is implemented
* Nodes operate independently and mine locally
* Chain state is not synchronized between nodes

Future networking support is out of scope for the current version.

## Known Deviations from Bitcoin Core

This project intentionally diverges from Bitcoin Core in the following areas:

* Difficulty representation (leading zero bits vs. target threshold)
* Difficulty retargeting algorithm
* Accumulated work calculation
* Absence of Bitcoin Script
* No mempool or transaction relay
* No peer discovery or networking
* No consensus-critical serialization compatibility

These deviations are intentional and documented.

## Intended Use

This implementation is intended for:

* Educational exploration of Proof-of-Work
* Protocol experimentation
* Systems programming practice in Rust

It is **not** intended for production deployment or economic reliance.

## Summary

Bitcoin v0.2 — Revelation Edition is a concise, Bitcoin-inspired Proof-of-Work blockchain designed for clarity and experimentation.

Consensus emerges from code and computational work alone.

**Time + Energy = Truth**
