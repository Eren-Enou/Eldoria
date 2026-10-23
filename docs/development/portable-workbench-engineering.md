# Portable workbench: engineering boundary and gates

Request: attachmenta5b07201. At audit start, HEAD and actual remote main were both
`477d35e6d7b7418b8117cce67115dfb14c240ec6`. Existing uncommitted V2 work is
preserved and reused. The starting V2 checkpoint was not a published release.
Foundation001–024 and the separate unadopted025 candidate remain unchanged.

## A: observed baseline

The V2 source contains strict023/024/corrected025 configuration, real execution,
hash-checked native projection, reproduction, comparisons, external observations,
and a single-source Python Explorer bridge. Its14 explicit backend tests and
217root tests passed; full foundation/candidate gates passed. V1's actual GUI
workflow passed. Final V2 GUI acceptance remains uncompleted; code alone is not
acceptance. Prior evidence is in `rust-workbench-v2.md` and ignored validation logs.

Relocated actual binaries were executed in an ordinary TEMP folder with spaces,
with PATH restricted to Windows System32. `where` could not locate Cargo, rustc,
Git or Python. The relocated workbench failed ordinary execution with
`Git is required for source identity: program not found`. The original023 runner
successfully produced a64-tick actual history (1,232,030bytes), SHA256
`8a4619c030cddc49072c76b03ab1910103c3258a422d43ad6597073a16033145`.
The source checkout still existed; this does not establish checkout-independent
execution or clean-machine acceptance. Exact paths/results are in
`target/portable-workbench-validation/baseline-relocation.json`.

The relocated original024 MealRelief run also succeeded. Corrected025 failed its
Python preflight with a missing-program error. These are actual executions, not
only inferred dependencies.

Source inspection establishes further dependencies: the workbench resolves a
compile-time checkout path, checks source manifests, invokes Git and launches
Cargo.023/024 ordinary runners do not need Python after compilation, but their
CLI output guards canonicalize the compile-time checkout. Historical replay and
corrected025 preflight still invoke Python. Native summary reading itself needs
no Python. These boundaries must be resolved, not silently disabled.

## B: additive execution design

Preserve all original producers, generation scripts, laws and archives. A small
separate Rust portable runner will link the unchanged activity-valuation library,
which exposes both inherited023 and024 trial functions. Its existing source-lock
verification/Python host generation remains a developer build step. Prove exact
ordinary023/024 serialized-history parity against their original executables
before accepting that reuse. No simulation implementation is copied or rewritten.

Replace only the development CLI transport/output dependency with an explicit
portable contract: typed configuration; fixed supported modes; fresh output under
a validated user-data workspace; no links/reparse points, traversal, installation
writes, or scientific `experiments`/`reinforcement` destinations. Release identity
and binary hashes replace runtime Git/source-checkout discovery. This is an
additional safety boundary, not removal of the original guard.025 stays available
in development mode but unavailable in portable mode unless its full preflight can
be preserved; do not bypass it.

The existing desktop will detect an installed manifest beside its executable,
verify fixed packaged runners, and use the same configuration/history/observation
logic. Missing or mismatched portable assets fail explicitly, with no Cargo
fallback. Store portable histories/notes outside installation in user application
data (or an explicitly selected safe workspace). Preserve old development runs.

Native inspection will add bounded recorded-moment selection and four actor lanes
to existing projected inputs/execution/consequences, without inventing cognition
or rendering all events at once. Python Explorer remains optional/developer-only.

## Acceptance and publication

Before packaging: original/portable exact parity across cases, seeds, boundaries,
short/long runs; required source/evidence checks. Then produce an actual versioned
Windows ZIP with executable(s), truthful build/source/lock/checksum manifest,
quickstart and required notices. Check relocation in two paths including spaces,
tool-free child environments, offline operation and missing/corrupt runner errors.
Perform the actual native walkthrough, including reopen and saved notes. Qualify
isolation precisely if no genuinely clean VM is available. No claimed release
until files and acceptance exist. Keep logs/measurements outside scientific archives.

## Independent transport and native reconstruction

The initial portable transport passed36 exact byte comparisons against original
023/024 executable output: all12 public scenarios,023 and both024 modes, varied
seeds including u64 maximum,1/64/192/1024ticks and1/11/128reporting periods.
All external opportunities/configuration fields match. This establishes transport
parity for those cases; it is not new scientific evidence or statistical replication.
Detailed hashes/configurations: ignored `target/portable-workbench-validation/parity.json`.

