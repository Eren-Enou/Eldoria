# Persistent worlds: current headless workflow

Explicit025 material continuation is validated headlessly in a separate host; see
[persistent material contract and acceptance status](persistent-material-world.md).
The existing workbench commands below continue to use the023/024 reader. The new
host does not enable025 in the GUI or an installed release.

The recovery/storage backend can be built and tested without desktop access.
GUI Pause/Finalize/024 acceptance and a new Windows release remain **DEFERRED**.
Do not infer GUI acceptance from the commands below. The earlier GUI guide follows
for future authorized use; no desktop interaction is required for this milestone.

Build the Rust command-line workbench in a developer checkout:

```powershell
cargo build --release --locked --manifest-path tools/workbench/Cargo.toml --features world-sessions -j 1
$wb = '.\tools\workbench\target\release\woi-workbench.exe'
& $wb world-create # default bounded023 world; JSON returns its actual id
# Or explicitly: & $wb world-create .\tools\workbench\configurations\024-baseline.json
& $wb world-advance WORLD-ID 3 --codec zlib
& $wb world-storage WORLD-ID --budget-bytes 100000000 --max-generations 20 --project-saves 10
& $wb world-check WORLD-ID
& $wb world-verify WORLD-ID --generation 2
```

Use the JSON `id`/`path` returned by creation, and the actual generation shown by
inspection. `world-advance` executes actual steps and saves; atomic inquiry can
span multiple physical ticks. Omit `--codec zlib` for the unchanged raw default.
`world-check` verifies complete committed state; `world-verify` independently
checks raw metadata, hashes, order and primitive physical consistency. Both report
JSON. A tail can leave a valid checkpoint but prevents ordinary continuation.

For an interrupted world, keep its original directory. Inspect its per-generation
errors, then select a verified generation explicitly:

```powershell
& $wb world-recover WORLD-ID 'C:\NEW-BRANCH-ROOT\WORLD-ID' --generation 2
& $wb world-advance 'C:\NEW-BRANCH-ROOT\WORLD-ID' 2048 --codec zlib
& $wb world-finalize 'C:\NEW-BRANCH-ROOT\WORLD-ID'
& $wb world-verify 'C:\NEW-BRANCH-ROOT\WORLD-ID' --generation 3
```

Replace the example paths with fresh absolute destinations and the exact existing
world basename. Never reuse a destination. Recovery is a new execution branch
from saved state, not restoration of the unsaved endpoint. Excluded history remains
separate evidence; event IDs must be interpreted within their branch. If the
world is not terminal, continue actual execution before finalizing. Finalize
never advances it. `world-interrupt WORLD STEPS` deliberately exits without saving
for disposable failure demonstrations only; it leaves genuine unsaved events.

No generations are automatically deleted. Warnings are external and do not alter
actor behavior. Storage sizes are logical file bytes, not heap usage. See the
[recovery contract](persistent-world-recovery.md) for lineage, limits, supported
v1 compatibility and refusal conditions. This source build is not a new portable
release; existing installed binaries do not acquire the new commands automatically.

---

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
