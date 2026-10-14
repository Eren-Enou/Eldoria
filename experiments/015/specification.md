# Experiment015: self-initiated encounters and mutual availability

Pre-implementation preregistration,2026-10-14, foundation
`662de1e4a0d3b06341231a961787e7a3964cbb5a`. Gate A ACCEPT WITH QUALIFICATION at
`9e3a211`; Gate B passed all required checks and was pushed before this document.
All168 original001–014 artifacts and protected behavior stay unchanged. No016,
no015 protection or default adoption. Policies and plan below precede treatment runs.

## Question and hypotheses

Can three stationary individuals independently initiate, accept, decline and miss
encounters under limited mutual availability, producing actual interactions and
persistent personal/material history rather than an assigned meeting sequence?

A: existing cognition suffices with minimal scheduling. B: mutual availability
changes encounters despite compatible interests. C: own history/concerns influence
selection/acceptance. D: conflicting demands remove opportunities and persist.
E: simple initiation is comparable or better. F: fixture/order artifacts explain
apparent gains. No hypothesis is assumed; number of encounters is not welfare.

## Designs considered and selected boundary

Reject a global behavioral matcher, future-aware optimizer, movement/social graph,
interruptible rewrite of old resolvers and new invitation-learning map. Select a
bounded synchronous invitation wave with independent proposals, independent recipient
choices and objective arbitration. Three actors mean disjoint simultaneous pairs
cannot exist; existing resolvers can remain serial without suppressing disjoint work.
This is a controlled protocol, not a claim of natural timing or general scheduling.

The environment supplies stationary current co-presence and a public finite horizon,
never desired pairs. Each wave observes current public availability only. An actor
may choose Wait, propose Resource to a visible person, or request Inquiry from the
source selected by its existing bounded006/007/012 query logic. All sources are
current co-present non-self, excluding own concern targets to retain014's safe
third-party response protocol. No native capabilities/public offers are introduced.
The current encounter ordinal is public metadata in this shared room (old query
`scene` eligibility); all executed resource encounters are publicly counted, without
revealing their private participant components. No private past interaction payload
is supplied by that ordinal.

## Time, responses and arbitration

All proposals use the same pre-wave state before any private decision is revealed.
Sending occupies one shared tick; receiving/responding occupies the next tick.
Each proposer pays one tick of its own active time; each recipient receiving an
invitation pays one response tick. Other actors experience elapsed waiting time.
An actor may have sent and also receive: the phases do not overlap. A recipient sees
only incoming sender/type, current public availability/horizon, its own state/profile
and its own proposed plan. It chooses one incoming invitation, Continue or Wait.
The initiator learns only the actual public response, never the recipient's score.

An accepted invitation permits, but never guarantees, an existing interaction.
Accepted edges resolve in declared Ascending, Descending or round-rotating proposer
order; no private information changes order. Reciprocal requests collapse into one
execution; incompatible accepted edges lose the wave rather than executing twice.
Invitations expire at their single response/arbitration phase. No queued retroactive
acceptance is possible. Both participants must still be free and completion must
fit the current horizon. Failed invitations still spend the handshake time.

Admission reserves handshake2 plus resource6 or old selected Direct13/Evidence31.
Charge actual existing resolver ticks, not reservations. Both participants occupy
the entire executed interval; actual end determines when they are again free.
After execution or failure, fresh independent choices may initiate another wave.
All-Wait ends the period; no actor sees future availability release. Every non-Wait
wave advances at least2 ticks, horizon cap128 ensures termination. Explicit consumption
of IDs0/1/2 adds three ticks after the horizon. No resets, negative budgets, production,
overlap, duplicate execution or free failed invitations. An optional already-existing
busy commitment is an explicit environmental intervention, not a selected meeting.

## Exact pure replaceable policies and locality

Three initiation modes:

- Simple: eligible candidates score1; Wait0, resource partners in stable-ID order,
  then the old selected Inquiry. Strict greater-than keeps first positive tie.
- History: resource value uses014 max(best own scarcity-policy score,0)+own hunger/2
  minus handshake2; inquiry uses old selected query score minus2. Wait0 and same ties.
- NoHistory: identical History except resource ranking masks own trust/episodes.
  Actual interaction and query cognition retain all history. This ablates one
  specific resource-history input, not cognition/storage generally.

Common recipient policy: Wait0; incoming senders in stable-ID order. Resource score
uses the recipient's true own014 resource readiness minus2; Inquiry score is own
relationship_goal−privacy−2. Continue is considered last with own outgoing score;
strict greater-than keeps an equally valued incoming invitation. An invitation
acceptance is not a resource Accept, and a decline is not an invented resource
Refuse. No new trust or006 learning follows invitation decline alone; actual old
resource refusals and valid selected queries retain their established consequences.

Scores are uncalibrated heuristics. Policy input excludes others' hunger/food/traits,
private intentions/knowledge, future responses/schedules, truth, hidden provenance,
observer-only history, seeds and fixture labels. Own retained belief signatures
may be synchronized as established006/014, never rebuilt from evicted archives.
Own005 concern age advances once per wave and actual resource encounters. New
environment arrays cap at3, incoming/pending cap3; no persistent actor cognition.

