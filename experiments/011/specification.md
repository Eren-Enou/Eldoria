# Experiment 011: bounded inquiry value under competing concerns

Preregistered 2026-10-08, baseline f5695731dd9d06d82e0f3bc3056ddcce235ef892,
before implementation and before executing any 011 suite.

Question: can limited inquiry be allocated using own importance, expected answer
usefulness and retained state, without giving uncertainty intrinsic value or knowing
future relevance? A: Unchanged is sufficient; B: CurrentNeed is sufficient;
C: StatusValue adds a useful bounded distinction; D: apparent improvement requires
privileged inputs or harness tuning. No acceptance or universal winner is assumed.

## Frozen rule

Reuse the complete 006/007 candidate enumeration and expected_gain, including locally
learned source/method estimates, public offers, cost and known independence. Reuse
010's projection of at most eight active own concern/event/support triples from the
live at-most32 Grouped items. For each existing Ask candidate:

```text
g = existing candidate.expected_gain                       # 0..100
d = max(1, 60 - abs(current retained Grouped support))
leverage = min(100, g * 100 / d)
value = (g + leverage) / 2
benefit = existing concern importance * value / 100
score = benefit - existing cost
```

All divisions truncate integer arithmetic. Pause stays zero. Strict greater-than
selection preserves existing candidate order on ties. The existing active-status
legality rule excludes resolved/abandoned concerns. No concern is reopened. Missing
retained support is zero; history cannot fill it. This formula and its equal weighting
are frozen for both controlled and held-out execution; no outcome-based tuning.

`leverage` is a heuristic relating expected usefulness to current distance from the
established absolute-support60 status threshold, not a simulated future state,
probability of resolution, expected sampled evidence quality, or truth probability.
It adds a near-threshold distinction: a small expected useful answer can be valuable
near resolution, while its value is lower far from resolution. Zero usefulness or
zero importance gives zero benefit; uncertainty alone creates no value. Equal weighting
retains sensitivity to answerability even when leverage saturates. This proxy may
overvalue repeated claims, wrong confidence or unreliable expectations; preserve that
failure. There is no rollout, resource-policy lookahead or search over future states.

Modes: Unchanged calls the original provenance inquiry policy; CurrentNeed calls the
exact finalized 010 function; StatusValue uses the rule above. All are explicitly
enabled only through the new 011 experiment path. FIFO retention remains fixed. No
new persistent cognitive state, default, capacity or previous behavior changes.
New state consists only of observer audit references and experiment configuration.
The replaceable pure scoring function receives the existing actor-local decision
and bounded support projection, never a Simulation or observer record.

Allowed: own current concerns/status/importance, live Grouped support, existing
expected_gain, own learned cells, public source/offers/cost, legitimate attribution
already reflected in 007. Forbidden: truth, sampled future answers, hidden roots/depth,
future partner/event/action/status/usefulness, scenario/seed/experiment labels,
observer history or private partner inventory. Resolver fixtures are never policy
inputs. Observation of future resource readiness is offline audit only.

## Fixed design and scenarios

Three actual refusal interactions establish three meaningful concerns, using declared
initial relationship profiles and physical interventions. All modes receive identical
agents, concerns, deliveries and opportunities until voluntary divergence. No reset
after divergence. Two sequential public meetings normally permit one actor inquiry
each, fewer slots than the three initial concerns. Existing voluntary answers,
assessment/status updates and 006 learning change the next choice; silent answers
consume the opportunity. The later existing resource scene executes legal actions,
then consumption tests persistence. Automatic inquiry after a resource decision is
preserved; it cannot retroactively recover a lost transfer or an earlier slot.

| Family | Controlled distinction |
|---|---|
| uncertain_minor | High uncertainty/minor stakes versus important moderate support |
| poor_answerability | Important concern/poor public answer opportunity versus useful source |
| near_resolution | Small expected answer against support near60 |
| far_resolution | Small response against support far from60 |
| expensive | High public evidence effort |
| learned_poor | Actual prior silent inquiries lower selected local estimates |
| new_opportunity | New public channel/quality after a poor prior opportunity |
| resolved | Legitimate reply removes a concern from subsequent candidates |
| wrong_confidence | Inverted strong retained evidence closes a concern incorrectly |
| minor_later | Locally minor concern is fixed later material partner |
| mirrored_future | Same inquiry setup, different fixed later resource partner |
| equal_tie | Equal local values preserve stable ordering |
| partial_answers | Weak responses permit continued unresolved competition |
| silent | Voluntary silence wastes a limited opportunity; learning remains original |
| same_uncertainty | Same support, different established importance |
| same_importance | Same importance, different public expected usefulness |

Cases may legitimately overlap in outcomes. Weak/silent/partial cases must not be
reported as successful resolution merely because selected queries differ. Include
locally/materially worse allocations for every mode, preserve wrong confidence, and
do not define all transfers as wins. Future partner is fixed independently of mode.
The mirrored family pairs the uncertain_minor inquiry setup with a different future;
compare full first local input and choices, not just scenario names.

