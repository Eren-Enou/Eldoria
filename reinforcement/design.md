# History foundation reinforcement — before refactoring

2026-10-07. Behavioral contracts 001–007 are frozen. This is not Experiment 008.
AGENTS contains the ten-part Simulation Constitution; there is no separate
constitution file. Baseline formatting, Clippy and all 83 tests passed. Earlier
uncommitted Experiment 007 work and historical evidence are preserved.

Measured baseline: `baseline-throughput.csv`, `baseline-measurements.csv` and
`baseline-structure.json`. The added measurement harness changes no simulation
rules. It separates initialization, execution, legacy snapshot construction and
serialization and includes 100/1,000-agent workloads plus 100/1,000/4,000 paired
refusal/disclosure encounters. The latter measures 0.88/65.14/1,034.90 ms execution.
Public snapshot label differences account for 52 bytes versus the saved 007 population.

The 1,000-agent snapshot occupies 11,952,309 compact bytes; provenance contributes
3,483,413, with 3,441,387 in queries and zero live provenance knowledge. Within those
queries the repeated base inputs occupy 664,916 bytes, repeated native decisions
1,457,716 and repeated native responses 403,323. These three categories do not
overlap each other in the provenance record. Their authoritative equivalents already
exist in the linked native inquiry record. Additional repeated cell/outcome/time
fields are also exact copies. Inspection/exchange contexts and older TalkInput,
follow-up, inquiry and forecast contexts repeat bounded state too, but most are
distinct versions and should not be indiscriminately normalized.

Findings from implementation inspection:

- Scene admission, idle checks and each advance scan all completed scenes.
- Receipt processing scans the information archive for source immutability, first
  contact credibility, and positive/negative evidence maxima.
- Every memory revision recollects all replacement contributions and scans all
  resource events to reconstruct one directed relationship, with saturation.
- Inspection-slot uniqueness scans every objective root.
- Direct event/root/parent and bounded-belief receipt references already use indexed
  lookup; bounded cognition scans are appropriate. These need no global knowledge cache.
- Reports copy entire archive prefixes at checkpoints. This is derived report
  convenience, not information required to execute the next step.

Planned narrow changes:

1. Store provenance query extensions by typed native inquiry ID plus an immutable
   local context ID, factors and receipt link. Intern only identical local overlays;
   never infer hidden attribution or reconstruct forgotten knowledge. Preserve
   malformed policy output faithfully in an exceptional input override. Materialize
   the old public Query shape only at the compatibility/report boundary.
2. Provide a versioned compact observer snapshot, deterministic reconstruction and
   diagnostic reference validation. Add reusable typed-ID/context storage primitives
   justified by existing cross-record duplication, not a general event framework.
3. Track active scenes/participants in stable ordered sets. Index evidence summaries
   and immutable inspection slots using exact historical predicates.
4. Maintain per-relationship ordered contribution/prefix caches. Revision replaces
   one contribution and recomputes only its affected suffix, stopping when cached
   trust converges. Preserve every clamp; do not substitute a naive sum/refund.
   Rebuild/check indexes from authoritative records without exposing them to policies.

Risks: exceptional invalid-policy outputs must still serialize exactly; index drift
could alter behavior; saturation makes trust updates noncommutative; same-tick nested
records need explicit ownership links rather than naive timestamp inference. Tests
will cover these, corruption, forgetting and complete old archive equivalence.

Rejected at this stage: deleting objective history, serving policy contexts from the
observer graph, generic recursive JSON compression, caching away legitimate forgetting,
global context deduplication without evidence, and a database/persistence service.
Old public snapshots intentionally retain their byte/structural shape. Compact output
will be a separately versioned format with lossless expansion, not silent migration.
Unbounded authoritative archives and indexes remain; no large-world claim follows.
