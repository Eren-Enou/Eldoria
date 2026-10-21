# World of Individuals — History Explorer

A working local, read-only application for actual recorded simulation histories.
No accounts, API keys, package installation, cloud service or Bevy window.

From the repository root, with **Python 3.10+** (tested with 3.12):

```powershell
python -B tools/history-explorer/explorer.py
```

The command starts a loopback-only server and opens your browser at
[127.0.0.1:8765](http://127.0.0.1:8765). Leave the terminal running; Ctrl+C stops it.
Use `--no-browser` to open the URL yourself, or `--port 8766` if the port is occupied.
The authentic default example is Experiment 019, useful_contact, seed 42, Baseline,
original `seed-42.jsonl.gz` record **103**. Its paired Scale110 history is record **107**.

Normal launch uses the bundled **derived discovery catalog**, then verifies the
original compressed and complete decoded archive and selected JSONL member before
showing the trial. The catalog itself is not scientific evidence and is not marked
verified merely because it loads. No whole decoded corpus enters the browser.

The main application now also supports all164 original020 histories through
`experiment-020-trial/v1`, not an018/019 schema substitution. First launch builds
a derived020 catalog under `target`; subsequent starts reuse it, verifying original
membership on selection. The default catalog now has814 histories (336019+164020+214021+100022).
020 labels use original **zero-based trial ordinals**;019 labels remain one-based.
Use the020 sharing, missed-donor, last-unit, exhaustion, null and reconvergence
presets. Actor decisions include their actual cache observation/candidates; Observer
Perspective separately exposes claims, deadlines, grants/failures and conserved
cache+owned+consumed balances. Causal links retain exact original references and
qualify chronological context. [020 acceptance](validation/finite-integration/acceptance.md).

Optional full verification/index preparation, creating the catalog and authentic
default pair exports under ignored `target/history-explorer`:

```powershell
python -B tools/history-explorer/explorer.py --prepare
```

It is not required before normal launch. Optional valid Experiment 018 support:

```powershell
python -B tools/history-explorer/explorer.py --include018
```

That performs the slower full018/019/020 index scan; it does not include018's invalid
development candidate. Experiments 001–017 and arbitrary unregistered archives are
not supported by this release. Neither command runs a simulation or evaluator.

Use the left-hand real-history presets, select an individual, inspect its local
candidates, switch to Observer Perspective and follow actual event/point links.
Previous/next, significant-event navigation, play/pause, speed, a slider, tick jump,
next individual decision, source inspection and matched comparison are functional.
The interface renders at most 45 timeline entries near the selected moment.

In Observer Perspective use **Trace recorded chain (Observer)** for typed original
links from choice through consent, arbitration, scene/query, delivery and consequence.
The resource filters count completed encounters separately from dispositions:
ongoing/core85 after258 has65 zero-transfer Request→Leave encounters. Click a result
to open its exact scene and terminal event. Native null responses and provenance
receipts are distinct. The personal summary never equates intentions with execution.
See [causal-navigation acceptance](validation/causal-navigation-acceptance.md).

Download a current viewer export from Import / export. Importing it verifies the
trial against source membership again and rebuilds display projections; imported
claims, timelines and actor projections are not trusted. JSON is data, never code.
A local export does not grant an individual access to observer history.

Read the [practical guide](../../docs/development/history-explorer-user-guide.md),
[technical architecture](../../docs/development/history-explorer-architecture.md),
[validation record](validation/acceptance.md) and [export schema](schema-v1.json).
The original viewer [scope proposal](../../docs/development/experiment-viewer-scope.md)
remains a historical proposal; the guide distinguishes implemented coverage.

Developer checks:

```powershell
python -B -m unittest discover -s tools/history-explorer/tests -v
node --check tools/history-explorer/app.js
node tools/history-explorer/tests/perspective.test.cjs
node tools/history-explorer/tests/causal.test.cjs
node tools/history-explorer/tests/finite.test.cjs
python -B tools/history-explorer/tests/test_causal.py --all-sources
python -B tools/history-explorer/measure.py
```

Node is only an optional development syntax check. The application needs Python and
a modern browser. `tests/browser-smoke.js` is a reproducible browser interaction
recipe for Codex's browser-control runtime, with manual equivalents in the guide.
Measurements default to ignored `target/history-explorer/performance.json` and are
wall-clock engineering data, not deterministic experiment outcomes.

## Separate Experiment020 demonstration

The earlier020 delivery included an explicit small observer-only
`finite-demo.py` adapter supplies ten original020 histories with current cache stock,
pending attempts,local selection fields,grant/failure,meals,transfers and exact source
references. Serve `experiments/020` on localhost8766 and open `/demo.html`.
It has no actor mode or complete inquiry browser and never feeds cognition or
scientific validation. See the [actual acceptance and screenshots](../../experiments/020/viewer-acceptance.md)
for commands,source verification and its historical limitations. It remains preserved
as original viewer evidence; current first-class020 support is in the main application
above. Scientific status comes from the current review/protection documents.


## Experiment021: owned dated environmental knowledge

All214 original021 worlds are first-class in the same application,through
`experiment-021-trial/v1`/extractor4. Original ordinals are zero-based;labels and
identifiers include both Need/Acquire policy and knowledge mode. Use the021 presets
for useful continuity,independent rival depletion,stale full-cost work,mixed
individual distribution,expiry,nulls and stored-zero trust. No simulation is run.

Actor Perspective shows only its actual captured optional slot:receipt,date,age,
last-seen stock,own removals and upper estimate. Unknown is not zero;no archive lookup
fills a missing slot. Observer Perspective separately compares actual stock and
recorded knowledge,shows expiry/overwrite/grants/failures and follows immutable
source links. Retained relationships/food can survive quantity expiry. Comparison
keeps material reconvergence,semantic trust and raw stored-zero map shape separate.

[Actual acceptance and costs](validation/knowledge-integration/acceptance.md) and
[user walkthrough](../../docs/development/history-explorer-user-guide.md) describe
original81/82,31/32,1/2,162/163,121/122 and161/163. A fresh214index took31.43s in one
run;largest selected export28.83MB. Browser heap is unmeasured;Python process peak
about302MiB under concurrent validation. The complete corpus is not loaded into
the browser. Optional cost reproduction:`python -B tools/history-explorer/measure-knowledge.py`.
Viewer caches/exports are derived presentation,not foundation evidence or cognition.

## Separate native022 research preview

```powershell
python -B tools/history-explorer/concurrent_preview.py --port 8768
```

Open <http://127.0.0.1:8768/>. All100original022worlds are available through their
native `experiment-022-trial/v1` identities. Four actor lanes share one clock;
captured choices, independent consent, admissions, actual events, claims, meals,
need pulses and releases have exact original references. Actor Perspective uses
only its captured owned input. The server binds localhost, verifies source/archive
identity, caches at most2members and has no mutation/run endpoint.

This is a **separate read-only demonstration**, not full022main-Explorer integration.
The main three-person/multi-period schemas are not silently reused for four-person
multiple-scene worlds. [Nine-step walkthrough](../../docs/development/four-person-history-walkthrough.md),
[actual browser acceptance and screenshots](validation/concurrent-preview/acceptance.md).
There is no paired-comparison UI or semantic trust explorer in this preview.

Targeted checks: `python -B -m unittest discover -s tools/history-explorer -p test_concurrent_preview.py -v`,
`node tools/history-explorer/concurrent.test.cjs`, and `node --check tools/history-explorer/concurrent.js`.
Existing viewer suites remain separate. Scientific validation never depends on
these presentation adapters.

## Native four-person 022 support in the main application

Normal launch now includes all100 source-verified original022 histories, using
`experiment-022-trial/v1` directly. Select the022 presets or search022. Zero-based
original ordinals, archive hashes and source locks remain visible. Observer
Perspective adds four lanes on one shared axis; scroll the axis horizontally on
narrow screens. Intervals run from actual reservation to actual release. Click
an interval, cohort, scene or original-reference button to inspect the source.
Actor Perspective shows only the selected actor's captured input, including
recipient and resource decisions; it leaves atomic interiors and absent inputs
empty. Inquiry completion does not expose a partner's private response context.

Capacity comparisons distinguish first admission, executed event, missing decision
opportunity and selected choice at a common actor/tick. Food reconvergence need
not imply hunger or relationship reconvergence. The original finite public budget
and comparison-start phase remain explicit; this is historical playback, not a
continuous live world. The older separate preview remains available but is no
longer required for022. [Main integration acceptance](../../docs/development/main022-viewer-acceptance.md).


## Optional first continuous world (023)

```powershell
python -B tools/history-explorer/explorer.py --continuous experiments/023
```

This adds42original023research worlds through an explicit version7adapter; it does
not extend the protected001–023 foundation or adopt a simulation policy. Add another
`--continuous target/my-continuous-world.jsonl` to inspect an actual CLI-generated
custom run. Custom data is clearly labelled outside the registered matrix. Use the
[runner](../../prototypes/continuous-world/README.md) to generate it without source edits.
Default viewer behavior and older schemas remain unchanged.

The native body retains its original023physical-history schema. Checkpoint markers
show reporting/nominal/final boundaries and actual spanning commitments. Source
paths use `history.*` inside the original envelope; `checkpoints[*]` are separate
observer annotations. Actor inputs never inherit a checkpoint's observer state.
The actual provenance delivery route is linked even when the legacy native response
field is null. More contact or a receipt alone is not proof of material benefit.

[Actual browser/tests/cost acceptance](../../docs/development/main023-viewer-acceptance.md),
[walkthrough](../../docs/development/first-continuous-world-walkthrough.md).

## Optional activity valuation histories (024)

```powershell
python -B tools/history-explorer/explorer.py --continuous experiments/023 --valuation experiments/024
```

The explicit version 8 adapter adds all 56 original 024 histories without changing
default discovery or the version 7 adapter for 023. The original
`experiment-024-trial/v1` envelope and `experiment-024-physical-history/v1` body
remain lossless. Runtime-source, compressed/decoded archive and member hashes are
verified. Repeat `--valuation` with an actual prototype JSONL to inspect a custom
run, clearly outside registered scientific evidence. The prototype runner defaults
to Baseline; the viewer never executes or tunes a policy.

Use **024 · Costly earlier meal**, jump to tick 18, select individual 0 and choose
**Next decision of individual**. Actor Perspective shows captured own food/hunger,
local source receipt, legal alternatives and the original four-term score audit.
The matched columns show Baseline Consume35 versus MealRelief Consume70, against
the same Invite48. An exact score reference uses `valuations[*]`; physical and
decision references use `history.*`; checkpoints stay observer annotations.

Observer Perspective separately shows first choice/execution/material differences,
physical balances, final trust and actual hunger-area effects. The first different
score can precede the first different action: those are reported separately.
**Recorded activity evidence** finds owned-food/eligible-meal deferrals with an
actual positive-duration commitment, plus actual transfer and inquiry records.
Completion hunger is an observer value after all operations at that tick. Existing
cohort links and lanes connect send, reply, consent, admission and actual release.
A no-transfer scene is not automatically classified as useless.

Presets retain useful contact, costly earlier consumption, finite exhaustion,
unchanged information delivery, a complete null and longer-horizon evidence.
Query links distinguish native null from actual provenance/cognitive receipt.
Neither observer outcomes nor comparison summaries fill an absent Actor input.
Imported presentation fields are rebuilt from verified originals; altered original
terms or checkpoints are quarantined. Scientific validation remains separate.

[Actual browser, privacy, comparison and cost acceptance](../../docs/development/main024-viewer-acceptance.md).


## Optional finite material preparation histories (025)

```powershell
python -B tools/history-explorer/explorer.py --continuous experiments/023 --valuation experiments/024 --preparation experiments/025
```

The explicit extractor9 adapter (`preparation_history.py`) reads all50 original
025 envelopes without changing default discovery or older schema versions. It
checks the158 frozen source bindings and compressed/decoded/member hashes.
Exports preserve the complete original `experiment-025-trial/v1` envelope; imported
presentation fields are rebuilt from verified originals. No simulation executes.

Seven025 presets include later meals, stale failed work, competition for the final
batch, costly sharing, unknown-source nulls, escrow at the nominal cutoff and work
across reports. Actor Perspective uses only captured own input and five score
terms. An old raw upper bound is not replaced with current observer stock; an
evicted or replaced slot is not recovered from its before-record. Non-decision
material/checkpoint records do not invent an actor input.

Observer Perspective separates raw, owned escrow, ready cache, owned food, inert
waste and consumed units. Waste makes full-material persistence partly structural;
edible holdings, hunger and trust have separate comparison fields. Same-tick
material operations follow original numeric vector order and explicit receipt
links. `material.*`, `valuations[*]` and `checkpoints[*]` refer to the025 envelope;
`history.*` refers to its physical history. Links connect captured receipt, choice,
start, extraction, conversion, release and actual later debit. Fungible food does
not always identify a unique prepared unit, and sharing is not universal benefit.

[Executed tests and browser acceptance](../../docs/development/main025-viewer-acceptance.md).
The scientific gate remains separate: `validate025_evidence`. No025 protection,
policy adoption or026 follows from this read-only integration.