Native moment navigation streams each boundary into four compact action lanes,
then streams the selected decision/linked commitments and same-person temporal
resource context. It does not deserialize a full-history JSON graph. A read-only
independent test against a genuine024 run checks every lane and a late choice
beyond the old first32 sample. Context events are labeled as temporal context;
only commitment source/participant joins are claimed as execution links.
Window details load in a worker and reject stale run/person/moment results.
Native schema reading remains an observer projection, not a replacement validator.

Portable storage forbids source/evidence checkouts and installation descendants,
including archive names, traversal and reparse ancestors. Default user data is
under LOCALAPPDATA; explicit safe workspaces work without LOCALAPPDATA. Packaged
binary hashes, source locks and embedded build identity are recorded separately
from scientific outcomes. Checksums are not publisher authentication.

Saved annotations now load after reopening, checking the recorded run/history
binding. Older annotation files with no suggested-improvement field remain
readable. Notes never enter simulator configuration or actor state.36-case parity,
legacy exact reproduction, annotation recovery/misbinding, corrupt/missing release
and unsafe output tests are separate engineering checks.

## Executed acceptance

The static-CRT Windows desktop and runner were built and packaged. PE import
inspection found Windows system DLLs and OpenGL for the desktop, and only system
DLLs for the runner; neither imports the Visual C++ redistributable DLLs. The
manifest records actual compiler, target/features, release profile, Git revision,
dirty status, source locks, application source fingerprint and both binary hashes.
Runtime verifies those bytes and the application's embedded identity. Missing or
changed assets fail explicitly; there is no Cargo/source fallback.

The complete ZIP was extracted into two ordinary TEMP paths, including spaces.
Actual application execution/reproduction succeeded from both. The Rust package
test additionally relocates executables to two fresh installations, clears the
child environment except Windows necessities, and uses System32-only PATH.
An explicit workspace works with LOCALAPPDATA absent. Run, inspection, exact
reproduction, note save/recovery and installation non-writing pass. Missing or
corrupt runner/manifest, unknown configuration and unsafe installation output
fail. A real Windows junction workspace was rejected before execution.

Using the computer-use skill, the actual extracted native application was
launched with System32-only PATH and an external workspace. The seed was changed
to43, a world executed, moments0/1 and persons0/1 inspected, and actor-local
alternatives distinguished from actual Send/Reply/resource execution. The result
had485completed activities,10meals,183resource events,32transferred units and a
balanced food ledger; final food1/1/0/0 and hunger100 persisted into final state.
Person1's Invite and scene2's later Accept/one-unit transfer were inspected,
without attributing all nearby events to that choice. A bound human note was
saved, exact reproduction run, the window normally closed, and the app reopened.
Both runs reappeared; the original note loaded into the form with person1/tick6.
Both histories remained SHA256
`da8611ce53eb8b51db17c500a6e94e4cdde443107388c905ca1e0f8b78332230`.
This was real GUI interaction, not an instrumented replacement for user actions.

Evidence: ignored `target/portable-workbench-validation/gui-acceptance.json`,
`gui-recovered-note.txt`/`.jpg`, `preflight-package-tests.log`, `static-parity.log`,
`extracted-first-run.json`, `extracted-first-replay.json`, `junction-rejection.log`
and `pe-imports.json`. The GUI used the preflight executable; final clean-source
identity rebuilding is followed by fresh package/parity checks. No simulation or
desktop behavior is changed between those checks.

The checkout remained on the host. No clean Windows VM was available, and this
non-administrator session could not install a per-application firewall block.
Network connectivity remained enabled. These are actual relocation/tool-free
PATH tests plus source/import inspection, not independent clean-machine or
firewall-isolated offline certification. Ordinary installed paths contain no
runtime build, download, Git, Python or network step. Windows system graphics
support remains required. No Linux/macOS portability is claimed.

## Checks and preserved failures

- Root formatting, locked all-target Clippy with warnings denied and217tests pass.
- Full read-only `validate_contract` passes through024:408protected artifacts,
  425raw originals, scientific anchors and all required exact replays unchanged.
- Separate `validate025_acceptance` passes214custody bindings and50exact replays;
  original025 evidence, failed host and unadopted/unprotected status remain intact.
