# Bitcoin v0.2 — Revelation Edition

Rust-only, Proof-of-Work blockchain implementation Bitcoin.

No governance.
No identities.
No permissions.

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

On first run, the node deterministically creates the revelation block and begins mining locally.
Chain state is stored in the `data/` directory.

## Block 0 (Revelation)

**Revelation Block**

The genesis block is deterministic, reproducible, and verified across multiple independent nodes.

A message is permanently sealed into Proof-of-Work time.

## Consensus Rules

* Proof-of-Work (double SHA-256)
* Longest chain by accumulated work
* UTXO-based ledger model
* Initial subsidy: 50 coins
* Halving every 210,000 blocks
* Difficulty retargeting
* No administrative controls
* No governance
* No permissions

Consensus rules are enforced by code and Proof-of-Work alone.

## Doctrine

**Time + Energy = Truth**
