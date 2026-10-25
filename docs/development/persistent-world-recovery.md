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

Implementation, commands, measured outcomes and final gate results will be added
after execution. This prospective document is not a completion claim.
