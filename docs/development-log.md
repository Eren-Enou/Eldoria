# Development log

## 2026-10-06 — Foundation and Experiment 001

- Confirmed the workspace was empty. Wrote the architecture proposal before code.
- Verified Bevy ECS 0.19.1 through Cargo metadata and versioned documentation;
  minimum Rust 1.95. Selected the stable release over 0.20.0 release candidates.
  Updated the installed Rust toolchain from 1.88.0 to 1.99.0. Rustup's subsequent
  self-update reported access denied removing rustup-init.exe; compiler, Cargo,
  formatting and lint components updated and function correctly.
- Added one crate with a headless single-threaded schedule, deterministic generator,
  local observation policy, validated resource resolver, bounded episodes and trust.
- Kept objective balances/actions separate from subjective memories and retained
  score inputs and previous event IDs for investigation. Added stage intervention
  snapshots rather than concealing fixture changes as world events.
- Resolved a Bevy Event trait/domain Event name collision with explicit ECS imports.
- Added A–G scenarios, JSON/human reports, a multi-seed evaluator and a benchmark.
- Verified history effects across 128 seeds and conservation/legality/state replay
  over 256 suite seeds. Added invalid-policy, timeout, overflow, memory-capacity and
  population checks. Initial 11 tests, formatting and warning-free Clippy passed.
- Review tightened constructor validation to reject dangling imported memories and
  invalid trust, and replay verification now covers every selected trial including
  the history-free control. Snapshot restoration is intentionally unsupported.
- Restricted interpretation inputs to public signals and own state, excluding
  privileged partner decision traces. Re-ran formatting, Clippy, all tests, the
  complete seed-42 suite and the release benchmark after this interface change.
- Extended the sweep to compare repeated first actions and outcomes: 18 and 8
  changed respectively across 128 seeds. Verified documented population outcome
  counts directly against the final saved JSON.
- Measured serial interactions at 100 and 1,000 agents. Performance and behavioral
  evidence are stored under experiments/001; conclusions limit claims to the model.
