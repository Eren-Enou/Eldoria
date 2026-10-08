# Experiment 005 — persistent unfinished intent

Implemented 2026-10-07 on the existing Rust + Bevy ECS headless foundation.
The [hypotheses](specification.md) were recorded before behavior changes. The
Experiment 004 baseline passed formatting, Clippy, 43 integration tests and the
four-mode benchmark before implementation. Earlier evidence files were preserved.

## Architecture and experiment design

An opt-in `Concerns` resource retains at most eight refusal-related questions per
individual. Concerns are distinct from memories, beliefs, trust and predictions.
They reference an original event and last receipt, caching only importance,
uncertainty, age, attempts, failures and status. A replaceable local policy chooses
Continue, Reopen one concern, or Abandon one concern. No hidden partner state or
objective truth enters its scores. See [architecture](../../docs/architecture.md)
for API, scheduling, legality, bounded storage, transitions and accounting.

Creation requires actual refusal uncertainty and importance at least 40. On a
later encounter, relationship value and estimated information gain compete with
pursuit effort, age and unsuccessful attempts. Reopen costs one tick plus configured
effort, and elicits a voluntary bounded response through the existing communication
resolver. Receipt reliability changes belief support; absolute support at least 60
is sufficient subjective resolution. Abandonment does not assert certainty.

Eleven controlled variants were run and replayed over seeds 0–127 (1,408 trials,
plus matching replays). Initial requesters have food 0, hunger 90; refusers have
food 1, hunger 90, and privacy 100. The normal policies produce refusal and silence.
The listener preserves the unanswered question (importance 78). Later encounters
use documented physical/profile interventions, usually reducing refuser privacy
to permit voluntary evidence. The harness supplies opportunities, not messages.
Sensors are accurate/inverted fixtures for causal isolation. The seeded population
uses noisy evidence. Identical outcomes across controlled seeds reflect these
fixed conditions, not a statistical claim about diverse societies.

[Full seed-42 traces](seed-42.json), [readable causal chains](report.txt),
[128-seed summary](summary.json), and [population trace](population.json) separate
experimental interventions, objective events, local inputs, scores and outcomes.
The subsequent behavior measure is a read-only local resource-policy probe with
food 4, hunger 30, generosity 60 and caution 80; it is not an executed transfer.

## Actual controlled findings

| Scenario | Controlled result across all 128 seeds |
|---|---|
| A: Unresolved refusal | An Open concern survives the initial encounter; no same-encounter follow-up. Low-importance uncertainty is separately tested and not retained. |
| B: Later reopening | At cost 20 the listener selects Reopen, score 62. The partner provides evidence. Support becomes +80, status Resolved, uncertainty 20. |
| C: High follow-up cost | Cost 100 changes Reopen's score to -18 and Continue wins. The matter remains Open without a request. |
| D: Relationship importance | At cost 60, goal 60 gives Reopen score 22; goal 0 gives -8 and Continue wins. Present physical state and unfinished history match. |
| C: Rational abandonment | Goal 0 and cost 100 give Abandon score 28. Status becomes Abandoned despite uncertainty 100 and no new information. |
| E: Weak resolution | Quality 40 gives support +40, Partial, uncertainty 60, and no memory revision. |
| F: Strong resolution | Quality 80 gives support +80, revises Rejected to PossibleSelfProtection, restores trust -17 to 0, and changes the later probe from Leave to Offer. |
| E: Conflicting evidence | Opposed quality-80 sources yield final support 0 and Partial. The first receipt revises memory; the second reverses it and restores trust -17. |
| F: Mistaken resolution | Inverted quality-80 evidence produces support -80 and Resolved even though the scarcity inference is wrong. The negative interpretation and Leave probe persist. |
| G: Repeated failure | Three unanswered requests produce answer expectations 50→38→29→22 and three failures. At the fourth later encounter Abandon scores 2 and wins. |
| H: Competing concerns | Importance 58 and 98 yield reopen scores 48 and 72; only the latter is pursued and resolved. The older refusal remains open, so the later probe still selects Leave. |
| I: History counterfactual | Removing only the concern list from the same recorded local input changes Reopen to Continue. Removing only failure counters from the abandonment input restores Reopen in a separate test. |
| J: Compatibility | Earlier saved reports match structurally; repeated suites and empty, single-agent, odd and 1,000-agent populations replay exactly. |

The competing-concern test additionally equalizes ages and reverses importance to
isolate its effect, and checks stable tie handling. The history-only counterfactual
is a pure-policy ablation, not an independently evolved population with an otherwise
identical life history. These are deliberately narrow causal tests.

