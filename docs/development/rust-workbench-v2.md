# Workbench V2 engineering and acceptance

This records the preceding V2 development work. Its pending desktop acceptance
notes describe that earlier checkpoint. The subsequent
[portable engineering report](portable-workbench-engineering.md) records the
completed native run/replay/annotation/reopen workflow and current delivery limits.

This is a human research interface, not a behavioral experiment. V1's default
023 runner, configuration meaning, saved runs and exact reproduction remain
supported. Gate A exercised the native V1 run, reproduction (identical SHA256),
inspection and existing Explorer; its twelve presets also passed real execution.
The V1 acceptance report remains prior evidence.

Implementation plan: add explicitly versioned, strict mode-specific options for
024 Baseline/MealRelief and corrected025 preparation. Dispatch only to existing
fixed Cargo commands. Do not alter simulator sources, source locks, archives,
policies or the001–024 protected endpoint.025 remains unadopted research.
Preserve old unversioned023 JSON as version1; new modes require version2.

Use bounded Rust observer readers with explicit023/024/025 schemas. Project
recorded local inputs and valuation terms; connect boundary/choice references to
admission and execution records without equating selection with execution.
025 needs common-mass accounting including raw, escrow and waste, not the old
edible-only ledger. Meal/transfer attribution remains fungible and uncertain.

Detailed inspection will use a workbench-owned compatibility bridge to the
unchanged Python Explorer. Only the explicitly supplied verified run enters its
catalog. The startup interface must open that run without catalog searching;
no browser-supplied filesystem paths. New backend and UI logic remains Rust.

Compare actual configurations and like-for-like observer quantities. A matched
exploratory comparison requires unchanged world settings and runner identity;
it never becomes a registered causal conclusion or universal welfare ranking.
Human notes remain separate files and never feed simulation cognition.

Completion evidence, costs, limitations and publication will be recorded here
after real runner, GUI, browser, safety and read-only foundation/custody checks.


## Implemented boundary

Observed starting revision: `477d35e6d7b7418b8117cce67115dfb14c240ec6`, clean
and equal to actual `origin/main` at task start. Current publication remains
pending final desktop/browser acceptance. No simulator, frozen source/manifest,
root Cargo dependency, scientific archive, policy or default was modified.

| Human mode | Configuration | Existing executable / real interface |
|---|---|---|
|023 ContinuousWorld|Old unversioned V1 JSON remains version1; default unchanged|`continuous-world run NEW.jsonl --case CASE --ticks N --period P --seed S --phase Epoch --order Ascending --claim 4 --mode Continuous`|
|024 ActivityValuation|Version2; explicit Baseline or MealRelief|`activity-valuation run NEW.jsonl --case CASE --ticks N --period P --seed S --phase Epoch --order Ascending --claim 4 --valuation Baseline|MealRelief`|
|025 MaterialPreparation|Version2; valuation and typed preparation settings|`material-preparation-observer2 run CONFIG.json NEW.jsonl`|

Each executable is invoked with an argument vector through `cargo run --release
--locked -j 1 --manifest-path <repo>/prototypes/<runner>/Cargo.toml --target-dir
<repo>/target/<runner> --bin <runner> --`. These are fixed trusted commands,
never commands imported from saved metadata. Exact absolute arguments are saved
in each run's `metadata.json`.025's actual generated `runner-config.json` contains
world settings, valuation, observer family `workbench_demo`, null pair, feature
and access settings, raw0–16, one of the three established effort pairs and an
8-tick raw/ready observation plan. No raw JSON is required in the normal GUI flow.

`config.rs` rejects unknown fields, unsupported versions and irrelevant/missing
mode options. Common bounds are the12 existing scenarios, u64 seed, ticks1–1024,
reporting period1–128. Four people/Continuous/Epoch/Ascending/claim4/cadence8/
capacity2 and33-tick drain remain fixed. New025 presets expose actual existing
settings, including shared-slot raw/ready overwrites. NoAccess is not feature-off
parity. MealRelief retains exactly `H + min(H,40)` for eligible Consume. Neither
policy is promoted to a global default.

## Native reading and source navigation

