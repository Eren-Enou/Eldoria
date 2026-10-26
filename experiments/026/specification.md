# Experiment026 — actor-local valuation of delayed productive work

PROSPECTIVE DESIGN. No026 implementation or treatment outcome has run. Starting
revision `a3a715090987681582e67a8e2ff613ce9cfb3e5e`. Publish this specification,
policy identity, exact matrix, source bindings and design gate before implementation.
Treatment additionally requires qualified025 protection and a published runtime
freeze after disabled compatibility. No default adoption, GUI or persistent026
integration, canon, renewal, population expansion or subsequent experiment.

## Question and alternatives

Does replacing only the existing Prepare need proxy with a bounded projection of
own need at the known work-completion time change voluntary activities and their
actual later physical, cognitive and social consequences?

H0: changed scores are behaviorally inert, or execution differences do not establish
useful valuation. H1: anticipated own need changes selections. H2: Prepare sometimes
displaces another eligible action. H3: later causal changes can reduce preparation
or make earlier work disadvantageous. H4: extraction/conversion, meals or social
history changes. H5: some individuals face worse hunger or lost opportunities.
H6: stale source knowledge still causes full-cost failure. H7: end resources
reconverge despite different histories. H8: production changes without subsequent
use. H9: no observable advantage is established. None is a success quota.

At an identical local input this intervention can only raise or preserve Prepare's
score. It cannot make an already winning baseline Prepare lose there. This is a
structural restriction, not an empirical H3 finding. Total later attempts or use
may still fall after worlds causally diverge. Do not tune to obtain both directions.

## Baseline mechanism and interpretation before choosing numbers

025 uses `Prepare=H−D`, D the sum of positive durations in an own retained raw
observation. Consume and eligible ready acquisition use H under the fixed Baseline
valuation; Invite uses the already established contact heuristic. Those scores
are heuristic need/contact priorities, not calibrated common utility. Wait is0;
ineligible candidates are−1; first strict maximum in Wait,Consume,Invite,Acquire,
Prepare order wins. The same one-point-per-work-tick convention is retained.

Neither existing food creation nor more social transfer is intrinsically better.
024 showed that an immediate-need score modification can worsen later hunger.
025 showed actual full-cost stale failure and costly sharing. This motivates an
isolated prospective change, not confidence that projection improves decisions.

## One exact frozen policy:ProjectedNeed

For an already eligible Prepare candidate, using integer arithmetic only:

```
D = retained.extraction_ticks + retained.preparation_ticks
q = floor(D / 4)
projected_need = min(100, own_hunger + 5*q)
Baseline.Prepare = own_hunger - D
ProjectedNeed.Prepare = projected_need - D
```

The treatment replaces the current H term; it does not add several bonuses.
All other candidate terms, eligibility, old-first ties, contact selection, consent
and physics remain unchanged. Ineligible Prepare remains exactly−1 with no new
projection term. The inherited v::Mode is always Baseline: no MealRelief factorial.

The policy's fixed model prior is the existing own-need law+5 per four elapsed
ticks, capped100. It is not learned forecasting. Durations6/12/32 permit at most
8 complete four-tick intervals, so the increase is bounded0–40 and projected need
lies in0–100. Work effort stays the original D. Checked integer operations and
the existing hunger/duration validity bounds apply; no floating probability.

This is a phase-agnostic lower-envelope projection under continued exclusive work:
an interval of length D contains at least floor(D/4) need pulses under any inherited
phase. The actor does not receive the resolver's epoch, exact future pulses or
actual completion hunger. Missing phase prevents a stronger exact forecast.
The fixed rate/cap are policy-model assumptions grounded in the existing law,
not a newly received observation, private configuration shortcut or biological claim.

The estimated quantity is modeled own need conditional on spending the known work
interval. It is not extraction success probability, expected net food, actual
future meal relief, future opportunity value or welfare. Source depletion, stale
upper bounds, later eating/sharing and competing opportunities remain unresolved.
The comparison retains025's need-priority scale and effort convention, whose
interpersonal/social commensurability remains heuristic and uncalibrated.

The law increases need by1.25 points/tick before saturation while the inherited
effort convention subtracts1 point/tick. Consequently, at low H the projection
can value a longer fixed duration more highly. This is a prospective weakness of
using anticipated need as a priority proxy: waiting until hungrier is not itself
a productive benefit. The matrix has one inherited duration per world, no actor
choice among recipes, and cannot establish optimal duration selection or net
utility. Preserve this limitation even if some treatment outcomes look favorable.

## Information and eligibility boundary

The pure replaceable selector receives the existing025 actor-local Input and four
inherited candidate terms. It uses own H and retained D only for projection.
Original eligibility uses own food/availability, own source0 receipt/access/date/
upper bound and public now/end. No new observations, capacity or prediction state.

No actual stock, partner inventory/escrow/intent, hidden provenance, future consent,
future partners/opportunities, future hunger, actual response, scenario/cohort label,
observer history, result classification or truth-aware success estimate is available.
Audit records refer to captured inputs and never become cognition.

Prepare food0 versus Consume food>0 are mutually exclusive. Ready acquisition and
raw preparation also cannot both be knowledge-eligible with the inherited one tagged
slot. Immediate legitimate competitors are Invite/Wait; Acquire/Consume can differ
later after ownership or retained knowledge changes. No eligibility is changed to
manufacture co-eligible alternatives or a missing witness.

