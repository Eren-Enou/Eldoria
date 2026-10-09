# Experiment 012: contextual learning self-evaluation

Pre-implementation specification, 2026-10-09, baseline `5b9a5c6`. Experiments
001–011, their contracts, reviews and archived evidence remain protected. No 013.
This specification precedes treatment execution; later implementation repairs or
fixture adjustments must be disclosed rather than silently rewriting predictions.

## Question and existing capability

Can an individual use its own inquiry experience to develop context-sensitive
expectations about sources/methods, separately from concern progress and existing
novelty learning, and change later voluntary choices without privileged information?

006 already learns `(old + realized novelty)/2` in 32 concern/source/method cells.
Silence legitimately lowers that expectation without proving dishonesty. Public
untried offers can revive opportunity value. 007 discounts known dependence; 008
retains 32 local items and assesses Grouped support. 009 defines unchanged retention
research modes. 010/011 project current support but do not learn a separate estimate
of assessment change. Credibility, importance, trust and willingness are separate.
011 demonstrates positive novelty with no retained support change. Its short runs
do not establish the later value of negative source learning. Existing cells can
adapt within a concern, but do not transfer that experience across concerns sharing
an observable assessment context. This is the specific limitation tested here.

Learning about a concern: receipt/content, changed signed support, changed status.
Learning about an opportunity: selected response and effort, empirical frequency
of retained change in a comparable local context. Expectation error: observed change
versus the selected prior change expectation. Neither is truth or motive inference.

## Alternatives and hypotheses

A: existing learning suffices. No new state; repeated opportunities can already
change behavior. This is the exact Unchanged control and remains a live explanation.
B: merely expose existing prediction error. No new state, but algebraically repeats
the old moving-average update and cannot distinguish novelty from assessment change.
C: source/method/context estimates of observed retained change. Adds bounded state,
transfers across concerns, can overgeneralize from a misleading early encounter.
D: hybrid existing novelty with separate observed progress estimate. Chosen: preserve
006 learning unchanged, conservatively discount its gain after contextual experience.
No calibrated information value, source competence label, motive label or planner.

H1: extra estimates change locally comparable later choices beyond old learning.
H2: context separation prevents some harmful cross-context transfer seen in a global
estimate. H3: new learning is inert or harmful, especially under changed circumstances.
H4: effects stop at inquiry or cognition without executed material consequence.
H5: apparent effects depend on fixtures or hidden inputs, falsifying interpretation.
No hypothesis of universal superiority or adoption is assumed.

## Fixed mechanism

Three explicit 012 modes: Unchanged (capture/update ablation with exact old selection),
GlobalProgress (source × method estimate, ignores context), Contextual (same estimator
with context). All start empty and prospectively after Grouped/FIFO. They cannot be
combined with 010/011 projections. Those historical functions remain unchanged.

Context is one actor-visible Boolean: does the currently retained basis contain
positive-quality non-Claim evidence for this concern's event? It represents the
established claim-fallback boundary, not subject expertise or a hidden source trait.
GlobalProgress uses Any; Contextual uses EvidencePresent or ClaimFallback. Nothing
stores event propositions, quality, support, concern IDs or partner knowledge in cells.

Per actor, at most 32 cells `(source, method, context, attempts, expected_change,
last_inquiry)`. Existing cells are updated in place; insertion at capacity evicts
oldest inserted cell (FIFO), audited with prior last-inquiry reference. No lookup of
the observer ledger repairs an evicted cell. Initial expected_change is the existing
method prior, 55 Direct / 65 Evidence. Each selected valid reply trains only its cell:
`observed_change = 100` if retained signed support OR selected concern status changes,
otherwise 0; `new = (old + observed_change)/2`, truncating integers. Conflict changes
count as change without claiming resolution or correctness. Invalid replies and Pause
do not train; silence is a valid observed no-change outcome, not a dishonest-source label.

When no comparable cell exists, candidate scores are byte-identical to baseline.
When it exists, `effective_gain = min(existing expected_gain,
max(cell.expected_change, existing opportunity_value))`; benefit is importance ×
effective_gain /100, minus unchanged cost. This discount cannot bypass known-origin
independence or force exploration. Offers retain their existing attempted/seen rules.
Strict greater-than selection preserves stable order and Pause wins nonpositive ties.
No retuning after controlled or held-out results. Binary change is a disclosed coarse
heuristic: one point and resolution have the same target; movement away from a prior
belief can be useful; lack of change may still teach opportunity expectations.

Current observations distinguish no receipt, redundant receipt, novel unchanged
assessment, partial change, resolution and conflict. Public response actions are
recorded when exposed by the established channel. Silence cannot distinguish honest
ignorance from withholding. No new linguistic ignorance/refusal channel is invented.

## Local boundary and lifetimes

Inputs: existing own bounded inquiry/provenance input; own live retained assessment
context; own 32 cells; selected actual local reply; before/after own support/status.
Forbidden: observer history, objective truth, hidden provenance, motives, scenario,
seed, future usefulness/partners, actual responses before choosing, private partner
components. Post-update uses the current resolved interaction, not an archive search.
Own evidence eviction recomputes context; learned cells can survive separately, but
contain no forgotten evidence and cannot reconstruct it. Cell eviction loses its
estimate and returns to the old baseline. Existing cognitive capacities are unchanged.
New summaries are a separately bounded learning hypothesis, not unlimited biography.

