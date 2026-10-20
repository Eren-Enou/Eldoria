# World of Individuals

Headless Rust + Bevy ECS research foundation for individuals whose experiences
affect subsequent decisions. Experiment 001 implements food interactions, local
information, subjective episodic memory, learned relationships, reproducible
scenarios, and causal reports. It does not demonstrate consciousness or complete
personality. No graphics or full Bevy engine installation is needed.

For engineering navigation, start with the [concise architecture map](docs/architecture-map.md),
[experiment index](docs/simulation/EXPERIMENT_HISTORY.md) and
[selective context workflow](docs/context-workflow.md). For an integrity audit use
the read-only `validate_contract` gate described below; run/evaluate commands can
overwrite evidence and should be used only for authorized target generation.

Current protected endpoint: [qualified001–023 protection](docs/experiment-contract-001-023.md),
386original artifacts. The [independent023review](docs/experiment-023-review.md)
accepts with qualification. [Experiment 024](experiments/024/results.md) is complete
causal research, ACCEPT WITH QUALIFICATION and unadopted. Its separate 56-world
evidence is outside the protected foundation. No material production or025 exists.

## First Continuous World - optional023prototype

Four persistent individuals now execute one bounded continuous-opportunity world
across reporting checkpoints. The 42-world scheduling study is independently accepted with qualification and
protected through 001–023.
More opportunity can delay meals and increase hunger. No production/default adoption.

```powershell
cargo run --release --locked -j 1 --manifest-path prototypes/continuous-world/Cargo.toml --target-dir target/continuous-world --bin continuous-world -- run target/my-continuous-world.jsonl --case paired --ticks 192 --period 11 --seed 42
python -B tools/history-explorer/explorer.py --continuous experiments/023 --continuous target/my-continuous-world.jsonl
```

Choose a NEW output filename. The192ordinary ticks are followed by33charged drain
ticks; for paired: start6, nominal198, final231. Choose the actualprototype button
in the main viewer. Add `--port 8769` if8765is occupied. The viewer reads completed
history; the CLI executes the actual world. [Runner guide](prototypes/continuous-world/README.md),
[walkthrough](docs/development/first-continuous-world-walkthrough.md),
[results](experiments/023/results.md), [verification](experiments/023/verification.md),
[readiness/future material decision](docs/development/continuous-world-readiness.md).
Read-only023gate: `cargo run --release --locked -j 1 --example validate023`.
024 is a separately preregistered valuation study; no material production follows
from either result.

## History Explorer — inspect actual recorded lives

Launch the read-only local application with Python 3.10+; Rust is not needed to view
existing archives:

```powershell
python -B tools/history-explorer/explorer.py
```

It opens a real Experiment019 history,with individual perspectives,recorded replay,
causal event links and matched comparison. Source hashes/membership are verified
before display. All164 original020 histories are now first-class,with captured
actor cache observations,separate environmental observer state,claim/failed-time
chains and physically qualified comparisons.020 ordinals are zero-based;019 labels
remain one-based. No simulation,policy,canon or archive is changed. Optional018 support
is available with `--include018`. [Application and developer checks](tools/history-explorer/README.md),
[practical guide](docs/development/history-explorer-user-guide.md),
[architecture](docs/development/history-explorer-architecture.md) and
[real-browser acceptance](tools/history-explorer/validation/acceptance.md).

Start with the [guided first investigation](docs/development/history-explorer-first-walkthrough.md).
The [019 observational report](docs/development/history-explorer-observations-019.md)
and [ranked next priorities](docs/development/post-019-observation-backlog.md) examine
actual consequences, private reversals and repeated contact after finite food exhaustion.
These are post-evaluation observations, not a new experiment or policy adoption.

The [phase2 integration assessment](docs/development/world-of-individuals-phase-2-assessment.md)
ranks the remaining community-readiness barriers from actual evidence. The
[continuation checkpoint](docs/development/continuation-checkpoint.md) and
[delivery audit](docs/development/phase-2-delivery.md) record completed gates and
next review boundaries;no autonomous community or sustainable economy is claimed.

## Four-person concurrency research

Experiment022 executes100 registered four-person worlds, comparing one versus two
simultaneous social commitments under one clock. Independent reconstruction and
exact byte replay pass; the finding is ACCEPT WITH QUALIFICATION, with harmful,
failed and complete-null cases preserved. It remains opt-in research, now part of the
protected001–022 foundation as qualified scientific evidence, without adoption.
See [independent review](docs/experiment-022-review.md), [contract](docs/experiment-contract-001-022.md),
[integrity audit](docs/integrity-audit-001-022.md), [results](experiments/022/results.md),
[verification](experiments/022/verification.md) and the
[concurrency architecture](docs/development/concurrent-world-architecture.md).

