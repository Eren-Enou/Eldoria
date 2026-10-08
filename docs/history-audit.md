# History and decision audit foundation

The foundation reinforcement preserves Experiments 001–007. No behavior policy,
heuristic, random sampling order, time cost or cognition limit changes. Measurements
and compatibility evidence are in [reinforcement](../reinforcement/results.md).

## Ownership and layers

Live ECS components execute the next step. Individual memory, beliefs, concerns,
inquiry estimates and provenance knowledge retain their existing bounds and eviction
rules. Policies receive the same local projections as before. They cannot query the
observer indexes or recover forgotten acquisitions from history.

Objective events, information receipts, original interpretations, revisions, roots
and relay parents remain complete observer records. Append order determines stable
IDs within each archive; ECS entity handles are never serialized. A revision records
a new interpretation instead of editing the original event. Live cognitive state is
mutable; completed decision records and their captured contexts are immutable in the
runtime. Exported snapshots are detached copies, not mutable access to the world.

Decision audits preserve the input actually evaluated, candidates, selection and
outcome. Reports derive from those records without rerunning a policy. An observer
snapshot is not a resumable world save: it does not replace simulation initialization
or restore individual cognition.

## Referential query audits

Experiment 007 formerly duplicated the native inquiry input, decision, response,
learning cells and result. The native `Inquiry` record now owns that data once.
`audit::QueryRef` stores typed `InquiryId`, `QueryId`, `ContextId` and provenance
`ReceiptId` references, actor and timestamps, provenance factors, and its local
overlay. Receipt namespaces remain distinct: the new `ReceiptId` denotes a
provenance receipt, not a cognition receipt.

`ContextStore<LocalContext>` interns identical knowledge/hints/settings overlays.
Changed values receive a new first-occurrence ID. Hashes only select candidates for
exact equality checks; collisions cannot merge contexts. Hash buckets are omitted
from serialization and rebuilt when needed. Stable IDs and output order do not
depend on hash iteration. No context is reconstructed from present knowledge or
objective roots. Malformed replacement policies can return mismatched outer/native
inputs; `attempted_base` preserves that failure evidence rather than silently
normalizing it.

`Simulation::provenance()` still materializes the established public `Provenance`
shape. Existing experiment reports retain their schema and values. The separate
`Simulation::compact_snapshot(label)` export uses `causal-audit-v1` and avoids that
expansion. `CompactSnapshot::from_legacy` checks duplicated legacy fields for exact
agreement. `query(QueryId)` reconstructs one query; `expand()` validates and returns
the full old snapshot without rescoring, sampling or consulting mutable live state.
Use serde JSON serialization/deserialization for the compact format.

Validation checks reference existence, ID/slot agreement, ownership, timing,
earlier parents, relay/root consistency, cognitive bridges and locally available
attribution. A hidden root cannot replace an undisclosed local attribution token.
Errors are diagnostic strings. Validation covers these causal contracts; it is not
a cryptographic integrity system or an exhaustive verifier of every legacy field.

## Derived runtime indexes

`history::HistoryIndex` is private resolver infrastructure, derived from authoritative
events and receipts. It indexes received evidence summaries, immutable source
identity, inspection slots and directed relationship contributions. First-contact
trust and positive/negative evidence maxima retain the previous exact predicates.
These summaries accelerate resolution; they grant no new knowledge to agents.

Each relationship retains ordered contribution values, event positions and clamped
prefix trust. Revising an event replaces its contribution and recomputes the affected
suffix until its value converges. Every historical clamp is preserved. A simple sum
or refund would be wrong after saturation. Worst-case work still grows with that
relationship's affected suffix. All original events and revisions remain available.

Active scenes and busy participants use ordered sets; completed scenes remain in
the historical vector. Advancing active indexes preserves original scene order.
`validate_history_indexes()` rebuilds derived state from history and checks equality;
`history_index_counts()` exposes observer measurements. Neither supplies policy input.

## Adding a future system

1. Capture the actual local input at the decision boundary. Reuse an existing
   immutable record when it already owns the same decision; add only the extension.
2. Give every reference an explicit namespace, ownership and availability time.
   Validate targets before resolving them. Never infer actor knowledge from truth.
3. Intern only exact immutable versions. Do not alias mutable live collections or
   normalize distinct knowledge states because their eventual outcomes match.
4. Append causal outcomes and revision links; retain original interpretations.
   Preserve invalid attempts for debugging without treating them as valid evidence.
5. Keep individual eviction separate from observer retention. Add reconstruction,
   corruption, locality, replay and archived-output regression tests.
6. Measure export size and runtime separately. Add a derived index only for an
   identified cost, document growth, and provide a rebuild check.

Native inquiry contexts, other full decision inputs, report checkpoint prefixes,
objective archives and partner maps still grow. This pass does not introduce a
database, generic planner, universal event graph or persistent-world archival policy.

## Experiment 008 assessment audit

The 008 snapshot is a separate compact-base/assessment wrapper. Records retain local
incoming Items and explicitly namespaced native/provenance references, including
eviction and disclosed-attribution changes. Observer validation replays that stream
and checks the cognitive consequences; live assessment never reads the audit archive.
Native delivery of an acquired report resolves to its actual public payload, preserving
the same disclosed token or Unknown assumption as provenance delivery. Validation may
not substitute its objective root for unavailable attribution. Older exports remain
unchanged. See [008 findings](../experiments/008/results.md).

## Experiment 010 inquiry projection audit

010 references immutable query/inquiry IDs and the assessment record boundary at
decision time. It stores only active own (concern,event,support) triples, rather than
another copy of the full007 input. Prefix replay uses009's historical eviction
choices, including rejected incoming items and retained attribution replacement.
Receipt timestamps check that the prefix is exactly available at the query; future,
missing/reordered records and altered scores fail the010 built-in validator. This
observer replay is never a runtime cognition source. A prospective first-query
cursor permits enabling after existing history without importing it.

Checkpoints retain concern/agent state and causal boundaries for pressure, inquiry,
later deliveries, executed resource events and consumption. Existing response IDs,
assessment receipts, memory revisions, trust contributions and event memory causes
connect those steps. Added read-only resource probes diagnose mediation; actual
executed events/accounting and later inventory are the behavioral outcome. Timings
are separate CSV. Frozen001–009 evidence and001–008 contract history stay intact.