## Exact matrix and prospective controls

[registered-matrix.json](registered-matrix.json) has100 worlds: each of the50
unchanged025 configurations executes once as Baseline and once as ProjectedNeed.
Rows2*i/2*i+1 form the primary policy pair for source025 ordinal i. Only the policy
differs initially. Every full source config is copied verbatim, including access,
knowledge plan, stock0/2/3/4/8/16, durations6/12/32, actors/profiles/hunger, seeds,
phase/order, horizon/cutoff/drain, capacity and reporting. This reuses a complete
historical design rather than selecting favorable fixtures after treatment.

Source ordinals0–31 are controlled coverage,32–47 held-out parameter sensitivity,
48/49 report-placebo repetitions. Thus64 controlled,32 held-out and4 report rows.
Held-out is not external social-environment transfer. Baseline source compatibility
compares all50 original025 histories, in addition to existing feature-off024 checks.

Closed access, unknown/empty source and no-eligible-Prepare sites are null controls.
H100 and strong contact can suppress differences despite an eligible projection.
Reporting pairs preserve actual body histories after only observer/config metadata
projection. The old access contrasts within each policy are secondary descriptive
checks, not additional independent policy pairs. No new hunger/profile intervention,
extra source, opportunity or post-outcome case selection is permitted.

## Outcomes and causality

Primary: counts of eligible sites, changed Prepare scores, same-input changed
selection, actual paired-world first selection divergence, admissions, completed
attempts, successes and full-cost failures. Distinguish counterfactual baseline
scoring at treatment-local inputs from genuinely executed baseline trajectories.

Secondary by actor and world: hunger area and terminal H, meals, food holdings,
raw/escrow/waste/consumed, transfers, contacts/consent, directed trust/concerns,
occupied time by work, source exhaustion and complete-history divergence.
Separate edible reconvergence from full common-mass reconvergence and persistent
social/need differences. No scalar global welfare ranking is preregistered.

For every claimed first cause identify matching pre-divergence local input,
all candidate scores/tie order, selected action, actual admission and subsequent
material/resource/cognitive records. Trace selected cases to later actual choices.
More score is not a changed decision; a decision is not admitted/completed work;
output is not later nutrition; a transfer is not global benefit. Whole-policy-pair
effects do not identify a single meal's or transfer's isolated counterfactual effect.

Preserve adverse, favorable, mixed, null, reconvergent and unused-output cases.
Choose strongest observed cases only for explanation after full-matrix analysis,
never to replace the registered denominator. Report absent evidence explicitly.

## Implementation and frozen custody

Create a versioned opt-in Rust workspace using the unchanged corrected025 generator.
Keep original physics/coordinator methods available as the baseline authority.
Generate counted/reversible copies of the ordinary boundary/trial loop carrying
only a selector mode and replacing its single rank call. No historical edits.
The pure function initially calls original025 ranking for exact eligibility and
old terms, then changes only eligible Prepare's need term and reselects strict-first.

Wrap histories in an explicit026 schema; do not label treatment as original025.
Observer score audit references boundary/choice, owner/tick, H,D,q,projection,
baseline/final Prepare and unchanged alternatives. It introduces no ECS resource
or extra retained cognitive state and avoids copying a second local input.

Publish design/source/matrix first. Then implement and run pure scalar/tie/invalid
input controls and disabled baseline compatibility only. Freeze/publish actual026
runtime and reader sources after all50 originals match. Only then execute the
registered ProjectedNeed worlds; never retune formula/matrix after outcomes.

## Independent validation and falsification

Separate readers reconstruct projection/eligibility/ties from actual local fields,
source receipts, availability, admissions, need progression, physical phases/order,
escrow, waste, full failure cost, meals/transfers/consent, cognitive consequences
and registered identity. Reuse existing independent legacy readers through explicit
versioned scoring seams without changing their original files. All100 rows replay
exactly and reports reproduce without a viewer cache or archive writer.

Scratch controls must reject leaked/future stock input, changed H, extra bonus,
duration, tie, eligibility, duplicate output, missing waste, forged meal, incorrect
social admission, fabricated later use, wrong source and changed registered config.
Failure to preserve baseline history or local information stops treatment. Failure
to reconstruct score/physics/replay prevents a scientific acceptance claim.

Reject claims that depend on private epoch, observer truths or scripted work,
unregistered policy interaction, altered source/capacity, reset trajectories,
changed future schedules or post-outcome thresholds. A null or harmful effect is
valid. Do not claim calibrated rationality, human behavior, general planning,
optimality, sustainable survival or a productive economy.

## Cost and persistence boundary

New live computation is O(1) per ordinary decision: integer arithmetic and the
existing five-way ranking. No added actor state. Audit/history grows with decisions;
measure actual evaluator/replay cost and output size separately from deterministic
evidence. Do not refactor general audit infrastructure or optimize unrelated code.

The verified persistent025 host stays unchanged. A stateless selector might later
reuse physical/cognitive payloads, but026 mode/source identity and score audit must
be preserved and exact continuation/recovery separately proven. No persistent026
schema, workbench integration, desktop acceptance or portable binary is part of026.
