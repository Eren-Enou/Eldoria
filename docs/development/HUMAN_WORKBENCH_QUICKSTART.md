# World of Individuals — human workbench

The workbench launches real exploratory four-person worlds. It does not change
simulation rules, adopt research policies or turn a run into protected evidence.

## Launch

Requirements: a full Eldoria checkout, Git, Rust/Cargo1.95+ with a native C++ linker,
Python3.10+ on PATH for the existing prototype build host, and a desktop OpenGL
driver. The tested environment is Windows11, MSVC, Rust/Cargo1.99 and Python3.12.6.
Other desktops have not been accepted. Initial dependency download/build needs
Internet access and can take several minutes. Keep the checkout in its build location.

From the Eldoria directory, run in PowerShell:

```powershell
cargo run --release --locked -j 1 --manifest-path tools/workbench/Cargo.toml --features desktop
```

After building, double-click `tools/workbench/target/release/woi-workbench.exe`.
Keep Cargo, Git and Python on PATH; running the simulator may still need its host
build. A missing dependency is an execution error, not a completed world.

## First world

1. In **Run**, keep Continuous World023, `paired`, seed42, duration192 and period11.
2. Click **Run world**. Wait for completion; builds may take minutes on first use.
3. In **Results**, select the new run and click **Inspect**. Its directory is under
   `target/workbench-runs/<run-id>/`; **Copy run path** gives its exact location.
4. **History** shows an Observer Perspective summary and sampled actual decision
   inputs, activities and final states. The ordinary cutoff and33-tick drain differ.
5. Click **Reproduce in new directory**. A fresh result records whether the history
   hash matches. A source-revision warning qualifies exact historical equivalence.
6. For deeper inspection, **Start local History Explorer**, then open its loopback
   link. Expand **Choose a history and browse its timeline**, select the
   **023 · custom prototype** history (search for `023`), then click
   **Open selected history**.
   First catalog preparation may take up to3 minutes; later launches share a cache.
   The existing Python/JavaScript
   viewer is read-only; **Copy viewer command** provides a fallback. If port8766 in
   the copied command is busy, choose a free port. The automatic action chooses one.
7. Add a **Human observation**: person0–3, an actual run tick, context, activity,
   recorded outcome, category and notes. Saved JSON files live in this run's
   `observations/`. These are human interpretations, never the individual's thoughts.

Closing the workbench while a run is executing does not promise cancellation or
completion. Reopen Results and inspect its saved metadata and logs. Only a final
Completed record plus matching supported output counts as completed. An incomplete
run can be reproduced into a fresh directory; original partial output stays intact.

## Settings and limits

Available: the12 existing023 scenario names, any unsigned64-bit seed,1–1024 ordinary
ticks and reporting period1–128. Four people, Continuous scheduling, Epoch phase,
Ascending order, claim4, cadence8 and capacity2 stay fixed. No invented personality,
population, memory, resource or material-production sliders.

024 and025 are unavailable in V1 pending separate typed runner/reader acceptance.
For025 research source navigation, use the corrected
[observer2 successor](../../prototypes/material-preparation-observer2/README.md).
The original failed19-world host and its failure report remain historical evidence.
025 remains unadopted/unprotected; foundation protection ends at024.

Native inspection accepts one actual023 JSONL trial up to64MiB. It streams long
sequences and keeps bounded samples, not the entire historical state. The detailed
Explorer supplies full captured actor/observer navigation. Neither view may restore
forgotten cognition or infer motives from objective outcomes.

## Problems and CLI fallback

Report the run ID, source revision, chosen settings, exact observed tick/person,
expected versus recorded outcome and the relevant `stderr.log`. Preserve partial
history. Do not replace experiment archives with a workbench run. Human notes can
record plausible/questionable/unclear/possible-bug observations without changing policy.

The same executable provides `run [configuration.json]`, `list`, `inspect RUN-ID`,
`reproduce RUN-ID`, `viewer RUN-ID` (prints a command), and
`observe RUN-ID observation.json`. Without desktop dependencies:

```powershell
cargo run --locked --manifest-path tools/workbench/Cargo.toml -- run
```

An exploratory run cannot establish universal superiority, psychological realism,
consciousness or broad social emergence. Meals, contact and transfers are recorded
actions, not a global utility ranking. Notes and observer history never feed policies.
