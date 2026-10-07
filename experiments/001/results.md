# Experiment 001 findings

Executed 2026-10-06 on Windows x64, Rust 1.99.0, Bevy ECS 0.19.1;
rules `experiment-001-v1`. Artifacts: [full seed-42 JSON](results/seed-42/report.json),
[human trace](results/seed-42/report.txt), [128-seed sweep](sweep.json),
[benchmark](benchmark.csv), [memory sample](memory.json).

## Main finding: actual history changes subsequent decisions

All 128 controlled E trials (seeds 0–127) produced different present decisions
after helpful versus refusal histories. Present food, hunger, dispositions,
expectation, partner, amount and turn budget were equal; trust and episodes differed.
Histories were formed through the regular policy and resolver with documented past
interventions. A history-free control matched the refusal branch's withdrawal.

Seed 42 agent 0: helpful event 1 yielded trust +24 and episode valence +24. At
probe event 2, Offer scored `30 - 40/2 - 40/2 + 24 + 24 = 38`, Request scored 1,
Leave scored 0. It offered; agent 1 accepted; balances moved [4,0] -> [3,1].
Refusal event 1 yielded trust -17 and recent valence -17. The same present Offer
scored -44, Request -9, Leave 0; it withdrew. With learning removed, Offer was -10,
Request -5, Leave 0, also withdrawal. Cause references [1,0] trace learned inputs
to the preceding events; only event 1 had nonzero valence.

This supports persistent, causally inspectable behavioral consequences in the
implemented mechanism. It does not establish sophisticated social intelligence.
The fixtures deliberately choose an informative threshold region; the result is
not an estimate of how often history matters in an unconstrained world.

## Independent measurements

Across 128 generated first encounters: first actions were Request 66, Offer 42,
Leave 20. Outcomes were Agreement 53, Refusal 52, Withdrawal 23. Final directed
relationships included 105 positive and 53 negative values. With food and hunger
held fixed while generated dispositions varied, first actions were Offer 64,
Leave 55, Request 9. Thus variation exists under matched resource/need context.

Repeated encounters restored generated present circumstances while retaining
learning. First action changed in 18/128 encounters (14.1%); final outcome changed
in 8/128 (6.25%). Local learning ablation changed 18 decisions. A local ablation
holds the current public observation fixed; it is not a complete downstream
counterfactual replay. Seed 42 B strengthened trust but kept the same choices,
which shows memory need not change every decision.

C produced refusal under competing needs. D retained Rejected/-17 for requester
0 and ProtectedOwnNeeds/0 for refuser 1 for the exact same seed-42 refusal event.
Agent 0 does not know agent 1's hunger; the model does not yet permit informed
attribution of refusal to legitimate scarcity.

G reused the scene mechanism for 1,000 agents in 500 disjoint interactions.
It produced 219 agreements, 167 refusals and 114 withdrawals (959 action events).
The full seed-42 suite, including control trials, retained 979 events across 511
scenes: 225 agreements, 170 refusals, 116 withdrawals. Event counts quantify work,
not interestingness. Every selected trace reproduced exactly. Entire serialized
suite equality was also checked at seeds 0,42,u64::MAX with an odd population.

## Validation and performance

Formatting and Clippy for all targets with warnings denied passed. All 11 tests
passed. A 256-seed invariant sweep covers complete small suites, reconstructs
state from history, checks decision scoring against local snapshots, legal actions,
resource totals, termination and references. Additional tests cover extreme seeds,
invalid IDs, overlaps, invalid imported state, invalid policy actions, zero budgets,
timeout, overflow, memory eviction, private-information isolation and populations
of 101 and 1,000. Seeds are deterministic property sweeps without shrinking;
coverage is finite and platform testing was Windows only.

Release benchmark: 100 independent seeds per population, each with one disjoint
pairing round. Initialization includes generation, ECS creation and scene admission;
run time includes scheduling, policy, validation, interpretations and history writes.
JSON serialization, report formatting and replay verification are excluded.

| Population | Total events / 100 runs | Initialization ms | Simulation ms | Events/sec |
| --- | ---: | ---: | ---: | ---: |
| 100 | 9,640 | 7.033 | 15.300 | 630,061 |
| 1,000 | 95,578 | 49.640 | 120.451 | 793,503 |

One additional standalone benchmark process was sampled with Windows
PeakWorkingSet64 every 10ms: maximum observed working set 6,770,688 bytes
(6.46 MiB). It includes executable/runtime, setup and retained short histories;
it is not an allocation profile, a per-agent estimate, or a long-running bound.
These short local timings have no confidence intervals and depend on machine/load.

## Limits and Experiment 002

Decision weights are transparent hypotheses, not fitted models. Hunger is supplied
and does not decrease when food is obtained; private beliefs are minimal (prior
expectation and learned partner trust). Traits are stable; learning changes
relationships and decisions. Episodes are capped at 16; objective history and
relationship maps grow. No streaming persistence, snapshot restoration, geography,
trade, communication of motives, memory decay, or parallel scene execution exists.
No million-agent or long-run performance claim is supported.

The architecture passed its small foundation tests. Next: explicit consumption
and communicated scarcity, with uncertain, revisable beliefs. Test whether the
same refusal is interpreted differently after legitimate evidence about need;
compare calibrated beliefs, trust changes and later cooperation. Keep objective
reality private unless observed. This addresses interpretation quality before
adding population complexity.
