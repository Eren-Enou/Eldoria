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

Acceptance: exact run/pause/step/save/open continuity, failed-open preservation,
bounded frames/pages, cross-perspective privacy (including causal traversal), fresh
generation saves, unchanged engine fingerprint, and actual native create/run/pause/
inspect/save/reopen workflow. Core tests and inherited research contracts stay green.