`history.rs` checks explicit authentic trial and physical schema versions023,
024 or025 against the declared configuration. It bounds input at64MiB and each
historical sequence at65,536 records, skips full inherited cognitive archives,
and keeps first32 samples, aggregates, endpoints and four-person state. Native
progressions reference the same captured decision objects by index. Valuation
records join by boundary/choice, admissions by source/proposer, execution by
commitment ID; absent sampled links remain Unknown. Mode-specific score fields
must actually be present and match recorded policy/selection references. The
reader does not reconstruct missing cognition or recompute policy for the actor.

The native view includes local food/hunger, directed trust, public co-presence,
legitimately retained ready/raw receipts, candidates, recorded base/bonus/effort/
final terms, selected contact, sampled admissions, actual started/completed
commitments, resource actions/balances/interpretations, meals and final state.
Its bounded sample is not a complete timeline.025 adds extraction/conversion
operations with owners and escrow, prepared/failed counts, raw/waste/final state,
and common-mass accounting. Failed work retains full configured effort. Prepared
food remains fungible; no later meal or transfer is assigned an invented unit.

`desktop_history.rs` owns observer pages; `comparison.rs` compares verified run
IDs and actual configurations. Only valuation may differ for the conservative
Matched exploratory comparison label; all other config, runner fingerprint,
recorded initial id/food/hunger/trust, initial total and start must match. Other
pairs are Recorded difference. Cross-schema initial totals are Not comparable;
missing material quantities are Unknown/not this mode. Neither label is a
registered causal conclusion or a welfare ranking. Notes, including optional
suggested improvement, remain external JSON and never enter runner arguments.

`viewer_bridge.py` is a narrow compatibility exception to the Rust-first boundary.
The existing Python Explorer CLI cannot preselect a custom run and normally opens
an unrelated catalog default. The bridge imports its unchanged Repository/Handler,
provides only one explicit local source, checks its completed-history SHA256 and
membership before announcing readiness, and adds a same-origin startup script to
the served index. That script opens the sole catalog entry through the existing
load button. The native action opens its verified URL after readiness; its link
remains a fallback if browser opening fails. No existing viewer assets are rewritten; no filesystem path is
accepted through browser parameters. Existing Host/Origin guards, schemas and
independent readers remain. Rust owns configuration, safe execution, metadata,
native readers, comparisons and UI state. No dependencies were added.

The native async viewer result carries its originating run ID; changing selection
while startup runs closes the obsolete viewer rather than associating it with a
new run. Switching inspected runs also clears old viewer/comparison state.

## Executed engineering evidence

Gate A exercised the native V1 Run → Results → Inspect → Reproduce → Explorer →
external Human Observation workflow before integration. Fresh runs
`run-1792732960521-160284-0` and `run-1792733147240-160284-0` reproduced the V1
SHA256 `24e6046ec2af86d6bee7a3c2e3cb6a1b95459522f783f39192167f75e946b5be`.
The latter saved a real native observation separately. All three earlier delivered
V1 histories and these two fresh histories remain byte-identical. The original
[V1 acceptance](rust-workbench-v1-acceptance.md) remains prior engineering evidence.

These new root exploratory runs use paired/seed42/192ticks/period11:

| Run / mode | SHA256 | Recorded result |
|---|---|---|
|`run-1792734727380-122968-0` /024 Baseline|`8852f1fd90364976515748f3805f0b621c32fe941e9c71eda5c2c8101541558a`|10meals,182resource events,34transferred units; final food1/1/0/0|
|`run-1792733606395-157140-0` /024 MealRelief|`10642879869745d397e5a5fb113ad19d4e2b454d007cb8dffc9cade8742371e7`|12meals,188events,29transferred units; final food0/0/0/0|
|`run-1792733823770-145660-0` /025 KnownRaw/Short/6raw/MealRelief|`e1ce7734e327d033a1a08795f0b498950f44c0f102f71d665fcea6dc7b8e179f`|15meals,187events,35transferred units;3prepared/1failed conversion, final raw0/waste3/escrow0/owned food0each|

All four actors end at hunger100 in these examples. More meals, different
transfer volume and a real failed attempt are preserved without claiming policy
superiority. Native state/exact reproduction and independent raw JSON extraction
support these counts; these are interface acceptance cases, not new research.