Read-only reproduction: `cargo run --release --locked -j 1 --example validate022`.
The separate `experiment022` runner requires a new isolated JSONL output filename;
never regenerate frozen evidence in place. No default adoption is authorized;
023requires its separate prospective scheduling gate.

Inspect original four-person histories in the main read-only History Explorer:
`python -B tools/history-explorer/explorer.py`, then choose an022preset.
See [main integration acceptance](docs/development/main022-viewer-acceptance.md).
The earlier separate preview also remains available:
`python -B tools/history-explorer/concurrent_preview.py --port 8768`, then open
<http://127.0.0.1:8768/>. The [nine-step walkthrough](docs/development/four-person-history-walkthrough.md)
follows real invitations, consent, simultaneous transfers and later consequences.
This is an authentic research demonstration; full022main-Explorer integration
remains separate. The main application also supports all214original021histories,
with owned dated knowledge and separate observer stock.

The [community-readiness assessment](docs/development/four-individual-community-readiness.md)
finds a working controlled prerequisite, not continuous community life. The
[finite-input material design](docs/development/sustained-material-opportunity-design.md)
is prospective only; no production mechanism or023 was implemented.

## Install and run

Install Rust with Cargo and the Windows MSVC build tools (or a Rust-supported native
toolchain on another OS). Minimum Rust is 1.95; verified here on Rust 1.99.0,
Windows x64. Bevy ECS 0.19.1 is pinned; Cargo.lock pins resolved dependencies.

From this directory:

```powershell
cargo run --locked --release -- --scenario all --seed 42 --population 1000 --output experiments/001/results/seed-42
cargo run --locked --release -- --scenario E --seed 42 --output experiments/001/results/history
cargo run --locked --release --example evaluate
cargo bench --locked --bench throughput
```

Use `--help` for options. Scenarios are A (first), B (repeated), C (conflicting
needs), D (different interpretations), E (altered history), F (reproducibility),
G (population), or `all`. Population applies to G; defaults to 1,000. Odd counts
leave one agent uninvolved. All selected traces are replayed to verify equality.

`--output DIR` writes full `report.json` and readable `report.txt`; without it the
CLI prints the readable report. JSON contains generated agents, all stage before/
after states, explicit interventions, candidate scores, local observations,
actions, balances, outcomes, memory event references, and relationships. Human
output caps large stages; JSON is complete. No wall-clock data contaminates
deterministic reports. Reports overwrite files in the chosen directory.

## What the experiment establishes

With seed 42, agent 0's current food, hunger, traits and expectation are identical
in the altered-history probes. After receiving help it offers food (score 38);
after refusal it withdraws (offer score -44). Removing learning also produces
withdrawal (offer score -10). Earlier event IDs are recorded as causes of the
learned scores. The histories themselves are produced by the interaction policy
under documented experimental interventions, not inserted memories.

All 128 seeds in the evaluation show this controlled difference. Natural generated
encounters produce agreement, refusal, and withdrawal. See
[the findings](experiments/001/results.md) for exact measurements and limits.

## Architecture and verification

The [001–017 contract and coverage map](docs/experiment-contract-001-017.md)
documents mode-specific locality, retention, persistent consequences, exact bounds
and regression coverage, carrying forward the unchanged historical maps and 010's
qualified scientific acceptance, 011's qualified negative adoption result and012's
qualified experiential-learning findings with both new modes retained as research,
and013's qualified transient reconsideration with both review corrections protected.
015's independent consent and016's bounded contact-feedback findings are protected
with their recipient-control, ordering and negative-adoption qualifications.
017 protects real commitments and its narrow consumption chain,188 full-state nulls,
ordering sensitivity and lack of recurring needs;no research policy becomes default.
CurrentNeed remains experimental and opt-in; StatusValue remains unadopted.
Verify the protected foundation without writing archives (Python standard library
is required for direct gzip/SHA-256 verification):
`cargo run --release --locked -j 1 --example validate_contract`.
The independent `validate009` example uses the same direct frozen-archive checks.
The independent `validate010` example verifies 010 directly and reconstructs the
accepted unchanged-path chain and separate E_A/F classes without generated caches.
`validate011` protects all three011 archives, exact replay and reviewed negative/partial
findings without generated caches; accepting scientific evidence does not adopt its heuristic.
`validate012` and `validate013` share their respective foundation helpers;013 also
enforces the second independent reader,151-file inventory and corrected interpretation.

