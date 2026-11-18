# Eldoria Settlement Observer

A native desktop window onto the autonomous settlement. Watch people gather, eat,
travel, help and refuse; follow the recorded reasons and actual consequences.
The [settlement engine](../settlement/README.md), its rules and its save format are
unchanged. Research hosts, the History Explorer and the old workbench are untouched.

## Launch

From the repository root, with Rust 1.95+ and a working desktop/OpenGL environment:

```powershell
cargo run --release --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer
```

After building, run `target/settlement-observer/release/eldoria-observer.exe`
directly. No Cargo, Python, browser, inference service or network is used by the
running app. Dependencies may download during the first build. Actual acceptance
was on Windows x86_64; other desktop platforms and clean-machine packaging are not
claimed by this delivery.

Optional launch arguments, placed after `--` when using Cargo:

```text
--open ABSOLUTE-CHECKPOINT.json
--workspace ABSOLUTE-SAVE-DIRECTORY
--seed 42 --population 12 --horizon 512
```

The default starts eight people, paused, at tick 0 with a finite 512-tick horizon.
Use **New settlement** to choose seed, 8/12 people and a finite horizon. New/Open
never auto-run. **Run**, **Pause** and **Step** control logical time; **Pace** changes
wall-clock playback speed only. There is no editing of characters or forced action.

## Inspect a living history

- The location map shows the three places, visible people and food observations.
  Travel completes at arrival; the map keeps a traveller at its departure place
  while its activity describes the journey.
- **Follow a character** or click a person. **Observer** shows global inventories,
  needs, source stocks and commitments. **Character** shows only the followed
  person's own state, dated retained beliefs, co-present public signals and own work.
  Distant source figures are labeled “Last saw”; unknown stock stays unknown.
- **Trace decision** opens the actual deciding record. Expand a candidate to inspect
  named utility terms; follow related causes and consequences. The deciding/subject
  person and recorded tick are explicit, separately from the current character card.
- **Recorded history** is paged at 30 rows. Selecting history does not rewind the
  world or restore forgotten knowledge. Character mode can inspect recorded local
  inputs but cannot traverse into foreign private decision inputs. Shared outcomes
  are available through retained receipts; full causal context requires Observer.

The global follow selector is external observer navigation, not knowledge granted
to a character. Perspective filtering happens in the worker projection as well as
the UI. Neither history selection nor any frame is an input to agent cognition.

## Save and reopen

**Save checkpoint** pauses at a safe boundary, then saves complete world state to
a fresh filename. The default save folder is
`%LOCALAPPDATA%/Eldoria/settlement-observer` (a temporary-directory fallback if that
environment variable is absent). You may type another path; missing parent folders
are created only after the engine's safe-output checks. Existing files are never
overwritten. **Open** takes a checkpoint path, not a viewer export. A failed open
leaves the current world intact and paused, with the error displayed.

Native Close/New/Open protects unsaved progress and unresolved operations. Choose
**Save first**, explicitly discard unsaved work, or cancel. Save-before-close waits
for successful publication before closing. Closing does not advance logical time.
An abrupt process termination can lose uncommitted in-memory progress; committed
files keep the engine's existing integrity guarantees and limitations.

Headless CLI saves and observer saves are interchangeable at the same engine
identity. The actual native workflow reopened four pending meals after an application
restart, completed them on the next tick and produced byte-identical original CLI
checkpoints. The app does not loosen the engine's strict runtime/schema validation
or migrate older incompatible development saves.

## Ownership and checks

| Module | Owns |
|---|---|
| [host.rs](src/host.rs) | One ECS owner thread, bounded command queue, safe time/session controls, non-dropping acknowledgments and a replaceable latest-frame slot |
| [projection.rs](src/projection.rs) | Explicit observer/character payloads, authorization of record details/causal links, bounded history pages and incremental causal-reference index |
| [ui.rs](src/ui.rs) | Map, people, inspector, pacing, file dialogs, unsaved-work handling; no world mutation or policy access |
| [main.rs](src/main.rs) | Native window startup and optional launch arguments |

[Boundary contract](design.md), [acceptance report](../../docs/development/settlement-observer-acceptance.md),
[native receipt](../../docs/development/settlement-observer-validation.json),
[projection measurements](../../docs/development/settlement-observer-performance.json).

```powershell
cargo fmt --check --manifest-path products/settlement-observer/Cargo.toml
cargo clippy --locked --all-targets --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer -- -D warnings
cargo test --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer
cargo run --release --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer --example measure
```

Tests cover 8/12-person continuity under view changes, causal links, character-frame
privacy including link traversal, save/open failures and worker run/pause/step with
discarded presentation frames. Native interaction is accepted separately from tests.

## Current limits and next work

This is a finite scenario, not a sustainable community. Food eventually runs out;
null decisions, costly requests, refusals and failed work remain visible. The app
uses text paths rather than an OS file picker, has no recent-save catalog, and
does not persist UI selection or human notes in world saves. It is a source-built
desktop, not an installer or a hosted service. Historical detail is observational,
not state scrubbing or arbitrary restoration.

Frames contain bounded pages/one selected detail, not the full checkpoint history.
Projection construction still scans history for filtering/outcome counts; complete
checkpoints still grow with the world. See measured costs before extending the
horizon/population. GPU frame time, prolonged UI soaks and larger cohorts remain
unmeasured. Next useful work is observation-driven usability: significant-event
navigation, bookmarks/notes kept outside cognition, and better history indexing.
New character goals, renewable resources or institutions require separate product
rules and validation; none are silently added by this viewer.
