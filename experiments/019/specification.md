# Experiment 019: Robust Activity Priorities and Opportunity Timing

Preregistered before treatment implementation and comparative evaluation. Gate A:
018 reviewed at e870cd6ece1d143bc251a4d98b5b5882e063c3e0, ACCEPT WITH QUALIFICATION,
review committed/pushed bec1bc29ef7b95178c32f49434c25e35448af7cc. Gate B: protected
001–018 validated, committed/pushed c51bb771fa37bbbaae29240a5adb5b7dcc9b5e8c.
Main/origin matched and worktree was clean at Stage C start. All 269 original artifacts
remain unchanged. No default adoption, automatic 019 protection or Experiment 020.

## Question, hypotheses and protected authority

Are consequential choices among eating, resource contact, inquiry and waiting stable
under reasonable priority and opportunity-timing variations with common need physics,
or dependent on mixed heuristic scales and favorable scheduling? Do not select a winner.

Test the competing possibilities: the old maximum is adequate within this test;
small cross-activity scale changes cause large executed differences; simple priorities
compare well; local admission geometry changes behavior; conservative reservations
exclude some actually feasible activities; observation cadence matters independently
of valuation; effects reverse with order, rate or stock; history helps and harms;
early eating and social pursuit each cause costs; no policy justifies adoption.
Any can be supported. Repeated actions, food redistribution and favorable scores are
not competence, production or welfare. A negative mechanism result can be valid science.

Authority: unchanged018 law, sources and qualified review, protected001–018 and the
constitution. Preserve all 018 controls/findings, invalid development evidence and
saved-listener regression. Observer history cannot recover unavailable cognition.
Episode eviction does not erase independently persistent trust, concerns or effects.

## Existing numeric inputs and their limits

The old contact resource value is max(nonnegative maximum of own legal scarcity-policy
candidate scores) + max(own hunger,0)/2 − 2. Basic scores use own hunger, generosity,
caution, trust and last-four partner episodic valence (clamped −40..40). Accept uses
H+20+trust/4−caution/5; Offer generosity−H/2−caution/2+trust+valence; Request
H−35+trust/4−caution/4; Refuse H/2+caution/2−generosity/2−trust/2; Leave0. Scarcity
subtracts60 from Offer when H>=60 and own food<=amount. Action legality and strict
enum ties precede execution. This category ranking is not calibrated value of food.

Contact inquiry value is the existing selected query score minus2. The old bounded
006/007/012 mechanism already combines concern importance, remembered source/method
expectations, retained Grouped state, received quality, public source eligibility,
effort and the common floor(H/10) penalty. It selects a concern/source/method locally;
019 changes none of its formulas. See frozen parent code/contracts for exact integer
arithmetic. The penalty is not additional elapsed time.

The old activity layer compares Wait0, eligible Consume at own H, and the already
selected old Invite at its contact score. Strict order is Wait,Consume,Invite. Own
food>0, H>0 and one tick are required for eating. Resource and query scores already
mix different heuristic units; those units are not shared with hunger. Existing
contact selection also mixes resource and inquiry. This study perturbs only the
top Consume/Invite comparison and keeps within-contact partner/type/method ordering.
It cannot establish robustness of every possible resource-versus-query valuation.

## Minimum six policies, fixed before outcomes

All rules are pure, replaceable and deterministic over the old actor-local Input.
No new persistent cognition, free threshold, future-value signal or general planner.

| Mode | Exact rule |
|---|---|
| Baseline | Exact historical018 Recurring maximum. |
| EatFirst | Exact historical018 EatFirst: choose eligible Consume first, otherwise old maximum. |
| Deadline | Start from Baseline candidates. If Consume is eligible, choose it except when Invite is eligible and one meal plus its public conservative reservation cannot fit before current end; then choose Invite. Without an eligible meal use Baseline. |
| Scale90 | Baseline, replacing only eligible top Invite score s by floor(9*s/10), then the same strict maximum. |
| Scale110 | Same, floor(11*s/10). |
| NoHistory | Exact historical018 NoHistory, masking only outgoing resource trust/episode ranking. Actual cognition/negotiation/recipient/inquiry history persists. |

Deadline/EatFirst is the ablation pair isolating the new time condition. An eligible
Invite already fits its bound, so Deadline's exception occurs at the last safe slot
where adding a meal would make it unaffordable. It knows the bound, not actual duration
or whether contact will help. It may waste that slot or miss the meal. The three
historical controls are not retuned. Two-sided 10% factors test sensitivity,
not a weight search. Integer truncation may be a large relative change for tiny scores
(1→0); distinguish quantization/ties from smooth scaling. No post-outcome tuning.

## Common physics, legal execution and locality

