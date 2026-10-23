# World of Individuals — portable Windows workbench0.2

Extract the complete ZIP into an ordinary writable folder, then double-click
`woi-workbench.exe`. Keep `release.json` and `woi-portable-runner.exe` beside it.
Windows x64 is required. Normal operation needs no Rust, Cargo, Git, Python,
source checkout or internet. No installer or administrator privileges are needed.
This is a local evaluation build; publisher signing and clean-VM certification
are separate from checksum identity.

## Run and observe

1. On **Run**, choose023 Continuous World or024 Activity valuation. In024 choose
   Baseline or MealRelief explicitly. Change the seed, scenario or duration, then
   select **Run world**. The four people make their own bounded choices.
2. Open **Results**, select the completed run, and select **Inspect**.
   **History** distinguishes the observer's recorded outcomes from actor-local
   inputs. Use **Previous moment**, **Next moment**, or the recorded-boundary
   slider; choose a person in the four lanes. Playback never reruns the simulation.
3. Read the chosen activity, actual candidate scores, captured local information,
   linked commitments and execution status. Nearby resource/meal records are
   explicitly temporal context, not automatically proof of causation. A missing
   choice or execution link stays unknown. Final food/hunger and ledger totals
   describe persistent recorded consequences.
4. Select **Annotate selected decision**, enter your human interpretation under
   **Human observation**, then **Save human observation**. Notes never feed back
   into the people. Saved notes can be loaded into the form after reopening.
5. In **Results**, select **Reproduce in new directory**. A new run is saved; check its
   **Reproduction hash match** result. An exact match concerns recorded history,
   not scientific acceptance or proof that a policy is better.
6. Close and reopen the app. **Results** finds the same runs; inspect a run to
   recover its saved observations. **Compare** reports conditions and quantities
   conservatively; more meals or transfers is not a universal utility ranking.

## Your files

Default storage is `%LOCALAPPDATA%\WorldOfIndividuals\Workbench\runs`.
Histories, configurations, source/release identity, process logs and observations
are saved together in distinct run directories. Copy the entire user-data folder
for backup. Extracting a newer application elsewhere does not erase it.

An optional leading `--workspace C:\absolute\UserData` selects another safe
user-data directory. It must be outside the extracted installation, source
checkouts and scientific archive paths, without symlinks or reparse points.
The app does not write into its installation or packaged evidence. Do not move
active run directories while the app is working.

## Limits and recovery

Only023 and024 are packaged.025 remains available in the developer workbench with
its original source/Python preflight; it is deliberately unavailable here. The
existing Python History Explorer also remains a developer tool. Native inspection
is included and does not start a browser or server. No026 has been started.

Missing, changed or mismatched packaged executables/manifests cause an explicit
failure. Re-extract the complete ZIP; there is no compiler or network fallback.
Checksums detect differences from the manifest, not an adversary who replaces
both executable and manifest. Runs interrupted by process failure remain
**Incomplete**; inspect their saved logs. A partial history is not a completed run.
The app prevents closing its own window while an operation is running.

Native readers accept authentic supported history schemas up to64MiB, with
bounded record counts. The compact lane index is retained; individual details
are streamed on demand. Richer Python Explorer analysis and independent scientific
validators still require the developer checkout. Existing001–024 scientific
evidence and the separate025 candidate are not adopted or rewritten by this app.

See `portable-workbench-engineering.md` in the source repository for actual
acceptance results and their isolation limits. License inventory and notices are
included in the release directory.
