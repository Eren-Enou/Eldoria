# Headless persistent-world reliability

Headless engineering milestone validated, starting at9742295. Final validation,
measurements and limitations follow the implementation record below. This supersedes the previous
desktop acceptance task for current work only. Native Pause/Finalize/024 acceptance
and a GUI-dependent Windows release are DEFERRED. No desktop control is authorized.
Protected001–024, all025 research and behavioral rules remain unchanged.

## Failure audit before implementation

The original Session appends and syncs each completed scheduler iteration before
its next checkpoint. Save serializes the actual complete world, binds its SHA256
to the journal prefix SHA256, writes a new `.pending` generation with `create_new`,
syncs/readbacks/restores it, then renames it to `.cbor`. Earlier generations remain.
Open locks the session and chooses the lexically last committed filename; it never
tries an older checkpoint after failure. Numeric ordering beyond eight digits is
therefore an identified storage-reader defect to correct without changing physics.

| Interruption | Existing artifacts and behavior | Required successor behavior |
|---|---|---|
|After journal append, before save|Earlier valid state plus genuine unsaved tail; open refuses|Inspect and explicitly branch from selected verified state; preserve tail|
|During serialization, before file creation|Earlier files intact|Recover the selected earlier state without inventing later private state|
|During `.pending` write|Earlier commit plus incomplete file|Report pending file; never promote it|
|After pending sync/readback, before publication|Complete but unpublished candidate|Still not committed; preserve evidence|
|Immediately after publication|New immutable commit exists|Verify its complete state and exact prefix|
|Newer committed file corrupt|Existing open fails; earlier files remain|Report exact failure; require explicit generation selection|
|Normal/abnormal process exit|OS releases writer handle; lock file remains|Real process tests must establish this; a file alone is not lock ownership|

These are ordinary process-interruption semantics. File sync plus rename does not
establish filesystem-directory or sudden-power-loss durability.

## Planned safety boundary

Read-only inspection must not create a lock file or repair data. An existing writer
makes the snapshot potentially unstable. Recovery requires exclusive source and
destination ownership, an explicit numeric generation, a fresh destination, strict
state/prefix/source validation and non-destructive lineage/evidence records. No
default rollback, tail replay, bootstrap or guessed cognition is permitted.

Continuation world identity is distinct from external branch identity. The first
implementation may require the destination basename to retain the original world
ID beneath a fresh branch root, preserving all existing checkpoint and journal
namespaces byte for byte. The absolute branch path and lineage identify the new
execution; they never enter actor inputs. Recovery evidence records the selected
checkpoint, its actual tick, original journal hash, exact prefix, discarded-from-
continuation tail bytes and their hashes. The original source remains unchanged.

Storage inventory/warnings are external observations. No automatic deletion or
simulation termination is allowed. Compression adoption requires real size/time
measurements and exact decoding, with a versioned opt-in container and supported
v1 reading. Strict source identity must not be weakened to accept old checkpoints;
any compatibility mapping must explicitly pin the unchanged kernel and dependency
set and be tested with original v1 bytes.

## Initial implementation and interface

`session::reliability::inspect(path, budget)` reports generation validity/errors,
numeric newest/oldest valid generations, actual journal size and each verified
prefix length, pending files, lock availability, logical retained bytes and budget
warnings. A busy or unavailable existing lock explicitly prevents a stable-snapshot
claim. Inspection does not create a missing lock. `local_recovery_branches` counts
lineage records in the selected directory and immediate children; it cannot discover
successors stored under arbitrary other roots. Logical file sizes are not allocated
filesystem blocks or live heap. Projections assume unchanged state and format.

`recover(source, fresh_destination, explicit_generation)` preserves the original
directory, saves excluded evidence, publishes `recovery.json` lineage and copies
the selected checkpoint unchanged with its exact committed journal prefix. It
validates restored state equality before returning success. A corrupt newest file
does not authorize rollback: the generation argument remains mandatory. Nested
destinations inside the original world are refused. It is a new execution branch,
not repair of the unsaved endpoint. Do not merge its later reused event counters
with the separately preserved parent's tail; references are branch-qualified.

