# Bitcoin v0.2 — Revelation Edition

A **Bitcoin-inspired**, Rust-only Proof-of-Work blockchain implementation for experimentation and research.

This project is **not** a reimplementation of Bitcoin Core and is **not protocol-compatible** with the Bitcoin network.
It explores core Bitcoin concepts using a simplified and readable design.

## Features

* Proof-of-Work mining (double SHA-256)
* Longest chain selection by accumulated work
* UTXO-based ledger model
* Deterministic, reproducible genesis block
* Local, single-node operation
* No identities, permissions, or administrative controls

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

On first run, the node deterministically creates the revelation block and begins mining locally.
Chain state is persisted in the `data/` directory.

## Revelation Block 

The revelation block is deterministic and reproducible across independent nodes running identical code.

A message is sealed into Proof-of-Work time as part of the genesis block construction.

## Consensus Model

This implementation draws inspiration from Bitcoin while simplifying certain mechanisms for clarity.

* Proof-of-Work: double SHA-256
* Chain selection: highest accumulated work
* Ledger model: UTXO
* Initial block subsidy: 50 coins
* Subsidy halving interval: 210,000 blocks
* Difficulty adjustment: simplified retargeting algorithm
* Consensus enforcement: code and Proof-of-Work only

No governance mechanisms or privileged roles exist outside the protocol rules enforced by the software.

## Scope and Intent

This project is intended for:

* Educational use
* Protocol experimentation
* Systems and consensus research

It is **not** intended for production use, economic guarantees, or compatibility with existing Bitcoin infrastructure.

## Doctrine

**Time + Energy = Truth**