One cohesive Agent ECS component, resources for ordered scenes/history/clock, and
one single-threaded scheduled resolver keep this first experiment small. A pure
replaceable utility policy sees only the actor and its observation. Five actions
are validated before applying effects. Six turns is the scenario limit; memories
hold 16 episodes while aggregate trust persists. Objective history is separate
from subjective interpretation. [Architecture](docs/architecture.md) explains
tradeoffs, score rules, and future boundaries; [AGENTS.md](AGENTS.md) preserves the
simulation constitution for future work.

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests cover seeded generation, local information, reproducibility, actual history
effects, independent interpretations, resource conservation, legal actions,
event/state replay, termination, invalid participants, overflow, memory eviction,
and 1,000-agent populations. Seed sweeps test many inputs deterministically; they
are not exhaustive proofs and do not use a shrinking property-test framework.

Experiment 001 limits: no metabolism, exchanges, communication of private circumstances,
memory decay, snapshot loading, persistence service, or parallel scene resolution.
Relationships and objective audit history can grow. Hunger is a supplied motivation
and is not automatically satisfied by inventory. Recommend Experiment 002 on
communicated need, uncertain beliefs, and resource consumption before scaling.

## Experiment 002: communication and reinterpretation

Run `cargo run --locked --release --example evaluate002` to regenerate
[Experiment 002 evidence](experiments/002/results.md). This keeps the original
CLI and Experiment 001 rules available. The new experiment adds structured
communication scenes, bounded uncertain beliefs about particular refusals,
audited memory reinterpretation and food consumption. It compares silent,
unverified, verified, conflicting and altered-history cases across 128 seeds,
plus a 1,000-agent population. `cargo bench --bench throughput` measures both modes.

At seed 42, an explanation after helpful history changes a matched later choice
from Leave to Offer; contradictory disclosure restores Leave. Strong evidence
after negative history revises a memory but does not erase the other refusals.
These are controlled results of explicit rules. Communication is scheduled by the
experiment, disclosure uses a narrow instrumented observation, and confidence is
not calibrated. There is no claim of consciousness or complete human reasoning.

## Experiment 003: intentional communication

Run `cargo run --locked --release --example evaluate003`. The dedicated
[findings](experiments/003/results.md) link to reproducible JSON and a readable
causal report. Enable `Simulation::enable_intentional()` before creating scenes;
`run()` then lets both participants choose communication after refusals. Earlier
experiment defaults remain unchanged.

A local policy scores silence, explanation, evidence, information requests and
known-false scarcity claims. Separate credibility learns only from verified or
contradicted claims. Communication memories are bounded; original events stay
immutable. Controlled trials show disclosure costs, evidence-request costs and
communication history changing choices. Requests and deception sometimes fail to
change later behavior. See the findings for sensor assumptions and other limits.

## Experiment 004: anticipating communication consequences

Run `cargo run --locked --release --example evaluate004`; see
[findings and evidence](experiments/004/results.md). `enable_foresight(seed, sensor)`
adds a replaceable one-step predictor to intentional communication. Agents learn
local challenge/evidence-response expectations from public responses. Candidate
records separate immediate utility and anticipated consequences. Fallible evidence
has explicit reliability and disclosure effort; conflicting readings can leave
uncertainty unresolved.

Controlled trials show anticipation changing deception and evidence choices, and
prediction errors changing later expectations. They also expose a turn-budget
loophole: late claims cannot be challenged within the current conversation. Old
experiment modes and evidence remain intact. The four-mode benchmark measures
short populations only; see the findings for noise, storage and scaling limits.

## Experiment 005: persistent unresolved concerns

Run `cargo run --locked --release --example evaluate005`; see the dedicated
[findings](experiments/005/results.md), [causal report](experiments/005/report.txt),
and [pre-implementation hypotheses](experiments/005/specification.md).
Call `enable_concerns()` after enabling foresight to retain up to eight unfinished
refusal questions per individual. On later encounters a local replaceable policy
chooses one evidence request, abandonment, or normal continuation. Costs, age,
relationship value, expected answers and previous failures affect that choice.

Strong evidence can close a concern and revise a memory; weak or conflicting
evidence can leave it unresolved. Closure is subjective and can be wrong.
Controlled trials and three-encounter populations preserve the previous four
experiments. The benchmark now includes formation and matched repeated-encounter
workloads. Bounded concerns do not bound the full audit archive or partner maps.

## Experiment 006: adaptive inquiry