## Controlled families and held-out plan

Actual prior refusals establish three competing concerns. Preparation and all public
opportunity schedules match across modes until voluntary divergence; no target is
scripted. Multiple two-person meetings permit one inquiry per participant per slot.
After preparation, fewer scored slots than concerns make missed opportunities real.
Preparation learning costs are included separately and never hidden as free training.
Resource scenes use the existing scarcity policy and actual Offer/Accept/Leave;
subsequent consumption establishes persistence. No global utility ranks transfers.

Families: novelty without assessment; source useful for one concern but silent on
another; repeated retained evidence; partial weak acquisition; conflict; cheap method
versus costly evidence; changing source (useful then unavailable); later-informed
source; observationally identical ignorance/withholding; misleading training; changed
circumstances; forgetting via more than 32 legitimate receipts; limited concerns;
no-information equivalence; mirrored future (same inquiries, different resource partner).
Each family has a fixed external schedule independent of mode. Honest inability is
observational silence under existing semantics, not automatically a dishonesty update.

Held-out combinations fixed before implementation: lower/higher initial support,
received quality 20/50/80, importance shifts ±10, extra public effort 0/20, different
source ordering/subsets, two/four training encounters, two/three scored slots,
resource partner 1/2 and stakes 1/2. These are parameter/schedule sensitivity, not
external transfer or independent statistical samples. Policy is frozen throughout.
Replay seeds 0..7, 42 and maximum u64; accurate/inverted deterministic fixtures
test fallibility, not naturally occurring source reliability.

## Outcomes, verification and falsification

A observer-only explanation; B actor-local expectation revision; C later changed
choice caused by that revision; D actual executed/persistent consequence. Report
levels separately, including same choice/different scores, novel-no-progress,
expectation-only learning, negative adaptation and ambiguous mediation. No C from
B alone. GlobalProgress ablates context; Unchanged ablates behavioral use of new
learning while keeping capture/updates. Independent reader must reconstruct scoring,
cells, own retained context/support and selected outcomes without calling Rust scorer.

Falsify new capability if all choices equal old learning, it merely renames cells,
unmatched fixtures cause divergence, histories recover content, or hidden/future data
enter policy. Preserve failed expectations and outdated-source lockout. No replacement
formula after outcomes. No inference of motives, optimality, consciousness, human
introspection, intelligence, broad emergence or natural-world prevalence.

Live extra cognition caps at 32 small cells per actor; evaluation is bounded candidates
times 32. Observer audit grows with queries and links existing inquiry IDs rather than
duplicating complete worlds. Measure state serialization, audit bytes, policy cost,
whole-simulation timing separately; divergent timings do not isolate algorithm cost.
No general optimization. Run full required foundation checks, exact 012 replay and
independent verification, preserving every original 001–011 byte. 012 remains exploratory
pending separate scientific review/integrity incorporation.

## Implementation notes before held-out execution

The conflict fixture initially attempted to install a positive capability over an
already received contrary channel. The established immutability check rejected it
in the targeted suite. The fixture now preserves both conflicting readings and
offers new native capabilities only for the other two events. No cognitive rule
was changed. Changing-source competence uses a relay provider's real FIFO acquisition
eviction; changed willingness is a separate family. Later-informed uses the same
initially unknowing provider's subsequent voluntary inspection. These distinctions
avoid attributing silence to an unobservable motive. Compile-time wiring/test-field
repairs preceded the first evaluator run and did not change the specified mechanism.

Controlled preview exposed an inadequate ambiguity fixture: the unwilling source
also lacked the answer, so willingness differed but withholding was not isolated.
Before held-out execution, the withholding provider now legitimately inspects A
before becoming unwilling; the actor receives neither its acquisition nor an offer.
The paired ignorance provider has no acquisition. Actor decision inputs and observed
silence must still match. This repairs the intended hidden-cause comparison, not the
learning rule. The initial controlled preview is retained in the task output log.
No scoring, update, prior, cost or capacity adjustment follows preview outcomes.

Independent reconstruction after the first full replay found a second fixture defect:
the default native sensor was quality40 on channel0, while setup had already delivered
channel0 at initial quality20 (or the declared variation). Evidence fallback therefore
attempted to rewrite an immutable received channel; some invalid replies could not
train either old or new estimates. The final fixture default sensor now matches the
already declared initial channel quality, including partial10. This is a consistency
repair, not a policy change or improved future answer. The complete pre-repair summary
is retained as development evidence. Both controlled and held-out suites are rerun;
the first held-out run is no longer an untouched holdout, and the final sensitivity
findings are explicitly qualified. No learner or scoring rule is retuned.

Final boundary review replaced the update hook's full legacy audit-record argument
with a small typed current `Experience` containing only actor ID, selected action,
valid-result flag, own status, receipt presence, novelty/value and elapsed effort.
The update never receives responder-private fields. This changes no update, selection
or output semantics; exact archive equality and required checks are rerun.
