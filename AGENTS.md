# World of Individuals engineering instructions

## Context and inspection workflow

Start with `docs/architecture-map.md` and
`docs/simulation/EXPERIMENT_HISTORY.md`; reuse their source/report pointers instead
of reconstructing the architecture from every report. Follow
`docs/context-workflow.md`. These summaries aid navigation; source, experiment
reports and mode-specific contracts remain authoritative.

Inspect Git status first and preserve unrelated work. Prefer `rg` in mapped files
or directories, scoped globs, symbol/heading searches and bounded reads. Avoid
repeated full-repository listings and unchanged-file reads unless scope or new
evidence requires them. Do not dump full generated traces into context: extract
needed fields/counts and inspect the relevant causal slice.

Use targeted tests while iterating, then all required completion checks below,
including the read-only archive/replay gate. Keep verbose logs in task-specific
temporary files; summarize exit status, counts and failures. Inspect failures
without hiding them behind truncation. Do not repeat passing checks at unchanged
code/fixture revisions without a reason. Benchmark documentation-only changes
only if a relevant workload or performance question requires it.

Update affected map/index entries when ownership, mode status or findings change.
Carry concise checkpoints with objective, revision, changed files, decisions,
completed checks and remaining work; link detailed evidence. Verify changed files
after compaction rather than assuming a checkpoint is current.

Full codebase audits remain appropriate when requested, for cross-cutting changes,
unknown dependencies, stale documentation, integrity failures or unexplained
regressions. Expand inspection as necessary and state the reason/scope. Token
economy must not weaken locality, retention contracts, validation or negative results.

## Protected project contracts

Cross-experiment enforcement map: `docs/experiment-contract-001-019.md`.
The historical `docs/experiment-contract-001-018.md` remains authoritative for001–018.
The historical `docs/experiment-contract-001-017.md` remains authoritative for001–017.
The historical `docs/experiment-contract-001-016.md` remains authoritative for001–016.
The historical `docs/experiment-contract-001-015.md` remains authoritative for001–015.
The historical `docs/experiment-contract-001-014.md` remains authoritative for001–014.
The historical `docs/experiment-contract-001-013.md` remains authoritative for001–013.
The historical `docs/experiment-contract-001-012.md` remains authoritative for 001–012.
The historical `docs/experiment-contract-001-011.md` remains authoritative for 001–011.
The historical `docs/experiment-contract-001-010.md` remains authoritative for 001–010.
The historical `docs/experiment-contract-001-009.md` remains authoritative for 001–009.
The historical `docs/experiment-contract-001-008.md` remains authoritative for 001–008.
Preserve its mode-specific retention contracts: episode eviction does not erase
separately established trust, expectations, concerns or legacy evidence consequences.
Never recover unavailable cognition merely from observer history. Run the read-only
`cargo run --release --locked -j 1 --example validate_contract` archive/replay gate
alongside required checks; never regenerate frozen experiment evidence in place
for an integrity audit.

Experiment 009 preserves Grouped 008 assessment and capacity 32. FIFO remains the
default; Quality and Salient are explicit research modes using only retained local
received quality and, for Salient, current active own concern importance. Preserve
deterministic ties, mistaken retention, new-receipt redelivery and retained-only
attribution revision. Do not infer improved executed behavior from 009 policy probes.

Experiment 010 is accepted with qualification. Keep unchanged-path existence evidence
separate from opt-in CurrentNeed findings. CurrentNeed is a local confidence-discount
heuristic, not calibrated information value or future relevance, and remains experimental.
Preserve E_Q (changed inquiry), E_A (same inquiry/different assimilation) and ambiguous F
as distinct classes. Later correction cannot undo executed material history. Keep all
negative results and the cache-free read-only 010 gate in foundation validation.

Experiment 011 is a protected scientific result: StatusValue unadopted, negative
conclusion confirmed with qualification. Preserve its exact formula, integer ties,
controls and caps. Keep positive novelty, retained support/status progress, learned
source/method expectations and material consequences distinct. Protect the
same_uncertainty failure, far_resolution5→24 and same_importance0→50 partial gains,
and review corrections without rewriting frozen prose. No universal utility ranking
or long-term benefit of failed-inquiry learning is established. Keep cache-free011
validation in the normal foundation gate; do not regenerate frozen evidence.

