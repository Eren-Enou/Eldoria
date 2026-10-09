# Integrity pass: protected 001–009 foundation

Date: 2026-10-08. Baseline: `0242c9955590db6739c97a6de49347d49934311b`.
Finalized 009 was introduced by `cf82e30`; this pass accepts its actual documented
result into the foundation, without accepting subsequent 010 research.

## Scope and findings

The new [001–009 map](experiment-contract-001-009.md) incorporates the unchanged
001–008 map by reference and adds a mode-specific 009 contract. AGENTS, the
constitution's map link and README's foundation instructions point to it.
All ten constitutional requirements and legacy state lifetimes remain unchanged.
No actual conflict with the 001–008 foundation was found. Its historical statement
ending that audit at 008 is a scope boundary, not a new universal forgetting rule.

The material validation weakness was that `validate009` consumed manually
decompressed cache files without itself checking the frozen gzip/manifest. It now
shares `examples/support/contract009.rs` with `validate_contract`. The shared gate
reads frozen bytes directly, checks the full foundation inventory/hashes, verifies
009 compressed/raw sizes and hashes, compares exact seed-42 serialized JSON, and
checks all published core/held-out outcome counts. It writes no evidence files.
It requires Python's standard library; no new dependency is installed.

## Coverage audit and minimal correction

Reuse the nine existing 009 tests for bounded/local inputs, group representatives,
ties/conflict, matched extreme-seed replay, old 008 replay, enablement without history
import, invalid-decision rejection, paid-protocol fallback, receipt accounting and
audit tampering/reordering. One additional test explicitly protects Grouped default
FIFO equivalence at overflow and prospective Quality enablement without basis import.
Existing Salient enablement coverage remains in place.

Extend two existing tests rather than adding scenarios: both selective modes must
retain wrong evidence and preserve the minor-later-useful FIFO counterexample;
redelivery must have a fresh receipt without restoring its evicted predecessor;
later attribution must revise retained assumptions and leave FIFO's evicted receipts
unavailable. These enforce unavailable-cognition protection, not deletion of every
independently persistent consequence. No retention/inquiry policy, weight, capacity,
runtime code or archived scenario is changed.

The frozen experiments remain the evidence for narrow retained distinctions and
negative results. No improved executed behavior is inferred from 009 action probes.
Input types and targeted regressions support locality; they are not a comprehensive
proof against every possible future implementation leak.

## Validation and byte integrity

Required commands for this pass:

```text
cargo fmt --check
cargo clippy --locked --all-targets -j 1 -- -D warnings
cargo test --locked -j 1
cargo run --release --locked -j 1 --example validate_contract
cargo run --release --locked -j 1 --example validate009
```

Result: all five commands passed; the full suite passed 125 tests (including ten
009 tests), with no failures. Both read-only validators completed successfully.

The independent 009 gate and integrated foundation gate verify exact frozen core
and held-out seed-42 JSON plus deterministic replay/causal validity for seeds 0–127
and `u64::MAX`. Outcome counts for the original 128-seed evaluation match the
archived summary. Existing 001–008 archive, replay and compatibility checks remain
in the normal gate. The full test command also runs existing 010 tests as repository
regressions; this does not accept 010 into the foundation.

The new `foundation-001-009-sha256.json` records all 90 frozen files under experiments
001–009, including scripts, reports, manifests, compressed evidence and separate
timing measurements. A before/after check also guards all 19 tracked files whose
paths identify Experiment 010. The old contract/audit, all runtime source and all
010 files remain unchanged. No historical evaluator regenerates archives in place.
Result: all 90 frozen-file hashes and all 19 Experiment 010 hashes matched baseline.

No benchmark is needed: only documentation, regression assertions and validation
tooling change. Gzip decompression and repeated evaluation add cost to integrity
checking, not to simulated decisions. Archived performance/storage caveats are
carried into the new contract without new wall-clock claims.

FIFO remains default. Quality and Salient remain opt-in research modes. This pass
adds no behavioral capability, accepts no Experiment 010 conclusions and starts no
Experiment 011.
