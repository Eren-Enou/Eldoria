# Experiment 003 findings: intentional information sharing

Executed 2026-10-07, Windows x64, Rust 1.99.0 / Bevy ECS 0.19.1.
Rules `experiment-003-v1`. Reproduce:

```powershell
cargo run --locked --release --example evaluate003
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo bench --bench throughput
```

Evidence: [pre-implementation specification](specification.md),
[human causal report](report.txt), [full seed-42 trials](seed-42.json),
[128-seed summary](summary.json), [population audit](population.json),
[three-mode benchmark](benchmark.csv). Old Experiment 001/002 files are untouched.

## Architectural result

Communication is now selected by agents within an opt-in Simulation mode.
`Simulation::run` opens one four-turn conversation after each new refusal when
intentional mode is enabled. The experiment harness sets physical circumstances
and preferences but never calls `communicate` or specifies a message. Both parties
choose through a replaceable pure policy, with resolver validation. Enabling the
mode does not reopen historical refusals; calling run again does not duplicate
conversations. Original experiment modes remain the defaults.

The six actions are Silence, Explain, ProvideEvidence, AskExplanation, AskEvidence,
and Mislead. Silence is a real option, including after a request. Explain transmits
a truthful but unsupported proposition about scarcity at the refusal. Mislead is
legal only when the refuser knows that scarcity proposition is false. Its payload
is indistinguishable from ordinary testimony to the listener. ProvideEvidence
uses Experiment 002's explicit historical disclosure channel. No listener sees
hidden inventory, hunger, personality, intent to deceive, or another's beliefs.

A separate ECS resource holds preferences, bounded communication episodes and
local directed credibility. Credibility is not relationship trust: only comparing
a claim with provided evidence changes it, +20 for consistency and -30 for
contradiction, clamped to [-40,40]. Testimony weight is 50+credibility, clamped to
[10,90]. Direct evidence is 100. Silence alone never changes credibility.

Beliefs and memory reinterpretation use Experiment 002's existing mechanism.
Objective refusal interpretations stay immutable. Revised live memories and trust
replay preserve event links and clamp semantics. Conversation records retain
complete local inputs, legal candidate scores, chosen actions, truth annotations
for the observer, receipt IDs, credibility changes, food balances and tick cost.
The listener's episode says Explain even when the objective action was Mislead.

Communication spends one tick and a finite turn slot per decision. Privacy and
honesty/risk costs are utility terms; they are not food sinks or simulated harm.
Food is unchanged during all communication actions. Consumption remains a separate
explicit phase with its existing accounting.

## Controlled results

All comparisons below were evaluated at seeds 0..127 and all 128 complete suites
replayed exactly. Each suite has 16 variants. Identical present physical states
are saved before the later resource probes. Parameter interventions are explicit
in snapshots, not changes to selected actions. These boundary fixtures establish
causal capability, not frequency in unconstrained worlds.

| Comparison | Communication result | Later food decision |
| --- | --- | --- |
| A: privacy 0 vs 100 | Claim, requested evidence vs silence despite a question | Offer vs Leave, 128/128 |
| B: privacy 40 | Initially silent; explains only when asked | Leave, 128/128 |
| C: listener relationship goal 0 vs 60 | Leaves quietly vs requests and receives explanation | Both Leave, 128/128 |
| D: evidence request cost 100 vs 0 | Unsupported claim vs requested disclosure | Leave vs Offer, 128/128 |
| E: verified vs unanswered past conversation | New testimony support 70 vs 50 | Offer vs Leave, 128/128 |
| E: prior contradicted claim | New testimony support 20 | Leave, 128/128 |
| E: same present, verified vs unanswered response history | AskExplanation vs Silence | Offer vs Leave, 128/128 |
| F: costly private scarcity, uninterested listener | Both choose silence | Leave, 128/128 |
| G: honesty 0 vs 100, abundant inventory | False claim vs truthful evidence | Both Leave, 128/128 |

