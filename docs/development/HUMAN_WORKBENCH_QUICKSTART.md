# World of Individuals — Workbench V2 quickstart

For the compiler-free Windows ZIP, use the
[portable quickstart](PORTABLE_WORKBENCH_QUICKSTART.md). This page describes the
developer checkout workflow, including025 and the optional Python Explorer.

Run and inspect real four-person exploratory worlds. The workbench does not
change simulation rules or turn an exploratory run into protected evidence.
Foundation protection ends at024; corrected025 remains unadopted research.

## Start

Close an earlier workbench before rebuilding so Windows can replace its executable.
From the Eldoria directory in PowerShell:

```powershell
cargo run --release --locked -j 1 --manifest-path tools/workbench/Cargo.toml --features desktop
```

Requirements: full checkout, Git, Rust/Cargo1.95+ with a native linker, Python3.10+
on PATH for existing prototype build hosts and detailed Explorer, and OpenGL.
Tested environment: Windows11/MSVC, Rust/Cargo1.99, Python3.12.6. Other desktops
are not accepted. Initial builds need Internet access and may take minutes.
After building, double-click `tools/workbench/target/release/woi-workbench.exe`;
keep the checkout in its build location and dependencies on PATH.

## Run, inspect and reproduce

1. In **Run**, choose **024 Activity valuation**, `Baseline` or `MealRelief`,
   `paired`, seed42, duration192, reporting period11. Click **Run world**.
2. Wait for **Completed** in **Results**. Builds and diagnostics are saved in
   a unique `target/workbench-runs/<run-id>/` directory; **Copy run path** locates it.
3. Click **Inspect**. **History** shows recorded outcomes, final inventories and
   hunger, bounded individual decisions, valuation terms and execution records.
   Select a person, expand **Explain boundary…**, or inspect resource/meal records.
4. Click **Inspect detailed history**. After verification, the exact selected
   custom run opens automatically in your browser; no catalog search is required.
   If the browser does not open, use **Open local read-only Explorer**. Actor/Observer lenses, person
   selection and recorded tick navigation inspect existing history, never rerun it.
   **Copy viewer command** provides a PowerShell fallback; choose a free port if8766
   is occupied. Automatic startup chooses an available loopback port.
5. Return to **Results**, click **Reproduce in new directory**. A fresh run records
   the exact history-hash comparison. Source-revision warnings qualify equivalence.
6. To compare, select the left run in **Results**, open **Compare**, choose a second
   completed run and **Compare verified histories**. Read actual configurations,
   their differences and observer quantities side by side. Matching is conservative:
   only valuation may differ; other configuration, runner identity and recorded
   initial state must match. Other pairs are labeled **Recorded difference**.
7. **Human observation** saves person/tick, context, activity, outcome, category,
   notes and optional suggested improvement in the run's separate `observations/`.
   **Use for observation** links a captured decision. Notes never feed cognition.

## Available modes and bounds

| Mode | Actual controls | Status |
|---|---|---|
|023 Continuous World|The12 original cases, u64 seed,1–1024 ordinary ticks, period1–128|Preserved V1 default|
|024 Activity valuation|Same world controls; Baseline or MealRelief|Existing research policies; no adoption|
|025 Corrected preparation|Same world/valuation controls; KnownRaw, UnobservedRaw or NoAccess; raw0–16; effort2+4,4+8 or8+24|Corrected observer2 research; unadopted/unprotected|

All modes keep four people, Continuous scheduling, Epoch phase, Ascending order,
claim4, cadence8, social capacity2 and a33-tick final drain. Reporting is observer
annotation. No population, cognitive-capacity, personality or scoring sliders.
MealRelief adds `min(H,40)` to eligible Consume's existing `H` score; it is unchanged.

025 presets use an8-tick observation cycle: KnownRaw senses raw at0 and ready at1;
UnobservedRaw senses only ready at1; NoAccess has the same observations as KnownRaw
but forbids preparation. These use the existing single-slot semantics, including
raw/ready overwrite. NoAccess is not an old-world parity control. Preparation uses
2raw →1food +1waste. Failures retain full effort; escrow is owned and unavailable
for meals/transfers. Food is fungible: a later meal cannot identify its prepared unit.

Native readers accept one authentic023/024/025 trial up to64MiB and cap historical
sequences at65,536 records. They retain first32 samples plus aggregate counts and
final state. Missing sampled links are **Unknown**, not proof of nonexecution.
Use detailed Explorer for full records. Selection, consent, admission and execution
are distinct. More meals or transfers does not establish universal benefit.

## Files, errors and CLI

Each run saves configuration, source revision/fingerprint, fixed command arguments,
stdout/stderr, history hash and completion metadata.025 additionally saves its
actual typed `runner-config.json`. Old unversioned V1 JSON keeps its023 meaning;
new modes require `schema_version:2` and explicit mode-specific options. Unknown
fields and irrelevant mode options are rejected. Example configurations live in
`tools/workbench/configurations/`.

```powershell
cargo run --release --locked -j 1 --manifest-path tools/workbench/Cargo.toml -- run tools/workbench/configurations/024-meal-relief.json
```

The executable also provides `list`, `inspect RUN-ID`, `reproduce RUN-ID`,
`viewer RUN-ID` (prints the selected-source command), `compare LEFT-ID RIGHT-ID`,
and `observe RUN-ID observation.json`. Native inspection needs no viewing server.

An interrupted/failed run remains **Incomplete**; inspect its logs and reproduce
into a fresh directory. Normal closing waits for active work. Forced termination
has no cancellation/resume guarantee. Preserve partial files and report the run
ID, source revision, person/tick, expected versus recorded outcome and stderr.
Never replace scientific archives with exploratory output. Run folders, notes and
derived viewer cache grow with use; no automatic cleanup is provided.
