# Architecture map

Navigation summary, protected through022. Source and mode-specific
contracts remain authoritative; this map does not replace them. Update affected
rows when ownership, entry points or mode boundaries change.

## Start here

- Rules: [AGENTS.md](../AGENTS.md); protected [001–022 contract](experiment-contract-001-022.md),
  which links historical contracts. Read the affected mode's clauses and coverage.
- Findings: [experiment index](simulation/EXPERIMENT_HISTORY.md).
- Working procedure: [context workflow](context-workflow.md).
- Design rationale: [architecture](architecture.md), by experiment heading.
- Observer/runtime boundary: [history audit](history-audit.md).
- Recorded-life investigation: [019 observations](development/history-explorer-observations-019.md),
  [first walkthrough](development/history-explorer-first-walkthrough.md) and
  [evidence-backed priorities](development/post-019-observation-backlog.md).
- Phase2 integration diagnosis: [community-readiness assessment](development/world-of-individuals-phase-2-assessment.md);
  [current continuation checkpoint](development/continuation-checkpoint.md).
- Lore work: [Eldoria index](ELDORIA_INDEX.md); lore is not implemented simulation behavior.

## Source ownership

| Question / change | Inspect first | Related validation |
|---|---|---|
| Agent, local observation, stable IDs, resources, memory | [model.rs](../src/model.rs) | `tests/invariants.rs` |
| Resource action scoring | [behavior.rs](../src/behavior.rs) | `tests/invariants.rs`, 001 contract coverage |
| ECS setup, legality, resolution, accounting, enable boundaries | [simulation.rs](../src/simulation.rs) | Invariants plus affected experiment test |
| Beliefs, information receipts, original interpretation/revisions | [cognition.rs](../src/cognition.rs), `simulation.rs` | `tests/experiment002.rs`, contract tests |
| Voluntary communication, directed credibility | [intentional.rs](../src/intentional.rs), `simulation.rs` | `tests/experiment003.rs` |
| Local forecasts, fallible sensors, challenge learning | [foresight.rs](../src/foresight.rs), `simulation.rs` | `tests/experiment004.rs` |
| Unfinished intent, reopen/abandon, importance | [concerns.rs](../src/concerns.rs), `simulation.rs` | `tests/experiment005.rs` |
| Inquiry candidates, source × strategy learning, public offers | [inquiry.rs](../src/inquiry.rs), [adaptive.rs](../src/simulation/adaptive.rs) | `tests/experiment006.rs` |
| Received attribution, voluntary inspection/sharing, independence | [provenance.rs](../src/provenance.rs), [resolver](../src/simulation/provenance.rs) | `tests/experiment007.rs` |
| Unified local evidence basis and Grouped/Max support | [assessment.rs](../src/assessment.rs), `simulation.rs`, provenance resolver | `tests/experiment008.rs` |
| FIFO/Quality/Salient retention selection | [retention.rs](../src/retention.rs), `simulation.rs` | `tests/experiment009.rs` |
| CurrentNeed inquiry projection (experimental) | [attention.rs](../src/attention.rs), [adapter](../src/simulation/attention.rs) | `tests/experiment010.rs` |
| StatusValue inquiry prototype (protected scientific result; unadopted with qualification) | [inquiry_value.rs](../src/inquiry_value.rs), [adapter](../src/simulation/inquiry_value.rs) | `tests/experiment011.rs`, [scientific review](experiment-011-review.md), `examples/support/review011.py` |
| Contextual observed-change learning (012 protected with qualification, opt-in research) | [self_evaluation.rs](../src/self_evaluation.rs), [adapter](../src/simulation/self_evaluation.rs), [fixture](../src/experiment012.rs) | `tests/experiment012.rs`, independent `review012.py`, `review012_findings.py`, shared foundation/standalone `validate012`; [scientific review](experiment-012-review.md), [contract](experiment-contract-001-012.md) |
| Transient prior reconsideration (013 protected with qualification, opt-in research) | [experiment013.rs](../src/experiment013.rs), existing self-evaluation policy setter | `tests/experiment013.rs`, both independent readers, shared `contract013.rs` / standalone `validate013`, `measure013`, diagnostic `inspect013`; [scientific review](experiment-013-review.md), [contract](experiment-contract-001-013.md) |
| Compact query references, local contexts, export reconstruction | [audit.rs](../src/audit.rs) | `tests/history_foundation.rs`, `examples/validate_history.rs` |
| Scarce-time activity allocation (014 protected with qualification, opt-in) | [activity.rs](../src/activity.rs), [clock/resolver adapter](../src/simulation/activity.rs), [fixture](../src/experiment014.rs) | `tests/experiment014.rs`, `review014.py`, read-only `validate014`, `measure014`, separate `inspect014` diagnostic; [results](../experiments/014/results.md), [scientific review](experiment-014-review.md), second `review014_findings.py` |
| Independent invitation waves (015 protected with qualification) | [encounters.rs](../src/encounters.rs), [isolated adapter](../src/simulation/encounters.rs), [fixture](../src/experiment015.rs) | `tests/experiment015.rs`, shared/standalone `validate015` / `review015.py`, second `review015_findings.py`, `measure015`; [review](experiment-015-review.md); [results](../experiments/015/results.md); protected by [001–015](experiment-contract-001-015.md) |
| Finite contact feedback (016 protected with qualification, unadopted) | [contact_learning.rs](../src/contact_learning.rs), [adapter](../src/simulation/contact_learning.rs), [fixture](../src/experiment016.rs) | `tests/experiment016.rs`, shared/standalone `validate016`, `review016.py`, `review016_benefit.py`, complementary `review016_findings.py`; [review](experiment-016-review.md); [results](../experiments/016/results.md); [001–016 contract](experiment-contract-001-016.md) |
| Endogenous resource commitments (017 protected with qualification) | [opportunities.rs](../src/opportunities.rs), [adapter](../src/simulation/opportunities.rs), [fixture](../src/experiment017.rs) | `tests/experiment017.rs`, shared/standalone `validate017`, independent `review017.py` / `review017_findings.py` / complementary `review017_scope.py`, `measure017`; [specification](../experiments/017/specification.md), [results](../experiments/017/results.md), [scientific review](experiment-017-review.md), [contract](experiment-contract-001-017.md); no default adoption |
| Private derived history indexes, trust replay | [history.rs](../src/history.rs), `simulation.rs` | Foundation, invariants, `tests/contract_001_008.rs` |
| Recurring needs and competing activities (018 protected with qualification) | [recurring.rs](../src/recurring.rs), [isolated adapter](../src/simulation/recurring.rs), [fixture](../src/experiment018.rs); conditional shared clock hooks | `tests/experiment018.rs`, shared/standalone read-only `validate018`, `review018.py` / `review018_findings.py`, complementary `review018_scope.py`, `measure018`; [specification](../experiments/018/specification.md), [results](../experiments/018/results.md), [review](experiment-018-review.md), [contract](experiment-contract-001-018.md);269 artifacts,no default adoption |
| Activity priority/timing sensitivity (019 protected with qualification) | [priorities.rs](../src/priorities.rs), [small idle adapter](../src/simulation/priorities.rs), [fixture](../src/experiment019.rs); reuses protected018 clock/stepper | `tests/experiment019.rs`, shared/standalone `validate019`, `review019.py`, complementary `review019_findings.py`, post-evaluation `review019_slices.py`, `measure019`; [specification](../experiments/019/specification.md), [results](../experiments/019/results.md), [review](experiment-019-review.md), [contract](experiment-contract-001-019.md);296 artifacts,no adoption |
| Finite environmental acquisition (020 protected with qualification) | [cache/policy](../src/acquisition.rs),[isolated lifecycle](../src/simulation/acquisition.rs),[fixture](../src/experiment020.rs);no default path changes | `tests/acquisition.rs`,frozen standalone `validate020`,shared `contract020.rs`/`validate020_foundation`,`review020.py`/`review020_scope.py`,`measure020`;[specification](../experiments/020/specification.md),[results](../experiments/020/results.md),[verification](../experiments/020/verification.md);[review](experiment-020-review.md),[contract](experiment-contract-001-020.md);164 original histories,20 corruptions,321 protected artifacts;no adoption |
| Read-only History Explorer application | [app/tooling](../tools/history-explorer/README.md), `tools/history-explorer/explorer.py`, `causal.py`/`causal.js`, `finite.py`/`knowledge.py`, `perspective.js`, `app.js`; [technical guide](development/history-explorer-architecture.md) | Python real-archive and typed-link tests, JS owned-point/filter/order tests, explicit020/021 actor/claim/material/knowledge comparison,all164/214 original references,[020 acceptance](../tools/history-explorer/validation/finite-integration/acceptance.md),[021 acceptance](../tools/history-explorer/validation/knowledge-integration/acceptance.md),real-browser node navigation, [original acceptance](../tools/history-explorer/validation/acceptance.md), [causal acceptance](../tools/history-explorer/validation/causal-navigation-acceptance.md); one-way consumer, no foundation dependency |
| CLI and reproducible experiment fixtures | [main.rs](../src/main.rs), `src/experiment.rs`, `src/experiment002.rs` … `src/experiment021.rs` | Corresponding experiment tests/evaluators |
| Read-only frozen archive/replay gate | [validate_contract.rs](../examples/validate_contract.rs), shared `examples/support/contract009.rs` … `contract022.rs`, independent010–022 readers | Cache-free010–021 validation; shared standalone gates; historical summaries unchanged; [latest integrity audit](integrity-audit-001-022.md) |
| Throughput workload / history measurements | [throughput.rs](../benches/throughput.rs), `examples/measure_history.rs` | Separate wall-clock output; historical reports unchanged |

