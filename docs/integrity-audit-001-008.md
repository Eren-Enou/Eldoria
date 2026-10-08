# Integrity audit results — Experiments 001–008

Completed 2026-10-08 against baseline `9b54619`.
Scope: contracts, documentation, regression coverage and read-only validation.
No simulation policy, resolver, heuristic, capacity, default, dependency or historical
experiment file changed. Experiment 009 was not started.

The [contract/coverage map](experiment-contract-001-008.md) records all eight modes,
exact bounds, local/observer ownership, retained and persistent representations,
heuristics, compatibility, coverage classifications and growth limitations. The
formerly empty constitution document now states the ten existing AGENTS requirements.
AGENTS and README link the map and read-only regression gate.

## Findings and minimum hardening

Most material invariants already had direct tests: local decision isolation,
resource/time accounting, invalid-action termination, subjective interpretation,
saturated replacement trust, bounded concerns/inquiry/provenance/assessment,
selected-response learning, known/hidden lineage, later attribution and forgetting.
Foundation tests cover immutable contexts, reference corruption, exact compact
expansion, hash collision safety and index rebuild equality.

The apparent forgetting conflict was an overly broad audit requirement. User
clarification preserves separately established persistent consequences. The legacy
native path may retain received-evidence maxima after bounded belief eviction;
Experiment 008 uses only its retained prospective basis for support. Neither path
recreates an evicted episode. This distinction is explicitly documented per mode.
Earlier recommendations and pre-foundation performance descriptions remain historical
context; finalized 008 selects Grouped while retaining Max/Legacy as controls.

Three focused regression tests were added:

- Finalized008's complete seed-42 output equals its archived JSON value.
- After legacy belief/episode eviction, a legitimate new receipt uses persistent
  received-evidence consequences without restoring the episode, modifying original
  history or retroactively changing its trust contribution. Detached observer export
  mutation does not alter live cognition.
- Unequal-weight Grouped assessment gives identical support across all six delivery
  permutations of three origins, below-capacity repetition and unrelated-event noise.

`examples/validate_contract.rs` adds a read-only gate for exact archived suites,
populations, available readable reports, complete seeded replays and 008 causal
validation. Existing archived regression tests remain unchanged.

## Verification

All checks passed on the current Windows workspace using the locked dependencies:

| Check | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` | Pass |
| `cargo test --locked -j 1` | 109 tests pass (106 existing, 3 added) |
| `cargo run --release --locked -j 1 --example validate_contract` | All 001–008 seed-42 archives equal; 129 full seeded replays per mode, 0–127 and `u64::MAX`; populations and available readable reports equal;008 causal validation passes |
| `cargo build --release --locked -j 1 --examples` | Pass |
| Original `evaluate`, `evaluate002`–`evaluate008`, `validate_history` release executables | All pass in an isolated copy under `target/contract-audit`, including established compatibility/replay evaluators |
| Isolated deterministic evidence comparison | All 32 JSON files in the copied/regenerated tree equal repository values; independently regenerated001 sweep equals archived sweep |
| `cargo bench --locked -j 1 --bench throughput` | All established workload rows through 008 complete; no runtime optimization or performance improvement claim |
| SHA-256 archive preservation | All 72 files under `experiments/` byte-identical before/after evaluation; Git diff contains no experiment evidence changes |

The original evaluator executables ran with their relative output directory in the
isolated copy, never in the repository's frozen experiment directories. Timing CSVs
and transient logs stay under the ignored target directory; no wall-clock values
were added to deterministic JSON. Single-job builds preserve the documented Windows
memory constraint. No failed required validation remains.

## Limits intentionally retained

The coverage map labels evidence-only and indirect coverage as such. Tests do not
prove arbitrary future policy noninterference, scientific calibration, complete
forensic validation or large-world scaling. Source review remains necessary when
changing local projections, resolver observation channels or namespaces.

Unbounded observer archives, sparse partner maps, legacy received-source summaries,
worst-case relationship suffix replay, older forecast scans, export cloning and
serialization remain. Frozen008 weights and006 novelty versus joint-support mismatch
remain experimental limits. External opportunities, finite-horizon deception and
fallible confidence remain unchanged. No database, planner, arbitrary history deletion,
new behavioral capability or cognitive capacity increase was introduced.