Repeated weak proof produces a useful intermediate outcome: three requests followed
by five Continue decisions through encounter eight. The concern stays Partial with
support 40. Re-reading the same source never increases support. Answer probability
rises 50→62→71→78 because evidence is supplied, while two failed-information attempts
reduce willingness to keep pursuing it. The initial test expected abandonment by
encounter eight; the trace disproved that expectation. The assertion was corrected
to the observed partial/deprioritized outcome without tuning behavior to force it.

Additional tests show costly proof can be withheld, prediction learning can be
disabled, a prior unsupported claim can later gain evidence-consistency credibility
(0→16), contradictory receipts can reopen a resolved concern, and concern resolution
survives eviction of the original memory. The original event is never rewritten.
Capacity pressure has explicit abandonment and eviction records, rather than silent
loss of unfinished intent. Combined audits reconstruct concern state and every tick.

## Verification and population evidence

Formatting, Clippy with warnings denied, and the complete test suite pass: 57
integration tests, including 14 new Experiment 005 tests. The evaluator replays all
128 suites and the 1,000-agent population; automated archive comparisons preserve
Experiments 001–004. Source immutability, earlier deception, local prediction,
belief revision, accounting and replay tests remain active.

Seed 42's 1,000-agent population runs three encounters per fixed pair with consumption
after each and no replenishment. It produces 2,436 resource events, 1,328 talk turns,
185 created concerns, 214 selected follow-ups, and 54 information receipts. No
voluntary abandonment occurs within this short natural population window; abandonment
is demonstrated in the controlled trials. Compact JSON is 8,341,078 bytes. This
measures serialized audit volume, not heap usage. Resource conservation includes
consumed food and is tested for each population size.

## Benchmark and scaling limits

`cargo bench --bench throughput` uses release builds, seeds 0–99 and populations
100/1,000. [Baseline](baseline-benchmark.csv) and [post-change output](benchmark.csv)
contain wall-clock data separately from reproducible JSON. Initialization measures
generation, ECS setup and initial scene admission. Execution includes resource
resolution, automatic communication/concerns and final consumption. Final audit
cloning/counting and serialization are excluded. Three-encounter rows include
later scene admission and have consumption once at the end, unlike the separate
population evidence's consumption after each encounter.

| 1,000 agents × 100 runs | Execution total (ms) |
|---|---:|
| Experiment 004, one encounter | 325.697 |
| Experiment 005, one encounter | 369.654 |
| Experiment 004, three encounters | 1,041.337 |
| Experiment 005, three encounters | 1,466.173 |

Concern formation adds about 13.5% in this one-encounter comparison; the matched
three-encounter workload adds about 40.8%, including extra communication and audit
work. Record counts include concern decisions in mode 005, so records/second are
not directly comparable measures of efficiency across modes. Old-mode 004's
one-encounter time changed from 301.247 ms in the baseline to 325.697 ms here (+8.1%);
this is a single-run timing difference, not isolated proof of regression. No
allocation profiling, latency distribution or large-scale feasibility is claimed.

Per-agent concern storage and priority scans are bounded. Event references remain
valid because historical archives are unbounded. Audit snapshots, transitions,
credibility/expectation/trust maps, evidence searches and memory revision replay
remain scaling risks; long histories can still incur quadratic aggregate work.

## Limitations and recommendation

Only refusal-scarcity questions exist. Meetings are scheduled externally, and
follow-up runs at an encounter boundary after resource interaction. One concern
can be pursued per participant; agents do not choose whom to meet. Relationship
importance is a supplied local goal, not an emergent bond model. Costs, thresholds,
age penalties and expected-gain equations are explicit heuristics, not calibrated
psychology or optimal planning. Concern age advances only on the owner's encounters.

Abandoned matters remain abandoned even after later evidence; resolved matters can
reopen under contradiction while retained. Full memory eviction prevents the existing
memory mechanism from revising that episode, even when its concern resolves. The
responder still uses the existing narrow historical-self observation channel.
Bounded concerns do not provide restart persistence or bounded archival storage.

The main unresolved behavior is distinguishing *a likely answer* from *new useful
information*. Agents can repeatedly ask for the same uninformative source before
failure penalties dominate. Strong but misleading evidence can also close a question
incorrectly. Experiment 006 should test source novelty and expected information gain:
whether an agent can recognize that another request to the same source is unlikely
to reduce uncertainty, while valuing genuinely new locally accessible evidence.
Keep that experiment bounded and compare against these repeated-weak-evidence cases.
Experiment 006 has not been started.
