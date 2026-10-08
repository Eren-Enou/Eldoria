# World of Individuals

Headless Rust + Bevy ECS research foundation for individuals whose experiences
affect subsequent decisions. Experiment 001 implements food interactions, local
information, subjective episodic memory, learned relationships, reproducible
scenarios, and causal reports. It does not demonstrate consciousness or complete
personality. No graphics or full Bevy engine installation is needed.

## Install and run

Install Rust with Cargo and the Windows MSVC build tools (or a Rust-supported native
toolchain on another OS). Minimum Rust is 1.95; verified here on Rust 1.99.0,
Windows x64. Bevy ECS 0.19.1 is pinned; Cargo.lock pins resolved dependencies.

From this directory:

```powershell
cargo run --locked --release -- --scenario all --seed 42 --population 1000 --output experiments/001/results/seed-42
cargo run --locked --release -- --scenario E --seed 42 --output experiments/001/results/history
cargo run --locked --release --example evaluate
cargo bench --locked --bench throughput
```

Use `--help` for options. Scenarios are A (first), B (repeated), C (conflicting
needs), D (different interpretations), E (altered history), F (reproducibility),
G (population), or `all`. Population applies to G; defaults to 1,000. Odd counts
leave one agent uninvolved. All selected traces are replayed to verify equality.

`--output DIR` writes full `report.json` and readable `report.txt`; without it the
CLI prints the readable report. JSON contains generated agents, all stage before/
after states, explicit interventions, candidate scores, local observations,
actions, balances, outcomes, memory event references, and relationships. Human
output caps large stages; JSON is complete. No wall-clock data contaminates
deterministic reports. Reports overwrite files in the chosen directory.

## What the experiment establishes

With seed 42, agent 0's current food, hunger, traits and expectation are identical
in the altered-history probes. After receiving help it offers food (score 38);
after refusal it withdraws (offer score -44). Removing learning also produces
withdrawal (offer score -10). Earlier event IDs are recorded as causes of the
learned scores. The histories themselves are produced by the interaction policy
under documented experimental interventions, not inserted memories.

All 128 seeds in the evaluation show this controlled difference. Natural generated
encounters produce agreement, refusal, and withdrawal. See
[the findings](experiments/001/results.md) for exact measurements and limits.

## Architecture and verification

The [001–008 contract and coverage map](docs/experiment-contract-001-008.md)
documents mode-specific locality, retention, persistent consequences, exact bounds
and regression coverage. Verify all eight finalized modes without writing archives:
`cargo run --release --locked -j 1 --example validate_contract`.

One cohesive Agent ECS component, resources for ordered scenes/history/clock, and
one single-threaded scheduled resolver keep this first experiment small. A pure
replaceable utility policy sees only the actor and its observation. Five actions
are validated before applying effects. Six turns is the scenario limit; memories
hold 16 episodes while aggregate trust persists. Objective history is separate
from subjective interpretation. [Architecture](docs/architecture.md) explains
tradeoffs, score rules, and future boundaries; [AGENTS.md](AGENTS.md) preserves the
simulation constitution for future work.

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests cover seeded generation, local information, reproducibility, actual history
effects, independent interpretations, resource conservation, legal actions,
event/state replay, termination, invalid participants, overflow, memory eviction,
and 1,000-agent populations. Seed sweeps test many inputs deterministically; they
are not exhaustive proofs and do not use a shrinking property-test framework.

Experiment 001 limits: no metabolism, exchanges, communication of private circumstances,
memory decay, snapshot loading, persistence service, or parallel scene resolution.
Relationships and objective audit history can grow. Hunger is a supplied motivation
and is not automatically satisfied by inventory. Recommend Experiment 002 on
communicated need, uncertain beliefs, and resource consumption before scaling.

## Experiment 002: communication and reinterpretation

Run `cargo run --locked --release --example evaluate002` to regenerate
[Experiment 002 evidence](experiments/002/results.md). This keeps the original
CLI and Experiment 001 rules available. The new experiment adds structured
communication scenes, bounded uncertain beliefs about particular refusals,
audited memory reinterpretation and food consumption. It compares silent,
unverified, verified, conflicting and altered-history cases across 128 seeds,
plus a 1,000-agent population. `cargo bench --bench throughput` measures both modes.

At seed 42, an explanation after helpful history changes a matched later choice
from Leave to Offer; contradictory disclosure restores Leave. Strong evidence
after negative history revises a memory but does not erase the other refusals.
These are controlled results of explicit rules. Communication is scheduled by the
experiment, disclosure uses a narrow instrumented observation, and confidence is
not calibrated. There is no claim of consciousness or complete human reasoning.

## Experiment 003: intentional communication

Run `cargo run --locked --release --example evaluate003`. The dedicated
[findings](experiments/003/results.md) link to reproducible JSON and a readable
causal report. Enable `Simulation::enable_intentional()` before creating scenes;
`run()` then lets both participants choose communication after refusals. Earlier
experiment defaults remain unchanged.

A local policy scores silence, explanation, evidence, information requests and
known-false scarcity claims. Separate credibility learns only from verified or
contradicted claims. Communication memories are bounded; original events stay
immutable. Controlled trials show disclosure costs, evidence-request costs and
communication history changing choices. Requests and deception sometimes fail to
change later behavior. See the findings for sensor assumptions and other limits.

## Experiment 004: anticipating communication consequences

Run `cargo run --locked --release --example evaluate004`; see
[findings and evidence](experiments/004/results.md). `enable_foresight(seed, sensor)`
adds a replaceable one-step predictor to intentional communication. Agents learn
local challenge/evidence-response expectations from public responses. Candidate
records separate immediate utility and anticipated consequences. Fallible evidence
has explicit reliability and disclosure effort; conflicting readings can leave
uncertainty unresolved.