A seed-42 causal chain: the refuser has food=1, hunger=90, privacy=0 and relationship
goal=60. Explain scores 32, evidence 12, silence 0. The listener requests evidence
(score 45). Responding with evidence now scores 62, so disclosure is chosen.
Support changes 50 -> 100; original Rejected/-17 is retained in the refusal event,
while the live memory becomes PossibleSelfProtection/0 and trust -17 -> 0.
Credibility independently changes 0 -> 20 because the claim was verified.

The later matched probe has food=4, hunger=30, generosity=60, caution=80. Its Offer
score is `60-15-40+0+0=5`. Under privacy=100, the refuser's Explain scores -68;
after the listener's question it remains -48, so no claim is sent. Trust and recent
valence remain -17 each, making the same later Offer score -29 and selecting Leave.
The privacy comparison changes only the refuser's profile, not scarcity or the
initial food interaction. Resource histories through refusal are identical.

C is an informative non-effect: asking elicits testimony but support remains 50,
below the reinterpretation threshold 60. No behavioral change follows. The
four-turn protocol leaves no second question after a late explanation; this is
a termination/design limitation, not evidence that further inquiry is worthless.

History experiments form actual conversations before restoring current profiles
and physical states. A verified claim raises later testimony weight to 70; an
unanswered request leaves it 50; a contradicted claim lowers it to 20. Past trust
and episodic interpretations also differ, so the full resource-choice comparison
alone does not isolate credibility's contribution. The distinct evidence weights
are explicitly computed from credibility, independent of trust.

The request-history comparison adds a direct local ablation: with current goal=15
and hunger=90, a previous evidence response contributes +10, giving AskExplanation
score 13. A remembered unanswered request contributes -20, giving -17 and silence.
Clearing only communication episodes changes the latter to score 3 and a request;
copying the verified branch's episodes changes it to a request as well. All other
inputs, including resource memory and trust, stay fixed in this ablation. This
isolates communication history as a cause of subsequent information seeking.

## Deception findings and unresolved behavior

With food=4, the refuser knows the scarcity claim is false. Under goal=60,
honesty=0, caution=90, privacy=0, Mislead scores `60-0-0-45-10=5`, versus silence 0.
Removing the relationship goal makes silence win. Raising honesty to 100 instead
selects truthful evidence. Deception is motivated by the scoring rule, never a
random false statement. A test shows that a true and false scarcity claim produce
identical listener inputs/decisions until evidence is supplied.

In G's first-contact false-claim branch, support 50 is insufficient to remove
blame: deception is attempted but does not change the later resource decision.
When challenged, this low-honesty speaker provides contradictory evidence because
the request-response bonus makes its evidence score 2. The listener then records
support -100 and credibility -30. A later true claim receives support 20.

This self-incrimination exposes a weakness of the one-step heuristic: the speaker
does not forecast the credibility penalty or reason about the listener's private
belief. The goal term also rewards attempted persuasion without estimating its
actual success. These limits are retained and reported; the experiment does not
claim optimal strategy or successful deception. No deceptive action occurred in
the saved seed-42 generated population. Controlled deceptive fixtures establish
capability, not its natural prevalence.

## Population, verification and performance

A seed-42 population of 1,000 generated agents uses disjoint pairs and the same
policy/resolver: 957 resource events, 171 conversations and 614 communication
turns. Opening choices are 5 Explain and 166 Silence. Across all turns there are
9 Explain, 133 AskExplanation, 3 AskEvidence, 3 ProvideEvidence and 466 Silence.
The Silence count includes required closing turns; it is not a count of deliberate
concealments. Twelve information receipts produce three memory revisions. Food
consumption is 843 units; initial food equals final food plus consumption.

