# ARGUS

Production integration baseline for the five-module liquidation intelligence pipeline.

## Data path

`collector -> argus-contracts -> Redis Streams -> position state -> feature snapshots -> deterministic keeper requests -> paper/manual execution`

The shared contracts crate is the only cross-module type source. Amounts use checked 256-bit integers and decimal-string serialization. Redis Streams are the durable event boundary; consumers must acknowledge only after durable state commits. PostgreSQL stores receipts and reorg invalidations for idempotency and replay.

## Current safety boundary

The execution crate is deliberately fail-closed: it validates provenance, expiry, replay epoch, and snapshot version, then records a paper decision. It does not hold keys, sign transactions, broadcast transactions, or autonomously liquidate positions. Live-market testing is limited to read-only/simulation fixtures until an independently audited signer and protocol adapter are added.

## Checks

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
