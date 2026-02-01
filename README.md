# ⛓ Bitcoin v0.3.3 — Revelation Edition

A self-validating Proof-of-Work Layer-1 blockchain implementation  
**Consensus v3 — FINAL / FROZEN**

Repository:
https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2

---

## Overview

Bitcoin Revelation is a minimal, deterministic implementation of a
Proof-of-Work peer-to-peer blockchain.

All Layer-1 consensus rules are finalized, versioned, and enforced locally.
There is no trusted authority, no checkpoints, and no dependency on any
central service.

Nodes independently verify all data.

---

## Consensus Status

- **Consensus Version:** v3
- **Specification Version:** v1.0
- **Status:** FINAL / FROZEN
- **Genesis:** Hard-coded and cryptographically pinned
- **Monetary Policy:** Fixed halving schedule
- **Fork Choice:** Cumulative Proof-of-Work
- **Coinbase Maturity:** Enforced by consensus

⚠️ Any change to consensus rules requires a **version-gated hard fork**.

---

## Canonical Specification

The protocol specification is the **source of truth**.

```

src/spec.rs

```

Any change to this file constitutes a **consensus fork**.

Documentation can be generated via:

```

cargo doc --no-deps

```

---

## What This Repository Contains

- Full validating node
- Proof-of-Work mining
- Deterministic UTXO ledger
- Encrypted HD wallet (BIP-39)
- Coinbase maturity enforcement
- Fee-based mempool
- Transport-agnostic P2P networking
- REST API (non-consensus)
- Persistent chain & UTXO storage

---

## Network Model

- Permissionless
- Peer-to-peer
- No trusted seeds
- No checkpoints
- Offline-capable (store-and-forward)

Connectivity affects **speed**, not **validity**.

---

## Forking Philosophy

Forking is allowed.

Anyone may:
- Run a node
- Mine blocks
- Fork the code
- Compete with alternative rules

Consensus is **local and voluntary**.

---

## Disclaimer

This software is provided **as-is**.

There is:
- No warranty
- No authority
- No guarantee of value
- No recovery mechanism

Cryptographic systems are unforgiving.

---

⛓ *Time is the final judge.*
```
