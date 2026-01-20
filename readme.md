# Bitcoin v0.2 – Revelation Edition

Bitcoin v0.2 is an experimental peer-to-peer electronic cash system written in Rust.

The system allows nodes to maintain a distributed timestamped chain of blocks using proof-of-work and to synchronize this chain over a decentralized network without relying on a central server.

This project is intended for study, experimentation, and long-running node operation.

---

## Overview

Each node maintains a full copy of the blockchain and participates equally in the network.

Nodes:

* validate blocks
* mine new blocks using proof-of-work
* relay blocks to peers
* synchronize missing blocks automatically

Consensus is achieved by accepting the longest valid chain.

---

## Features

* Proof-of-Work block mining
* Fixed block structure with Merkle root
* Persistent blockchain and UTXO set
* Peer-to-peer networking over TCP
* Automatic block synchronization
* Simple HTTP interface for inspection
* No central authority or coordinator

---

## Network

Nodes communicate directly with each other using raw TCP connections.

Default ports:

* **8333** – peer-to-peer network
* **8080** – HTTP status interface

The peer-to-peer port is not HTTP and should not be opened in a web browser.

---

## Running a Node

### Build and run

```bash
cargo run --release
```

On startup, the node will:

1. Load the local blockchain from disk
2. Create the genesis block if none exists
3. Listen for incoming peer connections
4. Request missing blocks from peers
5. Begin mining once synchronization stabilizes

---

## HTTP Interface

The HTTP interface is provided for inspection and development.

Available endpoints:

* `/status`
  Returns current chain height, difficulty, and UTXO count.

* `/blocks`
  Returns a list of known blocks.

Example:

```
http://127.0.0.1:8080/status
http://127.0.0.1:8080/blocks
```

---

## Data Storage

Blockchain state is stored locally in JSON format.

```
data/
├── blocks.json
└── utxos.json
```

Nodes may be stopped and restarted without loss of state.

---

## Consensus Rules (Simplified)

* Blocks must reference the previous block hash
* Blocks must satisfy the current proof-of-work difficulty
* Chains with more accumulated work are preferred
* Invalid blocks are rejected and not relayed

---

## Limitations

This software is experimental.

It does not currently include:

* transaction signatures
* mempool prioritization
* difficulty retargeting
* network encryption
* denial-of-service protections

These may be added in later versions.

---

## Purpose

This project exists to explore and demonstrate the core mechanics of a peer-to-peer proof-of-work system in a clear and compact form.

It is not intended for production use.

---

## License

Open source. Free to use, modify, and redistribute.

---

Bitcoin v0.2 – Revelation Edition
