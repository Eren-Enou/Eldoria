# First persistent world — feasibility and prospective contract

Starting revision: `77707def0e13936c192a3bc987b8971acb3898d1` (clean).
This is opt-in continuation engineering, not Experiment026 or policy adoption.
Protected001–024 and separate025 remain untouched.

## Gate A: actual ownership and minimum successor

The023 producer owns an ECS World and a single-threaded Schedule inside
`Simulation`. `simulation::Runtime` owns tick, scenes, events, physical
circumstances, initial trust, conversation cursor, active scene indices and busy
actors. Entity handles occur only in `Index`; restoration must recreate that
mapping from stable actor IDs, never serialize it.

`simulation::adaptive::concurrent::State` owns the fixed physical configuration,
need epoch, last synchronized tick, public opportunity endpoint, remaining stock,
conserved total, active commitments, pending invitations/responses, transitions,
admissions, need pulses, grants, environmental observation audit, four bounded
LastSeen slots, receipt counters and next commitment ID. Pending records include
the original question and reply reference. These are continuation state.

Agent components own inventory, hunger, traits, trust and capped episodic memory.
Additional ECS resources used by023 are Cognition, Intentional, Foresight,
Concerns, Inquiry, RuntimeProvenance, Contexts, Assessment, Retention,
self_evaluation::Learning and concurrent::LocalContexts. These include bounded
beliefs, claims, concerns, evidence, attribution, estimates, contact signatures,
learned expectations and immutable audit records. Their whole typed values must
survive; no journal-derived repair of actor state is permitted. HistoryIndex is
a resolver acceleration containing ordered relationship contributions and source
indexes, not actor knowledge. Preserve it too, validating against the existing
independent index reconstruction. ContextStore's skipped hash lookup is rebuilt
lazily from saved immutable values by its existing intern method.

Policy function pointers and Schedule are executable identity, not serialized
data. Reinstall the exact registered scarcity/intentional/foresight/concern/
inquiry/provenance/Grouped/FIFO/Contextual functions on restore, with a strict
source/runtime identity. The bootstrap Generator is consumed before the first
continuation boundary. Later sensor randomness is keyed by saved seed and stable
event/source IDs; there is no advancing RNG cursor to invent or reset.

The existing023 loop has no continuation interface. Its final Trial omits private
runtime/cursors and cannot serve as a save. Use a separate Cargo successor host
that verifies inherited source bindings, adds Serde derives to otherwise unchanged
private storage declarations in generated files, and appends private persistence
children. Original source files remain untouched. Reuse the exact023 boundary and
bootstrap bodies. Binary typed payload supports tuple-key resolver maps; an outer
versioned envelope binds payload, configuration, runtime and journal prefix.

## Safe execution boundary and horizon

A step executes one existing continuous_boundary, then its ordinary concurrent_tick
if below terminal end. Save only before the next loop iteration, or at initial or
terminal state. An inquiry admission can atomically jump time; no save occurs
inside that call. Pending contact, meals, claims and social scenes can remain live
at these boundaries. Pausing simply stops calling step; it performs no physics,
selection, finalization, sensing or learning. Wall-clock time is absent.

The declared nominal cutoff is start+horizon; actual terminal end is cutoff+33.
Only the original loop's cutoff suppresses new choices, and charged drain remains
physical. Continuous public eligibility is now+33, independent of reporting period
or initial horizon; Windowed public eligibility depends on period. A fixed endpoint
still controls when choices cease. No endpoint extension is supported here.

## Prospective gates and risks

Before GUI: prove complete typed save/restore, then exact canonical023 history
against the original producer, including null, scarcity/contention, inquiry,
active/pending work, reporting crossings, nominal cutoff and drain. Compare saved
full state at every tested split as well as final Trial bytes. Reject corrupt
payload/checksum/version/identity/config/journal, duplicate IDs and illegal state.
Do not classify a plausible final total as equivalence.

Journal records contain only completed execution prefixes, stable boundary and
event references. Checkpoints contain genuine saved state. Immutable generations
published by rename retain the previous valid generation. Flush files before
publication; do not claim power-loss directory durability without platform proof.
Incomplete writes and unmatched journal tails must be reported, never silently
replayed into cognition.

