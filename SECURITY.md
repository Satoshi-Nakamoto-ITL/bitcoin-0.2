# Security Policy

## Overview

This repository implements a **frozen Proof-of-Work Layer 1 protocol**.

The project is intentionally minimal and experimental.
It is **not** a production financial system.

Security in this context refers only to **protocol correctness and determinism**, not to protection of funds, identities, or users.

## Security Model

The protocol enforces the following at consensus level:

* Deterministic block structure
* Double SHA-256 Proof-of-Work
* Block ordering by accumulated work
* Fixed monetary issuance schedule
* Difficulty retargeting
* Immutable genesis block

All consensus rules are enforced **solely by code and Proof-of-Work**.

There are **no administrative keys**, backdoors, or override mechanisms.

## What This Protocol Does NOT Secure

The following are **explicitly out of scope** for this repository:

* Cryptographic ownership of coins
* Transaction authorization or signatures
* Protection against double-spending by unauthorized parties
* Economic finality guarantees
* Resistance to hardware specialization (ASICs)
* Resistance to quantum computing
* Network-layer security (P2P, DoS, eclipse attacks)
* Privacy or anonymity guarantees

UTXOs exist as a state-tracking mechanism only.
They do **not** imply exclusive ownership.

## Intended Use

This codebase is intended for:

* Research and education
* Proof-of-Work experimentation
* Time anchoring and historical verification
* Protocol study and archival purposes

It is **not intended** to be used as:

* A secure currency
* A financial settlement layer
* A production blockchain network

## Reporting Issues

Because the protocol is **frozen**, only the following issues may be reported:

* Crashes or panics
* Determinism violations
* Consensus inconsistencies
* Build or runtime failures

Issues that request new features, security upgrades, governance changes, or protocol modifications will be closed without action.

## Responsible Disclosure

If a bug is discovered that violates **already-declared consensus rules**, it may be disclosed publicly.

There is **no guarantee** that fixes will be applied, as the protocol is sealed.

## Final Note

This repository does not promise safety.
It promises **immutability**.

Anyone using or extending this code does so at their own risk.

**Status: Frozen. Protocol sealed.**