Initial standalone Rust CLI syntax (developer checkout, no GUI):

```text
woi-persistent-world check ABSOLUTE-WORLD-DIRECTORY
woi-persistent-world storage ABSOLUTE-WORLD-DIRECTORY
woi-persistent-world storage ABSOLUTE-WORLD-DIRECTORY --budget-bytes 100000000 --max-generations 20 --project-saves 10
woi-persistent-world interrupt ABSOLUTE-WORLD-DIRECTORY 5
woi-persistent-world recover ORIGINAL-WORLD NEW-BRANCH-ROOT/WORLD-ID --generation 2
woi-persistent-world advance NEW-BRANCH-ROOT/WORLD-ID 2048
woi-persistent-world finalize NEW-BRANCH-ROOT/WORLD-ID
```

Paths must be absolute, outside scientific archives and free of links/reparse
ancestors. Create the fresh branch root beforehand; the final world directory must
not exist. `interrupt` explicitly exits after synced journal appends without saving
new private state. This models an uncommitted exit, not a killed process or power
loss. `check` emits JSON and fails if verification is incomplete/invalid; a valid
prefix plus tail is reported distinctly and does not authorize ordinary reopen.
`storage` emits JSON without executing or terminating simulated behavior.

The original strict complete source fingerprint still identifies every new build.
Named v1 reader compatibility is limited to the recorded Windows/compiler and
unchanged kernel/dependency inputs. It is not blanket acceptance of arbitrary old
identities. A genuine old engineering checkpoint fixture verifies decoding, actual
state and continued original history; scientific and user GUI files were not used.

Four original session/process tests passed after replacing the common reader. Four
new recovery tests then passed: read-only/busy/malformed inspection, exact
parent/tail/state preservation and continuation, explicit older selection under
corruption with truncated tail, and original v1 reading/continuation. Latest
containment and CLI refinements passed both four-test suites and all-target Clippy.
A real headless CLI sequence created a paired32-tick world, saved at tick9,
executed five unsaved iterations, preserved49,051tail bytes, explicitly recovered
generation2 into another root, resumed and finalized. Parent journal SHA256 remained
unchanged, and complete terminal JSONL matched the original coordinator reference
(`307c947f17a5a47684f6fb309db11323cbf330268c0df54127b288a7025db04d`).
This is genuine execution in separate CLI processes with an intentional uncommitted
exit; it is not the later forced-termination gate. Test development exposed
a sum type inference error and Windows refusing reads of its exclusively locked
placeholder; the test now excludes that empty lock from byte snapshots and still
checks actual inventory bytes. Clippy's constant-chunk suggestion was corrected.
These failures are retained in task-specific ignored logs.

## Recovery lineage v2: committed exclusion is not uncommitted history

Review identified an important distinction in the first v1 recovery prototype:
selecting an older generation can exclude history already covered by a later valid
checkpoint. Calling that whole suffix "uncommitted" was too broad, even though the
restored actor state and future execution were correct. No published GUI artifact
or historical experiment was changed to conceal this correction.

New branches use `woi-world-recovery/v2`. Under the source writer guard, recovery
verifies all candidate generations and records their validity/errors. It stores
`later-committed.bin` for bytes after the selected prefix through the furthest
verified prefix, and `unverified-tail.bin` for the remaining bytes. These plus the
branch's initial journal prefix exactly reconstruct the original journal. The
unverified suffix is not asserted to have *never* been committed: a damaged or
unsupported newer checkpoint can make that historical status uncertain. The
receipt records that qualification and the explicit chosen generation.

`recovery.sha256` binds the receipt. Reopening/verification checks the selected
parent checkpoint, original prefix and both evidence segments against their saved
hashes and lengths. Corrupt or missing lineage/evidence refuses both reopening and
further recovery. These are unkeyed consistency checks, not authentication against
an adversary who rewrites all files and hashes. Parent URIs are audit references;
JSON paths are never followed to execute filesystem operations. The external branch
creation path qualifies reused event IDs; moving the directory does not change
actor-owned identity or require the original path to remain accessible.

