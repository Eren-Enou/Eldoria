# History Explorer architecture

This is observational engineering infrastructure. The application has no import,
API or data path into simulation policy, ECS state, cognition, evaluator writers or
canon. Foundation correctness never depends on the viewer or its derived catalog.

## Delivery and first feasibility result

Python standard library serves static HTML/CSS/JavaScript on 127.0.0.1. No database,
framework, Node runtime, cloud account, telemetry, external font or Bevy renderer.
The initial full-scan feasibility spike verified/indexed336 real019 trials, generated
source-preserving default pair exports and rendered individual0's actual decision
and its separate objective execution. Full startup scanning cost about54s under
concurrent validation, so the shipping default uses lazy verification with a bundled
derived discovery catalog. The explicit --prepare path still performs the complete
scan and generates a catalog/default pair in ignored target/history-explorer.

The source archives remain authoritative. `demo/catalog-v1.json` is a discovery
accelerator produced from the real verified full scan, not new experimental evidence.
Its labels/metadata are unverified until a trial opens. A stale/forged record identity,
member digest, fingerprint, schema, source-freeze identity or archive digest is rejected.

## Pipeline and versions

1. Discover a registered archive/one-based JSONL ordinal from the derived catalog.
2. Check the repository's original archive and policy-freeze manifest identities
   against protected inventory where present. Verify the compressed SHA/length.
3. Stream the whole selected decoded archive, checking SHA/length including newline
   bytes. Bound individual JSONL records at64MiB. Retain only the requested record.
4. Verify the exact member digest (line without CR/LF terminator), semantic trial
   fingerprint and original identity/configuration. Dispatch explicitly to018/019/020.
5. Preserve the complete original Trial semantic JSON. Generate source-reference
   navigation and allowlisted actor projections. No policy or scoring is rerun.
6. Deliver one selected trial or pair to the browser, HTTP gzip when accepted. Rebuild
   projections from original data when loading a cache or imported viewer export.

The [v1 export wrapper](../../tools/history-explorer/schema-v1.json) includes schema,
extractor version, source metadata, unmodified trial, timeline and separate activity/
recipient/resource actor projections. The trial retains every original point, period,
step, final audit, needs and optional019 pauses. This first release chooses fidelity
over aggressive compaction: the largest17.12MB source becomes about29.47MB exported
because projections/indexes duplicate some captured input. It does not claim a compact
or resumable simulation-save format. HTTP compression reduces transfer bytes, not the
parsed object graph. Browser memory remains an additional cost.

018 and019 share the inspected physical/snapshot wrapper, but dispatch checks their
separate mode enums and019's pauses field.018 uses budget/period configuration and
can have zero-increment physics; the viewer preserves these facts and flags unequal
clock settings in comparisons. Valid final018 can be indexed with --include018.
001–017, arbitrary JSON/saves and018's invalid development bundle are unsupported.
Unknown or invalid input is rejected rather than normalized into another experiment.

020 has its own versioned `finite.py` adapter (`experiment-020-trial/v1`), export
extractor3 and original zero-based ordinals. It reuses only unchanged nested legacy
protocol record readers; no Trial is coerced or rewritten. Its actor projection
reads `choices[owner].acquiring_base.contact.input` for acquisition and the ordinary
activity decision otherwise, then allowlists the actual cache observation, policy,
lane flag and recorded candidates/selection. Final outcomes, pending rivals and
observer cache state never enter that projection. Import/cache display fields are
always rebuilt. Source identity uses020's source-freeze manifest; historical export
field `policy_freeze_sha256` names its digest for wrapper compatibility.

The first020 discovery scan streams164 original records, validates the envelope
and writes only ignored derived indexes. Cold selection rechecks full decoded archive
membership. Cached selection verifies original compressed bytes and semantic member
fingerprint and rebuilds projections; no complete corpus goes to the browser.
Server retains at most two selected histories, and UI renders45 nearby entries.
Selected largest020 source15.33MB becomes27.96MB exported; parsed/browser heap is
not measured. [Engineering costs](../../tools/history-explorer/validation/finite-integration/performance.json)
record28.33s first index,2.55–4.76s cold and0.48–2.99s cached selections in one run
under concurrent validation. These are not deterministic evidence or statistics.

020 typed chains link exact observation/choice/admission/attempt/outcome references.
Explicit skipped-response entries link busy exclusion to actual unavailable invitations.
Later owner operations are labeled chronological context unless stronger material
evidence exists. For the sole-initial-unowned-unit case, conservation supports a
grant→transfer→recipient-meal material link without claiming it solely caused consent
or assigning invented IDs to fungible food. Same-tick order follows the source
protocol: resource action, due claims, meals, then later cohort decisions.

