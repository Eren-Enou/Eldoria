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

Current limits: no metabolism, exchanges, communication of private circumstances,
memory decay, snapshot loading, persistence service, or parallel scene resolution.
Relationships and objective audit history can grow. Hunger is a supplied motivation
and is not automatically satisfied by inventory. Recommend Experiment 002 on
communicated need, uncertain beliefs, and resource consumption before scaling.
