# Read-only experiment-history viewer:scope only

This proposal follows [019](../../experiments/019/results.md). It does not implement
a viewer,change simulation behavior,start020 or authorize a game frontend. Independent
019 review is the first scientific priority;this tool is the next low-impact engineering
candidate because existing traces already contain difficult-to-inspect causal evidence.

## Minimum useful interface

Evaluate a small local static web interface with an optional local Rust/Python indexing
CLI. It should load an existing trace through local file selection,choose a trial and
individual,and navigate recorded moments. Bevy rendering,accounts,cloud hosting,
real-time simulation control and generated narrative are unnecessary. Prototype browser
memory/streaming behavior before choosing a delivery mechanism;no dependency is
installed as part of this scope.

Start with019 JSONL/gzip and018,using versioned adapters rather than rewriting old
archives. A trial identity includes experiment,schema,seed,case,configuration,mode and
archive/record identity. Display manifest/source-freeze hashes and validation status.
Valid final evidence and INVALID_PHYSICAL_INTEGRATION development evidence must be
separate selections;invalid traces remain available for audit and must not be pooled
into scientific summaries as valid negative results.

## Actor lens and observer lens

The actor lens shows only that individual's captured owned state and legitimate local
input at the selected recorded moment:own food/hunger,retained concerns,own directed
relationships,retained assessment/known attribution,available public people,action
candidates,selected action and current public bounds. It should show incoming response,
provisional consent and actual executed edge separately. An intended question can lose
arbitration;an accepted contact need not transfer food or deliver useful information.

Use captured contexts and retained-state prefixes under that experiment's documented
rules. Never fill a missing episode,evidence acquisition or belief merely because
objective history contains it. A missing field should say not captured. Distinguish
own persistent trust from current episode content and from experimental outgoing-only
history projection. Do not falsely present hidden roots,partner-private inventories,
source motives,truth or later events as what the individual knew.

The explicitly separate observer lens may show full resource accounting,actual global
events,hidden provenance and cross-agent state for audit. Label those as observer-only.
Truth,novelty,retained support/status progress and material consequence need separate
fields. A wrong but locally supported interpretation must remain visible as such.
Avoid automatic winning-policy or good-person badges.

## Timeline and causal navigation

Show actual resource events,queries/responses,meals,need updates,commitment start/release,
observation gaps and concern/trust transitions. Follow stable domain references across
events,receipts,acquisitions and assessments;their namespaces differ. Use trial-scoped
identities,never serialized ECS handles. Link a selected choice to its recorded input,
scores or priority,bounds,consent,actual time,outcome and later state.

Replay means chronological display of recorded history,not re-running a current policy.
Tie/priority explanations must use the captured rule/version and candidates. Do not
quietly substitute a later heuristic for an old choice. Inside an atomic query,there
may be no actor decision snapshot at an arbitrary tick;the interface must distinguish
recorded actor moments from observer reconstruction rather than inventing deliberation.

Paired comparisons should align relative world time and common endpoints,not period
index when window lengths differ. Separate first voluntary divergence,executed action,
temporary material change,full reconvergence and persistent relationship/history.
An admission exclusion does not establish physical impossibility or a bad priority.
Pressure-time is a model measure,not general welfare.

## Storage, read-only boundary and verification

019 stores about1.13GB raw JSON in19.90MB gzip,with large per-trial records. Do not load
all histories into a browser object graph by default. Stream one registered history
and keep a compact timeline/point-reference index. Plain gzip does not supply a simple
native per-record seek table here;use sequential scanning or an optional derived
per-trial cache keyed by archive digest and schema. Measure latency and peak memory.
Such a cache must not replace protected archives or become an input to validation or
simulation. Preserve original immutable point references instead of expanding copies.

The viewer's data flow ends at display/export. It must have no path back into live
policy inputs,actor cognition or resource state. It should never invoke an evaluator
to regenerate a missing archive. Treat loaded strings as data,not executable formulas.

Acceptance tests should include the018 invalid clock candidate,the019 four-tick
admission witness,identical initial requester inputs with private cooperation reversal,
declined/contended inquiry,wrong/redundant evidence,episode/evidence eviction,317 full
nulls and temporary reconvergence. Compare displayed numbers/references to read-only
validators,verify file hashes before/after loading,and test keyboard navigation and
clear lens labels. No simulation outcomes may change.

Next step if separately authorized:a feasibility prototype loading one immutable019
trial and one pair,with actor/observer separation and exact reference navigation.
Only then assess wider schema support or a larger frontend. No viewer is built here.
