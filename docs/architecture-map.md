# Architecture map

Navigation summary, protected through014. Source and mode-specific
contracts remain authoritative; this map does not replace them. Update affected
rows when ownership, entry points or mode boundaries change.

## Start here

- Rules: [AGENTS.md](../AGENTS.md); protected [001–014 contract](experiment-contract-001-014.md),
  which links historical contracts. Read the affected mode's clauses and coverage.
- Findings: [experiment index](simulation/EXPERIMENT_HISTORY.md).
- Working procedure: [context workflow](context-workflow.md).
- Design rationale: [architecture](architecture.md), by experiment heading.
- Observer/runtime boundary: [history audit](history-audit.md).
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
| Independent invitation waves (015 research, unprotected) | [encounters.rs](../src/encounters.rs), [isolated adapter](../src/simulation/encounters.rs), [fixture](../src/experiment015.rs) | `tests/experiment015.rs`, separate `validate015` / `review015.py`, `measure015`; [results](../experiments/015/results.md); no foundation extension |
| Private derived history indexes, trust replay | [history.rs](../src/history.rs), `simulation.rs` | Foundation, invariants, `tests/contract_001_008.rs` |
| CLI and reproducible experiment fixtures | [main.rs](../src/main.rs), `src/experiment.rs`, `src/experiment002.rs` … `src/experiment015.rs` | Corresponding experiment tests/evaluators |
| Read-only frozen archive/replay gate | [validate_contract.rs](../examples/validate_contract.rs), shared `examples/support/contract009.rs` … `contract014.rs`, independent010–014 readers | Cache-free010–014 validation; shared standalone gates; historical summaries unchanged; [latest integrity audit](integrity-audit-001-014.md) |
| Throughput workload / history measurements | [throughput.rs](../benches/throughput.rs), `examples/measure_history.rs` | Separate wall-clock output; historical reports unchanged |

Use `rg -n 'symbol' path/to/file.rs` to locate implementation before reading a
bounded range. A row identifies starting files, not permission to skip affected callers.

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