Use `rg -n 'symbol' path/to/file.rs` to locate implementation before reading a
bounded range. A row identifies starting files, not permission to skip affected callers.

## Four-person concurrent research (reviewed/protected022)

[Independent022review](experiment-022-review.md) at9fb6da8 passes Gate A:
ACCEPT WITH QUALIFICATION. The implemented comparison-epoch scheduler and matched
capacity findings stand; absolute phase invariance and continuous opportunities
remain untested. StageB passes: [001–022contract](experiment-contract-001-022.md),
[integrity audit](integrity-audit-001-022.md),361original artifacts; no adoption.

The bounded extension lives at `src/experiment022.rs` through
`experiment::concurrent`, and private `src/simulation/concurrent.rs` under adaptive.
Guarded adaptive/provenance time/context hooks are no-ops without opt-in state;
frozen lib.rs/simulation.rs and021sources remain unchanged. Four live reservations,
one per actor; core resource scenes share a clock with claims/meals. Atomic inquiry
remains globally exclusive. See [design](development/concurrent-world-architecture.md),
[results](../experiments/022/results.md), [verification](../experiments/022/verification.md).
Tests: concurrent.rs/concurrent_physical_gates.rs. Read-only validate022, independent
review022/corrupt022, complementary review022_coverage and measure022 own their
respective original/derived/wall-clock boundaries. Sharedcontract022/
validate022_foundation/normalgate protect361artifacts and independent review anchors.