Experiment 012 is a protected scientific result — Contextual and GlobalProgress
retained as opt-in research mechanisms; ACCEPT WITH QUALIFICATION; neither adopted
as default. See the unchanged [scientific review](docs/experiment-012-review.md).
Its Contextual/GlobalProgress modes are prospective and opt-in; keep old novelty
learning separate from observed assessment change. New per-actor cells cap at32,
use insertion FIFO, and contain no event content. Silence does not reveal hidden
motives. Preserve useful, misleading, missed and costly choices. Progress learning's
material chain is shared by both new modes; context separation demonstrates allocation,
not unique material superiority or minimal-state necessity. Preserve repair disclosures
and scheduled/automatic counts. The shared read-only012 gate is in normal foundation
validation and standalone `validate012`; never regenerate frozen evidence.

Experiment 013 is a protected scientific result — ACCEPT WITH QUALIFICATION;
see [scientific review](docs/experiment-013-review.md). Pulse/Ungated are opt-in
transient prior reconsideration over existing own concern age and failed006 attempts;
they preserve012 updates and add no persistent cognition. Preserve earlier discovery,
wasted probing, public-offer crowding and seed-dependent legacy trust effects. Do not
infer global superiority or promote either mode. Shared013 validation is in the normal
foundation gate and standalone validate013, including both independent readers.
Never regenerate frozen evidence; preserve all negative evidence and the retained preview.
Preserve the review corrections: high-support longitudinal C64 is not B64; late
episodic valence0 reflects the last-four partner-memory window, not episode eviction.
The gate's schedule-phase robustness and effort justification remain unestablished.

Build progressively toward a persistent fictional world. Experiment 001 is only
resource interaction, subjective memory, and subsequent behavioral consequences.
Keep systems replaceable; do not add speculative civilization or live Observer/Book
simulation features. The separately authorized read-only History Explorer is an
observational consumer, not a simulation controller.

Constitution: (1) identifiable causality, (2) information locality, (3) persistent
consequences, (4) distinguish individual perspective from objective events,
(5) explicit resource accounting, (6) controlled reproducibility, (7) no fixed
narrative outcomes, (8) historical traceability, (9) replaceable behavior systems,
(10) measured complexity. Preserve all ten when changing code.

Policies receive only local observations and their actor's state. Never expose
private partner components. New actions require legality checks, resolution,
accounting, structured records, and termination coverage. Do not serialize ECS
entity handles. Bound episodic memory and document other growing storage.

Before changing behavior, document the hypothesis and tradeoffs. Run cargo fmt
--check, cargo clippy --all-targets -- -D warnings, cargo test, and cargo bench
--bench throughput when relevant. Update Experiment 001 evidence after rule
changes. Report failures and limitations. Keep generated JSON reproducible;
wall-clock measurements belong in separate benchmark output. Do not claim
consciousness or broad social emergence from these experiments.

## Git Commit and Push Policy

Experiment 014 is protected ACCEPT WITH QUALIFICATION; see the scientific review
and `docs/experiment-contract-001-014.md`. Preserve actual versus reserved time,
single-use current windows, costly failures and simple-priority gains. No Local or
LocalPulse default adoption. Conservative resource admission is a bound, not actual
expected duration; rejected recipient outcomes remain unknown. The normal foundation
gate and standalone014 share exact replay and both independent readers. Preserve
all168 frozen files, including the disclosed post-evaluation13-tick diagnostic.
Experiment015 and016 protection preserve their separate accepted scientific reviews.

Experiment015 is protected ACCEPT WITH QUALIFICATION; see `docs/experiment-015-review.md`
and the001–015 contract. Preserve independent proposals/recipient consent, provisional
acceptance, exact ordering/time, executed resource/query consequences and missing
invitation-feedback cognition. Simple/History/NoHistory remain research; Continue scale
coupling prevents isolated superiority claims. Order is legal/reproducible, not fair.
Keep all186 frozen artifacts and both readers in shared/standalone015 validation.
016 is separately protected with qualification; no default adoption.

When a requested task is complete:

1. Run all required formatting, Clippy, tests, deterministic replay checks, and relevant evaluators/benchmarks.
2. Do not commit if required validation is failing.
3. Review `git diff` and ensure only files related to the completed task are included.
4. Commit the completed work with a concise, descriptive commit message reflecting the actual change.
5. Push the commit to `origin main`.

Prefer commit messages such as:

- `experiment-007: add provenance-aware testimony`
- `experiment-006: add adaptive inquiry`
- `foundation: compact causal audit history`
- `docs: add Oceanid worldbuilding notes`
- `fix: preserve provenance locality during inquiry`

Do not use vague messages such as `updates`, `changes`, `codex work`, or `fix stuff`.

