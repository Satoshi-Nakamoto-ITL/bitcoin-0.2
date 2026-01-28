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