All primary modes use the exact018 clock: fixed epoch E, interval4, increment5,
cap100. A→B pulses=floor((B−E)/I)−floor((A−E)/I). Eating removes one owned food and
reduces current H by40 after elapsed progression. I2/16 are separate physical-sensitivity
configurations, never evidence isolating priorities across unequal physics.

Reuse the018 stepper, clock, live input construction, consent/arbitration, resource
resolution, atomic inquiry, Grouped/FIFO and Contextual learning. Install a selected
pure callback through the existing replacement API. Use three actors, one resource
commitment and no disjoint scheduler. All modes retain voluntary incoming response
and real resource negotiation; an accepted meeting can still produce Leave/Refuse.
No controller chooses a pairing after scores. Source response may be empty, wrong
or redundant. No future cooperation is guaranteed.

Inputs remain own Agent/profile, now/end, public(id,currently available), own bounded
query projection. Exclude truth, hidden roots, source-private inventory/profile,
future relevance/responses/release, scenario names, observer history and total future
trial schedule. Public bounds Resource8/Direct15/Evidence33 and actual resolver charges
are unchanged. No crossing a window; actual termination releases commitments. Preserve
existing caps, source knowledge, original interpretations and saturated trust replay.

## Timing treatments and equal horizons

Ordinary families run exactly144 elapsed ticks after their matched epoch. Ongoing
and longitudinal run576; missed retains its historical12 ticks with two-tick windows.
Default window24. Variants partition the same total with windows6/8/12/48; information
also14. Final windows are clamped to the remaining horizon. No clock, state, concern,
food, trust, knowledge or expectation reset. New windows may immediately permit
another opportunity; a missed slot is not necessarily permanent loss. Actual elapsed
time and executed depletion cannot be undone. Compare terminal outcomes only at
equal horizons; cross-window period grids need not match.

Default idle sampling gap1 is exactly018. In gap2/5 variants, after an actually
quiescent all-three-Wait step (no active resource, invitations, consumption or query),
advance the remaining gap−1 ticks, clamped to the window. No policy evaluates during
that interval. A small guarded time adapter reuses018 synchronization; it cannot run
through active work or past end. Audit the gap with point/need-record references,
not invented decisions. Gap2/5 bracket the default need interval and avoid testing
only harmonics. Pressure follows elapsed time, not evaluations. The common sampler
may trigger differently after voluntary divergence; it is not tailored to a mode.

No other observation delay is added: busy free-person sampling and inquiry atomicity
stay unchanged. Gap/window changes can alter old cohort-based concern age and Rotating
wave phase. Identify those mediators rather than attributing every effect to priority.

Distinguish physically impossible, conservative-bound excluded, and legally offered
but unchosen. A rejection alone does not prove actual feasibility. Lower elapsed-time
bounds are3 for any resource outcome and method cost+4 for inquiry including handshake;
outcome-specific duration is unknown before selection. Post hoc matched executions
that finish inside a shorter excluded window may witness conservative exclusion;
they are observer diagnostics, not input to a policy or guaranteed future cooperation.

## Families and initial physical controls

Reuse all16 historical018 families: recurring,no_food,mutual,contended,competing_eater,
history,information,misleading,private_mirror,formerly_useful,refusal,unilateral,separate,
missed,ongoing,longitudinal. They cover actual busy partners, donor depletion, private
willingness, useful/wrong/redundant information, history effects, failures, nulls,
finite exhaustion and persistent longitudinal history. Do not require a specific
new policy to win or every proposed contrast to distinguish.

Add three controlled starts before clock enablement:

- low_food: historical recurring setup, all actors food1,H45,generosity0,caution100.
- useful_contact: historical mutual setup, actor0(food2,H80,G60,C20), actor1(6,20,100,0),
  actor2(0,0,0,100). High need can compete with a potentially helpful partner.
- unproductive_contact: same circumstances but historical refusal setup and actor1
  G0,C100. Do not script its response; existing resource policy remains authoritative.

Other traits, information and setup history come from the named old fixture. Ongoing
uses stock12,H20,G60,C20 by default; its declared stock/H sensitivities alter only
pre-clock initialization. Modes within every configuration receive identical starts,
profiles, source opportunities, physics and sampler parameters. Preserve divergence
after choice. Private-mirror current-focal equality is a locality control, not a fully
matched whole-world comparison between different private-source fixtures.

## Registered matrix and prospective sensitivity

Mode order Baseline,EatFirst,Deadline,Scale90,Scale110,NoHistory. Four deterministic
seeds0,1,42,u64::MAX. No training/outcome-driven optimization. Designed sensitivity
is not external transfer or independent statistical replication.

