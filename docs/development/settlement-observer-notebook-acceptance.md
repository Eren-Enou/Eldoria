# Settlement observer: history navigation and notebook

Date: 2026-11-18. Baseline: `009e9a292f024d259e0c50f80c0c1387ce6c9936`.
This is isolated product observer engineering. Simulation source, engine identity,
checkpoint schema, research producers, historical defaults and protected artifacts
are unchanged. The earlier [native acceptance](settlement-observer-acceptance.md)
remains a separate delivery record.

## Delivered behavior

Outcome categories and Earlier/Later/Latest navigation make recorded consequences,
social work and refusals/failures easier to find. These are explicit presentation
categories, not a model of importance, enjoyment or character motivation. All-record
navigation defaults to consequences. Character navigation applies the existing
local/retained-receipt boundary before selecting an event.

The observer notebook saves titles and notes outside checkpoints and cognition.
An annotation binds runtime/configuration identity, record ID, recorded tick and a
hash of the executed history prefix. Earlier checkpoints display future bookmarks
as unavailable; different prefixes are rejected. Identical deterministic runs
intentionally share annotations. Jumping selects history and never rewinds or runs
the simulation.

Revisions publish to fresh files with checksums and stale-write protection. A bad
revision fails notebook loading explicitly, without stopping the world or silently
selecting an older revision. Dirty drafts stay pinned across selection/perspective
changes, are hidden in character mode and survive failed saves. Save-before-leaving
handles notes and world progress separately against acknowledged current state.

Source ownership and operational details are in the
[guide](../../products/settlement-observer/README.md) and
[boundary contract](../../products/settlement-observer/design.md).

## Verification

- Observer: 10 tests pass (5 inherited integration, 4 new cross-boundary integration,
  1 in-flight draft acknowledgement test); formatting and all-target Clippy pass.
- Root: formatting, all-target Clippy and217 tests pass.
- Core settlement:14 tests pass. The full read-only026 foundation command
  is recorded in the [validation receipt](settlement-observer-notebook-validation.json).
  The foundation
  includes the unchanged001–024 `validate_contract` archive/replay gate. Exit0:
  C1 exact100 replays and independent readers;495 portable artifacts,512 raw
  originals and93 additional bindings pass. No frozen evidence was regenerated.
- No research source, fixtures, frozen readers or product engine source changed.
  No027 retuning, adoption, population/horizon extension or new behavioral law.

New integration coverage checks complete checkpoint-byte equality under navigation,
filters, note writes and character access rejection; note restart and immutable old
revisions; stale revisions, wrong world/tick/prefix and future-prefix rejection;
corrupt metadata failure without old-revision fallback; and cached/full engine
snapshot equality on every tick in a64-tick fixture. Rebuilt and incremental prefix
hashes agree. Existing8/12-person continuation, causal privacy and worker tests pass.

Actual Windows native interaction opened the prior12-person tick40 checkpoint,
selected the latest consequence (Cedar's tick24 gather, record537), wrote a note,
filtered consequences, navigated23↔24, switched into character mode with no notebook,
and jumped back through the notebook without advancing tick40. Closing with an
unsaved edit saved revision2 and closed. Restart recovered that text. A further edit
and Step41 exercised note-save then a separate world-save guard before normal close.
The resulting635,864-byte checkpoint exactly matches the original pre-change CLI's
tick41 output, including two pending commitments. Three metadata revisions remain
separate from the unchanged world bytes. Companion JSON records hashes and final
release follow-up separately.

The conflict-review build reopened tick41 and recovered revision3. A valid exclusive
scratch second-writer fixture supplied revision4 while a native draft was dirty.
Save rejected the stale edit; Reload retained the draft and displayed the newer
saved text. Explicit review/rebase followed by Save published revision5 with the
draft text intact and time unchanged. This tests conflict handling, not concurrent
publication stress. The final filter-selection fix was checked natively: clicking
Bryn's tick41 meal (record849) retained Consequences with75 visible records. Both
follow-up builds closed normally. During development Clippy caught test-module
placement; it was corrected before passing completion checks.

## Costs and limits

[Release measurements](settlement-observer-notebook-performance.json) are one sample
per point with the research gate active, not a statistical benchmark. At512 ticks,
8/12-person worlds contain6,889/10,872 records. Observer frames take35/40μs to build;
character frames226/247μs; all pages remain30 records. Full index rebuilding costs
21.2/33.6ms, excluding world loading. Advancing ticks64→512 with simulation and
incremental indexing costs40.3/63.0ms. These are workload observations, not guaranteed
latency or a controlled comparison against the earlier delivery.

Observer pages/counters now use incremental indices. Character filtering still
walks authorized candidates; causal/detail authorization still has costs. Index
memory and complete objective history grow linearly, including one64-character
prefix digest per record. Notebook files are bounded per world (128 bookmarks,
512 total revisions,120-byte titles,4000-byte bodies); no pruning, migration,
encryption or distributed locking. Corruption requires explicit human recovery;
old files are never overwritten. Native layout needs scrolling on smaller windows.
No GPU frame-time, notebook-at-cap latency, multiwriter stress, long soak or
clean-machine packaging claim. Notes are not private from other computer users.

Next useful product work: a recent-checkpoint catalog and a longer observation
session to identify what makes the existing world engaging before changing its laws.