Run `cargo run --locked --release --example evaluate006`; see
[findings](experiments/006/results.md), [causal report](experiments/006/report.txt),
[full seed-42 traces](experiments/006/seed-42.json), and
[compatibility evidence](experiments/006/compatibility.json).
Call `enable_inquiry()` after concerns to opt in. Resource encounters now choose
Direct, Evidence, or Pause using learned concern/source/strategy usefulness.
`inquiry_meeting(&[ids])` records bounded public co-presence and gives each
participant one voluntary opportunity; it supplies neither messages nor expertise.

Repeated weak answers lose useful value. Agents can change method, try a newly met
source, or retain a highly important question without asking again. Voluntary public
offers of genuinely new evidence can restore a path's value. New-source exploration
can fail; communicated evidence can be wrong. Gain scores are auditable heuristics,
not calibrated entropy estimates. The extension adds bounded local state but full
audit archives remain unbounded. Older policies, output defaults and evidence are
unchanged; matched 005/006 benchmark rows live under the new experiment.

## Experiment 007: provenance-aware testimony

Run `cargo run --locked --release --example evaluate007`; see the
[findings](experiments/007/results.md), [causal traces](experiments/007/report.txt)
and [compatibility evidence](experiments/007/compatibility.json).
Call `enable_provenance()` after inquiry. `inspect_refusal` offers a voluntary,
fallible inspection of a narrowly opened historical record. `provenance_exchange`
offers one structured sharing opportunity; `provenance_offer` permits voluntary
metadata disclosure. Ordinary resource encounters keep their existing response
mechanism. No autonomous gossip or search is added.

Known shared lineage suppresses redundant relay value; distinct observations can
corroborate or conflict. Hidden relay can cause overconfidence, and later voluntary
attribution can revise confidence, memory and concern status. Credibility remains
separate from independence. Three new local collections are bounded at 32; full
audit archives grow. The 007 report retains its original next-experiment recommendation.

## History foundation reinforcement

Experiments 001–007 remain frozen. Referential query audits and derived history
indexes reduce duplication and repeated scans while preserving complete history,
bounded local cognition and old report formats. See the [architecture guide](docs/history-audit.md)
and [measurements and findings](reinforcement/results.md).

Run `cargo run --release --example validate_history` for archived compatibility and
compact reconstruction checks, `cargo run --release --example measure_history -- after`
for isolated measurement output, and `cargo bench --bench throughput` for the
established workloads. New evidence stays under `reinforcement/`; these commands
do not rewrite prior experiment archives. Use `-j 1` on memory-constrained hosts.

The subsequent [next-target diagnosis](research/next-target/report.md) recommends
testing coherent assessment across native evidence and third-party testimony.
It includes mixed-channel counterexamples, history-depth measurements, five candidate
investigations and the falsifiable question implemented in Experiment 008 below.

## Experiment 008: coherent bounded assessment

Run `cargo run --locked --release --example evaluate008`; see
[results and controls](experiments/008/results.md) and [causal traces](experiments/008/report.txt).
After provenance, call `enable_assessment(assessment::Method::Grouped)` to assess
native receipts and third-party reports from one prospective FIFO basis capped at 32
per individual. The `Max` alternative remains an explicit comparison control.

Mixed evidence no longer reverses belief merely because one channel executes last.
Known relay remains redundant; independent reports can corroborate or conflict;
hidden dependence can still mislead. `native_acquired_exchange` delivers an actual
retained structured report through the native adapter without inventing another
origin. Prior modes and archives remain unchanged. Forgetting and legitimate new
information can change assessment; the mechanism does not discover objective truth.

## Experiment 009: bounded selective retention

[Results and limitations](experiments/009/results.md) compare FIFO, received-quality
retention and current-concern salience at the same 32-item capacity. Selective modes
preserve some useful distinctions but retain errors and miss later-useful minor
items. The evidence supports narrow opt-in research use; FIFO remains the default.

After `enable_assessment(assessment::Method::Grouped)`, explicitly call
`enable_retention(retention::Method::Salient)` or choose `Quality`/`Fifo` controls.
The policy sees only own retained items, incoming evidence and own capped concerns.
Use `experiment009::Snapshot::validate` for selective-history replay; the frozen 008
validator still reconstructs FIFO. Enabling does not import observer history.

Run `cargo run --release --locked -j1 --example evaluate009`, then
`python experiments/009/archive.py` and `python experiments/009/analyze.py`.
Full new 009 traces are reproducibly compressed; timings remain separate CSV.
Earlier experiments and their contract map are frozen.

## Experiment 010: survival-to-action

