# Implementation and evidence boundary

Registration commit `1f81324` precedes runtime implementation and outcome
inspection. The fixed 100-world matrix was not changed. Initial development
outputs remain separate in the task temporary directory; none replaced the
preserved original archive.

## Architecture actually used

`experiment::concurrent` is an opt-in namespace under the unfrozen
`src/experiment.rs` entry point. Its private adapter is a child of the unfrozen
adaptive integration module. Source-locked `src/lib.rs`, `src/simulation.rs` and
every original 021 source remain unchanged. The original multi-scene ECS core
executes resource actions; the new adapter does not clone its negotiation rules.

Live physical registry capacity is four, with at most one incompatible reservation
per actor. Send and Reply each take one tick; pending invitations occupy no actor
but prevent duplicate outgoing invitations. Resource negotiations use the old
six-turn bound, and each completes from its own scene's events. Environmental
claims use actual stock at their deadline. Meals remove owned food and reduce
hunger by the established amount. One global clock applies the old +5/4-tick need
law to all four people, including elapsed atomic-query time. Pulses use the
comparison epoch (`start + 4k`), not absolute tick multiples of four.

The material phase order is need, core scene actions, due claims, meals. The core
itself releases finished resource actors during its action pass. Registry resource
completion/concern bookkeeping follows that pass, before claims and meals; each
other lease releases with its own result. This refines the registration's coarse
word “release”: no decision/admission occurs between these material phases, and
incompatible reservations prevent the same actor participating in both a finished
scene and a claim/meal at that boundary. The independent reader checks the actual
ordering and all releases, rather than treating the registration as executed proof.

## Corrections before preserving originals

The first provisional adapter incorrectly initialized local encounter counts to
zero despite executed bootstrap history. This delayed inquiry legality. It was
corrected to reflect actual own completed encounters; no scores, thresholds,
conditions, retention modes or arbitration priorities were retuned.

A subsequent source audit rejected deriving those local context dates by filtering
observer scene history at decision time. The final implementation maintains owned
`LocalContexts` during actual completion, including bootstrap completion: four
counters and at most eight concern-ID/context entries per actor. These contain no
episode content. Decision projection reads this bounded owned bookkeeping.
Global event/scene references remain in the observer record and stored concerns;
the query input uses the corresponding local chronological context. Original
query concern-before/after audit values retain their actual stored references.

Neither correction is evidence of a new cognitive benefit. The superseded
development outputs are not the original results. All 100 preserved worlds use
the corrected implementation, independently reconstructed metadata, and fixed
registered fixtures.

## Capture and privacy

Full local choice input is captured once, alongside outer and contact candidates.
Full recipient response is captured once; a pending token stores only an immutable
boundary/index reference and selected reply. Commitments refer to their sending
boundary. These observer references are not policy inputs. Successful acquisition
updates a currently retained owned slot; its saved admission input cannot recreate
a missing slot. Other actors' takes do not silently update that slot.

Atomic inquiry remains exclusive with all physical work and pending deadlines.
Its currently selected question must match the invited question. A blocked query
does not execute a response or train inquiry learning. No free decisions or free
glimpses are invented inside an atomic interval. The new guarded time/context
hooks return unchanged behavior in old worlds without the opt-in state resource.

## Reproduction and scope

`trials.jsonl.gz` preserves 100 complete original JSONL members. The archive
manifest records compressed and raw hashes. `source-freeze.json` records 60
canonical-LF source/registration hashes, including 15 Python verification/writer
dependencies. `preregistration-source-freeze.json` links the unchanged design
artifacts to their prior commit. Frozen originals are not regenerated in place.

The read-only `validate022` gate verifies source/archive hashes, reconstructs all
100 worlds, rejects 23 scratch corruptions and repeats all 100 runs with exact
original JSON byte equality. It does not extend the 001–021 foundation.

Long cases run 192 continuous ticks in one world, spanning 24 absolute eight-tick
passive observation windows without state resets. They do not test reopening a
closed social-budget window. The final horizon still affects affordability; the
registered 16/48/96/192 horizon sensitivities must be kept visible when interpreting
late consumption and contact. Deterministic seeds are reproducibility and parameter
sensitivity, not independent statistical evidence.

The four-person preview is a separate read-only observation application. It is
neither an authoritative scientific validator nor a full adapter in the main
018–021 Explorer. Its before-reservation local inputs and after-start commitment
lanes are explicitly distinguished. Bootstrap evidence is labeled separately from
the declared initial physical intervention. There is no post-start world reset.

## Post-evaluation interpretation corrections

The prospective architecture/gate called need pulses and sensing cadence
“absolute.” The preserved implementation instead anchors both to comparison
start: need at `start+4k`, sensing at `start+cadence*k` when free. This is a
registration-to-implementation discrepancy, not evidence that absolute world
phases were tested. It was identified in the final document/source audit; the
originals and prospective documents remain unchanged. The independent reader
validates the implemented law, and each matched capacity pair has the same
bootstrap start/epoch. Thus matched capacity results and shared-time conservation
remain interpretable under the implemented law; transfer across initial phases
and exact preregistration adherence are not established. This qualification is
additional to the material-phase refinement above. No outcome is regenerated to
erase the discrepancy, and no retrospective policy/fixture tuning is performed.

The original `cross_bootstrap_pair_transfers` diagnostic counts104events across
the fixed ID partition{0,1}/{2,3};100cross two genuinely executed prior pairs.
The complementary [coverage](coverage.json) and [results](results.md) preserve both
counts without rewriting original reports. They also identify84case-label-neutral
histories among100rows, and distinguish food-ledger equality from terminal hunger.
