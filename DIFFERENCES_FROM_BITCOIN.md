This document provides a **line-by-line comparison** between the consensus rules stated in `CONSENSUS.md` and the behavior of **Bitcoin Core**, highlighting intentional and unintentional deviations in **Bitcoin v0.2 — Revelation Edition**.

The purpose is explicitness and technical honesty.

## Proof-of-Work

**Consensus.md**

* Proof-of-Work: SHA-256 double hash

**Bitcoin Core**

* Uses double SHA-256 on the serialized block header
* Hash is interpreted as a 256-bit integer
* Validity requires `hash <= target`

**This Implementation**

* Uses double SHA-256
* Hash validity is determined by **counting leading zero bits**
* No big-integer target comparison

**Status**

* ✔ Same hash function
* ❌ Different validity rule

## Block Interval

**Consensus.md**

* Block interval: 600 seconds

**Bitcoin Core**

* Target block time is 600 seconds
* Enforced indirectly via difficulty adjustment
* No per-block timing enforcement

**This Implementation**

* Timestamp is recorded
* Difficulty adjustment logic is simplified
* No strict enforcement of per-block timing

**Status**

* ✔ Same target interval
* ❌ Different enforcement mechanism

## Difficulty Adjustment

**Consensus.md**

* Difficulty adjustment: every 2016 blocks

**Bitcoin Core**

* Retargets every 2016 blocks
* Uses median timestamps
* Difficulty adjustment bounded to 4× increase or decrease
* Based on actual elapsed wall-clock time

**This Implementation**

* Difficulty retargeting logic is simplified
* May not strictly retarget every 2016 blocks
* Uses abstract difficulty units rather than compact targets (`nBits`)

**Status**

* ❌ Behavior diverges from Bitcoin Core

## Initial Block Reward

**Consensus.md**

* Initial reward: 50 coins

**Bitcoin Core**

* Genesis subsidy: 50 BTC
* Subsidy enforced by consensus rules

**This Implementation**

* Initial subsidy is set to 50 coins
* Enforced by code

**Status**

* ✔ Equivalent behavior

## Halving Schedule

**Consensus.md**

* Halving: every 210,000 blocks

**Bitcoin Core**

* Subsidy halves every 210,000 blocks
* Integer truncation applies

**This Implementation**

* Uses the same halving interval
* Simplified arithmetic

**Status**

* ✔ Equivalent behavior

## Maximum Supply

**Consensus.md**

* Max supply: 21,000,000 coins

**Bitcoin Core**

* Max supply emerges implicitly from subsidy schedule
* Never explicitly enforced as a hard cap

**This Implementation**

* Maximum supply follows from halving logic
* May not be explicitly capped by invariant checks

**Status**

* ✔ Economically equivalent
* ❌ Not enforced identically

## Block Size Limit

**Consensus.md**

* Block size limit: 1 MB

**Bitcoin Core**

* Enforces serialized block size limits
* Includes transaction data and headers

**This Implementation**

* Block size limits may be absent or simplified
* Serialization format differs

**Status**

* ❌ Not enforced equivalently

## Coinbase Maturity

**Consensus.md**

* Coinbase maturity: 100 blocks

**Bitcoin Core**

* Coinbase outputs cannot be spent until 100 confirmations
* Enforced by consensus

**This Implementation**

* Coinbase maturity may be enforced or omitted
* Depends on UTXO validation logic

**Status**

* ⚠️ Possibly divergent (implementation-dependent)

## Chain Selection Rule

**Consensus.md**

* Chain selection: most accumulated work

**Bitcoin Core**

* Selects chain with greatest total cumulative work
* Work is derived from difficulty targets

**This Implementation**

* Accumulated work is derived from simplified difficulty values
* Not equivalent to Bitcoin’s work calculation

**Status**

* ✔ Conceptually similar
* ❌ Mathematically different

## Maximum Reorganization Depth

**Consensus.md**

* Max reorg depth: 100 blocks

**Bitcoin Core**

* No fixed reorg depth limit
* Deep reorganizations are possible in theory

**This Implementation**

* Enforces a maximum reorganization depth
* Reorgs beyond this depth are rejected

**Status**

* ❌ Explicit deviation from Bitcoin Core

## Governance

**Consensus.md**

* No governance

**Bitcoin Core**

* No on-chain governance
* Protocol changes occur via software adoption

**This Implementation**

* No governance mechanisms
* Rules enforced solely by code

**Status**

* ✔ Philosophically aligned

## Upgrades

**Consensus.md**

* No upgrades

**Bitcoin Core**

* Soft forks and hard forks occur via versioned software adoption

**This Implementation**

* No built-in upgrade or version negotiation mechanism
* Rule changes require manual software replacement

**Status**

* ❌ More rigid than Bitcoin Core

## Networking

**Bitcoin Core**

* Peer-to-peer networking
* Gossip-based block and transaction propagation

**This Implementation**

* No P2P networking
* Nodes operate independently

**Status**

* ❌ Major architectural deviation

## Summary Table

| Area          | Bitcoin Core     | This Implementation |
| ------------- | ---------------- | ------------------- |
| Hash Function | Double SHA-256   | Double SHA-256      |
| Difficulty    | Target threshold | Leading zero bits   |
| Retargeting   | 2016 blocks      | Simplified          |
| Chain Work    | Precise          | Approximate         |
| Max Reorg     | Unlimited        | 100 blocks          |
| Script        | Bitcoin Script   | None                |
| Networking    | P2P              | None                |
| Compatibility | Bitcoin mainnet  | Not compatible      |

## Conclusion

Bitcoin v0.2 — Revelation Edition is **Bitcoin-inspired**, not Bitcoin-equivalent.

While many economic parameters mirror Bitcoin’s values, the consensus mechanics, difficulty model, and networking layer intentionally diverge for clarity and experimentation.

These differences are documented to prevent ambiguity or false equivalence.

**Time + Energy = Truth**