A shared prelude in learned_poor/new_opportunity supplies actual prior inquiries
under the unchanged mechanism before treatments are enabled. No cells are injected.
This is history preparation, separately recorded from the limited treatment budget.
All other source announcements/responses are voluntary existing mechanisms. Source
capability changes between meetings follow a fixed external sequence, not treatment
selection. No autonomous scheduling, travel, planning or social expansion.

## Held-out plan and causality

Freeze four additional parameter/schedule combinations: concern profile goals and
support ±10/15, response reliability35/65/95, evidence effort0/12/35, source subsets,
two versus three meetings, normal/reversed public listing and evidence delivery order,
later partner/stakes1/2. Each combination varies several factors; this is sensitivity,
not an independent dataset or isolated factor estimation. Include mirrored futures.
Run seeds0..15 plus42 and u64::MAX, exact complete-state reruns. Do not tune policy
after viewing held-out outcomes. Fixture corrections needed for promised coverage
must be disclosed, never disguised as untouched blind preregistration.

Each opportunity records own active concerns/basis, all existing candidate components,
selected action, actual reply receipts, resulting assessment/concern transitions,
learned cell, time and remaining budget. Recomputable StatusValue components use
immutable local query input and the bounded captured basis; do not duplicate full
bounded inputs into another audit collection. Checkpoints retain current agents and
concerns for causal comparison. Pre/post readiness diagnostics never select inquiry.

E_Q requires changed inquiry sequence, equal pre-inquiry readiness, changed immediate
post-inquiry readiness agreeing with actual opening actions, conserved executed
transfer and post-consumption divergence. E_A is same inquiry with different
assimilation, never allocation evidence. F is unresolved mediation, including intervening
delivery or pre-existing readiness differences. C is changed inquiry without action
change; equivalent results stay equivalent. Never promote a different policy probe
to executed consequences. Report concern coverage, resolution, repeated/unproductive
queries, unresolved remainder and cost separately from resource outcomes.

Reject privileged information, observer recovery, future-aware schedule, scripted
targets, private leakage, disguised planning, changed bounds or CurrentNeed retuning.
No justification if all controls are equivalent, distinctions disappear in held-out,
or meaningful mistakes vanish. Accept/qualified acceptance requires interpretable
new locally grounded distinctions and failures, not more Offer/Accept totals.

## Complexity, verification and limits

Projection costs O(8×32) plus existing Grouped work; scoring O(candidates×8).
No new persistent cognition. Observer records grow with inquiries; capture only query,
inquiry and assessment-prefix IDs plus <=8 support triples. Measure serialized observer
bytes and whole-run/serialization time separately from deterministic evidence; these
are not isolated policy CPU or resident-memory measures. No audit infrastructure
refactor or general optimization. Execute throughput and focused three-mode timings.

Run fmt, locked all-target Clippy with warnings denied, locked tests, protected
validate_contract, validate009 and validate010, exact 011 evaluator replay and all106
frozen archive hashes. Save new evidence under011 only, do not extend protected contracts.
Document successes, failures, ambiguity, acceptance classification and strongest open
question. Controlled schedules, fixture sensors and heuristic usefulness limit
external validity; no human reasoning, optimality or natural-world claim. No012.

## Fixture construction corrections before held-out interpretation

The first construction attempts violated existing capability limits (channels0–1,
quality<=90, immutable content/quality under an existing provenance). Correct these
without changing old APIs: retain stable channel1 for ordinary opportunities;
new_opportunity starts without A evidence and advertises a new channel0 on the next
slot. The declared95 held-out response is capped at90 by the established limit,
and initial native evidence is likewise capped at90. No capacity/rule is enlarged.

Privacy100 with relationship_goal100 did not guarantee silence under the old voluntary
response policy. Shared preparation and the silent family use goal0/privacy100 on
sources, then restore goal100/privacy0 before treatment in preparation families.
Preparation is one unchanged meeting for learned_poor/new_opportunity, seven with
source1 for near/far, nine with all three sources for partial_answers. This deliberately
supplies learned estimates without injecting them; no experimental policy chooses
these preparatory inquiries. Source1 alone is present during near/far treatment.

The controlled preview also exposed that a nominal partial-answer family could select
an unknowing source for its second inquiry. To actually exercise multiple partial
answers, that family uses the fixed public source sequence1,2,(3); H0 shifts this to
2,3. It still permits all active concerns, including incorrect queries to that source.
This is a disclosed coverage correction after controlled preview, not an untouched
blind preregistration. It is not evidence that StatusValue improved; wrong choices
remain. No scoring constant, importance threshold or retention rule was tuned.

An additional locality test changes the private inventory of unmet agent4 before
treatment (food0 versus9), later encountering4, while keeping the actor's inquiry
input and public meeting identical. This private fixture variable is not policy input.
