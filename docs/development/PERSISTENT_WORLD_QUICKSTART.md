# World of Individuals — persistent world sessions

Version 0.3 engineering candidate. Final native/portable acceptance and publication
are pending; use the accompanying engineering report for the current status.
The protected001–024 experiments and separate025 research remain unchanged.

## Start the application

In a validated portable folder, open `woi-workbench.exe`. Keep `release.json`,
`woi-portable-runner.exe` and the other packaged files beside it. Supported native
operation uses Rust binaries and Windows system graphics, with no runtime Cargo,
Python, Git, checkout or network requirement. This is not clean-machine acceptance.

For a developer build from the repository:

```powershell
cargo run --release --locked --manifest-path tools/workbench/Cargo.toml --features desktop --target-dir target/persistent-native -j 1
```

Developer compilation still uses the existing Python source-host generator.
That build dependency is separate from installed application operation.

## Create, advance and save

1. Choose **World Sessions**. The existing Run/Results/History/Compare pages remain
   available for finished exploratory runs.
2. Choose 023 Continuous, a scenario, a deterministic seed, nominal horizon 1–1024
   and report period 1–128. 024 Baseline and MealRelief are explicit research modes;
   neither changes the global default. 025 persistence is not supported.
3. Choose **Create world**. This executes the established fixture bootstrap and
   saves its actual initial continuation state. Actual tick can therefore begin
   above zero; elapsed time starts at zero. Four individuals and finite food are
   preserved from that fixture.
4. Use **Start / Advance 1**, **Advance 16**, **Advance 64** or **Advance 256**. Counts
   are scheduler boundaries. An indivisible inquiry can span multiple physical
   ticks; the displayed actual clock records that cost.
5. **Pause** requests a stop after the current atomic operation. It does not run an
   extra tick, settle unfinished work, refresh knowledge or change anyone's needs.
6. Choose **Save checkpoint** and check the success message. The previous valid
   generation remains on disk. Pending contact, active work, deadlines, inventories,
   memories, trust and learning survive; observer history does not refill cognition.

New-world settings apply when creating a world. The current overview separately
shows the open world's saved configuration and actual state.

## Close, reopen and resume

After a successful save, close the application normally. Closing a paused world
also attempts a save and blocks closing if saving fails. If execution is still
running, closing requests a pause; close again after its safe-boundary response.

Start the same compatible application, choose **World Sessions**, select the saved
world ID and choose **Reopen world**. Verify the actual tick and unfinished work.
Advancing now resumes that state; no bootstrap or tick-zero replay occurs and time
spent with the application closed does not advance the world.

**Close saved world** saves and releases its writer lock without exiting the app.
Use this before reopening that same world in the current application or accessing
it through another process. Only one Session writer may own a world directory.
The remaining `session.lock` file is normal; OS handle ownership is the lock.

## Inspect what happened

The current four-person overview is objective observer state. The recorded-boundary
slider contains only executed history. Select an individual to inspect the exact
captured local input, alternatives and selection, with linked physical transitions.
Selection, consent/admission and material execution are distinct.

**Inspect completed step record** opens the actual journal record for that step,
including recorded responses, resource operations and information delivery. Its
observer audit is labeled separately from what an individual knew. Viewing it
does not change the world or provide knowledge to any actor.

At the fixed nominal cutoff, ordinary new choices cease. Advance through the
existing 33 ticks of charged drain until the world is terminal and settled. Then
**Finalize terminal world** writes the ordinary complete `history.jsonl` export.
Finalizing does not secretly execute or prematurely settle the simulation.

## Files and failures

Portable default user data:
`%LOCALAPPDATA%/WorldOfIndividuals/Workbench/runs/world-sessions/`.
Developer default: `target/workbench-runs/world-sessions/` in the repository.
The overview shows the exact current world directory. An explicit absolute
`--workspace` can select another approved user-data root for installed operation.

`journal.jsonl` records completed steps. Numbered `.cbor` checkpoint generations
contain actual continuation state and journal linkage. They serve different
purposes. `.pending` files are incomplete writes, retained as diagnostics; retry
can publish a later unused generation without overwriting the incomplete file.

Unknown versions, mismatched runtime identity, corrupt bytes or a mismatched
journal are rejected. A journal tail beyond the last committed checkpoint is
preserved and reported as unsaved/interrupted progress. Automatic tail recovery
and checkpoint migration between different runtime identities are not supplied.
Keep the saved files and diagnostic message; opening never falls back to a fresh
world. Do not claim power-loss durability from these ordinary restart checks.

This remains a finite four-person world with finite resources and capped hunger.
Pausing and resuming do not establish sustainable life, population scaling or
civilization. No Experiment 026 is included.
