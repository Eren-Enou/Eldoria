# Experiment 026 — verification status

C1 completed with qualification. Current full read-only command:

```
cargo run --release --locked -j 1 --example validate026_c1_isolated
```

Passed100 independent reconstructions,50 comparisons,17 corruption categories,
exact100 C1 replay and exact43-row original v1 failure. Cold native builds are
isolated at `target/026-v1-isolated` and `target/026-c1-isolated`; warm builds and
replays pass too. Frozen scientific source/assertions are unchanged. The original
shared-target wrapper failed artifact-lineage isolation; its diagnostics remain.
The I1 projection changes only counted/reversible transport paths.

Root formatting, locked all-target Clippy and217 tests pass. The unchanged full
`validate025_foundation` gate previously passed, including historical
`validate_contract`001–024; current custody checks preserve455 portable artifacts,
472 raw originals and214 bindings. Both16-file v1/C1 freezes remain exact.

See [C1 verification](continuation-c1/verification.md),
[durable receipt](../../docs/development/experiment-026-c1-validation.json) and
[completion audit](../../docs/development/experiment-026-completion-audit.md).
Original v1 `validate026` still requires100 worlds and remains unpassed; the
[frozen stop-time verification](failure/v1-verification.md) is unchanged. It is
not replaced or weakened. No protected026 extension or default adoption.