Do not rewrite, squash, force-push, or alter existing published history unless explicitly instructed.

If `main` has changed remotely since the task began, do not force-push. Reconcile safely or stop and report the conflict.

Experiment 002 adds event-specific structured communication, uncertain beliefs,
revisable interpretations and explicit food consumption. Keep Experiment 001's
policy/report defaults and historical evidence intact; save compatibility evidence
under the new experiment. Testimony must not consult objective truth. Disclosure
must remain an explicit narrow observation channel. Preserve original event
interpretations; record revisions separately and replay trust with replacement
contributions, including saturation. Cap beliefs as well as episodic memory.

Experiment 003 intentional communication is opt-in and automatically follows new
refusals. Preserve old defaults and evidence. Communication policies receive only
local TalkInput; never pass objective truth or deceptive intent to a listener.
Credibility is directed and separate from trust; update it only from legitimate
claim/evidence comparisons. Bound communication episodes, validate every action,
account for time and preserve explicit termination. Profile changes are recorded
experimental interventions, not permission to schedule or force agent messages.

Experiment 004 foresight is opt-in. Predictors receive only own local TalkInput
and Context; never expose sensor outcomes, channel, seed or private partner state.
Learn only from an actual public response to a selected action; exclude forced
terminal closure from challenge learning. Keep forecasts/errors separate from
objective events and subjective beliefs. Fallible readings are not truth; preserve
source identity, uncertainty under conflict, effort accounting and old modes.

Experiment 005 concerns are opt-in after foresight. Keep unfinished intent separate
from memory, belief, trust and prediction. Cap each individual's concern list at
eight, audit capacity abandonment/eviction, and preserve stable event/receipt links.
Follow-up policy inputs contain only own state and public context. Reopen only at
a later relevant encounter, at most one matter per participant, with explicit time
cost and voluntary bounded response. Resolve from local support, never objective
truth; repeated evidence cannot manufacture certainty. Preserve older modes and
archive compatibility under the new experiment rather than rewriting old evidence.

Experiment 006 adaptive inquiry is opt-in after concerns. Keep concern importance
separate from source × strategy usefulness. Inquiry policies receive only local
Input and public co-presence/offers. Cap estimates, signatures, episodes, contacts,
offers and capabilities; audit eviction. Learn only selected valid responses;
repeated claims/readings/announcements cannot manufacture information or credibility.
Pause a method without abandoning or resolving its concern. New evidence offers
are voluntary; configuration never schedules a message. Mark attempted offers so
unfulfilled promises cannot renew value indefinitely. Keep evidence provenance
immutable, sensor channels/seed/outcomes private to the resolver, and all old modes
and reports intact. Save compatibility and performance evidence under 006.

Experiment 007 provenance is opt-in after inquiry. Distinguish objective roots and
relay parents from bounded locally known attribution and confidence. Never give
policy inputs hidden roots, relay depth, sensor outcomes or a global provenance
graph. Inspection and sharing remain voluntary; inspection samples only after a
local decision. Explicit attribution may replace an earlier local assumption;
hidden attribution can cause overconfidence. Keep communicator credibility separate
from evidence independence and compare only legitimately received fallible evidence.
Bound knowledge, hints and comparison keys at 32, audit FIFO eviction, and do not
recover evicted acquisitions from the objective archive. Preserve native self-
knowledge responses and all 001–006 defaults/evidence. Archive new compatibility
under 007. Do not infer statistical independence or social emergence from root IDs.

Experiment 008 assessment is opt-in after provenance and prospective only. Use one
bounded local basis across native readings and legitimately received reports;
never import observer archives or equate undisclosed origins using objective roots.
Keep origin/content/received quality separate from delivery receipt namespaces.
The same retained acquisition keeps its origin across native/provenance delivery.
Keep cap 32, explicit FIFO eviction and immutable assessment records. Order/repetition
controls require identical retained inputs and fixed weights; invariance need not
survive forgetting or legitimate credibility changes. Preserve original events,
saturated trust replay and older mode outputs. Archive new evidence under 008.

Experiment016 is protected ACCEPT WITH QUALIFICATION as science; NO JUSTIFICATION
for adopting two-cell Typed; see docs/experiment-016-review.md and001–016 contract.
Preserve own post-wave feedback,caps2/Recent1,6/12/2 expiry,FIFO/overwrite,type separation,
no inquiry/trust fabrication from failure,Continue0 as separate recipient control,
provisional consent,order dependence,costly until57 missed inquiry and narrow seed1
Recent refusal/trust/consumption gains. Expiry is lazy; State equality alone is
agents/counters, with stronger input/prefix checks for the positive slice. Keep all207
artifacts and three independent readers in shared/standalone016 read-only validation.
No default adoption.017 was preregistered after016 Gate B validated/committed/pushed.
Experimental scientific acceptance does not establish generally useful cooldowns.

