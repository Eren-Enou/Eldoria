# Experiment 001: personal history through resource interaction

Question: can generated individuals make independent decisions, interpret shared
events, and change future behavior because of those experiences?

Protocol and rules: `experiment-001-v1`, integer ticks and SplitMix64 generation.
CLI scenarios use one food unit and a six-turn limit. Food moves only on accepted
offers; total participant food is conserved within a scene. Interventions occur
between scenes and are retained in stage snapshots and descriptions. All seeds,
generated attributes, score inputs and consequences are recorded.

| Scenario | Intervention and observation |
| --- | --- |
| A | Two independently generated agents; inspect initial choice and outcome. |
| B | Run A, restore original current circumstances, retain trust/memories, repeat. Compare first choices/outcomes and local learning ablations. |
| C | Both hungry, requester with zero food, donor with one unit; guarded dispositions. Observe conflict without forced actions. |
| D | Request under scarcity; compare requester rejection and refuser protection interpretations of the same event. |
| E | Form actual helpful/negative histories under documented past traits; set identical present circumstances; compare with a learning-free control. |
| F | Run a complete generated trial twice and compare all structured states/events. Every selected scenario also verifies its entire trial replay. |
| G | Generate 100–1,000 agents; run disjoint pairs through the identical scene mechanism. Odd populations leave one unpaired agent. |

E's present probe sets agent 0 to food 4, hunger 40, generosity 30, caution 40;
agent 1 to food 0, hunger 90, generosity 30, caution 20. Generated expectations
are identical within each seed. Helpful past: agent 1 is satiated/generous and
initiates; refusal past: agent 1 is hungry/guarded and agent 0 initiates. Different
past circumstances and initiator order are controlled history interventions.
Present initiator order, scene amount, turn budget and private attributes match.
Time and event numbering differ between probes/control but neither enters scores.

Separate measurements: initial action distribution, outcome distribution, directed
relationship signs, local decisions changed by learning removal, changed repeated
first choices and outcomes, divergent subjective records, history-probe action
differences, exact replay equality, throughput and sampled working-set memory.
Counts are evidence for these small implemented mechanisms, not an interestingness
index. Learning ablations remove both trust and episodes; they do not establish
which representation alone is necessary.

`examples/evaluate.rs` evaluates seeds 0 through 127, plus matched-context trials
where food/hunger are fixed and generated dispositions vary. The JSON sweep is
saved alongside the results. Tests independently reconstruct state changes from
events, check references, and probe edge cases. No long-term population dynamics,
families, economies, Observer or Book are implemented.