[Specification](experiments/010/specification.md) and [results](experiments/010/results.md)
test multiple actual concerns, one voluntary inquiry before an executed resource
encounter, and subsequent inventory persistence. FIFO, Quality and Salient retain
their finalized009 rules. The unchanged inquiry mechanism is compared with an
opt-in current-basis need projection; its effects are conditional on that heuristic.
No mode becomes the default. Run `cargo run --release --locked -j1 --example evaluate010`,
then `python experiments/010/archive.py` and `python experiments/010/analyze.py`.
Read-only009 compatibility uses `validate009` without prepared target caches.
Do not regenerate historical evidence in place.

## Experiment 011: bounded inquiry value

[Specification](experiments/011/specification.md), [results](experiments/011/results.md)
and [scientific review](docs/experiment-011-review.md)
compare exact Unchanged/CurrentNeed with opt-in StatusValue across three actual
concerns, sequential limited opportunities, fixed mirrored futures and executed
resource consequences. **Protected scientific result — StatusValue unadopted;
negative conclusion confirmed with qualification.** Cheap queries often fail to
change retained support/status, yet partial-information gains and source/method
learning remain real. Failures, sensitivity and persistent missed transfers are
protected by the [001–011 contract](docs/experiment-contract-001-011.md).
No new persistent cognition or default change; original prose corrections stay in the review.

Use `cargo run --release --locked -j 1 --example validate011` or the normal
`validate_contract` gate. `python -B examples/support/review011.py --verify-finalized`
independently checks frozen traces/counts without writing files or reading target caches.
The evaluator writes replay outputs to target; use isolated directories if needed.
The archived writer is historical tooling: do not regenerate frozen011 evidence.
Prior `validate009`/`validate010` gates remain unchanged. The011 experiment did not introduce012.

## Experiment 012: contextual learning self-evaluation

[Specification](experiments/012/specification.md) and [results](experiments/012/results.md)
compare unchanged inquiry learning with bounded source/method progress estimates,
with and without a locally visible retained-evidence context. **ACCEPT WITH QUALIFICATION**:
new experience changes later inquiry, sometimes causing executed persistent consequences,
but coarse context can miss changed sources and misleading support can train optimism.
The [scientific review](docs/experiment-012-review.md) confirms ACCEPT WITH QUALIFICATION.
**Protected scientific result — Contextual and GlobalProgress retained as opt-in
research mechanisms; ACCEPT WITH QUALIFICATION; neither adopted as default.**
The [001–012 contract](docs/experiment-contract-001-012.md) and
[integrity audit](docs/integrity-audit-001-012.md) preserve the qualified findings.

Run `cargo run --release --locked -j 1 --example evaluate012 -- --output target/experiment012-final`.
Independent read-only reconstruction is
`python -B examples/support/review012.py --self-test --replay-dir target/experiment012-final`;
the normal foundation gate and standalone `validate012` share exact frozen-byte/replay
and independent findings checks, without cache preparation or writes.
`measure012` emits fixed-input policy timings; full trial timing is separate.
Original001–012 evidence and all defaults remain unchanged. The012 protection pass did not start013.

## Experiment 013: adaptive expectations in a changing world

[Specification](experiments/013/specification.md) and [results](experiments/013/results.md)
retain original evidence; the [scientific review](docs/experiment-013-review.md)
establishes **Protected scientific result — ACCEPT WITH QUALIFICATION** through the
[001–013 contract](docs/experiment-contract-001-013.md). Pulse uses existing local concern age
and failed inquiry estimates to reconsider a prior without new persistent cognition.
It can discover changed sources earlier and alter executed history, but pays for many
failed probes; Ungated reconsideration can crowd out useful Evidence. Longer histories
also expose preserved legacy trust effects. Neither mode is a default. The review
corrects C64 (not B64) and distinguishes the last-four partner scoring window from
memory eviction. Period robustness and general exploration value remain unresolved.

Run `cargo run --release --locked -j 1 --example evaluate013 -- --output target/experiment013-final`.
The normal foundation gate and read-only `validate013` share151-entry protection,
all original archives/replays and both independent readers: `review013.py --self-test`
and `review013_findings.py`. Neither requires a prepared cache or writes evidence.
`measure013` separates fixed-input policy cost from history-length work. No014 was started.
See the [integrity audit](docs/integrity-audit-001-013.md) and
[next-stage research handoff](docs/research-handoff-001-013.md).

## Experiment 014: competing activities under scarce time

