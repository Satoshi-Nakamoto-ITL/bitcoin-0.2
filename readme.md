# Bitcoin v0.2 – Revelation Edition

Bitcoin v0.2 – Revelation Edition is an experimental peer-to-peer electronic cash system written in Rust.

It is designed to demonstrate the **core mechanics of a decentralized proof-of-work blockchain**, including mining, block propagation, chain synchronization, and fork resolution — without relying on any central coordinator or server.

This project is intended for **study, experimentation, and long-running private node operation**.

---

## Overview

Each node maintains a full copy of the blockchain and participates equally in the network.

Nodes can:

- validate blocks independently
- mine new blocks using proof-of-work
- relay blocks to peers
- synchronize missing blocks automatically
- resolve forks using cumulative proof-of-work

There is no master node, leader, or trusted authority.

---

## Consensus Model

Consensus follows a **Bitcoin-style fork choice rule**:

- Blocks must be valid and satisfy proof-of-work
- Multiple competing chains are allowed
- The chain with the **most accumulated work** is selected as canonical
- Nodes may reorganize their local chain when a stronger chain is discovered

Block height is **derived from ancestry**, not enforced by the network.

Temporary disagreement is expected and resolved automatically.

---

## Revelation Block

The first block (height 0) is referred to as the **Revelation Block**.

- It has no parent (`prev_hash = 0x00…00`)
- It contains a single `revelation_tx` transaction
- It establishes the initial state of the chain
- It is deterministic and shared by all nodes

Internally, it follows standard genesis mechanics, but is named *Revelation* to emphasize intent rather than tradition.

---

## Features

- Proof-of-Work mining
- Fork-capable consensus with reorganization support
- Merkle-root–based block structure
- Persistent blockchain and UTXO storage
- Peer-to-peer networking over raw TCP
- Automatic block synchronization
- Optional HTTP API for inspection
- No central authority or coordinator

---

## Network

Nodes communicate directly using raw TCP connections.

### Default Ports

- **8333** – peer-to-peer network
- **8080** – HTTP status interface (optional)

> ⚠️ The P2P port is **not HTTP** and should not be opened in a web browser.

When running multiple nodes on the same machine, ensure that:
- each node uses a different P2P port
- the HTTP API is either disabled or assigned different ports

---

## Running a Node

### Build

```bash
cargo build
Run a single node
cargo run -- --port 8333 --mine
On startup, the node will:

load the local blockchain from disk

create the Revelation block if none exists

listen for incoming peer connections

request missing blocks from peers

begin mining once synchronization stabilizes

Running a Private Network (Multiple Nodes)
Node A (seed / first node)
cargo run -- --port 8333 --data-dir nodeA --mine
Node B (peer)
cargo run -- --port 8334 --data-dir nodeB --peers 127.0.0.1:8333 --mine
You may run additional nodes by assigning:

a unique --port

a unique --data-dir

Nodes will automatically discover the best chain and resolve forks.

HTTP Interface
The HTTP API is provided for inspection and development only.

Endpoints
/status
Returns chain height, difficulty, and UTXO count

/blocks
Returns a list of known blocks

Example
http://127.0.0.1:8080/status
http://127.0.0.1:8080/blocks
When running multiple nodes on one machine, only one node should bind to port 8080, or the API port should be changed.

Data Storage
Blockchain state is stored locally in JSON format.

data/
├── blocks.json
└── utxos.json
Nodes may be stopped and restarted without loss of state.

Consensus Rules (Simplified)
Blocks must reference a known parent block

Blocks must satisfy the current proof-of-work difficulty

Multiple forks are allowed

Chains with greater accumulated work are preferred

Nodes reorganize automatically when a better chain is found

Invalid blocks are rejected and not relayed

Limitations
This software is experimental.

It does not currently include:

transaction gossip / mempool prioritization

network encryption

denial-of-service protections

fast sync or snapshots

production-grade peer discovery

These may be explored in later versions.

Purpose
This project exists to explore and demonstrate the core mechanics of a peer-to-peer proof-of-work system in a clear, compact, and readable form.

It is not intended for production use.

License
Open source.
Free to use, modify, and redistribute.
