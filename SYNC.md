# Chain Synchronization

Nodes synchronize by requesting blocks from peers and validating them locally.

---

## Initial Sync

- Connect to peers
- Download blocks
- Validate sequentially
- Build UTXO set deterministically

---

## Ongoing Sync

- New blocks announced by peers
- Blocks validated before acceptance

---

## Reorganizations

If a stronger chain is received:
- Validate the chain
- Reorganize if valid
- Update UTXO set

---

## Guarantees

- No block is trusted without validation
- State is derived from chain history

---

## Status

Synchronization is stable under Consensus v3.
