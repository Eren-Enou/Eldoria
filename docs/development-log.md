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

## 2026-10-06 — Experiment 002

- Read all foundation specifications, source, tests and prior findings. Established
  passing fmt, Clippy, 11-test and benchmark baseline before editing behavior.
- Wrote experiments/002/specification.md before implementation. Preserved the
  original rules by selecting the scarcity policy explicitly in new fixtures.
- Added structured testimony/disclosure scenes, bounded event-specific beliefs,
  immutable original interpretations plus revision audit, clamp-correct trust
  replay, and explicit food consumption with sink accounting.
- Kept speech scheduling and an instrumented disclosure sensor explicit; no private
  partner state enters a policy or testimony update. These are documented limits.
- Found that a generous probe offered with or without explanation; chose a matched
  boundary probe and retained cases where revision leaves decisions unchanged.
- Prevented credibility feedback by freezing testimony reliability for each event
  at first information contact. Avoided copying the growing cognition audit during
  every communication scene by temporarily taking the ECS resource.
- All 128 controlled suites replayed; testimony changed the matched helpful-history
  probe in all 128. False testimony also persuaded those listeners, exposing a
  limitation. Strong conflicting evidence reversed it. Negative history resisted
  weak testimony; disclosure revised one memory without erasing the other history.
- Saved full seed-42 and 1,000-agent evidence, separate timings, and measurements.
  Regenerated Experiment 001's archived seed-42 report with exact structural
  equality; did not overwrite historical evidence.
- Passed fmt, Clippy, 21 integration tests and both benchmark modes. New tests
  include complete population audit reconstruction, saturation, eviction, false
  claims, local information and accounting. Findings include costs, limitations
  and recommendations only; Experiment 003 was not begun.

## 2026-10-07 — Experiment 003

- Reviewed instructions, README, architecture, prior findings, code and tests.
  Baseline formatting, Clippy, 21 tests and both benchmark modes passed. Existing
  uncommitted Experiment 002 work was preserved in place.
- Documented hypotheses/tradeoffs before editing behavior. Added an optional
  automatic post-refusal communication phase with local policy and four-turn
  bound. The harness configures preferences; it does not schedule messages.
- Added silence, testimony, evidence, requests and motivated known-false claims;
  separated credibility from trust and bounded communication episodes at 16.
  Reused Experiment 002's belief/revision/ledger implementation with an internal
  credibility-based testimony weight for the new mode only.
- Corrected an initial Rust match-expression parenthesization error, then passed
  compilation and warning-free lint. Listener records normalize Mislead to Explain
  so intent cannot leak through the public signal or local memory.
- Evaluated 16 variants across 128 seeds. Disclosure/evidence costs changed later
  resource decisions; explanation requests alone often did not. Verified versus
  unanswered communication history changed subsequent information seeking, with
  local episode ablation isolating the cause. Motivated deception was attempted
  but did not improve the later resource choice; challenged speakers sometimes
  self-incriminate because the current heuristic lacks outcome prediction.
- Reproduced all suites and a 1,000-agent audit. Saved earlier 001 and 002 reports
  compare equal; original evidence was not overwritten. New evidence is under 003.
- Passed formatting, Clippy and 32 tests, including full population audit replay,
  private information/intent isolation, legality, termination, history ablation,
  saturation and bounds. Measured all three benchmark modes; at 1,000 agents the
  new workload adds about 34% run time over 002. Documented distinct workload and
  audit-storage costs instead of claiming broad scaling.
- Recommended outcome prediction and fallible evidence access based on observed
  failures. Did not start Experiment 004 or larger social systems.