Observer archives embedded in resources grow even though cognition is bounded.
First measure state size, save/load, journal growth, simulation versus observer
cost, working set and UI responsiveness. Full final snapshots on each short pause
would duplicate history and are a known risk; avoid additional redundant exports.
Only after023 equivalence assess024 reuse;025 escrow/production remains deferred.
No serializer or GUI implementation preceded this audit.

## Secondary024 feasibility decision, after mandatory023 proof

The mandatory gate passed twelve1024-tick worlds and228 full-state split checks
with exact original history (local test time162.97s, not an isolated runtime
benchmark). Seven64-tick cases also passed restoration at every legal iteration;
separate-process resume, semantic corruption and forgotten-episode/pending-reply
checks passed. No physical or cognitive divergence was observed.

024 is feasible with the same state boundary: its only additional continuation
state is explicit registered valuation configuration and accumulated immutable
valuation records. Reuse the original guarded024 generator and boundary body,
changing only method visibility in generated code. Preserve `H+min(H,40)` exactly,
ordinary eligibility/masks, stable ties and Baseline/MealRelief opt-in status.
Add independent original024 comparisons before claiming support. Neither mode
replaces023 as the default.025 is not included by this decision.

## Single-writer and interrupted-save correction

Storage review identified an external race: two application processes could load
the same saved generation and append competing continuations. Hold an OS file lock
for each open Session; its handle is external execution state and is never saved
as actor/world data. A second writer must fail before executing. Process exit
releases the lock, while its harmless lock file remains. This is not a claim of
power-loss durability or automatic recovery of unsaved journal tails.

A failed `.pending` generation must remain diagnostic evidence without permanently
blocking a later successful save from the still-live world. Retry chooses a new
unused generation, leaving both the prior committed checkpoint and incomplete
write intact. Neither lock ownership nor generation gaps enter actor policy input.

The process-restart prototype cost39.47s for a1024 world under host load. Inspection
identified an unnecessary complete checkpoint serialization on every journal step,
used only to obtain a state hash. The correction binds complete saved-state SHA256
and completed journal-prefix SHA256 together in each committed checkpoint. Ordinary
journal steps serialize only their new records. Restore still checks the full state,
both hashes, their binding, and actual boundary-record equality; it never reconstructs
cognition from the journal. Compare timings separately from deterministic evidence.
File ownership uses [Rust's file locking API](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock).

## Checkpoint and journal boundary

The published numbered file is `woi-world-commit/v1`. Its fields are schema,
world ID, runtime identity, generation, actual tick, journal byte length and SHA256,
state SHA256, binding SHA256 and the complete inner state bytes. Binding hashes
the typed metadata tuple, including both content hashes. The inner
`woi-world-checkpoint/v1` envelope contains schema, runtime identity, payload SHA256
and a native CBOR byte-string payload. The payload's complete fixed configuration
is its configuration identity, checked against the physical configuration,
valuation registration and declared endpoints. It is not imported from a journal.

The typed payload contains execution phase (`BeforeBoundary` or `Terminal`),
configuration, optional024 registration/valuation audit, start/end, initial agents,
bootstrap references, initial local contexts, initial stock/total, completed
boundaries, current agents and the fourteen data resources inventoried above.
`Runtime.tick` is authoritative; the outer actual tick must match. Policies and
Schedule are reconstructed executable wiring. Entity mapping is recreated from
the saved domain IDs. No bootstrap, generator, previous-step replay or elapsed
wall-clock update runs during restore.

The journal header contains the actual completed bootstrap. Each subsequent line
records one original scheduler iteration: its actual clock interval/boundary,
physical changes, recipient/admission records, needs, grants, meals, environmental
memory changes, information, queries, provenance/context deltas, assessments,
concern transitions, communication, retention and learning audit. Only new records
are appended; newly interned context values retain their original IDs. The native
step inspector reads these records as observer audit, never actor input.

Missing or inconsistent cognitive data cause rejection. Read-only checks compare
retained episode membership and the actual last environmental-memory transition;
they do not populate missing cells from those audit records. Persistent trust,
concerns, expectations and other separate effects retain their established
lifetimes. Raw resource/need checks similarly validate the saved state against
executed records without changing it.

Generations are retained rather than automatically pruned. Repeated frequent saves
therefore grow disk usage even when live cognition remains bounded. Source audit
histories also grow inside complete checkpoints. This milestone supplies neither
history compaction nor automatic recovery from an unsaved journal tail. Unknown
runtime identities are rejected; no cross-version checkpoint migration is claimed.
