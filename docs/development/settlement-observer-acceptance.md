# Settlement observer: native desktop acceptance

Date: 2026-11-18. Source baseline: `0fc6594550ac9934e98736cb49403a23de5369af`.
This additive package implements the next observer milestone over the unchanged
headless engine. [Run guide](../../products/settlement-observer/README.md),
[boundary contract](../../products/settlement-observer/design.md),
[native receipt](settlement-observer-validation.json),
[projection performance](settlement-observer-performance.json).

## Delivered boundary

`products/settlement-observer` is an independent Rust/eframe workspace linking the
existing Bevy ECS settlement library. Its own manifest/lockfile do not change root
or settlement manifests/lockfiles. The engine fingerprint remains
`80a54fc216ef8417d9564da77bc8dc2d82a7b199ac3ee784db5b3070c023237a`.
No protected research source, archive, historical runner, Explorer or workbench code
changes. No new behavior, sensing, LLM, faction or resource law is introduced.

One worker owns the simulation. The UI submits bounded control/projection requests;
it receives copied frames with 30 history rows, one selected detail and at most 24
causal links. Superseded presentation frames can be dropped; authoritative history
and command acknowledgments cannot. UI selection and wall-clock pacing never enter
the world state. Source/actor panels explicitly distinguish current state from
recorded historical inputs. There is no rewind or journal-derived cognition.

Character-frame filtering happens before data reaches widgets. It excludes actual
remote stocks, foreign inventory/hunger/beliefs and foreign decision inputs. Own
work and co-present public signals remain visible. Causal traversal applies the
same checks as paging; a hidden foreign decision cannot be retrieved by record ID
or by following a shared commitment. Static map labels and the follow-name selector
are external navigation, not claims about character knowledge.

## Actual native workflow

Using the computer-use skill on the real Windows desktop:

1. Created 12 people in the New dialog. Step 1 showed 11 simultaneous commitments.
2. Traced Dara's invitation and expanded its selected candidate: appeal +50,
   caution −5, need +15, trust 0, total 60. An alternative travel score 48 and other
   candidates remained visible. This was the original captured local decision.
3. Switched to Aster's character perspective: the global ledger, other inventories/
   hunger and Dara's private decision detail disappeared. Unknown distant stocks
   were labeled unknown; only co-present names/availability/request signals remained.
4. Ran, paused at 36, saved through the native filename dialog and closed normally.
5. The release app reopened 36, stepped to 40 and saved four pending meals. It closed
   and restarted; reopening 40 showed those four meals still pending. A step to 41
   completed them and started new independent work; a selected recorded decision
   remained labeled tick 40 while the live character card showed 41.
6. Saved 41 in the application workspace. Character perspective was rechecked in
   the release UI. Global/private fields stayed absent.
7. After tightening close handling to include in-flight operations, the final
   executable stepped an eight-person world to 1, displayed its unsaved-progress
   dialog on native Close, and completed Save first→Save now→normal close. Six
   pending commitments were preserved. No Discard action was automated.

The original, independently built headless CLI regenerated all these saved states:

|Population|Tick|Pending commitments|Native file equals original CLI|
|---:|---:|---:|---|
|12|36|0|Exact full checkpoint bytes|
|12|40|4|Exact full checkpoint bytes|
|12|41|2|Exact full checkpoint bytes after restart/continuation|
|8|1|6|Exact full checkpoint bytes from final save-before-close|

SHA256 values and pre/post close-guard executable identities are in the native
receipt. Preflight debug and release workflows were both exercised; the final
close-guard build additionally exercised dirty-close/save continuation. The worker,
projection and engine did not change during that final UI guard correction. This
is actual interaction, not inference from compilation, screenshots or widget mocks.
It is not clean-machine, installer, network-isolation or multi-platform acceptance.

Visual inspection also identified a potentially confusing historical-decision
label: a foreign recorded decision could appear beneath the currently followed
person's live card. The final inspector explicitly names the recorded subject and
tick and labels its input as historical. Character switching invalidates old frame
data and detail selection immediately, including the same render pass.

## Tests and measurements

Five observer tests pass: 8/12-person exact continuity under interleaved perspective/
history changes; record/link privacy; failed-open/no-overwrite and pending-work
restore; actual causal links; threaded run/pause/step and frame-drop invariance.
Fresh original settlement tests pass 14/0; root tests 217/0. Native GUI acceptance
above is separate evidence and does not substitute for these data contracts.

One release-mode sample per point, no selected detail, background research gate
active. These measure projection construction and serialization only, not GPU time.

|People|Tick|Observer frame bytes|Character frame bytes|Observer build µs|Character build µs|Full history records|
|---:|---:|---:|---:|---:|---:|---:|
|8|64|10,509|3,698|65|30|846|
|8|512|16,834|4,429|196|110|6,889|
|12|64|14,853|3,976|48|35|1,283|
|12|512|25,447|4,839|362|184|10,872|

All pages have 30 rows. Unlike a 7–12MB complete checkpoint, no full history is copied
into a frame. Filtering/outcome aggregation still scans the retained journal, so
construction cost grows even when payloads stay bounded by population, cognition
caps, page size and one selected detail. Large populations, long UI soaks, peak
heap and renderer frame time are not measured or claimed.

## Completion checks

Verbose task logs: `%TEMP%/eldoria-observer-20261118/`. Research source/fixtures and
headless engine sources are unchanged from the baseline; only the new package,
acceptance/measurement documents and navigation/readme pointers are changed.

|Command|Result|
|---|---|
|`cargo fmt --check --manifest-path products/settlement-observer/Cargo.toml`|Pass|
|`cargo clippy --locked --all-targets --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer -- -D warnings`|Exit 0|
|`cargo test --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer`|Exit 0; 5 passed, 0 failed|
|`cargo build --release --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer`|Exit 0;actual native execution accepted above|
|`cargo run --release --locked --manifest-path products/settlement-observer/Cargo.toml --target-dir target/settlement-observer --example measure`|Exit 0; 8 projection points|
|`cargo test --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement`|Exit 0; 14 passed, 0 failed|
|`cargo fmt --check`|Exit 0|
|`cargo clippy --all-targets -- -D warnings`|Exit 0|
|`cargo test`|Exit 0; 217 passed, 0 failed|
|`cargo run --release --locked -j 1 --example validate026_foundation`|Exit 0;unchanged 001–024 archive/replay, separate 025 and isolated 026 C1; 495 portable/512 raw originals, 93 custody bindings and 3 layout faults pass|

The separate 027 read-only gate passed during the prior headless delivery; its source
and original artifacts remain untouched at this baseline, and no 027 module is linked here. Its result
is reused from the [headless acceptance](settlement-runtime-acceptance.md), not
claimed as a new run. No simulation throughput benchmark was repeated because
neither runtime nor workload changed; new projection measurements address this
change. No browser smoke check applies to this native-only addition. Existing
finite-world/storage qualifications remain; no sustainability, policy superiority
or adoption claim follows from the observer.