## Reuse and fixtures

Reuse stable identities/ECS/Runtime and001/002 scarcity negotiation,005 concerns,
006 usefulness,007 acquired replies,008 Grouped,009 FIFO,012 Contextual. Do not use
010/011 or013 reconsideration merely because available.014 resource/inquiry safe
caps and actual-time logic are reused through a separate015 adapter; frozen014
source is not refactored. Resource execution omits automatic follow-up queries.

Fresh resource fixtures enable existing modes without fabricating cognition.
History/information fixtures reuse014's genuine two-refusal setup; same setup is
held fixed across initiation modes. Explicit fixed circumstances precede treatment.
Between periods hunger+20 (clamped100) is an audited exogenous intervention; food,
history, learning and cognition persist. Longitudinal food may exhaust legitimately.

13 families:

1. Mutual: hungry requester and possible donor, independently chosen reciprocal contact.
2. Unilateral: requester versus recipients with little own reason to participate.
3. Contended: competing hungry actors and donor; accepted edges can conflict.
4. Unavailable: current public prior commitment occupies a potential partner initially.
5. Missed: horizon2 prevents a complete handshake/interaction despite co-presence.
6. History: two genuine refusals can change partner ranking versus masked projection.
7. Information: active concerns and legitimately informed third-party supplier compete with resources.
8. Misleading: the supplier's hidden reading is inverted; valid learning may be wrong.
9. Private mirror: same proposer input, different hidden supplier willingness; response may differ.
10. Formerly useful: real setup inquiry yields useful partial evidence, followed by
    legitimate provider FIFO crowding; continued expectations may fail.
11. Refusal: invitation acceptance can still lead to an actual resource refusal and concern.
12. Separate: low need/readiness permits all actors to remain separate under History.
13. Longitudinal: same people for24–32 periods; no forced reunion or restored stock.

## Matched comparisons and prospective variation

Core budget24,6 periods (longitudinal24), Ascending. Four sensitivity configs:
same budget/horizon Descending and Rotating to isolate declared ordering; budget16
and48 with8 periods/long32 under Ascending to test coupled budget/horizon sensitivity.
No policy retuning between configs.13×3=39 core,156 sensitivity combinations;
seeds0,1,42,u64-max give780 worlds, each exactly rerun. Seed42 archives retain195
full histories plus a separate development preview; all seeds retain summaries.
Deterministic seeds are reproducibility controls, not independent statistical samples.
Only initiation mode differs within direct matched comparisons; private mirrors
and order comparisons are explicitly distinct interventions. No divergence reset.

## Measurement, independent verification and falsification

Record each local input/candidate/selection, public co-presence/availability,
incoming invitations, own recipient choice, arbitration and lost alternatives,
handshake/execution spans, exact event/query references, learning, concerns, food,
consumption/hunger, trust, persistent later state and contact distribution.
Read-only Python independently reconstructs policies/matching/time, old query
assessment/learning/provenance and resource accounting from immutable records.
Test private mirrors, simultaneous/reciprocal conflict, no double booking, expiration,
repeat refusal, no automatic-budget escape and observer recovery exclusion.

Reject causal claims if source/recipient is scripted, policy sees private/future
state, a response is forced, accounting fails, an archive restores cognition, or
only decisions differ without executed consequences. Keep no-effect, costly and
starvation results. Classify Demonstrated, Mechanism-dependent, Ambiguous and
Unsupported separately. Adoption and overall utility superiority are unestablished.

Time fixed-input initiation/response separately from complete worlds and growing
observer captures; record serialized storage separately from resident memory.
Expected added work O(actors²) for at most3 actors plus bounded old query scoring;
audits grow with waves. No general optimization. Required completion commands:
fmt, locked all-target Clippy, full locked tests, foundation and standalone009–014,
015 evaluator/exact replay/independent reader,168 protected hashes and relevant benchmark.

No movement, economy, production, networks, institutions, language, general planner,
friendship, human psychology, general intelligence or emergent civilization claim.

## Disclosed development corrections before sensitivity

The first targeted run passed7/8. The failed test assumed formerly-useful would
produce a receipt-free inquiry; the actual selected query instead acquired a
redundant B report, with support unchanged, rather than asking forgotten A. Preserve
that negative result. The test now checks the true invariant directly: source2's
live A is absent while its archive retains A; an actual old exchange cannot deliver
it. No policy, cognitive capacity or fixture was tuned to force another choice.

The first39-world development preview is retained in `development-preview.json.gz`.
Its matching audit called reciprocal same-kind requests Contention. Before final
evaluation, separate Reciprocal from genuine incompatible-edge contention. This is
an audit classification repair; proposals, responses, execution and outcomes must
remain identical to that preserved preview. No weights or behavioral rules changed.
An observer transition-count reference was also added to each compact checkpoint
so rejected/unused query previews can be checked against the exact local concern
prefix, without duplicating a full concern collection. This adds no policy input.
