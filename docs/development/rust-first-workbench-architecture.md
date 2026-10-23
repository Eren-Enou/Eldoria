# Rust workbench V1 architecture

V1 design baseline, preserved as prior engineering evidence. Current023–025
interfaces and the narrowly required Python viewer bridge are documented in
[Workbench V2](rust-workbench-v2.md). V1-only availability statements below
describe that earlier milestone.

Current compiler-free Windows operation is documented in the
[portable guide](PORTABLE_WORKBENCH_QUICKSTART.md) and
[engineering report](portable-workbench-engineering.md).


This engineering milestone starts from `969b52b`. It introduces no simulation
policy. The foundation ends at024;025's qualified research and separate integrity
candidate remain unadopted and unprotected.

## Verified execution capability (Stage A)

| Runner | Supported interface | Workbench V1 |
|---|---|---|
|023 continuous-world | `run NEW.jsonl --case paired --ticks 192 --period 11 --seed 42` | Enabled; four people, Continuous, Epoch, Ascending, claim4, cadence8, capacity2 |
|024 activity-valuation | `run NEW.jsonl --valuation Baseline\|MealRelief ...` | Unavailable in V1; needs a separate typed configuration/reader acceptance |
|025 material-preparation-observer2 | `run CONFIG.json NEW.jsonl` | Research-only, unavailable in V1; full configuration contract differs |
|Original025 material-preparation | Original failed19-world prefix | Historical failure, never the recommended current runner |

Two fresh actual023 executions under `target/workbench-interface-audit/` on
2026-10-22 produced identical3,416,858-byte histories, SHA256
`24e6046ec2af86d6bee7a3c2e3cb6a1b95459522f783f39192167f75e946b5be`.
The CLI and model source establish the exposed case list, ticks1–1024, period1–128,
u64 seed, fixed four-person setup and33-tick drain. Reporting is observer annotation,
not an agent reset. These are exploratory runs, not new registered evidence.

## Ownership and dependency boundary

`tools/workbench/` is an isolated Cargo workspace. New reusable configuration,
execution, storage, hashing, bounded history projection, reproduction, observations
and desktop UI belong in Rust. The root Cargo graph and frozen prototype hosts stay
unchanged. The workbench calls the existing Cargo entry point with an argument
vector, never a shell or a saved command. Typed saved configuration cannot select an
executable, output path, policy or arbitrary CLI option.

Every run reserves a new directory under `target/workbench-runs/`. Metadata records
source revision, dirty status, verified023 source-lock fingerprint, exact arguments,
process result and history hash. Configuration and initial metadata are written
before invocation; an absent completion record means incomplete. Reproduction uses
a fresh directory and compares actual history bytes. A changed revision is a warning,
not a claim of historical equivalence. Output remains separate from observer notes
and all scientific archives. The parser is an observer consumer, never actor input
or an independent scientific validator.

Python remains necessary for the existing023 Rust host generator at build time
(`generate_host.py` verifies inherited source locks). The Rust binary performs the
simulation. Existing independent scientific readers under `examples/support/` are
Python by design: they must not be replaced with calls to the simulation and then
called independent verification. `tools/history-explorer/explorer.py` and its
versioned adapters are ordinary read-only viewing tools; JavaScript renders the
detailed historical interface. No new Python infrastructure or adapter is added.

Typed public runner interfaces might later become a shared Rust library, once
several accepted consumers need them. Wholesale migration now would disturb source
freezes and duplicate independent readers without helping the first human workflow.
V1 instead accepts the existing build-time Python dependency and optional detailed
viewer dependency explicitly. Rust native summaries remain usable without the viewer.
