# Network Overview

Bitcoin Revelation operates as a decentralized peer-to-peer network.

Nodes exchange blocks, transactions, and peer addresses directly.

---

## Peer Discovery

Peers are discovered via:
- Public seed nodes
- Address exchange (`getaddr`)
- Manual connections (optional)

---

## Public Seed Node

bitcoin-0-2.fly.dev:8333


The seed node:
- Accepts inbound connections
- Shares peers
- Does not mine
- Does not influence consensus
---
### 🌱 Public Seed Node Disclaimer

The public seed node is provided **solely for initial peer discovery**.

It is **not required** for normal network operation and **may be shut down, moved, or replaced at any time without notice**.

Nodes are expected to:

* Discover peers automatically
* Maintain connections independently
* Continue operating without reliance on any single endpoint

The network does **not** depend on the availability of any specific seed node.

> **Seed nodes are for bootstrapping only and may be discontinued at any time. Network operation does not depend on any single node.**
---

## Transport

- TCP-based P2P protocol
- All messages are verified locally

---

## Trust Model

Peers are untrusted.  
All data is validated before acceptance.

---

## Resilience

The network survives:
- Node churn
- Seed node failure
- Temporary partitions
