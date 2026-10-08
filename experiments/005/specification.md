# Experiment 005 — hypotheses recorded before implementation

The unchanged Experiment 004 baseline passes formatting, Clippy, all 43 integration
tests and the four-mode throughput benchmark. Earlier experiment evidence is retained.

H1: A listener preserves a consequential unresolved refusal after its conversation
ends. A separate bounded concern is neither a memory nor a belief. H2: On a later
encounter a local policy sometimes spends time asking about that event. H3: Higher
pursuit cost, lower relationship importance, age, and repeated unsuccessful pursuit
can favor continuing normally or abandoning the issue. H4: Fallible evidence can
partially or sufficiently resolve a concern through existing belief revision, even
when the resulting belief is objectively incorrect. H5: Competing concerns receive
different priorities; at most one can be pursued by each participant per encounter.

Implementation plan: opt-in ECS resource, eight concerns per individual, stable event
and receipt references, one question (scarcity behind a refusal), four statuses
(open, partial, resolved, abandoned). Creation requires uncertainty and importance
at least 40. Sufficient resolution is absolute belief support at least 60. Cache only
the subjective uncertainty scalar and last receipt so eviction of an episode does
not erase unfinished intent. Terminal concerns count toward capacity; evict terminal
first, otherwise least important with an explicit capacity-abandonment record.

Follow-up runs at the boundary after a later resource encounter, before that
encounter's new refusal conversation. No retroactive or same-encounter reopening.
Age counts the owner's subsequent encounters. A replaceable policy receives only
own concerns, own needs/profile, partner identity and own answer expectation plus
public sensor quality. Continue scores zero. For each relevant live concern:
expected gain = answer probability * uncertainty * reliability / 10000;
benefit = saved importance/2 + current relationship goal/2 + expected gain/2;
cost = configured pursuit cost + 4*age + 20*failures + hunger/10.
Reopen scores benefit-cost; abandon scores cost-benefit-20. First candidate wins ties.
This is a testable heuristic, not optimal planning or calibrated social psychology.

A selected request costs one turn plus configured pursuit effort. The existing
communication resolver supplies a voluntary evidence-or-silence response followed
by closure, with a two-turn limit. Evidence uses the original source key, preventing
repeated requests from manufacturing independent corroboration. Belief updates and
trust replay remain unchanged. Failed attempts mean uncertainty did not decline;
the actual response also updates the existing local answer expectation. All selected
actions, costs, transitions, response references and prediction updates are audited.

Controlled conditions cover A–J: persistence, reopening, cost, relationship value,
weak/strong/conflicting and misleading evidence, repeated failures, competing
concerns, history-only policy ablation, deterministic replay and population use.
Keep interventions separate from learned state and audit them in trial snapshots.

Tradeoffs: only refusal-related evidence questions; no general planner or autonomous
meeting scheduler. Age is encounter-relative, not wall-clock time. Audits, original
event archives, and existing partner maps remain unbounded. Bounded concern scans
are cheap, but evidence lookup and trust replay still search historical archives.
Measure initialization and execution separately; benchmark both formation and later
encounters. No claims about unmeasured population sizes.
