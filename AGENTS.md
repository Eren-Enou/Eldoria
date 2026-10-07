# World of Individuals engineering instructions

Build progressively toward a persistent fictional world. Experiment 001 is only
resource interaction, subjective memory, and subsequent behavioral consequences.
Keep systems replaceable; do not add speculative civilization or Observer/Book UI.

Constitution: (1) identifiable causality, (2) information locality, (3) persistent
consequences, (4) distinguish individual perspective from objective events,
(5) explicit resource accounting, (6) controlled reproducibility, (7) no fixed
narrative outcomes, (8) historical traceability, (9) replaceable behavior systems,
(10) measured complexity. Preserve all ten when changing code.

Policies receive only local observations and their actor's state. Never expose
private partner components. New actions require legality checks, resolution,
accounting, structured records, and termination coverage. Do not serialize ECS
entity handles. Bound episodic memory and document other growing storage.

Before changing behavior, document the hypothesis and tradeoffs. Run cargo fmt
--check, cargo clippy --all-targets -- -D warnings, cargo test, and cargo bench
--bench throughput when relevant. Update Experiment 001 evidence after rule
changes. Report failures and limitations. Keep generated JSON reproducible;
wall-clock measurements belong in separate benchmark output. Do not claim
consciousness or broad social emergence from these experiments.