Observation: separate `tools/history-explorer/concurrent_preview.py` serves native
original022members; concurrent-model.js owns interval/actor projections, concurrent.js
renders four lanes and original references. No simulation dependency on viewer.
[Browser acceptance](../tools/history-explorer/validation/concurrent-preview/acceptance.md),
[walkthrough](development/four-person-history-walkthrough.md). Full main022adapter
is outstanding; main018–021 schemas are not coerced.

Next-boundary assessment: [four-person readiness](development/four-individual-community-readiness.md)
and [finite-input material design](development/sustained-material-opportunity-design.md).
Both are post-evaluation/design documents, not023authorization or implementation.
The [program delivery audit](development/concurrent-program-delivery.md) maps gates,
validation and remaining limitations.

## Latest protected environmental-knowledge result

021 local environmental knowledge:[specification](../experiments/021/specification.md),
[qualified gate](../experiments/021/design-gate.md),[results](../experiments/021/results.md),
[verification](../experiments/021/verification.md).214 executed originals,independent
reconstruction/exact replay;[independent review](experiment-021-review.md) accepts
with qualification;[001–021 protection](experiment-contract-001-021.md),no adoption.
Owners:`src/environmental_knowledge.rs`,`src/simulation/environmental_knowledge.rs`,
`src/experiment021.rs`;`tests/environmental_knowledge.rs`;frozen original validate021,shared contract021/
validate021_foundation/normal gate341 artifacts,complementary review021_scope,
independent knowledge021/review021/compare021/findings021/review021_trust readers. One owned slot,
passive observations,unchanged020 scores/physics. Preserve stale failure,unknown
and expiry versus persistent material;no archive recovery.022 is separately authorized,
conditional on committed protection/prospective design;no023.

