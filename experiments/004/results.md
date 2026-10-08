# Experiment 004 findings: anticipating communication consequences

Executed 2026-10-07 on Windows x64, Rust 1.99.0, Bevy ECS 0.19.1.
Rules `experiment-004-v1`. Reproduce with:

```powershell
cargo run --locked --release --example evaluate004
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo bench --bench throughput
```

Evidence: [pre-implementation hypotheses](specification.md), [readable causal
report](report.txt), [full seed-42 suite](seed-42.json), [128-seed summary](summary.json),
[1,000-agent trace](population.json), [benchmark](benchmark.csv).
All Experiment 001–003 archived evidence remains intact.

## What changed

A new opt-in Foresight ECS resource augments the existing post-refusal conversation.
It holds directed expectations, explicit priors, forecast/error audit records and
sensor configuration. `enable_foresight(seed, sensor)` enables the existing
intentional mode and the new predictor. `set_predictor` replaces the pure predictor;
in foresight mode it supplies the communication decision rather than TalkPolicy.
There is no general planner or search tree.

The predictor sees only the actor's existing TalkInput plus their own expectation,
settings and publicly known evidence quality/effort. It cannot read partner state,
sensor seed, channel configuration or the next sampled observation. Agent-private
truth is available only for the actor's own original refusal. The listener still
sees ordinary testimony, not the intent to deceive. Truth/correctness annotations
are retained solely in objective audit records.

Each candidate records immediate utility, a predicted response where applicable,
its percentage estimate, anticipated consequence and combined score. The six
communication actions and four-turn legality/termination protocol remain unchanged.
Silence stays a zero-score alternative; predictions do not force communication.
Earlier modes retain their exact old behavior and serialization.

## Prediction and learning rules

The immediate utility is Experiment 003's score, minus configured effort for
ProvideEvidence. Additional consequences are deliberately small hypotheses:

| Candidate | Anticipated adjustment |
| --- | --- |
| Mislead | -40 * expected challenge / 100 |
| Explain | -8 * expected challenge / 100 |
| AskEvidence | expected answer * evidence quality / 100 / 4 |
| Evidence contradicting one's own previous lie | -40 * evidence quality / 100 |
| Other candidates | 0 |

A claim predicts the observable event AskEvidence; a request predicts
ProvideEvidence. This is a binary event probability, not a declaration that this
response is always most likely. Default priors are 50/100. Disabling anticipation
removes only the downstream adjustments; evidence and the rest of the policy
remain the same. Learning can be disabled independently.

When the next actual public response arrives, error = outcome(0 or 100) - expected.
The actor's appropriate directed expectation changes by error/4, truncating toward
zero. Feedback is recorded only for the selected action, never from unchosen
alternatives or another individual's private belief. Claims too late in the
conversation for an evidence challenge have no challenge prediction or learning
sample: forced closure is censored rather than learned as unwillingness to ask.
The evidence-contradiction adjustment is a modeled consequence, not a private
belief response used as a training label.

## Controlled experiments

Eighteen variants were evaluated at seeds 0..127. Every complete suite replayed
exactly. Profiles and physical interventions are saved in snapshots. The requester
starts hungry with no food; the refuser is hungry and guarded, with one unit for
scarcity scenarios or four for deception scenarios. Actions are always selected
by policy. The later physical probe uses requester food=4, hunger=30,
generosity=60, caution=80, with partner food=0, hunger=90, generosity=30, caution=20.
These are controlled threshold tests, not prevalence estimates.

| Scenario | Finding across 128 seeds |
| --- | --- |
| A: immediate versus anticipated utility | Opening Mislead becomes Silence in 128/128 |
| B: challenge prior 90 versus 0 | Opening Silence versus Mislead in 128/128 |
| B: an unexpected challenge to a lie | Evidence is withheld, avoiding the old self-incrimination response |
| C: privacy 0 versus 100 | Explanation/evidence versus concealment; later Offer versus Leave |
| D: correct weak versus strong readings | Support 40 versus 80; later Leave versus Offer |
| E: equally strong opposing readings | Support 0; unresolved disagreement and later Leave |
| F: cheap versus expensive evidence request | Request/disclosure versus no verification; Offer versus Leave |
| F: disclosure effort 80 | Speaker refuses the evidence request; support stays 50 and later Leave |
| G: six non-challenges | Challenge expectation 50 -> 10; later opening Mislead |
| H: six challenges | Challenge expectation 50 -> 90; later opening Silence |
| G: same non-challenge training, learning disabled | Expectation stays 50; later opening Silence |

