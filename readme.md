
# Bitcoin v0.3.1 — Revelation Edition

### Wallet and Transaction Layer Activated

**Repository:**
[https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2](https://github.com/Satoshi-Nakamoto-ITL/bitcoin-0.2)

This release activates the transaction and wallet layer on top of the stabilized **Layer-1 Consensus v3**.

No consensus rules are modified.
Layer-1 remains frozen.

---

## Release Status

Version 0.3.1 introduces a complete end-to-end transaction flow while preserving full compatibility with v0.3.0 nodes.

New capabilities in this release:

* Deterministic HD wallets
* Encrypted on-disk wallet storage
* Transaction creation and ECDSA signing
* Mempool transaction validation and fee policy
* Peer-to-peer transaction relay
* Miner inclusion of mempool transactions into blocks
* Command-line wallet interface

Layer-1 validation rules are unchanged.

---

## Transaction Layer

The transaction system is fully operational.

### UTXO Ownership Model

Each output is locked to a public key hash (PKH).
Spending requires:

* Revealing the corresponding public key
* Providing a valid ECDSA signature

### Node Enforcement

For every transaction input, nodes verify:

* `hash(pubkey) == referenced UTXO owner`
* Signature validity against the transaction sighash
* Total inputs greater than or equal to total outputs

These rules establish cryptographic ownership without introducing trusted parties.

---

## Wallet System

Version 0.3.1 includes a deterministic wallet implementation:

* BIP39 mnemonic seed generation
* Hierarchical deterministic key derivation
* Automatic change address generation
* AES-GCM encrypted wallet storage
* PBKDF2 password-based key derivation
* Memory locking of sensitive key material
* Automatic wallet auto-lock timeout

Wallet behavior operates above consensus and does not alter validation rules.

---

## Transaction Flow

1. Wallet selects owned UTXOs
2. Inputs are signed using derived keys
3. Node validates transaction signatures and ownership
4. Transaction enters the mempool
5. Mempool applies fee and double-spend policy
6. Miner selects transactions by fee rate
7. Block is mined under existing consensus rules
8. UTXO set updates deterministically

This enables peer-to-peer value transfer under fixed Layer-1 rules.

---

## Mempool Policy

The mempool includes:

* Double-spend prevention within the mempool
* Fee calculation and minimum fee enforcement
* Transaction size limits
* Deterministic fee-rate-based mining selection
* Reorganization-safe transaction resurrection

These are local policy rules and are not part of consensus.

---

## Command-Line Wallet

The client includes a built-in wallet CLI:

```
cargo run wallet address
cargo run wallet balance
cargo run wallet send <address> <amount>
```

The CLI interacts with the local node and mempool without altering consensus behavior.

---

## Backward Compatibility

* Consensus v3 rules remain unchanged
* Existing chains remain valid
* Nodes running v0.3.0 continue to validate blocks correctly
* Transaction functionality operates entirely above consensus

Version 0.3.1 is a non-forking feature release.

---

## Release Identifier

**Tag:** `v0.3.1-wallet-layer`
**Client version:** `0.3.1-wallet-layer`

---

## Scope of This Release

Version 0.3.0 established a stable base layer.
Version 0.3.1 enables use of that base layer for signed value transfer.

Layer-1 consensus remains intentionally stable and unmodified.

---

## Disclaimer

This software is provided for study, experimentation, and independent node operation.

There is no warranty, no guarantee of future updates, and no central authority.

The rules are enforced by the code.

---

**Satoshi-Nakamoto-ITL**
Bitcoin v0.3.1 — Revelation Edition