Eight targeted tests passed with v2, including54independent checkpoints, older
selection with a valid later generation, exact evidence reconstruction, and damaged
receipt/marker/tail refusal. All-target Clippy and formatting pass. v1 *checkpoint*
reading remains supported under its explicit guards; unpublished v1 recovery
sidecars from development are not silently migrated. Recover again from their
preserved original parent if needed. No GUI release or automatic cleanup follows.

## Independent raw verification

`verification.rs` parses raw CBOR/JSON with its own protocol declarations, SHA256
binding calculation and primitive checks. It does not call World restoration,
Session loading or simulation validation. The filesystem wrapper only supplies a
locked prefix and checks directory/generation/runtime eligibility. The reader
checks configuration/boundary correspondence, event IDs/order, commitment starts,
completions and occupancy, execution phase and the aggregate food ledger. It is
explicitly not a private-cognition validator or a substitute for exact full-state
and original-history comparisons.

`verify WORLD --generation N` exposes this separate check. It passed54 checkpoints
in18controlled scenario/mode configurations (six scenarios ×023/Baseline/MealRelief,
three checkpoint positions; seeds0/42/u64MAX). Rehashed food/configuration/duplicate-
completion corruptions and byte/prefix damage were rejected independently. Six
targeted tests including recovery passed, as did all-target Clippy/formatting.
An initial test used underscores in an external directory ID; the established ID
validator correctly refused it, and only the test's directory spelling changed.
The v1 compatibility guard now also pins inherited file digests, direct kernel
dependencies and exact package sets for the baseline package names.

Killed-process/publish failure probes, many-process023/024 stress, workbench CLI
integration and compression decisions remain pending.
The complete persistent-world release suite passed22 tests after the reader change,
including the original seven short cases, twelve long worlds/228splits, all024
registered cases, existing process sessions and the four new recovery tests.
All-target Clippy and formatting pass at this stage. This component gate does not
substitute for the final protected-foundation/workbench gates after implementation.
## Opt-in storage compression

Genuine old engineering checkpoints were independently verified before the Rust
codec investigation. Each decoded exactly to its original bytes. These local
wall-clock observations include host/cache effects and exclude future container
header overhead; they are not simulation evidence or statistical estimates.

|Checkpoint|Raw bytes|Zlib bytes|Encode ms|Decode ms|Independent verification ms|
|---|---:|---:|---:|---:|---:|
|Initial paired|8,848|2,191|0.318|0.090|3.81|
|Paired128 terminal|1,776,083|54,454|9.76|1.80|233.0|
|Paired1024 terminal|13,720,146|343,331|62.84|10.00|1594.8|

The large reduction justifies an opt-in `woi-world-storage/v2` container using
zlib level6. It encloses the exact uncompressed v1 commit, with declared raw length
and SHA256. It preserves the inner complete checkpoint, source/configuration and
all actor state. Raw v1 remains the default; neither older generations nor journals
are rewritten. `Session::save_with_codec(Codec::Zlib)` and
`advance WORLD STEPS --codec zlib` select it explicitly. Raw and compressed files
can coexist. Decompression is bounded at160MiB, checks exact lengths/hash/trailing
data, and refuses unsupported codecs, versions and nested containers.

Ten targeted tests pass, including compressed recovery, exact restored state,
raw-default continuation, original v1 bytes,54independent checkpoints and compressed
corruption/size/truncation/trailing/nesting controls. All-target Clippy/formatting
pass. The primitive reader shares this transport decoder, while its v1 metadata,
binding and physical checks remain separate from simulator validation. No claim
of independent codec implementation is made. Actual container/save/recovery costs
and many-process tests remain to be measured; no bounded disk-use claim follows.

Dependency resolution initially attempted unrelated libc/tokio downgrades and
Windows/getrandom edge changes. These were reverted: existing dependency versions
and graph edges remain intact, except the new codec dependency on the persistence
crate. The named v1 guard now compares complete baseline dependency graphs, allowing
only the declared additional transport dependency. Diagnostics remain in task logs.

No cleanup is implemented: cross-directory lineage dependencies make silent pruning
inappropriate. All previous valid generations and recovery evidence remain. This
is an active engineering checkpoint, not a completed reliability claim.

