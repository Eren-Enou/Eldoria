# Headless persistent-world reliability

Prospective engineering contract, starting at9742295. This supersedes the previous
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
directory, saves `uncommitted-tail.bin`, publishes `recovery.json` lineage and copies
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

Independent raw verification, killed-process/publish failure probes, many-process
023/024 stress, workbench CLI integration and compression decisions remain pending.
The complete persistent-world release suite passed22 tests after the reader change,
including the original seven short cases, twelve long worlds/228splits, all024
registered cases, existing process sessions and the four new recovery tests.
All-target Clippy and formatting pass at this stage. This component gate does not
substitute for the final protected-foundation/workbench gates after implementation.
No compression or cleanup is adopted yet. Compression investigation will compare
small, medium and1024-tick real checkpoints; material size reduction and bounded
encoding/decoding cost must be demonstrated before enabling an opt-in transport.
This is an active engineering checkpoint, not a completed reliability claim.
