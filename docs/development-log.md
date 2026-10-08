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

## 2026-10-07 — Experiment 004

- Reviewed instructions, architecture, prior findings, source and tests; established
  passing formatting, Clippy, 32 tests and three-mode benchmark baseline.
- Documented prediction/evidence hypotheses before behavior changes. Added optional
  local one-step forecasts, candidate score decomposition and directed expectations.
  Actual public next responses produce auditable errors and quarter-error updates.
- Added seeded fallible evidence, explicit reliability/effort, conflicting source
  aggregation and source immutability. Preserved the legacy disclosure path and
  shared memory revision/trust replay. No hidden sensor result enters prediction.
- Fixed development Clippy warnings on explicit Bevy guard drops by relying on
  normal borrow scopes; later validation remained warning-free.
- Evaluated 18 controlled variants over 128 seeds. Anticipation changes a +5 lie
  into a -15 choice under 50% expected scrutiny; silence wins. Six non-challenges
  reduce expectation to 10; six challenges raise it to 90. Expectation-only ablation
  isolates the effect on later communication. Costly proof remains unprovided.
- Noisy trials produced 15 incorrect readings out of 128. Opposing equal-weight
  sources yield support zero and restore initial blame. Credibility can be wrongly
  penalized by bad evidence; reports distinguish observation from objective truth.
- Found a finite-horizon loophole: a late lie after an explanation request cannot
  be challenged. In a trusted-history branch this persuaded the listener. Kept this
  result and recommended persistent unresolved questions for 005, without starting it.
- All 43 tests, formatting, Clippy and four benchmark modes passed. Automated saved
  report comparisons preserve 001–003. Full suite/population reports replay, with
  dedicated JSON, human report, findings and separate timing evidence under 004.
  Measured roughly 23% run-time overhead over 003 at 1,000 agents; no larger-scale
  claim. Documented unbounded maps/audits and linear per-update scans.
# Experiment 005 — 2026-10-07

Reviewed AGENTS, README, architecture, Experiments 001–004, source, tests and
benchmarks. Unmodified baseline passed fmt, Clippy, 43 integration tests and the
four-mode benchmark. Recorded hypotheses before adding behavior in
`experiments/005/specification.md`; preserved earlier uncommitted 004 work.

Added an optional bounded concern resource and pure replaceable follow-up policy.
Actual unresolved refusals create concerns; later encounters supply opportunities
to request evidence, continue, or abandon one matter. Reused the communication
resolver and existing evidence/belief/memory mechanisms with explicit request
accounting and two-turn voluntary response conversations. Added concern transition,
creation, decision, and public-response learning audits without changing older
serialized schemas. Terminal and active capacity eviction are explicit.

Controlled sweeps demonstrate cost/relationship counterfactuals, strong/partial/
mistaken resolution, abandonment after three failed requests, competing priorities,
and history-only input ablations. Repeated weak evidence remained Partial after
eight later encounters, contrary to an initial test expectation; corrected that
assertion to the trace without changing the policy. Fourteen new tests include
state/tick reconstruction, bounded storage, memory eviction, credibility, locality,
costly proof and disabled learning. Full historical compatibility is retained.

Added dedicated 005 evidence, readable reporting and formation/repeated-encounter
benchmarks. At 1,000 agents the matched three-encounter benchmark is about 40.8%
slower with concerns, including added conversations/audits. No scaling claim beyond
measurement. Recommend a bounded Experiment 006 on source novelty and expected
information gain; no implementation of 006 begun. Details and commands are in
`experiments/005/results.md`.
## 2026-10-07 — Experiment 006

Read repository instructions, architecture, prior reports, concern/communication/
prediction/accounting implementations and tests. The starting working tree already
contained Experiments 004/005 changes; preserved them. Baseline: 57 integration
tests and old workload benchmark passed. Saved baseline timing under 006, leaving
all older evidence untouched. Recorded the hypothesis before implementation.

Added an opt-in local inquiry resource/policy and isolated integration module.
Source × strategy estimates, provenance signatures, public offers and inquiry
episodes are separate from concern state and trust. Reused voluntary local reply,
receipt, revision and saturated trust accounting. Added explicit bounded social
opportunities; introduced people gain no historical expertise from co-presence.

Controlled evaluation shows Direct→Direct→Evidence→Direct→Evidence→Pause under
weak repeated information; returning briefly to a cheap method is an observed
heuristic consequence, not a scripted strategy sequence. Strong new evidence
renews Evidence; unknown-source exploration can fail. High importance does not
prevent pausing an uninformative method. History/novelty ablations keep asking.

Review identified an unfulfilled-offer floor that could otherwise renew a method
indefinitely. Added attempted-offer state and a regression test. Added atomic
evidence provenance preflight, preventing external receipts from causing an
inconsistent default sensor batch to panic or mutate prior observations. No test
or score tuning was used to force the controlled action sequences.

New tests cover full seeded replay, local input isolation, useful repetition,
novelty categories, source introduction, voluntary offers, invalid actions,
provenance, all local storage caps, learning reconstruction and time/resource
accounting. Dedicated results, compatibility and matched population performance
evidence are under experiments/006. Experiment 007 is not implemented.

## Experiment 007 — provenance-aware testimony (2026-10-07)

Read the repository instructions, architecture, constitution and 001–006 reports.
Started from a clean checkout. Saved the existing benchmark under 007 and documented
the hypothesis before behavior changes. Preserved prior defaults, schemas and evidence.

Added opt-in bounded provenance knowledge/hints/comparison keys, observer-only roots
and parent receipts, voluntary instrumented inspection and structured exchange, and
local inquiry overlap factors. Later attribution uses the established bounded belief,
memory revision, concern and saturated trust-replay machinery. The real communicator
is distinct from the native refusal subject. Learning references native Inquiry IDs.
Ordinary refusers keep their existing response channel; no gossip scheduler was added.

Fourteen controlled variants across 128 seeds replay exactly. Independent corroboration
adds value, known relay does not, hidden relay overcounts, disclosure reduces confidence,
independent conflict remains uncertain, and independent wrong readings can falsely close
a concern. Credibility is conditioned through actual fallible evidence comparisons.
Alternative-source inputs match despite independent/relay/empty private acquisitions.
FIFO tests show forgetting can make old evidence seem independent again.

Full83-test suite, formatting, warning-denied all-target Clippy, release evaluator and
benchmark pass. All 001–006 archives compare equal; the natural population preserves
006 resource events, agents and native cognition. Performance/storage and prospective
legacy-evidence limits are documented in the dedicated007 report. Recommendation:
uncertain and potentially misleading attribution; Experiment008 is not implemented.