Experiment017 is protected ACCEPT WITH QUALIFICATION;see docs/experiment-017-review.md
and docs/experiment-contract-001-017.md. No default adoption.
Preserve resource commitment/current availability versus willingness,independent
consumption/contact choices,actual shared time/consent/release and zero new cognition.
Inquiry stays atomic,three actors cannot form disjoint meetings,no postsetup hunger/
profile/busy scripts. Preserve full-state contact-only null and narrow consumption
chain/foregone transfer,NoHistory gains/losses,order effects and finite stocks.
Shared foundation/standalone validate017 protect228 artifacts and exact original
replay,all three readers,188 full-state nulls and narrow7/188 material effect.
No recurring needs or disjoint concurrency are retroactively imposed on017.
018 may begin only after the001–017 protection gate validates and is committed.
018 protection requires its separate independent review/integrity gate.

Experiment018 is independently reviewed ACCEPT WITH QUALIFICATION; see
docs/experiment-018-review.md, experiments/018/results.md and specification.md. Preserve
epoch-based shared need progression, saturation, consumption after elapsed pressure,
one-tick idle cadence, no mandatory period meals and all seven control modes.
Three actors, one resource commitment and atomic inquiry are deliberate limits.
Extra opportunities do not improve consumption totals in these fixtures; preserve
the same-input foregone-transfer chain, reconvergence, hungrier outcomes, ordering
and history-ablation reversals. No new cognitive cells or default adoption.
Keep separate read-only validate018, exact replay and independent physics/cognition
reconstruction; retain the invalid initial clock-hook candidate and regression.
Historical001–018 protection keeps all269 original artifacts,including13 invalid
development files,remain immutable. Shared foundation/standalone validate018 include
all three independent readers and the original exact replay.019 was separately
authorized after that gate was validated/committed/pushed;its later qualified review
and explicit protection do not adopt a default.020 requires its own authorization.

Experiment019 is independently reviewed and protected with qualification;
see experiments/019/specification.md and results.md. Baseline/EatFirst/NoHistory reuse
018 exactly at default timing. Deadline uses only the last safe public admission slot;
Scale90/110 perturb only top Invite scores with integer truncation. Keep the six rules
and fixtures frozen. Window/gap comparisons share total elapsed horizons;rate/stock/H
changes are separately labeled sensitivity. No new cognition or planner. Preserve
mostly inert small perturbations,seed/private-cooperation reversals,317 full nulls,
conservative exclusion witness,and pressure-time gains caused by ineffective contact.
Normal foundation and standalone validate019 share contract019.rs and four read-only
readers; protect296 original artifacts and qualified negative/null results. See
docs/experiment-019-review.md and docs/experiment-contract-001-019.md.
No default adoption.020 is separately authorized prospective research,not part of this protection.
History Explorer is separately authorized observational engineering under tools/history-explorer;
never feed its caches/exports into actor cognition or make it a foundation dependency.
Its explicit018/019 adapters preserve source identity and captured local/owned-state
boundaries; do not infer missing cognition or explain old decisions with new policies.
Run its Python data tests, actual JS owned-point tests when Node is available, and
real browser smoke checks for changes to ingestion, projection, replay or comparison.
See docs/development/history-explorer-user-guide.md and
docs/development/history-explorer-architecture.md.

Experiment020 is opt-in finite environmental acquisition research,not protected or
adopted. Read experiments/020/specification.md,design-gate.md,results.md and
verification.md. Preserve the declared five controls,equal potential supply,four-tick
baseline/two/eight sensitivities and deterministic claim ranks. Claims reserve actors,
not cache units;completion rechecks stock and atomically conserves food. No observer
history,future information or partner-private state enters actor inputs. Acquisition
adds no learning mechanism;existing bounded learning remains. Preserve failed
claims,missed help,unequal sharing,nulls and later depletion. Atomic inquiry cannot
leap a pending claim's deadline;do not silently replace this scheduling qualification.
The standalone read-only validate020 does not extend validate_contract or001–019.
Independent scientific review/protection remain separate tasks. Do not begin021.
Its separate finite-demo adapter exposes original020 observer records only;it must
not be silently ingested as018/019 or become a simulation/cognition dependency.
