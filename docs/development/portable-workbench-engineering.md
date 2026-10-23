# Portable workbench: engineering boundary and gates

Request: attachmenta5b07201. Base HEAD and actual remote main both
`477d35e6d7b7418b8117cce67115dfb14c240ec6`. Existing uncommitted V2 work is
preserved and reused; do not describe it as an already published release.
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

## Independent transport and native reconstruction (in progress)

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

Final static-runtime build, ZIP acceptance and actual portable GUI walkthrough are
pending. No clean-machine or offline-firewall isolation has yet been demonstrated.
