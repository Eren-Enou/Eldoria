# Experiment 008 pre-registration

2026-10-07, before behavioral implementation. Baseline a7a4e4a; review includes
001–007 specifications/results/tests, constitution in AGENTS, architecture,
history-audit guide and research/next-target/report.md. No older evidence changes.

Question: can a bounded local assessment of native readings and third-party reports
make beliefs and their consequences depend on retained evidence rather than the
last delivery channel, without losing uncertainty, correction or forgetting?

H1: the +90/−40 alternating diagnostic is caused by separate evidence aggregation,
not by a necessary attention/recency mechanism. One shared local basis should yield
the same final support for permutations without intervening learning or eviction;
duplicates below capacity should not change support.
H2: grouping legitimately known origins permits useful corroboration while keeping
known relays redundant. A simpler max-positive minus max-negative aggregate may
solve H1 equally well but fail to distinguish independent corroboration from relay.
Neither hypothesis asserts calibrated probabilities or psychological realism.

Controls: Legacy 007 dispatch, SharedMax, SharedGrouped. All use identical voluntary
inspection/sharing, native reading interventions, costs, source content/quality,
physical state and bounded original cognition. New assessment caps at 32 delivered
items per individual, matching the existing provenance capacity; no limits increase.
It is prospective: enabling never imports prior receipts from observer history.
Existing provenance acquisitions remain necessary for providers to share; the new
basis records what the listener assesses across channels, not new observation powers.

Native local origins explicitly distinguish refuser testimony, historical disclosure,
and event-local sensor channel. Received provenance uses only disclosed token or the
existing unknown-speaker assumption. No objective root, sensor channel/outcome before
receipt, or hidden partner component enters assessment. Native readings and inspected
roots are treated as distinct acquisitions in these controls; cross-channel shared
origin is not inferred without an explicit locally available link. No invented roots.
Received content and quality are fixed at receipt; later credibility learning does
not retroactively rescore them. Testimony supplies a fallback only while no retained
evidence exists for the event. Within a known origin use maxima on each sign.
SharedGrouped uses 007 residual combination in deterministic origin order; SharedMax
uses strongest positive minus strongest negative across evidence. Later attribution
updates only retained items bearing the same provider/message handle.

Scenarios (seeds 0..127, complete deterministic replay; additionally extreme seed
coverage in tests): native+90/provenance−40 in both orders and repeats; matched +50
corroboration; equal opposite 50 conflict; known versus hidden relay; later public
attribution correction; independent misleading observations; native weak testimony
versus stronger evidence; repeated receipts at/beyond capacity; evidence after resource
memory eviction; voluntary inquiry after mixed conflict. Check original events,
food/time, native and provenance receipt links, memory/trust, concern transitions,
and matched-present local action (food4/hunger30/generosity60/caution80). That local
action is a policy probe, not a transferred food outcome. Initial refusal remains
policy-selected; fixture evidence delivery isolates assimilation, not inquiry prevalence.

Distinguishing observations: Legacy order/repeat dependence persists; both shared
rules remove it below capacity. Grouped independent 50+50 yields75 versus Max50;
known duplicate50 remains50; equal conflict remains0. Misleading independent readings
can still yield wrong confidence. Correction may reopen a concern. FIFO eviction may
legitimately restore order dependence; no claim of invariance across different retained
inputs. Novelty/inquiry selection and credibility rules remain 007 to isolate belief
assessment; their sufficiency is a limitation to investigate, not silently retuned.

Falsification: identical retained evidence with fixed received quality still gives
different final support/concern/action solely due to channel/order; below-capacity
repetition changes support; conflict becomes certainty without evidence; known relay
counts as independent; correction needs hidden truth; or prior histories are recovered
when enabling/evicting. If Max and Grouped are indistinguishable on these controls,
prefer the simpler mechanism and report H2 unsupported. Do not tune score thresholds
to force choices. Failed expectations require recorded analysis and revised controls.

Risks: duplicate local storage, mismatched receipt namespaces, saturated trust replay,
local attribution versus actual lineage, temporary prefix closure under sequential
receipts, inconsistent novelty learning, capacity floods and old-mode compatibility.
Audit new assessment updates by receipt references, not copied native decision inputs.
Use a separate 008 snapshot containing the compact base plus the assessment archive;
do not silently append 008 state to legacy snapshots. Measure 100/1,000 populations
and controlled scenarios; no scaling optimization, general evidence framework,
uncertain attribution, planner, reputation, geography or institutions. No 009 work.

## Completion review, 2026-10-08 (before further behavior changes)

The repository contains the preliminary implementation but no completed evaluation
or tests. The preview currently fails: memory-eviction churn can trigger genuine
follow-up evidence at the original native slot, after which the fixture tries to
rewrite that slot. Isolate the eviction control using a documented privacy
intervention that permits voluntary silence, rather than weakening immutability.

Also test one already acquired, explicitly attributed report delivered through both
native receipt and provenance adapters. Both must preserve the same public origin,
quality and source decision; selecting a delivery adapter must not manufacture a
new origin. This narrow 008-only adapter copies retained source knowledge through
the existing validated voluntary exchange, preserves objective parent references,
and changes only the assessment receipt namespace. It cannot map anonymous native
readings to hidden observation tokens or introduce uncertain attribution.

Keep the existing cap32 and prospective enable boundary. Compare Max and Grouped
before selecting the default: reject extra aggregation if no distinguishing case
supports it. Record any mismatch between unified support and older novelty values;
do not silently retune the inquiry/credibility mechanism. Primary controls use fixed
credibility, no action between receipts, no eviction and matched origin/content/
quality. A true later correction uses an additional legitimate independent reading.

## Completed evaluation, 2026-10-08

Evaluated 56 variants across 128 seeds with exact replay (7,168 trials), all seven
archived compatibility outputs, population replays and required engineering checks.
Grouped is retained because it distinguishes independent corroboration from known
relay while Max does not. Both solve the tested mixed-channel final reversals.
Full controls, surprises, performance and limits are recorded in results.md.
