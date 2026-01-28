## ⛓ Bitcoin v0.3.3 — Revelation Edition  

Stable Node, Wallet, Mining & Public P2P Network

Consensus v3 — FINAL / FROZEN

Repository: 👉 https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2

---

## Overview

Bitcoin Revelation v0.3.3 is a stable release that finalizes the wallet, transaction, mempool, mining, API, and P2P networking layers on top of a **frozen Layer-1 Consensus v3**.

This release hard-codes the Genesis block and permanently freezes all consensus rules.  
No chain reset is required and all existing coins remain valid.

A live public seed node is available, allowing anyone to join and mine without manual peer coordination.

✅ No consensus rule changes  
✅ No chain reset required  
✅ Safe upgrade from v0.3.2  
✅ Public P2P network online  

---

## 🌍 Public Seed Node (Live)



bitcoin-0-2.fly.dev:8333


This node:
- Accepts inbound P2P connections
- Shares peer addresses (`getaddr`)
- Does not mine
- Does not alter consensus
- Exists purely for decentralized bootstrapping

---

## Release Status

**v0.3.3 is a finalization release following v0.3.2.**

This release:
- Hard-codes the Genesis block
- Freezes Consensus v3 permanently
- Marks the protocol layer as final

Any future consensus change requires a version-gated fork.

---

## What’s Included

- Deterministic HD wallets (BIP39)
- Encrypted wallet storage (AES-256-GCM + PBKDF2)
- ECDSA transaction signing & validation
- Coinbase maturity enforcement
- Mempool validation & transaction relay
- Miner transaction selection from mempool
- Full P2P block & transaction propagation
- Public seed node support
- REST API block explorer
- Command-line wallet interface
- Persistent blockchain & UTXO storage

---

## Wallet System

- Recovery phrase (BIP39) is shown **once** on creation
- Phrase is **never stored**
- Wallet data is encrypted and saved locally

If the recovery phrase is lost, funds are unrecoverable.

---

## REST API

Default:


http://127.0.0.1:8080


Endpoints:
- `/status`
- `/blocks`
- `/block/height/{n}`
- `/tx/{txid}`
- `/address/{pubkey_hash}`

---

## ⛏️ Mining & Full Node

⚠️ Always build from **release tags**, not `main`.

👉 https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2/tags  
Recommended: `v0.3.3`

---

## Backward Compatibility

- Compatible with v0.3.0+
- No fork
- No replay risk

---

## Release Identifier

- Tag: `v0.3.3`
- Client: Bitcoin Revelation v0.3.3
- Consensus: v3 (FINAL / FROZEN)

---

## Disclaimer

This software is provided as-is for research and independent node operation.

There is no warranty, no central authority, and no permission system.

⛓ Satoshi-Nakamoto-ITL

