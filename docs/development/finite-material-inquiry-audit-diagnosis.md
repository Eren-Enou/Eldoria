# Experiment025: independent scene-namespace diagnosis

## Finding

The first matrix rejection is an **observer namespace mismatch**, not a failed
ownership or past-event-availability predicate in the rejected inquiry. The frozen
original025runner still rejects the case, and its original partial archive and
failure logs remain intact. This diagnosis does not establish completed025results.

An isolated scratch build called the frozen generator and added only serialization
of the immutable final snapshot immediately before the unchanged failing assertion.
It reran only the exact failed configuration and still exited101. It did not export
a completed trial or change any policy, input, action, resource or clock. Original
snapshot, instrumentation, diagnostic wrapper, transformation records and logs are
preserved in [diagnostic evidence](../../experiments/025/diagnostics/scene-namespace/manifest.json).

## Exact rejected reference

There is one valid inquiry and one concern reference in the captured snapshot:

| Field | Value |
|---|---|
| Inquiry / actor / concern | 0 / 3 / 0 |
| Completed tick / elapsed time / observed tick | 52 / 12 / 40 |
| Referenced event / event tick | 9 / 10 |
| Event's global scene ID | 3 |
| Concern's input creation scene | 1 |
| Independently counted prior encounters for actor3 | 1 |
| Current input scene / actor3 completed encounters by40 | 2 / 2 |
| All completed scenes by40 | 7 |

Owner equality, target equality, actor participation and10≤40 all pass. The sole
failed predicate in the reported audit error is1≠3. Source inspection confirms
`concurrent_concern_projection` deliberately projects the concern's creation scene
into actor-local encounter counts. `concurrent_input` and `provenance_follow`
likewise use actor-local context counts. The inherited compact snapshot audit
instead compares this input field directly with the event's global scene ID.
Changing live policy inputs to global scene IDs would expose foreign activity
counts and is not a valid correction.

## Independent reconstruction and failed alternative

The new read-only `examples/support/diagnose025_scene.py` reconstructs local
creation/current counts from participating scenes and executed event times; it
does not call the simulator or the Rust projection. It verifies all ownership,
target, participant and availability predicates before producing an observer-only
copy. It rejects six scratch corruptions: owner, target, future event, creation
counter, current counter and participation.

First, translating only concern creation1→3 makes the compact audit pass, but the
whole inherited validator then correctly reports `learning decision mismatch`.
The established inquiry eligibility test is `created_scene < input.scene`.
Changing only one coordinate makes a legitimately eligible concern appear too new.
This failed diagnostic alternative is preserved, not omitted as an inconvenience.

After independently checking both original local coordinates, translating them
together on a copy (creation1→3; current2→7) preserves1<2 as3<7. The **unchanged
whole typed `experiment013::Snapshot::validate` passes** that copy, including
policy/learning replay, retention, assessment and receipt checks. No field other
than those two observer-copy coordinates was changed. The actual input remains
creation1/current2. No translated snapshot was used to resume the simulation.

## Correction boundary

The [prospective observer-validation plan](finite-material-observer-validation-plan.md)
requires a separate successor and strict verification before translation. A new
`prototypes/material-preparation-observer2` candidate reuses the entire frozen025
physical generator and source. Its sole simulation-host seam replaces the final
observer validation call with a namespace bridge. That bridge checks original
local counters, owner, target, participant and timing; translates both coordinates
only in a clone; runs the full inherited validator; and discards the clone. It
never returns observer-derived state to the live actor or exported history.

This does not authorize weakening the original validator, modifying protected
source, retuning behavior or overwriting the first attempt. Before a new matrix
can run, the successor must pass its own regression, old-mode compatibility and
19-world exact-prefix gates, and freeze its own actual source revision. Until
then, the original required matrix remains failed and025is incomplete. A fresh
failure in actual physical/locality/time invariants stops dependent work again.

The diagnostic establishes the reason for this snapshot rejection. It does not
by itself validate the failed world's full material trajectory, provide all50
registered outcomes, measure benefits or establish community readiness. Those
remain separate obligations after an observer-correct successor passes its gate.

## Additional validation-provenance finding

Inspection of the original24-entry025helper lock found that its filename glob
omitted `review025_findings.py`. The full025reader and material checker were
bound, but the complementary findings reader was not. Do not claim the original
pre-treatment manifest covered the complete reader closure. The omission is
recorded separately in
[binding-gap evidence](../../experiments/025/diagnostics/validation-binding-gap.json).
Preserve the original manifest and explicitly bind the unchanged complementary
reader in the successor freeze before new outcomes. This is a validation-source
coverage correction, not a change to the simulation or its outcomes.