## Final interface and headless regression gate

The workbench `world-sessions` feature exposes the same inspection/recovery/codec
Rust APIs; no GUI controls were added. `world-storage`, `world-check`, `world-verify`
and `world-list` use existing-only storage. Inspection of an absent portable
workspace creates nothing. See the [actual CLI workflow](PERSISTENT_WORLD_QUICKSTART.md).
An interrupted exit emits `checkpoint_saved:false` and a recovery warning.
`world-check` verifies committed state; a reported tail does not authorize normal
reopen. `world-verify WORLD --generation N` checks raw primitives independently.
Only `world-recover WORLD FRESH-ABSOLUTE-ROOT/WORLD-ID --generation N` creates a branch.

One bounded Rust entry point from the repository root:

```powershell
cargo run --release --locked --manifest-path prototypes/persistent-world/Cargo.toml -j 1 --example validate_reliability -- --foundation
```

It runs prototype formatting/private rustfmt, locked all-target Clippy, all library,
binary and integration tests with explicit failure probes, headless workbench
checks, and existing genuine-runner/HTTP-reader/recorded-history regressions. The
HTTP test opens no browser. `--foundation` additionally runs root formatting,
Clippy/tests, read-only `validate_contract` and separate `validate025_acceptance`.
Without that flag it runs the component suite only. Full per-step diagnostics and
JSON summaries go to fresh `target/reliability-suite/run-*` directories; a failure
returns nonzero and the exact log. No scientific archive is an output location.
No repository CI workflow existed to extend; actual platform coverage is Windows
x86_64 MSVC. Installed-package/runner asset acceptance is separate from this gate.

`failure-probes` is test-only, opt-in and has a distinct runtime identity. It holds
a real child after journal advancement, after pending sync/readback before rename,
or immediately after publication. The parent kills only its own ready child via
`Child.kill`, then checks files and OS lock release. Readiness is bounded at15s;
the child stops itself at30s if not killed. Ordinary builds expose no probe command
or environment trigger. No desktop, crash dialog or user operation is involved.

Final review also reproduced a Windows path-guard defect in a disposable fixture:
`experiments.` resolves to `experiments`, bypassing literal-only name checks.
`check_path` now checks the canonical nearest existing ancestor after rejecting
links/reparse ancestors. A Windows regression covers normalized aliases of
experiments, reinforcement and .git, refusal before Session creation and empty
scratch destinations. The actual default CLI now refuses the original probe.
No scientific archive or user session was touched. Filesystem roots are also
rejected before inspection traversal. This is transport validation, not an actor law.

## Final measured storage and costs

The [engineering receipt](../../prototypes/persistent-world/benchmarks/headless-reliability-windows.json)
contains12 actual rows and a separate process-memory observation. It is not frozen
scientific evidence; wall times are not exact-replay expectations. Reproduce with:

```powershell
cargo run --release --locked --manifest-path prototypes/persistent-world/Cargo.toml -j 1 --example measure_reliability -- C:\FRESH-MEASUREMENT-ROOT
```

Use a fresh approved absolute directory. Paired/seed42/period11 are fixed, with
H128/H1024,023/024Baseline/024MealRelief and Raw/Zlib. All12 recoveries restored the
exact selected state and continued to exact terminal state/history equality.
The inventory below follows three quarter-horizon saves (four generations including
the initial raw generation), then seven unsaved steps, before branching. Journals
are byte-count equivalent between codecs. Logical file sizes are not heap or
allocated filesystem blocks; branch evidence adds further storage later.

|Horizon / mode|Latest raw checkpoint bytes|Latest zlib checkpoint bytes|Journal bytes|All source bytes raw to zlib|
|---|---:|---:|---:|---:|
|128 / 023|1,183,071|40,387|1,708,481|3,975,373 to 1,798,222|
|128 / 024 Baseline|1,224,354|41,842|1,766,865|4,121,532 to 1,859,900|
|128 / 024 MealRelief|1,186,714|41,197|1,715,568|4,014,629 to 1,807,767|
|1024 / 023|10,130,624|256,805|13,635,184|33,767,602 to 14,170,795|
|1024 / 024 Baseline|10,390,171|261,333|13,984,331|34,645,217 to 14,531,219|
|1024 / 024 MealRelief|10,521,642|283,278|14,158,171|35,026,737 to 14,742,363|