## Actor and observer ownership

Activity inputs come only from Decision.contact.input: own Agent/profile, now/end,
public people and the captured local query input/candidates. Explicit own fields
are allowlisted. Recipient views use Response.input.own at that response boundary,
not the earlier activity input. Resource views allow only the acting individual's
captured decision fields, public observation, candidate scores, directed trust,
episodic contribution and cause IDs. They exclude partner balances, other agents,
objective events/roots and later knowledge. No prefix of observer history is joined
to fill a missing actor memory, belief, assessment or learning cell.

`perspective.js` additionally allowlists the selected own Agent from an **explicit**
original point reference. It never substitutes a nearest snapshot or reads final
cognition/events to fill that view. Deep copies detach the display from source.
A captured point can show retained memory/trust without implying a selected action;
concerns/assessment/source knowledge absent from that point remain not captured.
Its actual JS function is tested against real private twins, observer-history poison,
legacy persistence, detachment and missing-reference cases.

The UI consumes these projections for Actor Perspective, including its exact-value
panel, cards and paired actor columns. Whole-world comparison findings and private
state are observer-only. Other event labels are masked in the actor timeline.
Public metadata identifying an archive/scenario is navigation context, never claimed
as an actor input. Observer-reference buttons explicitly change perspective.

Observer Perspective can inspect point Agents and recorded resource/query/audit
records. Exact associated point IDs are used where recorded; otherwise the last
captured snapshot at/before the tick is labeled with its actual timestamp. It is
not interpolated; it can precede the selected action or include other phases at the
same tick when no exact point is associated. The UI states this limitation. Concern and
assessment audit prefixes use recorded counters. They never become actor cognition.

## Timeline and causal references

Each derived navigation item carries original source path, type, tick, point and
period/step references where available. Separate kinds represent window, decision,
provisional response, arbitration, commitment start/release, resource event, meal,
query completion, need progression, gap and snapshot. Namespaces remain trial-scoped;
an event ID is never substituted for a receipt, query, assessment or point ID.

Global ordering uses world tick. Same-tick order uses original step ordinals,
point/counter references and the documented protocol phases: captured state/decision,
elapsed progression, resource action then any independent meal, response/matching,
actual execution and after-point. A prior completion precedes a later decision at
that tick. These ordering positions are viewer metadata, not invented fractional
ticks or cognition. Atomic query completion uses its enclosing recorded after-point;
no unrecorded internal decision snapshots are manufactured.

Direct reference buttons follow step before/observed/sent/admitted/after points,
scene event records and actual native query IDs. Chronological adjacency alone is
not asserted to be a sole causal explanation. Old event interpretations are retained;
wrong evidence is never rewritten as objective truth. Voluntary intent, consent,
arbitration and execution remain separate fields.

Pre-epoch preparation remains in the source/export but is outside the current
period-based replay UI. It is not silently removed from the trial.

The browser renders at most45 timeline entries near the current index. It loads a
single trial or pair, not336 parsed histories. Replay advances the index; it never
calls an evaluator or current policy. Playback can pause, change speed or jump ticks.

## Typed causal navigation, extractor2

`tools/history-explorer/causal.py` is a policy-free018/019 protocol reader. It builds
observer-only nodes with verified trial identity, exact source path, original ID
and ordinal sequence, captured tick, actor/recipient, source kind, outcome, containing
navigation anchor and explicit before/after points where present. Edges name their
basis: exact invitation in a recipient's incoming list, allocation/commitment identity,
scene ID, actual query ID, typed receipt, assessment receipt or recorded memory-cause ID.
Neither temporal adjacency nor shared numeric IDs across namespaces creates an edge.

The complete source Trial remains unmodified. Nodes carry small metadata rather than
copies of full decision inputs. Whole-world actor/state records are resolved from
the original Trial only when an observer opens an exact source link. All explicit
paths are checked with a strict non-evaluating resolver. Missing/duplicate referenced
IDs reject extraction. Before-epoch memory causes can be inspected as original
records without inventing a replay moment. Later recorded memory-use links are
contributions, not claims of isolated causal sufficiency.