Forward plans:[community roadmap](development/long-term-autonomous-community-roadmap.md),
[non-canon Eldoria bridge](development/eldoria-simulation-bridge.md). Neither authorizes
future implementation or changes fictional authority.

[Completed program audit](development/phase-2-delivery.md) links all gated deliverables
and final verification.

## Flow and boundaries

Local actor state + public observations → replaceable policy → resolver checks
legality and accounts time/resources → immutable records + bounded local updates.
Later received information can revise interpretation/trust/concerns; it cannot
undo executed transfers. Objective events and lineage are distinct from local belief.

Modes progressively opt in: 002 cognition → 003 communication → 004 foresight →
005 concerns → 006 inquiry → 007 provenance → 008 assessment. 009 selects retention;
010/011 are separate optional inquiry projections, not combined defaults. Grouped
assessment and FIFO remain the protected defaults where applicable. CurrentNeed is
qualified research; StatusValue's negative adoption result is protected with qualification.
012 is a separate prospective learning mode, excluding combined010/011 projections.
It adds32 bounded source/method/context estimates of observed assessment change;
Grouped assessment, old novelty cells and all earlier limits/defaults stay unchanged.
Its observer records reference current inquiry IDs; unavailable content is never rebuilt
from that audit. See [012 results](../experiments/012/results.md) for costs and failures.

Episodes and beliefs are capped at 16, concerns at eight, assessment at 32;
other mode-specific caps and lifetimes are in the contracts. Eviction does not erase
separately persistent consequences. Observer archives/indexes validate history but
never supply unavailable cognition. Query audits use references and immutable local
contexts; snapshots are observer exports, not resumable saves. Audit histories and
some relationship storage grow: consult the history guide before storage changes.

020 is first-class in the main Explorer through `finite.py` (`experiment-020-trial/v1`,extractor3),
with original zero-based ordinals,captured actor cache inputs,separate environmental
observer state and exact claim/response/material chains. The earlier observer
demonstration remains preserved as `finite-demo.py`/`finite-demo.html`,
[ten source histories](../experiments/020/demo-data.json) and
[acceptance](../experiments/020/viewer-acceptance.md). The explicit020 adapter preserves unmodified source Trial values and old018/019
meaning. Neither the viewer nor its caches contribute to foundation validation.


021 is first-class in the main Explorer through knowledge.py
(`experiment-021-trial/v1`,extractor4),all214 zero-based original ordinals and policy+
knowledge-mode identity. Actor reads actual Choice.retained;Observer links original
receipt/retention/claim/results. Unknown/expiry,independent rival depletion,semantic
versus raw trust and reconvergence remain distinct. Full41Python/4JS tests and real
main8765 source→grant→meal acceptance pass;[evidence](../tools/history-explorer/validation/knowledge-integration/acceptance.md).
No viewer output becomes cognition or scientific-gate input.022 remains prospective.

Main Explorer022adapter: `tools/history-explorer/concurrent_history.py` and
`native-concurrent.js`; captured-only four-person perspective, typed causal joins
and capacity comparison. [Acceptance](development/main022-viewer-acceptance.md).
