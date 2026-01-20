# Network Protocol

This document describes peer-to-peer network behavior.

The network is decentralized and does not rely on centralized servers.

---

## Transport

Nodes communicate using TCP connections.

All messages are sent directly between peers.

Default peer-to-peer port:

```
8333
```

The peer-to-peer protocol is binary and does not use HTTP.

---

## Message Encoding

Messages are serialized using a compact binary encoding.

Each message represents a single logical action.

---

## Message Types

Nodes exchange the following message types:

* **SyncRequest**
  Requests blocks above a specified height.

* **Block**
  Transmits a full block.

* **Transaction**
  Transmits a transaction (currently optional).

* **Ping / Pong**
  Maintains connection liveness.

---

## Synchronization

A node initiates synchronization by sending a sync request containing its current block height.

Peers respond by sending all blocks above that height in ascending order.

---

## Propagation

When a node mines or accepts a new block:

* the block is validated
* the block is broadcast to peers
* peers repeat the process

This results in rapid propagation throughout the network.

---

## Connections

Nodes may accept inbound connections and initiate outbound connections.

The network topology is dynamic and unordered.

---

## Summary

The network is simple, direct, and decentralized.

Nodes cooperate by relaying valid data.

Invalid data is ignored.
