# Activity valuation architecture — prospective 024

Question: can an isolated actor-local change in relative activity value change
consumption, contact and acquisition consequences under the unchanged continuous
scheduler? This is a design, not a claim of improved or optimal behavior. Read the
[observational diagnosis](continuous-activity-valuation-diagnosis.md) first.
No treatment code/outcome was run while selecting this design.

## Alternatives and selected causal target

| Candidate | Scientific purpose | Decision for024 |
|---|---|---|
| Own hunger/meal effect | Isolate comparison of eligible immediate consumption with other existing activity scores. No new memory or hidden outcome estimate. | Primary: add current modeled meal relief to the inherited consumption score. |
| Declared public duration | Test time/opportunity cost using known reservation, not future actual duration. | Defer. Independently scaled scores divided by duration would confound calibration and penalize uncertain longer contact. Different question. |
| Retained contact feedback | Use legitimately retained personal experience rather than observer classifications. | Defer. Current episodes/trust do not encode a calibrated zero-transfer utility; adding outcome-summary state or interpreting every Leave as failure would change cognition/meaning. Existing016 negative findings caution against assuming feedback helps. |
| Combined urgency/time/history | Assess interaction after individual mechanisms are identified. | Reject for this first treatment: too many simultaneous causes. |

Primary **MealRelief** changes only eligible Consume from H to H+min(H,40), where
40 is the existing modeled meal effect and H is current own hunger. Other scores,
eligibility, inner contact choice and ordering remain exact. Baseline applies zero
bonus. This is an explicit heuristic addition to existing score units, not expected
future utility or a physiological model. It can still prefer Invite above the new
score. The constant is taken from existing law, not fitted to024 outcomes. There is
no threshold sweep, learning of weights or post-outcome tuning.

No new persistent cognition. A pure/replaceable selector receives existing bounded
owned Input and the four legally projected candidates, AFTER inherited protocol
masks. External experiment configuration dispatches the scoring function; scenario,
mode/outcome labels, global history and private state are not selector inputs.
Stable first-maximum order Wait,Consume,Invite,Acquire remains. No recipient policy
or resource-negotiation value changes. Actual outcomes are resolver-owned.

The intervention applies to ordinary activity choices only. Incoming replies and
admissions occur earlier in a boundary and may reserve an actor before any ordinary
choice. Unchanged recipient acceptance can therefore still displace consumption.
Distinguish unavailable decision time from an eligible meal losing on score; the
new rule is not an intervention on every decision that can affect eating.

## Ownership boundary

Keep all source-frozen001–023 files untouched. Use a separate opt-in versioned
prototype host, compiling the original source and023 module unchanged. The only
024 coordination seam is final ordinary activity scoring after original masks;
reserve/admit/reply/tick/need/sense/resource/claim/meal/drain logic stays identical.
Guard mechanically copied code and demonstrate Baseline parity with original023
histories before final treatment. No root default entry point or library mutation
is required. No general planner, event-engine refactor or new resource system.

Original owned input is captured once in its boundary. An observer-only valuation
record references boundary/choice and records four base/bonus/final terms, eligibility,
policy identity and selection. Do not duplicate full history or add this audit to
actor state. New values must be derivable from exact local inputs; no archive query
is permitted. Later observer outcomes stay separate from scores.

## Scope and testability

Only the relative score changes initially. Each pair shares original bootstrap,
food/cache/traits/seed, knowledge, concerns, inquiry learning, social opportunities,
consent, clock/phase/cadence, order, durations and continuous bound. After voluntary
divergence preserve actual causal consequences rather than resetting worlds.

Acquire and Consume are mutually exclusive at one instant under inherited rules
(F=0 versus F>0). All three activities compete over a continuing actor's changing
state, and contact can compete with either; do not fabricate a literal simultaneous
three-way eligibility by changing old acquisition law. Explicitly verify sequential
acquisition/contact/consumption witnesses and report any missing coverage.

Potential harm is essential: consume food before a valuable meeting, give less,
ignore information, waste newly available acquisition time, retain unconsumed food,
create unequal need, or only change social history. More meals is not global benefit.
A neutral result or complexity rejection is scientifically valid.

The minimal extra runtime cost is a four-candidate pass and small score audit;
measure it separately from growing original history, serialization, replay and
viewer reconstruction. Source-host compilation/coupling remain limits. No production,
new supply, cognitive increase, hunger change, consent change, scheduler change,
025 or default adoption belongs here.
