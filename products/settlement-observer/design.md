# Settlement observer boundary

This standalone desktop is a consumer/controller of the unchanged settlement v1
runtime. It adds no character policy, action, source observation, physical rule or
research adapter. Existing checkpoint identity and bytes must remain compatible.

One worker owns the ECS world. UI requests are bounded commands: create, open,
save a fresh checkpoint, advance one tick, run/pause, change wall-clock pacing and
request read-only projections. Presentation frames use a replaceable latest slot;
acknowledgments/errors are separate and cannot be lost when rendering is slow.
Only `Settlement::advance` moves logical time. Save pauses first at a boundary.
Open validates into a new world before replacing current state; failure preserves
the current world. Closing or replacing unsaved work requires an application-level
save/discard choice. No autoplay on open, rollback, replay into cognition or silent
checkpoint migration.

Perspective is an explicit host boundary. Observer frames expose copied global
state. Character frames contain only `OwnedView`, own commitments, public map
labels/topology and external follow-selector names. Foreign inventory, hunger,
beliefs, actual remote source stock and foreign decision inputs are excluded.
Switching perspective invalidates old presentation frames and selection immediately.
Historical local records are explicitly labeled as records, never current memory.
Shared records may be inspected only through current retained receipts; foreign
causal context stays behind an Observer-only boundary. Filtering is performed by
the host, not merely hidden by widgets. No projection feeds character cognition.

The history page is bounded at30 rows. A causal index appends only newly produced
record IDs and commitment links; no full history JSON is sent to the UI per frame.
Decision inspection displays recorded inputs, candidates and terms separately from
current character state. Selecting an old record does not rewind or restore a world.

## History and notebook milestone

Hypothesis: explicit outcome filters and durable human annotations make a finite
world easier to follow without changing its choices. Acceptance requires unchanged
complete-state bytes under annotation/navigation, exact cached/full-snapshot parity,
restart recovery, rejection of mismatched anchors and no notebook in character frames.
This is observer engineering, not an experiment or a measure of social significance.

`Index` appends event-category IDs, actor IDs, outcome counters and a SHA-256 record
prefix chain as records arrive. Observer pages borrow category indices; character
pages merge own record candidates with currently retained receipts and reauthorize
them. Per-frame observer outcome counting no longer scans the full history.
Indices remain disposable observer state and rebuild on Open. Their storage grows
linearly with the retained objective history; character filtering still scales
with authorized candidate history. Nothing is copied back into the ECS world.

`notebook.rs` stores strict versioned, checksummed JSON in a separate workspace.
Its world identity binds runtime, configuration, stable actor IDs and initial food;
each anchor additionally binds record ID, tick and the entire executed prefix.
Identical deterministic worlds intentionally share a notebook. Branches share
annotations only on identical prefixes. Future bookmarks remain unavailable in an
earlier checkpoint: opening/jumping never runs the world to satisfy an anchor.

Each edit publishes a fresh immutable revision using the engine's existing safe
output/publication helper. Expected revision rejects stale edits; same-revision
writer races cannot overwrite files. Reload surfaces corruption instead of silently
falling back. These checks detect accidental damage, not hostile forgery. There is
no distributed lock: concurrent different-anchor publications near the global cap
can exhaust it; use one writer per workspace. Existing files remain intact.

Bounds per world: 128 bookmarks, 512 total revision files, 120 UTF-8 title bytes,
4000 body bytes, 32 KiB accepted file size, at most1024 directory entries scanned.
There is no automatic pruning/deletion or notebook migration. Unpublished pending
files are ignored but count toward the directory scan bound. Notes are local human
metadata, not encrypted secrets or a sandbox between computer users.

`notebook_ui.rs` owns a pinned draft. Perspective changes hide it without clearing
it; selection cannot silently replace a dirty draft. Acknowledging an in-flight
save preserves newer typing. New/Open/Close handles the draft before rechecking
world progress against a fresh acknowledged frame. A failed note write leaves the
draft and world intact. After a stale-write rejection, Reload plus explicit review
and rebasing preserves draft text; saving a new revision is a separate action.

Acceptance: exact run/pause/step/save/open continuity, failed-open preservation,
bounded frames/pages, cross-perspective privacy (including causal traversal), fresh
generation saves, unchanged engine fingerprint, and actual native create/run/pause/
inspect/save/reopen workflow. Core tests and inherited research contracts stay green.