At1024, latest checkpoints are97.31-97.48% smaller and four-generation source
folders57.91-58.06% smaller. The unchanged journal dominates compressed storage.
The receipt's full size arrays show actual initial/quarter/half/three-quarter
growth; every earlier generation remains. No cleanup is implemented, because
cross-directory recovery dependencies make pruning inappropriate. Warnings do not
terminate or change simulation; projections assume unchanged checkpoint size and
do not query free space. Neither warning thresholds nor compression bound disk use.

Local timing ranges below are milliseconds across three modes. Save ranges contain
nine saves each; other ranges contain three observations. They are not confidence
intervals or isolated causal policy comparisons. No own test/compiler load ran
during the measurement. Open includes verification and restore; inspection checks
all four generations; recovery verifies, preserves evidence and publishes a branch.

|Horizon / codec|Save|Open/restore|Inspect4 generations|Independent selected check|Recovery branch|
|---|---:|---:|---:|---:|---:|
|128 / Raw|17.0-42.2|82.0-93.3|181.8-207.0|166.4-203.5|401.6-459.0|
|128 / Zlib|19.9-49.4|80.0-105.0|171.9-212.4|165.9-198.9|381.9-444.2|
|1024 / Raw|74.9-239.0|617.7-755.1|1288.8-1331.5|1425.4-1493.8|2800.1-3015.1|
|1024 / Zlib|90.1-283.4|645.4-660.4|1233.4-1294.5|1414.0-1484.8|2854.2-2908.5|

Compression adds save work; no universal speed benefit is shown. Full validation
and recovery remain costly because retained histories and multiple generations
are decoded/checked. Three-quarter execution+journal+save totals were275.5-321.3ms
at128 and2142.0-2513.4ms at1024 across both codecs, not pure kernel throughput.
The own measurement CLI peaked at372,862,976bytes (355.59MiB) working set over832
samples at50ms; observed OS high-water and sampled maximum matched. This includes
references, retained buffers and verification across all12 rows. It is not Rust
heap, single-world production memory, actor cognition, codec-specific memory or
GUI responsiveness. No GUI process was measured.

## Final validation and preservation

- Final corrected-source component gate:35 prototype tests, including243 real
  process save/reopen cycles (27perworld) across nine1024 worlds. All nine include
  pending/active, hunger/trust, memory occupancy16, cutoff and charged-drain splits.
  Full checkpoint bytes match each split; terminal bytes and complete original
  023/024 histories match. All three forced-kill stages pass, including writer
  exclusion, lock release, pending-file preservation and real terminal continuation.
- Independent observer audit confirms15 successful acquisitions,14depleted
  attempts and three actual inquiries. Each world loses1,916-1,976 distinct
  owner/episode IDs from retained boundary memories. Recipient responses and
  reporting crossings occur at period11. This distinguishes actual eviction from
  merely reaching buffer capacity. Other legacy persistent effects are not erased.
- Headless workbench:13 ordinary tests, seven explicitly enabled integration
  regressions and one separately enabled genuine-run library regression pass (21).
  Actual CLI create/advance/compressed-save/unsaved-exit/inspect/recover/continue/
  finalize/independent-verify runs for023 and both024 modes. Originals stay intact.
  Two asset-dependent installed-package/portable-runner tests were not rerun as
  acceptance of a new release. No new package or GUI release was produced.
- Root formatting, locked all-target Clippy and217 tests pass. Read-only foundation
  validation confirms408 artifacts and425 raw originals unchanged, original replay
  and independent readers. Separate025 acceptance passes214 custody bindings and
  50exact byte replays.025 remains separate research, not adopted/protected.
- Default and failure-probe prototype Clippy, private rustfmt, headless workbench
  Clippy/formatting, and desktop+portable all-target compilation-only Clippy pass.
  The original v1 fixture ran with its guarded alias enabled. Exact raw/compressed
  decode, corruption, lineage and independent primitive controls pass.