[Specification](experiments/014/specification.md) and [results](experiments/014/results.md)
report **ACCEPT WITH QUALIFICATION**, confirmed by the
[scientific review](docs/experiment-014-review.md) and protected by the
[001–014 contract](docs/experiment-contract-001-014.md).
An opt-in public clock makes elapsed inquiry/resource time consume shared opportunity
windows. Existing local cognition can change executed transfers and later state;
information-first choices can also miss meals. Simple priorities share the strongest
benefits, and the local selector has no established overall advantage.
Run `evaluate014 -- --output target/experiment014-final` for840 matched/replayed
worlds; read-only `validate014` checks archives and independent reconstruction,
including five separately labeled post-evaluation budget diagnostics. `measure014`
separates fixed-input cost from full-suite timings. No default changes. Normal foundation validation includes014 and both readers;
[168-entry integrity audit](docs/integrity-audit-001-014.md). A separate
[handoff](docs/research-handoff-001-014.md) identifies the encounter-initiation boundary.

## Experiment015: self-initiated encounters and mutual availability

[Specification](experiments/015/specification.md) and [results](experiments/015/results.md)
report **ACCEPT WITH QUALIFICATION**, pending independent review. Three stationary
individuals independently invite, accept, decline or continue their own plan; shared
time and declared arbitration determine which existing interactions execute.
History changes contacts and persistent distribution, but simple policies share
benefits; repeated refusal, starvation and strong ordering effects remain. Contact
and transfer counts are not welfare. No global default changes. [Independent review](docs/experiment-015-review.md) accepts
with qualification; [001–015 contract](docs/experiment-contract-001-015.md) protects the findings.

Run `cargo run --release --locked -j 1 --example evaluate015 -- --output target/experiment015-final`.
Read-only `validate015` replays780 worlds and independently checks195 histories,
locality, matching, accounting and all186 protected hashes. The normal foundation
includes015 and both independent readers; [integrity audit](docs/integrity-audit-001-015.md). `review015.py --self-test` reconstructs evidence without a prepared
cache; `measure015` separates fixed-input timing from full histories.
[Handoff](docs/research-handoff-001-015.md) identifies failed-contact learning as a separate016 investigation.

## Experiment016: Learning From Failed Contact

[Specification](experiments/016/specification.md), [results](experiments/016/results.md):
**NO JUSTIFICATION for adopting two-cell Typed retry state over simpler controls.**
Own invitation outcomes can change later contact,actual refusal/trust/food history
and persistent meals. Fewer failures often leave material unchanged; long delays
can miss useful inquiry. A one-record control has narrow gains,with harmful choices
and strong ordering sensitivity preserved. [Independent review](docs/experiment-016-review.md): ACCEPT WITH QUALIFICATION; no default adoption.

Run `cargo run --release --locked -j 1 --example evaluate016 -- --output target/experiment016-final`
only for authorized016 evidence generation. Read-only `validate016` checks1000 exact
worlds,250 primary independent histories and ten post-evaluation seed1 extractions;
`review016.py --self-test` and `review016_benefit.py` reconstruct actual causal chains.
`measure016 -- experiments/016/observer-cost.json` separates policy/update/serialization
cost; timings are separate evidence. Shared foundation now protects001–016 and207 artifacts, including complementary
`review016_findings.py` and nine corruption categories. [Audit](docs/integrity-audit-001-016.md),
[handoff](docs/research-handoff-001-016.md).017 may begin only after Gate B passes.

## Experiment017: Endogenous Opportunities and Changing Social Availability

[Specification](experiments/017/specification.md),[results](experiments/017/results.md):
**Protected ACCEPT WITH QUALIFICATION**;[independent review](docs/experiment-017-review.md). Real selected
resource commitments change public availability;free-person consumption can alter
later invitations,executed transfers,trust and persistent meals. Contact-only stepping
is inert;effects with consumption are rare and order/fixture dependent. Preserve
foregone cooperation,history-ablation gains/losses and information without material
benefit. Three actors,no disjoint meetings,inquiry still atomic,no metabolism or
postsetup busy/willingness scripts. No default adoption;018 is a separate prospective question.

Run `cargo run --release --locked -j 1 --example evaluate017 -- --output target/experiment017-final`
for authorized017 evidence generation only. Read-only `validate017` reruns940 worlds,
checks235 independently reconstructed histories/47 groups/130 ordering controls,
seven corruption categories,four causal slices and228 protected hashes. Independent
`review017.py --self-test`/`review017_findings.py` need no target cache. `measure017`
separates policy/world/serialization timing and observer point-table cost. The normal
foundation gate protects001–017,including complementary `review017_scope.py`.
[Contract](docs/experiment-contract-001-017.md),[integrity audit](docs/integrity-audit-001-017.md),
[research handoff](docs/research-handoff-001-017.md).

## Experiment018: Recurring Needs and Competing Independent Activities

