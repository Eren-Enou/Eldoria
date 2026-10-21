# Experiment 025: runtime gate failure

Follow-up [independent diagnosis](finite-material-inquiry-audit-diagnosis.md)
isolates this rejection to actor-local versus global scene coordinates. A separate
observer-only successor is under validation; the original failed version and
evidence remain unchanged. The initial stop below remains the status of that
version, and no new matrix may run before the successor passes its own gate.

## Decision

**STOP dependent implementation and evaluation. Experiment 025 is incomplete.**
The first registered matrix attempt fails an inherited inquiry-audit invariant.
Do not adopt the mechanism, protect025, start026, or treat the surviving prefix as
a completed scientific experiment. No runtime behavior or registered parameter was
changed after this failure.

The user program's design gate explicitly says: “If a critical physical, locality,
or time invariant fails, stop implementation. Deliver a precise blocker report.”
This report follows that instruction. It does not weaken or bypass the validator.

## What completed before the failure

Independent024review accepted the evidence with qualification; qualified001–024
protection passed. Those milestones were published as4dfc71c and68b4c70.
Prospective025 architecture, specification,24-question design gate and50-world
matrix were published asb2487dc before implementation or treatment outcomes.

The isolated `prototypes/material-preparation` successor implements the registered
finite2raw→1food+1waste law, two positive effort phases, owned escrow, full failed
attempt time, one tagged environmental memory slot and actor-local Prepare scoring.
It does not change the protected source, old activity policies or global defaults.
These are implementation properties tested before treatment, not an accepted025
scientific finding. See [pre-treatment checks](finite-material-runtime-validation.json)
and [runtime gate](../../experiments/025/runtime-gate.json). Their PASS describes
the pre-treatment gate only; it does not supersede this subsequent failure.

## Exact failure and reproduction

The first matrix invocation exited101:

```powershell
cargo run --release --locked -j 1 --manifest-path prototypes/material-preparation/Cargo.toml --target-dir target/material-preparation --bin material-preparation -- matrix target/preparation025-original.jsonl
```

Ordinals0–18 completed and were flushed. Ordinal19 (`meal_and_work`, pair9,
access enabled) failed before it could export a trial. Its world case is
`consumption`, Baseline valuation, seed42, Ascending order, Epoch phase,
ordinary horizon96, raw8, Alternate observation, extraction2/preparation4.
The exact configuration is preserved in
[failed-config.json](../../experiments/025/aborted-run/failed-config.json).

The panic is `concern event ownership mismatch`, raised by
`final_state.validate().unwrap()` in generated `preparation_trial25`, after the
ordinary loop, drain, empty-work assertions, material ledger and history-index
validation. The generated coordinator location in the original build is line223.
The originating check is in `src/audit.rs`, in `Snapshot::validate`.

For each valid inquiry, that check requires all of:

- The concern owner equals the inquiry input owner.
- The concern target equals the referenced event's decision actor.
- The referenced event tick is no later than `record.tick - record.time_spent`.
- The inquiry owner participated in that event.
- The concern's creation scene equals the referenced event's scene.

The error does not identify which predicate failed. The failed trial's snapshot
was not exported. **At the initial stop, the offending inquiry, event and predicate had not been
independently isolated; the linked subsequent diagnosis now supplies them.** It is not established whether this is an actual invalid
local input, incompatible time/scene semantics in a newly reached path, or another
inherited/new integration defect. A deterministic panic alone does not distinguish
these possibilities. Do not describe it as proven private-state leakage, or as a
harmless validator defect. Protected source must remain intact during diagnosis.

One replay of the exact failed configuration, at unchanged source and source locks,
also exited101 with the same error. Its output file is empty; it did not create a
replacement outcome. The original and replay logs, including the replay backtrace,
are preserved under [aborted-run](../../experiments/025/aborted-run/manifest.json).
No subsequent registered worlds were run.

## Preserved evidence and independent checks

[Original aborted evidence](../../experiments/025/aborted-run/partial-trials.jsonl.gz)
contains exactly19 completed worlds, decoded57,851,417bytes. Its decoded SHA256 is
`0f2583e8ae9adb5767264a0f37a715b61bc0eacb91599313770c9711509f9c7b`.
The gzip was written once with deterministic metadata. The manifest binds archive,
logs, failed configuration, partial checks and the unchanged pre-treatment source
manifests. This is intentionally not named the complete `trials.jsonl.gz` archive.

All19 completed configurations match their registered ordinals. The full
independent reader passes all19, checking local selection, material, slots, clock,
cognition, consent and checkpoints. A separate standard-library reconstruction
passes physical and activity/contact arithmetic for each. The unchanged compiled
runner replays all19 byte-identically. These validations establish only the
surviving prefix; they provide no validation of the failed world's unexported
state or the31 remaining worlds. There are nine complete primary pairs and one
unmatched closed arm. No whole-matrix causal estimate or coverage claim is made.

Post-failure identity checks confirm all114 runtime source bindings and24 helper
bindings, all107 foundation-validation source hashes, all408 protected portable
artifacts and425 raw originals remain unchanged. Original024 evidence is intact.
No unfavorable evidence was deleted or replaced, and no retention, inquiry,
activity or conversion formula was retuned.

## Validation scope and remaining work

Before treatment, root formatting, locked all-target Clippy with denied warnings
and217 tests passed; prototype formatting/Clippy and15 Rust tests passed; four
Python component tests and six component corruption rejections passed. All56
original024 worlds replayed exactly; all56 new feature-off projections matched
the original histories and decisions exactly and passed both independent readers.
The disabled scratch suite rejected11 corruptions;11 active-work witnesses were
explicitly unavailable in that disabled evidence. Full normal001–024 foundation
validation and the separate024foundation gate passed at stageB. Their107 bound
sources were rechecked unchanged after the failure; those long gates were not
needlessly repeated at the same source revision.

The full50-world025 evaluator/replay, active-evidence corruption suite, scientific
analysis, completed-run performance/storage comparison, viewer integration and
community-readiness claims have **not** passed. The first attempt's short elapsed
time is not a throughput benchmark. The partial decoded archive size is not a
comparison of runtime memory or complete025 observer overhead. Nothing here
establishes productive activity, useful sharing, renewable supply or a sustainable
community.

The failing matrix is a required validation failure. Accordingly the runtime,
helpers, source freezes and this failure report are left uncommitted; the last
published milestone remains prospective registrationb2487dc. No further dependent
implementation is authorized by a passing gate.

The provided user attachment also ends mid-sentence inI3; the requested remainder
has not arrived. This does not cause the runtime failure, but a resumed program
must reconcile the missing instructions before claiming complete delivery.

## Next defensible decision

Resolve the failing ownership/time predicate in an explicitly separate diagnostic
pass before proposing any behavioral correction. Retain this first attempt and
its source hashes. A correction must distinguish actual invalid cognition from
an audit/clock integration issue; it must not weaken locality to finish the matrix
or rewrite the protected001–024 foundation. Any resumed treatment must have an
explicit new revision and preserve these failed original outcomes. The strongest
open question is whether the new preparation-driven trajectory reaches an invalid
inquiry state or reveals an inherited audit/time assumption that old fixtures did
not exercise. No025 survival/production conclusion is defensible yet.