The failure matrix includes missing/corrupt commit, committed-prefix corruption,
truncated suffix, pending files, failed publication and successful retry without
reexecution, wrong source/runtime/world/configuration/schema, semantic cognitive/
physical/pending-reply faults, ambiguous names, numeric u64MAX/exhaustion, existing
or protected destination, busy writer, and damaged lineage. Corruption uses own
scratch data; nothing regenerates historical archives in place.

Final component logs: `target/reliability-suite/run-112084-1792985572621056100`.
Root/protected-source logs: `target/reliability-suite/run-155220-1792966112695745300`.
Final process/audit receipt: `target/recovery-validation/many-sessions-113956-1792985912840651600`.
Measurements: `target/recovery-validation/measured-1792986461383` and the versioned
engineering receipt. Other logs: `target/recovery-default-clippy-final.log`,
`target/recovery-desktop-compile-only.log`, `target/recovery-workbench-ignored-lib.log`,
`target/recovery-normalized-path-refusal.log`. These are engineering outputs.

Preserved development failures: a root-path diagnostic ran against a library built
before its guard was added; Windows LNK1104 occurred when tests tried to overwrite
the running example runner. Explicit --lib/--bins/--tests fixed the self-build.
An overlapping replacement runner also hit the OS lock; waiting for the first to
exit fixed that launch. Neither linker failure was a simulation assertion. The
normalized-path bypass was a real transport defect, fixed and covered by a full
component rerun. No actor law, threshold, capacity or outcome was tuned for passes.

## Limitations, identities and handoff

These actual stress histories use frozen LastSeen knowledge: Sense/Taken/Depleted,
with no Expired transition. Expire12 is not a023/024 configuration option; adding
an expiry law solely to manufacture coverage would violate scope. Real expired-
observation process continuation is therefore **not demonstrated**. Existing
semantic controls reject missing/inconsistent environmental cells without audit
recovery. Four actors, H1-1024 plus33 charged drain, episode16/evidence32/learning32/
concerns8/environment1 remain unchanged. Observer histories and immutable generations
still grow.160MiB commit/256MiB journal limits,4096 inventory entries/eight levels
are explicit bounds, not indefinitely sustainable worlds. Process-kill tests do
not prove sudden-power-loss/directory-fsync durability. Hashes are unkeyed consistency
checks, not adversarial authentication. Arbitrary version migration is unsupported.

Validated runtime identities, rustc1.99.0, Windows x86_64 MSVC:

- Standalone/default: `3a02e70599c930096e8b5077e01062e96ed4bfb45037f98162dd1ad4541cd2d4`.
- Failure-probe: `8b16f2a3113272391069652fcf7de77525a6b9126238bbe8ba2cf75e39115639`.
- Workbench host: `a88a861a2e8c273a9e3d55b5fe44af7f7946ec6a50ebcfaf92560bdb4b1e54ec`.

Validation binaries were built from the reviewed working tree above d18eb3b;
workbench metadata correctly reports that pre-commit dirty state. Source publication
does not make those binaries an accepted release. Named original v1 aliases remain
kernel/compiler/target/dependency guarded; unpublished intermediate headless-build
identities are not silently migrated.

Changed ownership: prototype recovery/verification/codec, guarded generation, CLI,
failure/continuation tests and measured receipt; workbench CLI/session APIs and
read-only workspace validation; engineering/readiness/quickstart/maps. No protected
contract, archive, actor source/default or025 producer changed. Runtime.rs differs
only by its guarded identity predicate. This is no foundation extension or026.

Future GUI work requires separate authorization and native mid-batch Pause, terminal
Finalize,024 interaction and GUI responsiveness/memory acceptance. These remain
**DEFERRED**. No desktop control or new Windows release occurred. Start a later
headless session with the committed checkpoint and CLI guide; do not resume GUI
control merely because backend checks passed.

Accepted implementation commit: `185962dd9782ab09ad3ba6e28284a309e3f6d79b`.
The subsequent documentation commit contains this final report and checkpoint.