[Specification](experiments/018/specification.md), [results](experiments/018/results.md):
**Independently reviewed ACCEPT WITH QUALIFICATION**; [review](docs/experiment-018-review.md).
Consistent elapsed-time pressure creates repeated voluntary choices and can change
later executed transfers, trust and material circumstances. Extra opportunities to
eat during resource interaction often only change timing and can leave people
hungrier by exhausting finite stocks sooner. Preserve harmful choices, nulls,
reconvergence, ordering and history-ablation reversals. Three people, one resource
commitment, atomic inquiry and heuristic score scales bound the conclusion.
No new cognitive cells or default adoption. Protected by
[001–018 contract](docs/experiment-contract-001-018.md),
[audit](docs/integrity-audit-001-018.md),[handoff](docs/research-handoff-001-018.md).
019 followed a separate prospective gate;020 has its own later authorization.

Read-only `cargo run --release --locked -j 1 --example validate018` checks1064 exact
worlds,266 full independent reconstructions,16 causal diagnostics,10 corruption
categories and269 frozen artifacts without target caches. The normal foundation
shares the same gate,including complementary `review018_scope.py`. `measure018` separates
pure policy/need, world, resolver activity and serialization costs. New evidence
generation uses `evaluate018 -- --output target/experiment018-new` only when authorized;
never regenerate historical archives in place. The invalid first clock integration
candidate remains preserved alongside the valid corrected evidence.

## Experiment019: Robust Activity Priorities and Opportunity Timing

[Specification](experiments/019/specification.md),[results](experiments/019/results.md):
**Independently reviewed ACCEPT WITH QUALIFICATION**; [review](docs/experiment-019-review.md).
Six stateless controls compare historical selection,meal priority,last-safe-slot
admission,10% Invite scaling and outgoing history projection under common018 physics.
Timing variants keep equal total horizons. Small perturbations are mostly terminally
inert but can change real execution;private cooperation and seeded feedback reverse
some apparent gains. Preserve ineffective-contact gains,nulls and reconvergence.
No new cognition or default adoption. Protected by [001–019](docs/experiment-contract-001-019.md),
[audit](docs/integrity-audit-001-019.md) and [handoff](docs/research-handoff-001-019.md).020 is separate research.

Read-only `cargo run --release --locked -j 1 --example validate019` checks1296 exact
worlds,336 raw histories,296 frozen artifacts and independent priorities/physics/
admission/cognition/prefix reconstruction. `review019_slices.py` adds21 registered
post-evaluation causal diagnostics. `measure019 -- experiments/019/observer-cost.json`
separates work units and audit cost. New generation uses `evaluate019 -- --output`
with an isolated target directory;never overwrite historical evidence. Foundation
protection is001–020. Shared/standalone019 includes the independent scientific scope reader.

## Experiment020: Finite Environmental Resource Acquisition

[Specification](experiments/020/specification.md),[design gate](experiments/020/design-gate.md),
[results](experiments/020/results.md),[verification](experiments/020/verification.md):
**Independently reviewed ACCEPT WITH QUALIFICATION and protected through001–020;
no policy adoption.** See the [review](docs/experiment-020-review.md),
[contract](docs/experiment-contract-001-020.md) and [integrity audit](docs/integrity-audit-001-020.md). One finite co-present cache,local voluntary selection,
actor reservations,costly failed claims and atomic conserving grants.164 complete
worlds compare equal potential supply in Owned/Closed/Ignored/Need/Acquire controls.
All164 are independently reconstructed with20 corruption categories. Sharing and
missed help both occur;finite depletion and repetitive empty contact remain.

