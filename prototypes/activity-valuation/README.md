# Experiment 024 activity valuation prototype

An explicit opt-in workspace over the immutable 023 host. Baseline remains the
default. MealRelief changes only the ordinary eligible Consume score from H to
H+min(H,40). It adds no supply, action, persistent cognition or consent rule.
See the [preregistered specification](../../experiments/024/specification.md)
and [complete results](../../experiments/024/results.md).

From the repository root:

```powershell
cargo run --release --locked -j 1 --example validate024
cargo run --release --locked -j 1 --manifest-path prototypes/activity-valuation/Cargo.toml --target-dir target/activity-valuation --bin activity-valuation -- baseline-parity experiments/023/trials.jsonl.gz
cargo run --release --locked -j 1 --manifest-path prototypes/activity-valuation/Cargo.toml --target-dir target/activity-valuation --bin activity-valuation -- run target/valuation-demo.jsonl --valuation MealRelief --case paired --ticks 192
```

The demo output must not already exist. Runtime writes are prohibited throughout
`experiments` and `reinforcement`. `matrix NEW.jsonl` reproduces all 56 registered
worlds into a new scratch location; `replay ORIGINAL.gz` compares complete exact
serialized records including score audits, not just aggregate snapshots.

The generator verifies original 023 source locks and the exact reverse of every
declared coordinator substitution. The original physical/cognitive implementation
and incoming recipient decisions remain unchanged. The policy is a replaceable
pure function of captured own input and four legal candidates. Observer valuation
records reference the original boundary/choice and contain four scalar term rows;
they do not duplicate the entire actor state or feed information back to actors.

New envelope `experiment-024-trial/v1` and physical schema
`experiment-024-physical-history/v1` are explicit. This is bounded research with a
nominal stopping endpoint and a charged 33-tick completion period, not an indefinite
service or a viable economy. Neither MealRelief nor Continuous is adopted globally.
