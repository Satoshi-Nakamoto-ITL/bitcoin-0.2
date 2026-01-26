
# ⛓ Bitcoin v0.3.2 — Revelation Edition

### Stable Node, Wallet & Transaction Layer

**Consensus v3 — Frozen**

**Repository:**
👉 [https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2](https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2)

---

## Overview

**Bitcoin Revelation v0.3.2** is a stable, non-forking release that activates the full **wallet, transaction, mempool, mining, API, and P2P networking layers** on top of a **frozen Layer-1 Consensus v3**.

* ✅ No consensus rules are modified
* ✅ No chain reset is required
* ✅ Safe for long-running nodes

---

## Release Status

Version **0.3.2** is a stabilization and integration release following **v0.3.1**.

### What’s Included

* Deterministic HD wallets (BIP39)
* Encrypted wallet storage (AES-GCM + PBKDF2)
* ECDSA transaction signing & validation
* Coinbase maturity enforcement
* Mempool validation & transaction relay
* Miner selection from mempool
* Full P2P block & transaction propagation
* REST API block explorer
* Command-line wallet interface
* Persistent chain & UTXO storage

### What’s NOT Changed

* ❌ No consensus rule changes
* ❌ No reward schedule changes
* ❌ No difficulty changes
* ❌ No protocol fork

➡ **Consensus v3 remains frozen**

---

## Transaction Layer

The transaction system is fully operational and enforced by nodes.

### UTXO Ownership Model

Each output is locked to a **public key hash (PKH)**.

To spend an output, a transaction must:

* Reveal the corresponding public key
* Provide a valid ECDSA signature
* Satisfy coinbase maturity rules

---

## Wallet System

The wallet operates **above consensus** and does not alter validation rules.

### Features

* BIP39 mnemonic recovery phrase
* Hierarchical deterministic key derivation
* Automatic change outputs
* AES-256-GCM encrypted wallet file
* PBKDF2 password hardening
* Secure memory locking (`mlock`)
* Automatic lock on inactivity

---

## Transaction Flow

1. Wallet selects spendable UTXOs
2. Inputs are signed locally
3. Node validates ownership & signatures
4. Transaction enters the mempool
5. Mempool applies policy rules
6. Miner selects transactions
7. Block is mined under **Consensus v3**
8. UTXO set updates deterministically

---

## Mempool Policy (Non-Consensus)

* Double-spend prevention
* Transaction size limits
* Fee-rate based selection
* Reorg-safe transaction handling

⚠ These rules are **local policy**, not consensus.

---

## Command-Line Wallet

Built-in wallet CLI:

```bash
cargo run wallet balance
cargo run wallet send <pubkey_hash_hex> <amount>
```

Wallet commands interact with the **local node and mempool**.

---

## REST API (Explorer)

**Default endpoint:**

```
http://127.0.0.1:8080
```

**Available endpoints:**

```
/status
/blocks
/block/height/{n}
/tx/{txid}
/address/{pubkey_hash}
```

---

# Installation & Running the Node

## Requirements (All Platforms)

* Internet connection
* ~200 MB disk space
* Rust toolchain (stable)

---

## 📱 Termux (Android)

### 1️⃣ Install dependencies

```bash
pkg update && pkg upgrade
pkg install git rust clang openssl pkg-config
```

### 2️⃣ Clone repository

```bash
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2.git
cd bitcoin-0.2
```

### 3️⃣ Build & run

```bash
cargo run
```

The node will:

* Create a wallet
* Start P2P networking
* Start mining
* Launch API on port `8080`

---

## 💻 Linux / macOS

### 1️⃣ Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
```

### 2️⃣ Clone repository

```bash
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2.git
cd bitcoin-0.2
```

### 3️⃣ Run node

```bash
cargo run
```

---

## 🪟 Windows (PowerShell)

### 1️⃣ Install Rust

Download and install:
👉 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Restart PowerShell after installation.

### 2️⃣ Clone repository

```powershell
git clone https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2.git
cd bitcoin-0.2
```

### 3️⃣ Run node

```powershell
cargo run
```

---

## 🔗 Connecting to a Peer

```bash
cargo run -- --connect IP:PORT
```

**Example:**

```bash
cargo run -- --connect 203.0.113.5:8333
```

---

## Data Storage

All node data is stored locally:

```
data/
 ├─ blocks.json
 ├─ utxos.json
 └─ wallet.dat
```

⚠ Deleting `data/` resets the node state.

---

## Backward Compatibility

* Fully compatible with v0.3.0+ peers
* No fork, no replay risk
* Existing chains remain valid

---

## Release Identifier

* **Tag:** `v0.3.2`
* **Client:** Bitcoin Revelation v0.3.2
* **Consensus:** v3 (frozen)

---

## Scope of This Release

* **v0.3.0** → Base layer stabilization
* **v0.3.1** → Wallet & transaction activation
* **v0.3.2** → Stable integrated node release

---

## Disclaimer

This software is provided **as-is** for research, experimentation, and independent node operation.

There is:

* No warranty
* No central authority
* No permission system

**The rules are enforced by code, not humans.**

---

**Satoshi-Nakamoto-ITL**
*Bitcoin v0.3.2 — Revelation Edition*