Controlled trials show anticipation changing deception and evidence choices, and
prediction errors changing later expectations. They also expose a turn-budget
loophole: late claims cannot be challenged within the current conversation. Old
experiment modes and evidence remain intact. The four-mode benchmark measures
short populations only; see the findings for noise, storage and scaling limits.

## Experiment 005: persistent unresolved concerns

Run `cargo run --locked --release --example evaluate005`; see the dedicated
[findings](experiments/005/results.md), [causal report](experiments/005/report.txt),
and [pre-implementation hypotheses](experiments/005/specification.md).
Call `enable_concerns()` after enabling foresight to retain up to eight unfinished
refusal questions per individual. On later encounters a local replaceable policy
chooses one evidence request, abandonment, or normal continuation. Costs, age,
relationship value, expected answers and previous failures affect that choice.

Strong evidence can close a concern and revise a memory; weak or conflicting
evidence can leave it unresolved. Closure is subjective and can be wrong.
Controlled trials and three-encounter populations preserve the previous four
experiments. The benchmark now includes formation and matched repeated-encounter
workloads. Bounded concerns do not bound the full audit archive or partner maps.

## Experiment 006: adaptive inquiry

Run `cargo run --locked --release --example evaluate006`; see
[findings](experiments/006/results.md), [causal report](experiments/006/report.txt),
[full seed-42 traces](experiments/006/seed-42.json), and
[compatibility evidence](experiments/006/compatibility.json).
Call `enable_inquiry()` after concerns to opt in. Resource encounters now choose
Direct, Evidence, or Pause using learned concern/source/strategy usefulness.
`inquiry_meeting(&[ids])` records bounded public co-presence and gives each
participant one voluntary opportunity; it supplies neither messages nor expertise.

Repeated weak answers lose useful value. Agents can change method, try a newly met
source, or retain a highly important question without asking again. Voluntary public
offers of genuinely new evidence can restore a path's value. New-source exploration
can fail; communicated evidence can be wrong. Gain scores are auditable heuristics,
not calibrated entropy estimates. The extension adds bounded local state but full
audit archives remain unbounded. Older policies, output defaults and evidence are
unchanged; matched 005/006 benchmark rows live under the new experiment.

## Experiment 007: provenance-aware testimony

Run `cargo run --locked --release --example evaluate007`; see the
[findings](experiments/007/results.md), [causal traces](experiments/007/report.txt)
and [compatibility evidence](experiments/007/compatibility.json).
Call `enable_provenance()` after inquiry. `inspect_refusal` offers a voluntary,
fallible inspection of a narrowly opened historical record. `provenance_exchange`
offers one structured sharing opportunity; `provenance_offer` permits voluntary
metadata disclosure. Ordinary resource encounters keep their existing response
mechanism. No autonomous gossip or search is added.

Known shared lineage suppresses redundant relay value; distinct observations can
corroborate or conflict. Hidden relay can cause overconfidence, and later voluntary
attribution can revise confidence, memory and concern status. Credibility remains
separate from independence. Three new local collections are bounded at 32; full
audit archives grow. The 007 report retains its original next-experiment recommendation.

## History foundation reinforcement

Experiments 001–007 remain frozen. Referential query audits and derived history
indexes reduce duplication and repeated scans while preserving complete history,
bounded local cognition and old report formats. See the [architecture guide](docs/history-audit.md)
and [measurements and findings](reinforcement/results.md).

Run `cargo run --release --example validate_history` for archived compatibility and
compact reconstruction checks, `cargo run --release --example measure_history -- after`
for isolated measurement output, and `cargo bench --bench throughput` for the
established workloads. New evidence stays under `reinforcement/`; these commands
do not rewrite prior experiment archives. Use `-j 1` on memory-constrained hosts.

The subsequent [next-target diagnosis](research/next-target/report.md) recommends
testing coherent assessment across native evidence and third-party testimony.
It includes mixed-channel counterexamples, history-depth measurements, five candidate
investigations and the falsifiable question implemented in Experiment 008 below.

## Experiment 008: coherent bounded assessment

Run `cargo run --locked --release --example evaluate008`; see
[results and controls](experiments/008/results.md) and [causal traces](experiments/008/report.txt).
After provenance, call `enable_assessment(assessment::Method::Grouped)` to assess
native receipts and third-party reports from one prospective FIFO basis capped at 32
per individual. The `Max` alternative remains an explicit comparison control.

Mixed evidence no longer reverses belief merely because one channel executes last.
Known relay remains redundant; independent reports can corroborate or conflict;
hidden dependence can still mislead. `native_acquired_exchange` delivers an actual
retained structured report through the native adapter without inventing another
origin. Prior modes and archives remain unchanged. Forgetting and legitimate new
information can change assessment; the mechanism does not discover objective truth.

## Experiment 009: bounded selective retention

[Results and limitations](experiments/009/results.md) compare FIFO, received-quality
retention and current-concern salience at the same 32-item capacity. Selective modes
preserve some useful distinctions but retain errors and miss later-useful minor
items. The evidence supports narrow opt-in research use; FIFO remains the default.

After `enable_assessment(assessment::Method::Grouped)`, explicitly call
`enable_retention(retention::Method::Salient)` or choose `Quality`/`Fifo` controls.
The policy sees only own retained items, incoming evidence and own capped concerns.
Use `experiment009::Snapshot::validate` for selective-history replay; the frozen 008
validator still reconstructs FIFO. Enabling does not import observer history.

Run `cargo run --release --locked -j1 --example evaluate009`, then
`python experiments/009/archive.py` and `python experiments/009/analyze.py`.
Full new 009 traces are reproducibly compressed; timings remain separate CSV.
Earlier experiments and their contract map are frozen. No Experiment 010 is added.