`cargo run --release --locked -j 1 --example validate020` is a separate read-only
gate,retained unchanged because its source is frozen. Shared `contract020.rs` adds
321 original-artifact and independent-scope checks to the normal foundation and
`validate020_foundation`. New evaluator output
requires a fresh directory:`evaluate020 -- --output target/experiment020-new`.
Prospective021 requires its own design gate;no022 or global policy adoption. The [minimal original020 playback](experiments/020/demo.html)
is served with `python -m http.server 8766 --bind 127.0.0.1 --directory experiments/020`;
open [local demonstration](http://127.0.0.1:8766/demo.html). See its
[historical acceptance and limitations](experiments/020/viewer-acceptance.md). The
main History Explorer now reads all164 originals with actor/environmental views,
typed causal navigation and cache-inclusive matched comparisons;
[current acceptance](tools/history-explorer/validation/finite-integration/acceptance.md).

## Experiment021: Local Environmental Knowledge (research)

[Specification](experiments/021/specification.md),[qualified design gate](experiments/021/design-gate.md),
[results](experiments/021/results.md),[verification](experiments/021/verification.md).
214 preregistered worlds, exact replay and independent reconstruction. One owned
last-seen record can change real acquisition, meals and later social/material
history; stale positives can also waste full-cost claims. All retention contrasts
reconverge materially by the endpoint. ACCEPT WITH QUALIFICATION,unprotected and
unadopted;fixed passive sensing is not exploration. Foundation remains001–020;no022.

```powershell
cargo run --release --locked -j 1 --example validate021
# New evaluator outputs must use a fresh directory, never historical archives:
cargo run --release --locked -j 1 --example evaluate021 -- --output target/my-new021
```

The [autonomous-community roadmap](docs/development/long-term-autonomous-community-roadmap.md)
sets evidence gates rather than promising civilizations. The
[Eldoria research bridge](docs/development/eldoria-simulation-bridge.md) separates
ordinary-life questions from canon and unimplemented game-support possibilities.


Experiment021 is independently reviewed/protected **ACCEPT WITH QUALIFICATION**,
with no default adoption:[review](docs/experiment-021-review.md),
[001–021 contract](docs/experiment-contract-001-021.md),
[integrity audit](docs/integrity-audit-001-021.md),
[handoff](docs/research-handoff-001-021.md). The normal read-only foundation gate
includes341 original artifacts (321 inherited+20 original021),214 exact original
replays,primitive/locality/expiry checks and both complementary scientific readers.
Frozen standalone `validate021` remains unchanged;`validate021_foundation` shares
the normal gate's superset. Preserve stale losses,mixed distribution,expiry tradeoffs,
nulls and material reconvergence/social persistence. FreshReference has more free
sensing;none of these modes is a general memory optimum.022 is separately authorized
prospective concurrency research,conditional on passed/committed protection and a
design gate;no023 or automatic022 protection.


The main local History Explorer now includes all214 original021 knowledge histories
alongside018/019/020,with a distinct versioned adapter,actual owned retained slots,
unknown/expiry semantics and observer receipt→choice→grant→meal navigation.
[Run and inspect](tools/history-explorer/README.md),
[real acceptance/screenshots/costs](tools/history-explorer/validation/knowledge-integration/acceptance.md).
No archive rewriting,policy execution or simulation-control route is introduced.

The completed First Continuous World program's
[delivery report](docs/development/first-continuous-world-delivery.md) answers the
fifteen scientific questions; its [completion audit](docs/development/continuous-world-completion-audit.md)
maps all twenty acceptance requirements to actual evidence. See
[readiness](docs/development/continuous-world-readiness.md) for the qualified
community verdict and conditional future material study.

The subsequent [independent023review](docs/experiment-023-review.md) accepts with
qualification. [001–023protection](docs/experiment-contract-001-023.md) now passes
its [integrity checks](docs/integrity-audit-001-023.md) for386original artifacts,
without adopting Continuous scheduling. The subsequent activity-valuation program
completed diagnosis and published024 preregistration before treatment; material
preparation and025 remain outside scope.

## Experiment 024 — locally grounded activity valuation

The [published preregistration](experiments/024/specification.md) preceded a fixed
56-world study: 27 Baseline/MealRelief pairs and two reporting placebos. Only an
eligible ordinary Consume score changes from H to H+min(H,40). Source, timing,
consent, cognition, supply and all other scores remain unchanged. Baseline remains
the prototype default. No outcome-directed retuning occurred.

[Results](experiments/024/results.md): 19 pairs change executed histories, 15 have
more meals and four fewer; seven raise at least one actor's cumulative hunger.
Eight pairs remain behavior/state nulls apart from score fields. Exhaustion can
consume all finite food without becoming a sustainable world. All 56 exact replays,
independent reconstructions and 34 corruption controls pass; see
[verification](experiments/024/verification.md). This is not adoption or 024 protection.

```powershell
cargo run --release --locked -j 1 --example validate024
python -B tools/history-explorer/explorer.py --continuous experiments/023 --valuation experiments/024
```

The real main viewer exposes exact own-state score terms, matched decisions,
actual meals/transfers, timed meal deferrals and separate observer consequences.
Use the **024 · Costly earlier meal** preset and inspect tick18, individual0.
[Runner guide](prototypes/activity-valuation/README.md),
[actual viewer acceptance](docs/development/main024-viewer-acceptance.md),
[community readiness](docs/development/post-024-community-readiness.md),
[future material handoff](docs/development/post-024-material-preparation-handoff.md).
The handoff does not register or implement025. The
[delivery report](docs/development/activity-valuation-delivery.md) and
[completion audit](docs/development/activity-valuation-completion-audit.md) cover
the entire review, protection, diagnosis, experiment and viewing program.