Seed-42 A: immediate Mislead is +5. With a 50% expected challenge, its anticipated
cost is -20, yielding -15 against silence 0. Only the speaker's anticipation switch
differs, and original resource events through refusal compare equal. Both branches
later Leave: changing a communication choice does not guarantee changing food
sharing. With prior 90 the lie scores -31; with prior zero it remains +5.

In the unexpected-challenge case, the speaker initially expects zero scrutiny and
lies. The listener asks for evidence; the speaker's challenge estimate updates
0 -> 25. Providing evidence would have immediate value +2, but the actor remembers
its own false claim and expects contradictory evidence, adding -32 for combined
-30. It chooses silence. The listener's expected evidence response also fails,
updating 50 -> 38. The unverified claim does not become known truth or a proven lie.

Costly truth remains costly: high privacy can make disclosure lose despite its
potential social benefit. With effort 80, requested disclosure scores 62-80=-18;
no reading is acquired and uncertainty remains. Each actual disclosure incurs its
effort in clock ticks as well as utility cost, plus one tick per evidence receipt.
Food is unchanged by communication; ordinary consumption accounting is preserved.

## Prediction history and causal isolation

Six actual truthful training encounters generate response history. Non-challenges
produce estimates 50 -> 38 -> 29 -> 22 -> 17 -> 13 -> 10. Challenges produce
50 -> 62 -> 71 -> 78 -> 83 -> 87 -> 90. Present physical conditions and profiles are
then restored for the abundant-state deception probe. The speaker's own Agent
state compares equal between the histories. At probability 10, Mislead scores
+1; at 90 it scores -31.

A local ablation takes the same recorded TalkInput and Context, replacing only
challenge expectation 10 with 90. The choice changes Mislead -> Silence. This
isolates prediction history from communication episodes, credibility or physical
state. Disabling learning under the non-challenge training leaves probability 50
and silence. Actual responses and signed errors are retained for all training
turns; the results do not demonstrate calibration on a held-out population.

The entire downstream histories are not otherwise identical: listeners also
learn credibility and remember responses. Therefore later resource differences
between training histories cannot be attributed solely to the speaker's prediction.
The H branch actually later Offers, for the protocol loophole described below.

## Fallible evidence and uncertainty

EvidenceKind::Fallible contains a received scarcity reading, reliability and an
event-local source ID. It is distinct from testimony, the old authenticated
Disclosure kind, and objective truth. Automated foresight conversations always
use the fallible path. Legacy public disclosure remains for older experiments.

Seeded sensors are correct with a designed probability of 50 + reliability/2
percent. Reliability is restricted to 0..90 and acts as an ordinal support weight,
not a posterior truth probability. Sampling is keyed by seed, event, speaker and
source; revisiting a source cannot reroll noise. AccurateFixture/InvertedFixture
are explicit controlled observation interventions to separate reading direction
from weight. They are not claims that real evidence is infallible.

Once direct fallible readings arrive, they supersede unsupported testimony.
Support = strongest positive reliability - strongest negative reliability.
A correct reading of weight 40 produces support 40 rather than inheriting the
claim's 50. A weight-80 reading gives support 80; two opposing weight-80 readings
give zero. Repeated sources do not accumulate confidence; attempts to rewrite a
received source are rejected. Claims after readings cannot erase that evidence.
This rule intentionally does not count the speaker's claim and their own evidence
as independent corroboration. Earlier experiments retain their prior combination
rule. Memory reinterpretation still requires support >=60; unresolved evidence
restores/retains the original negative interpretation rather than inventing blame
relief. Original historical events remain immutable and trust uses replacement
ledger replay, including saturation.

Credibility updates only when combined evidence magnitude reaches 60. The previous
+20/-30 consistency deltas are scaled by that magnitude. This is consistency with
observed evidence, not omniscient lie detection: a bad reading can harm an honest
speaker's credibility. `verified_claim` in the reused turn schema means observed
consistency in this mode; `verifiable` remains false for fallible receipts.
Each Reading separately records its objective correctness for evaluation only.

Across the 128 noisy single-refusal trials, 15 of 128 readings are wrong; 113 lead
to later Offer, 15 to Leave. This is a deterministic finite sample from the 90%
correctness design, not a calibrated claim that belief support 80 means 80%
probability. Opposite fixture readings produce two audited memory revisions:
initial relief of blame, then restoration when combined support becomes zero.

## Revealed behavioral problems