- Workbench all-target desktop/portable Clippy and runner Clippy pass. All19
  explicitly enabled workbench tests pass, including every public023/024/025 case,
  schema/score/hash controls, developer single-source Explorer, native independent
  reconstruction, installed runtime checks and36original/portable byte comparisons.
  Defaults remain023;024/025 policies remain explicit. No behavior/capacity/canon
  changes, foundation extension, frozen-output regeneration or026.

Initial failures were retained in engineering logs: V2 runtime Git dependency,
025 missing Python preflight, a temporary-directory collision fixed in tests,
and Windows installer-name elevation heuristics affecting a test executable
renamed `package_runtime`. Packaging review caught omitted optional desktop
dependencies and bundled-font notices. The corrected packager enables actual
desktop features, inventories220dependencies and requires the eframe/egui/winit
and font notices. The original incomplete preflight ZIP is not the distributable.
No first-party redistribution license is declared in the repository; the package
does not invent one. Third-party expressions and copied notices accompany it.

## Measured observer cost

Wall-clock results, cached local filesystem, release build, no standardized
machine/throughput claim:

| Recorded024 run | History | Native inspection incl hash | Moment index | Selected late detail | Serialized index |
|---|---:|---:|---:|---:|---:|
|1tick|124,398B|3.1–4.5ms|4.7ms|3.6ms|6,655B|
|1024ticks|18,588,739B|166–183ms|289ms|293ms|566,691B|

The long index contains1,057boundaries; its selected late detail is boundary1023.
The bounded summary serializes to74,625B. These are serialization sizes, not RSS
or allocator measurements. Moment indexing uses four streamed passes; selected
detail uses five. Storage grows with bounded recorded activity, not a duplicated
full cognitive graph. Selected detail loads asynchronously with stale-result
rejection and visible loading/error state. The GUI's actual192-tick process took
225ms initially and68ms for replay; these exclude application preflight/parsing.
Cold developer compilation took7m33s for the desktop and4m25s for the runner;
end users do not compile. Timings stay out of deterministic histories. The
simulator is unchanged, so no new simulation-throughput claim or optimization
pass is made. See `reader-bounds-cost.log` for raw engineering measurements.

## Reproducible release and handoff

Canonical local artifact:
`target/releases/woi-workbench-0.2.0-rc.1-windows-x64.zip`.
Its extracted directory contains the two executables, `release.json`, quickstart,
notice inventory and copied upstream notices. Final byte counts/SHA256/source
revision and publication verification are recorded in ignored
`target/portable-workbench-validation/final-publication.json` and adjacent
`target/releases/SHA256SUMS.txt`. A fresh clone can rebuild instead of relying on
those local ignored files. Runtime user data is separate from all release files.

From a developer Windows checkout with the locked dependencies available:

```powershell
$env:RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release --locked --manifest-path tools/workbench/Cargo.toml --features desktop,portable --target-dir target/portable-static -j 1
cargo build --release --locked --manifest-path tools/portable-runner/Cargo.toml --target-dir target/portable-static -j 1
Remove-Item Env:RUSTFLAGS
New-Item -ItemType Directory -Force target/releases | Out-Null
cargo run --release --locked --manifest-path tools/workbench/Cargo.toml -j 1 --example package_portable -- target/portable-static/release/woi-workbench.exe target/portable-static/release/woi-portable-runner.exe target/releases/woi-workbench-0.2.0-rc.1-windows-x64
$env:WOI_PORTABLE_PACKAGE=(Resolve-Path target/releases/woi-workbench-0.2.0-rc.1-windows-x64).Path
$env:WOI_PORTABLE_RUNNER=(Resolve-Path target/portable-static/release/woi-portable-runner.exe).Path
cargo test --locked --manifest-path tools/workbench/Cargo.toml -j 1 --test package_runtime -- --ignored
cargo test --locked --manifest-path tools/workbench/Cargo.toml -j 1 --test transport_parity -- --ignored
```

The packaging directory/ZIP must be new; preserve older artifacts rather than
overwriting them. Build-time Python/source verification remains a developer
dependency. The portable runner deliberately supports023/024 only;025 requires
its complete established preflight. Developer Explorer remains an advanced
single-source bridge and is absent from portable execution.

The next useful work is independent hands-on observation. Recurring decisions
that remain questionable after local-input/execution inspection and exact replay
could justify a separately preregistered behavioral investigation. A more
accessible interface alone establishes no new cognition, broad emergence,
optimal policy, psychological realism or scientific adoption.
