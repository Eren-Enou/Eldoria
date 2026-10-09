# Architecture map

Navigation summary, protected through011 with exploratory012 implementation. Source and mode-specific
contracts remain authoritative; this map does not replace them. Update affected
rows when ownership, entry points or mode boundaries change.

## Start here

- Rules: [AGENTS.md](../AGENTS.md); protected [001–011 contract](experiment-contract-001-011.md),
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
| Contextual observed-change learning (012 exploratory, opt-in) | [self_evaluation.rs](../src/self_evaluation.rs), [adapter](../src/simulation/self_evaluation.rs), [fixture](../src/experiment012.rs) | `tests/experiment012.rs`, independent `review012.py`, standalone `validate012`; outside protected foundation |
| Compact query references, local contexts, export reconstruction | [audit.rs](../src/audit.rs) | `tests/history_foundation.rs`, `examples/validate_history.rs` |
| Private derived history indexes, trust replay | [history.rs](../src/history.rs), `simulation.rs` | Foundation, invariants, `tests/contract_001_008.rs` |
| CLI and reproducible experiment fixtures | [main.rs](../src/main.rs), `src/experiment.rs`, `src/experiment002.rs` … `src/experiment012.rs` | Corresponding experiment tests/evaluators |
| Read-only frozen archive/replay gate | [validate_contract.rs](../examples/validate_contract.rs), `examples/support/contract009.rs`, `contract010.rs`, `contract011.rs`, `review010.py`, `review011.py` | Cache-free 010/011 validation; standalone `validate011`; shared observer `summary011.rs` |
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