Validation logs are under ignored `target/workbench-v2-validation/`. Completed:

- Root `cargo fmt --check`, `cargo clippy --locked --all-targets -j 1 -- -D warnings`,
  `cargo test --locked -j 1`:217tests pass.
- Full `cargo run --release --locked -j 1 --example validate_contract`:PASS through
  024,408protected artifacts/425raw originals, including all existing replay gates.
- `cargo run --release --locked -j 1 --example validate025_acceptance`:PASS,
  214custody bindings, original/review controls and50exact observer2 replays;
  no adoption/protection. A later inventory recheck also passes.
- Workbench routine and explicit real-run tests (14total, all explicitly run): legacy JSON, strict new modes,
  path confinement/unique outputs, failure/zero-exit-missing-output handling,
  source identity, command-injection resistance, corruption/schema/score controls,
  external annotations and exact fresh-path replay. Real023/024/025 execution
  covers all12 exposed cases; new modes exercise duration/reporting/seed bounds.
- Corrected025 tests preserve six setting combinations, actual failed full effort,
  extraction/conversion ledger and both valuation policies in every scenario.
- Real selected-source HTTP viewer tests for023/024/025 verify exactly one native
  source, successful verified trial loading, startup script serving, invalid-host
  rejection, stale-history hash rejection and unchanged original viewer assets.
- Desktop-feature compilation/Clippy and library tests pass. V2 release window
  starts; actual V2 native interaction/browser auto-opening acceptance is pending.

The native control session was stopped with the physical Escape key. Its initial
release window was launched before the final label/selection-guard changes. Do
not use its mere presence or the HTTP tests to claim final desktop/browser
acceptance. A resumption question is pending; no further computer control has
been performed since the stop. No final commit/push is claimed yet.

## Cost, failures and limits

Observed first024/025 child builds took164.4s/225.5s; warmed024 Baseline child
execution took383ms. An intermediate release desktop build took33.94s. These
uncontrolled wall-clock observations include build/cache/contention and are not
simulation evidence. Five release hash+native inspections per3.4–3.53MB history
took roughly31–38ms in the final measurement; serialized bounded projections were
57,341/74,619/85,492bytes for023/024/025. These are serialized sizes, not peak RAM. The reproducible
measurement command is `cargo run --release --locked -j 1 --manifest-path
 tools/workbench/Cargo.toml --example measure_reader -- RUN-ID ...`. New samples
remain separate from deterministic history output. No peak-RAM or broad simulator
performance claim is made; no engine benchmark is warranted by unchanged physics.

A first test launch collided with a running Windows CLI executable lock; the test
workspace was isolated. A text edit hit Windows' default encoding and damaged the
UI working copy; it was restored from V1 and reapplied with explicit UTF-8, then
compiled/Clippy checked. Neither failure touched scientific sources or evidence.
Other retained failures include genuine runner failure/missing-output cases,
corrupt reader fixtures and the real025 full-effort failed conversion.

Limits: first32 native samples and partial cross-links; detailed full navigation
still uses the existing Python/JavaScript viewer. No general history-import UI,
installer, portable standalone deployment, cancellation/resume, automatic paired
execution, global utility score, or actor/tick URL deep link is introduced. Native
UI rendering/interaction cost and final browser startup remain unaccepted until
actual interaction completes. No026, canon, cognitive capacity or scientific
protection changes occur. The smallest remaining acceptance gap is proving the
current V2 desktop → exact selected history workflow in the real UI.


Final build addendum: the current desktop was built successfully in the isolated
`target/workbench-v2-final/` directory (6m45s clean dependency/build cost;
0.49s subsequent freshness check), avoiding the still-running older Windows
executable lock. The final binary's CLI help,025 inspection and024 matched-pair
comparison pass. Binary size/SHA256 and normalized source hashes are recorded in
`target/workbench-v2-validation/final-desktop-artifact.json` and
`final-source-snapshot.json`. Desktop Clippy/library tests pass after automatic
verified-URL opening was added. This does not establish real GUI/browser acceptance;
that remains dependent on resuming computer control after physical Escape.