The v1 display wrapper has additive `causal` metadata and extractor_version2;
extractor1 imports remain accepted after source verification, with all derived
fields discarded/rebuilt. No viewer module is imported by foundation validation.
Pure `causal.js` helpers filter actual scene completions separately from invitation
dispositions and preserve timeline anchor ordering at equal ticks. Scene filter
navigation anchors the terminal resource event and separately displays the exact
scene record. Result DOM is bounded at80; original timelines remain bounded at45.
The observer-only personal summary uses recorded activity/meal/query/action data.

Native response, provenance query, provenance receipt, retained assessment and
observed learning progress are separate typed nodes. Receipt lookup matches its
namespace and owner; it never equates a native null with no provenance delivery.
Support/status changes do not establish truth or usefulness for a later action.
On switching to Actor view the entire observer section is hidden and its rendered
source/chain/summary contents are cleared. None enters actor projections or policies.
See [acceptance](../../tools/history-explorer/validation/causal-navigation-acceptance.md)
for real602-source traversal, fixture/corruption tests, browser node clicks and costs.

## Comparison scope

Read-only Python comparison uses captured state, choices and actual records. It
checks initial point/profiles, configuration, physical clock and endpoint equality.
The first differing voluntary choice includes target/type/method, with captured
input/state matching flags. Different decision schedules receive a qualified label.
First executed difference includes meals, resource actions and completed inquiry
selection/response/cost; a separate field isolates resource events. Source references
are carried for inspection, not treated as outcome differences in themselves.

States align by world tick. Shared-tick material comparison uses the last original
point at that tick; each displayed column labels its own captured state timestamp.
Temporary material difference plus equal final material is reported as material
reconvergence. Equality of the entire original final Snapshot is separately computed,
including relationships/cognition/history stored there. It is not a claim that the
whole Trial's activity scores, clock audit or every trajectory record is equal.
No aggregate welfare ranking, ecological frequency or scientific acceptance is inferred.

## Cache, imports and local security

Cache IDs depend on original trial identity, archive digest and export schema. Cache
files live under ignored target by default; experiments/ is forbidden as a cache
location. Writes use atomic derived-file replacement. In-memory retention caps at
two histories. Source compressed hashes are rechecked on subsequent loads. Cached
trial fingerprints are checked; presentation/status fields are always rebuilt.

Imports require the supported schema and an exact verified registered member. An
import cannot forge its source status, actor view or timeline. Changed/unregistered
trials and unsupported schemas are quarantined through explicit errors. Source
archives are never repaired or regenerated. The viewer does not accept arbitrary
server filesystem paths from requests. Only fixed assets and registered IDs are served. The export route supplies a standard
HTTP JSON attachment; it does not rely on browser-specific Blob downloads.

The server binds loopback, checks Host/Origin, exposes no mutation/evaluation endpoint,
limits import bodies to64MiB and serializes archive/import work to bound decoding/cache writes. A threaded HTTP
listener allows browser idle preconnections and static assets without blocking that work.
JSON strings render through textContent, not innerHTML/eval. CSP permits only local
assets/connections and blocks framing. No external account or transmission is needed.
As a local research tool it is not a hardened multiuser internet service.

## Verification and costs

See [acceptance evidence](../../tools/history-explorer/validation/acceptance.md),
[data tests](../../tools/history-explorer/tests/test_explorer.py) and
[engineering measurement tool](../../tools/history-explorer/measure.py).
A collapsible narrow-screen library and native panel anchors retain access to the
current moment/perspective without scrolling through the whole trial catalog.

Tests exercise real source membership, hashes, schemas, phase/reference ordering,
physical records, private twins, real eviction, missed inquiry, gaps, comparisons,
forged caches/exports and local HTTP boundaries. Browser tests are separate from
backend correctness checks. Wall-clock/memory observations remain outside original
experiment evidence. No simulation-throughput benchmark is warranted by this consumer
or the validator-only Rust changes; no throughput improvement is claimed.

Future improvements should be driven by use: optional018 lazy catalogs, segmented
long-trial payloads, richer typed causal links and prospective capture of local
assessment/progress inputs. They must not make unavailable historical cognition
appear captured or feed observer history into individuals.

## Separate Experiment020 demonstration

The main018/019 reader remains unchanged by020. An explicit small observer-only
`finite-demo.py` adapter supplies ten original020 histories with current cache stock,
pending attempts,local selection fields,grant/failure,meals,transfers and exact source
references. Serve `experiments/020` on localhost8766 and open `/demo.html`.
It has no actor mode or complete inquiry browser and never feeds cognition or
scientific validation. See the [actual acceptance and screenshots](../../experiments/020/viewer-acceptance.md)
for commands,source verification and limitations.020 remains unprotected research.