The full population replays exactly. A separate automated audit reconstruction
checks each policy decision and local input, beliefs, communication memories,
credibility, ordered ticks, resource balances and final agent states. Odd-size
populations and extreme seeds also replay. Both saved earlier baselines compare
equal: Experiment 001's full seed-42 report and Experiment 002's eight seed-42
trials. Those files were not regenerated in place.

Formatting, warning-free Clippy and all 32 integration tests pass (11 Experiment
001, 10 Experiment 002, 11 Experiment 003). New coverage includes counterfactual
sweeps, history ablation, known-false intent, intent hiding, private preferences,
invalid-action termination, atomic profile validation, repeated-run idempotence,
opt-in boundaries, credibility saturation, bounded episodes and full audit replay.
The first development compile caught a parenthesization error in score expressions;
it was fixed before evaluation. No failing checks remain.

| Mode | Population | Rounds | Init ms | Run ms | Records |
| --- | ---: | ---: | ---: | ---: | ---: |
| 001 | 100 | 100 | 7.804 | 15.481 | 9,640 |
| 002 | 100 | 100 | 6.779 | 20.008 | 21,341 |
| 003 | 100 | 100 | 6.422 | 22.577 | 25,865 |
| 001 | 1,000 | 100 | 52.822 | 119.226 | 95,578 |
| 002 | 1,000 | 100 | 42.576 | 178.713 | 212,751 |
| 003 | 1,000 | 100 | 42.930 | 238.702 | 258,428 |

At 1,000 agents the new workload takes about 34% more run time than 002, and about
twice 001. It performs different work: 002 forces disclosure for each refusal,
whereas 003 evaluates agent choices, often requests/silence. Record throughput
is not a like-for-like speed comparison. 003 counts resource events, communication
turns and consumption, without double-counting nested information receipts.
Timing excludes full JSON/reporting/replay and the final audit clone used only to
count records. Baseline modes retain their prior measurement boundaries. These
short local runs have no confidence intervals or multi-platform validation.

The compact population JSON is 2,169,219 bytes (about 69% above 002's 1,284,547).
This is serialized audit size, not RAM. Each individual has at most 16 communication
episodes, 16 resource episodes and 16 beliefs. Profiles are one per individual;
credibility is sparse per directed relationship and can grow without bound, as
can the existing trust maps. Audit records, snapshots, circumstance archives and
scene vectors also grow. Decision snapshots duplicate bounded local history.
Receipt/revision searches still scan growing ledgers, and scene admission/idle
checks scan scenes. Long histories can therefore incur quadratic aggregate work.
No feasibility claim beyond the measured 1,000-agent short encounters is supported.

## Limits and Experiment 004 recommendation

This is intentional communication within a post-refusal phase, not a unified
planner choosing food offers and speech in the same turn. All resource scenes in
a batch finish before conversations run, in deterministic event order. Only the
two refusal participants can communicate about that refusal. The evidence channel
still assumes an authentic historical sensor record, not ordinary fallible witness
observations. Disclosure costs express preference; no exploitation or physical
privacy harm is implemented. Population profiles derive deterministically from
existing caution, generosity and expectation, so they are correlated with traits.

Confidence and utility constants are explicit hypotheses, not learned calibration.
Equal-strength conflicting evidence preserves the existing belief. Credibility
updates only for a claim followed by disclosure within that conversation; it does
not infer dishonesty from silence, propagate reputation or revisit earlier claims
across conversations. Episodes evict, aggregate credibility persists, and audits
retain provenance. Long-term storage/snapshot restoration remains future work.

Recommend Experiment 004 on bounded prediction of communication outcomes and
fallible, costly evidence access. Compare predicted persuasion/verification costs
with observed results, including the demonstrated ineffective deception and
self-incrimination. Test whether an agent can decide to continue inquiry when a
late explanation leaves uncertainty, under a genuine opportunity budget. Retain
locality and causal ablations before adding any larger social system. Experiment
004 has not been started. No consciousness or broad social emergence is claimed.
