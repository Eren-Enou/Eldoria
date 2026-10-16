# Integrity audit: foundation 001–019

Gate B: PASS. Qualified findings are protected; no policy is adopted.

Scientific review accepted 019 with qualification at fd8e1b08cd2f96390a5e042517e9a18395e25908;
separate review commit 8c964a1 was pushed. No behavior, policy weight, cognition,
resource rule, physical clock or default changes in this integrity pass.

The new contract inherits unchanged 001–018. Inventory is exactly 296: 269 inherited
entries with identical values plus 27 original 019 files. New review and viewer files
are excluded. Canonical text/gzip hash conventions remain mode-specific. A separate
296-file raw working-byte guard covers all original experiment files, including the
invalid 018 development bundle. No archive writer is used.

Original standalone019 validation moves into shared contract019.rs. Normal foundation
and the thin standalone call the same implementation: 1296 exact worlds,336 full raw
histories, source and archive envelopes, original summary equality, all original
independent reconstruction/corruption/prefix checks. Added full 296 inventory, independent
scope reconstruction and exact original 21 causal slices. The scientific validator
imports no viewer code, reads no viewer cache and writes no export.

## Completed validation

Every required command exited 0 at the shared019 implementation:

* `cargo fmt --check`.
* `cargo clippy --locked --all-targets -j 1 -- -D warnings`.
* `cargo test --locked -j 1`:197 tests.
* `cargo run --release --locked -j 1 --example validate_contract`:001–019.
* The same locked release invocation for every standalone validate009 through
  validate019, all exit0.

The shared019 gate checks all296 protected hashes,1296 exact worlds,336 original raw
histories,original source/archive/summary checks,13 corruption categories,840 paired
prefixes,317 full nulls,162 timing/order starts and four independent readers including
complementary scope and exact21-slice reconstruction. It does not import the viewer.

The newly built standalone019 also passed from an empty directory with Python file
writes and target-cache reads denied,creating zero files. Both denial rules were
self-tested before launch. This checks portability/read-only independence, not a
new scientific replication. All296 original review-start working-byte hashes match;
all269 inherited manifest values remain identical. No source archive was regenerated.
No simulation runtime, default, capacity, policy, clock or resource rule changed.

Changed files: new contract,296-entry inventory,audit,handoff,shared019 helper,
thin standalone/normal foundation integration and relevant governance/navigation.
Original review remains a separate commit. Logs are outside frozen evidence in
`%TEMP%/history-explorer` (`B-*.log`). No throughput benchmark is warranted for
validator/documentation changes. The History Explorer has its own separate tests
and delivery; it is not required by this protection gate.