- Core:19 families ×6 modes =114 worlds/seed.
- Descending and Rotating: competing_eater,information,refusal,ongoing (8 groups).
- Windows6/8/12/48: useful_contact,information,ongoing (12 groups).
- Gaps2/5: recurring,information,history (6 groups).
- I2/16: information,ongoing (4 groups; different physics clearly labeled).
- Stock4/20: ongoing (2 groups).
- Initial H0/60: ongoing (2 groups).
- Window14: information (1 group).

Total54 groups,324 worlds/seed,1296 registered worlds. Every world is exactly replayed.
Archive all324 full seed42 histories, plus12 preregistered seed1 default history/
useful_contact histories; other seeds have exact summaries. The latter12 are already
part of the1296 worlds, not extra trials. Independently reconstruct all336 raw full
histories. Primary comparisons:54 groups ×15 overlapping unordered mode pairs=810;
all-seed summary comparisons3240. Do not pool these as statistical samples.

## Outcomes, falsification and causal reconstruction

Record invitations, declines/contention, executed meetings, requests/offers/refusals,
transfers, consumption, remaining food, actual pressure added/reduced, final H per
actor, inquiry attempts/receipts/support/status, trust, contacts per actor, repeated
failures, current availability, bounds/excluded candidates, selected alternatives,
activity time, gap time, intermediate/terminal material and reconvergence. Export
per-actor hunger accumulation as sum H(t) for integer ticks E<=t<end, after that tick's
completed actions; it is an operational pressure×time measure, not global welfare.
Compute it from immutable need segments with saturation and intervening meals.

Do not reward every action, response or transfer. Keep information novelty, retained
assessment change, truth (observer only) and material effect separate. A timing-only
or temporary distinction is not persistent material benefit. Preserve individual
tradeoffs; do not combine food, trust and information into an invented utility score.

For major claims give archive record, period/step/point, current input/candidates,
priority or score, bounds, consent, actual time/event/query, resulting state and later
history. Identify the earliest voluntary divergence under matched conditions. Keep
scoring-only differences distinct from executed ones. Diagnose positive, harmful,
null and reconvergent cases without replacing unfavorable raw outcomes.

Independent Python reconstruction reuses reviewed006–018 primitive cognition/resource
arithmetic, adding separately coded019 priorities and sampling checks. It must derive
physics, input locality, scores/ties, reservations, consent, execution, retained state,
learning, hunger accumulation and comparisons from records, not evaluator labels.
Reject scratch corruptions of time,H,food,input,selection,reservation,consent,transfer,
hidden-private input and retained cognition. Test old-control exact compatibility.

Falsify effects attributed to priorities when caused by different pre-treatment
conditions, unequal physics/horizons, unavailable candidates, hidden future/source
information, scripted target/outcome, observer recovery, unrelated fixture mismatch
or merely different scores with identical execution. Do not claim the time heuristic
helps if only its EatFirst component explains the change. No test establishes a
universally best policy; a null new mechanism remains a valid finding.

## Cost, completion and non-goals

Measure pure policy, local input construction, public admission arithmetic, actual
clock synchronization, resource completion, query activity, full trials, observer
bytes, serialization and independent reconstruction separately. Record work counts;
different activities/worlds are not interchangeable timing units. Clock and cognitive
capacities stay unchanged. Gap audits add small references; old large observer data
still grow. No audit refactor or general optimization, and no population-scaling claim.

Required completion:fmt,locked all-target Clippy,full tests,foundation001–018,
standalone009…018,new evaluator/replay/independent/corruption checks,raw269 guards,
relevant measurements,results and scoped acquisition/viewer proposals. Commit/push
019 separately only after validation. Classify demonstrated, mechanism-dependent,
ambiguous and unsupported findings. Verdict ACCEPT/ACCEPT WITH QUALIFICATION/
NO JUSTIFICATION/INCONCLUSIVE follows actual evidence.

No planner,new utility engine,cognitive stores,capacity increase,retention/inquiry
retuning,production,renewable supply,currency,jobs,travel,language,institutions,
reputation networks,viewer implementation or automatic adoption/protection. Optimal
planning,psychological realism,fairness,sustainability,intelligence and natural social
emergence remain unsupported. Do not start020. Later correctness repairs must be
disclosed; freeze policies, fixtures, timing and criteria before final outcomes.

## Pre-evaluation implementation record

Initial compile and all seven targeted019 tests pass. They cover public-bound priority
and its ablation, score perturbation, 48 exact old-control/case comparisons, no-food
pressure-time invariance, real sampler progression, active/deadline guards and local
private-state isolation. All-target locked Clippy passes after collapsing a nested
writer condition; explicit focus-file flushing was added. These are writer/style
repairs, not policy or fixture changes. The specification's control-count wording was
clarified without changing its six rules. No final comparative outcomes were inspected.
Freeze the three new treatment/fixture sources, specification, evaluator and exact
shared module-declaration patch now. Existing018 stepper/clock code remains unchanged.