The four-turn limit creates a timing loophole. With expected scrutiny, a speaker
can initially stay silent. If asked for an explanation on turn two, it can lie on
turn three with no room for another challenge. The predictor correctly uses zero
within-scene challenge probability then; it does not consider a later encounter.
This happens in all 128 dedicated delayed-lie trials. No multi-step plan selected
initial silence to arrange this: the two choices are independent one-step decisions.

After honest/challenging training, this delayed lie can succeed because the
listener has learned high credibility. In H, testimony receives support 90 and
removes blame, producing later Offer. This is a combination of learned credibility,
listener request history and finite-horizon blindness, not proof that anticipation
eliminates deception. It also prevents presenting initial silence as permanent
concealment. The simple untrusted lie remains ineffective in A/B.

Expectations are conditioned only on partner and broad response category. The
agent does not distinguish truthful from false claims when learning challenge
rates. Unchosen actions provide no feedback, so selective silence can preserve a
bad expectation. Integer updates stall within three points of 0/100. Evidence
sources are limited synthetic channels; no observation ecology, exploitation of
revealed weakness, strategic sensor choice, or general causal model is implemented.

## Verification, performance and storage

Formatting, warning-free Clippy, all 43 integration tests and all four benchmark
modes pass. Baseline checks passed before behavioral changes. Early development
Clippy rejected explicit drop calls on Bevy borrow guards; ordinary borrow scopes
fixed that warning. No unresolved failing check remains.

Tests retain all 32 older checks and add 11 for counterfactual sweeps, expectation-
only ablation, noise replay, evidence conflicts/source immutability, local inputs,
invalid predictor termination, atomic configuration, effort ticks, population
resource accounting and reconstruction of forecasts/expectations/agent state.
The saved Experiment 001 full seed-42 report, Experiment 002 suite and Experiment
003 suite compare exactly as JSON values in an automated regression test.
Population replay includes seeds 0, 42 and u64::MAX at odd size. The full 1,000-agent
report replays too. Finite tests are not an exhaustive proof or multi-platform test.

Seed-42 population: 957 resource events, 171 conversations, 618 communication
turns, 10 scored response predictions and five fallible readings, one incorrect.
Actions are Explain 9, AskExplanation 133, AskEvidence 5, ProvideEvidence 5 and
Silence 466. Closing silence is included and is not necessarily concealment.
There are four memory revisions and 843 food units consumed. No generated seed-42
agent chose Mislead; deliberate deception findings come from the controlled cases.

| Mode | Population | Rounds | Init ms | Run ms | Records |
| --- | ---: | ---: | ---: | ---: | ---: |
| 001 | 100 | 100 | 7.992 | 15.654 | 9,640 |
| 002 | 100 | 100 | 6.869 | 19.255 | 21,341 |
| 003 | 100 | 100 | 6.380 | 22.835 | 25,865 |
| 004 | 100 | 100 | 8.819 | 30.012 | 25,891 |
| 001 | 1,000 | 100 | 52.487 | 118.869 | 95,578 |
| 002 | 1,000 | 100 | 46.096 | 197.144 | 212,751 |
| 003 | 1,000 | 100 | 45.351 | 247.883 | 258,428 |
| 004 | 1,000 | 100 | 46.734 | 303.830 | 258,642 |

At 1,000 agents the new short workload takes about 23% more run time than mode 003.
Counts include resource events, communication turns and consumption, not duplicated
nested receipts/forecasts. Mode 004 uses effort=5, advancing simulated ticks without
wall-clock sleeps. Timings exclude JSON/report generation, replay and final audit
clones used to count records. Workloads differ, measurements have no confidence
intervals, and no inference about massive or long-running populations is justified.

Compact population JSON is 2,654,086 bytes, about 22% above 003's 2,169,219 bytes.
This is serialized trace size, not RAM. Each directed partner expectation stores
two integers, but maps are unbounded across partners. Existing credibility/trust
maps, scenes and all audit vectors also grow. Resource/communication episodes and
beliefs remain bounded at 16 each. Forecast lookup, evidence combination and trust
revision scan retained audits; long histories can have quadratic aggregate work.
Settings add one record per individual; full snapshots amplify report size. No
heap allocation profile, snapshot restoration or persistence service was added.

## Recommendation for Experiment 005

Investigate persistent unresolved questions across encounters with an explicit
cost to continue or reopen verification. The immediate reason is the demonstrated
late-claim loophole: a question should not cease to matter merely because a local
turn budget expired. Keep the prediction horizon shallow and test whether carrying
uncertainty forward changes strategic timing. Include noisy-evidence cases to
measure mistaken credibility penalties and correction, without adding reputation
networks or a general planner. Experiment 005 has not been started. These results
establish limited rule-based anticipation, not consciousness or human reasoning.
