# Selective context and experiment workflow

## Authority and conflicts

Architecture maps, experiment indexes and summaries are navigation aids, not
independent authorities. Resolve discrepancies using the appropriate authority:

- Current source code establishes actual implemented behavior.
- Protected contracts establish behavior that must remain compatible; source that
  violates a contract is a regression, not a replacement contract.
- Frozen experiment evidence establishes historical observations.
- Finalized scientific reviews establish accepted interpretations and qualifications.
- Correct navigation documents when their pointers or summaries become outdated.

Never silently alter historical evidence or reinterpret a protected experiment to
match a summary. Report conflicts explicitly and preserve their revision context.

## Research status and mechanism adoption

Track research status separately from whether a mechanism should be adopted:

| Status | Meaning |
|---|---|
| Exploratory | Experiment is still being developed or interpreted. |
| Reviewed | Scientific conclusions have been examined and qualified. |
| Protected | Accepted conclusions are incorporated into the foundation contract and read-only integrity validation. |
| Rejected mechanism | Adoption of a proposed mechanism was not justified; preserve its negative evidence. |

These are not mutually exclusive adoption categories: an experiment can be
scientifically accepted and protected while its proposed mechanism is rejected.
Protection preserves accepted findings and qualifications, not a mandate to adopt
every tested mechanism.

Experiments 001–011 are protected by the
[foundation contract](experiment-contract-001-011.md) and its inherited historical
maps. Experiment 011's [scientific review](experiment-011-review.md) is confirmed with
qualification: its evidence is protected, while StatusValue remains unadopted.
The [integrity report](integrity-audit-001-011.md) records the protection pass.
Reverify source, contracts and reviews before
updating status, and do not change unrelated conclusions.

## Canon and simulation evidence

World of Individuals experiments can inform Eldoria worldbuilding, but their
results do not automatically establish Eldoria canon. Established Eldoria lore
likewise does not require experiments to produce predetermined historical outcomes.
Keep fictional canon authority (see [Eldoria index](ELDORIA_INDEX.md)) separate from
simulation evidence and scientific interpretation.

## Review findings

These are observed navigation/output risks, not measured token totals:

| Source of waste | Evidence | Change |
|---|---|---|
| Rebuilding architecture from chronological history | `architecture.md` contains 001–011 sections; contracts and history ownership are separate | Use `architecture-map.md` for source ownership, then affected sections |
| Rereading all reports to learn current conclusions | Experiment history previously stopped at 008; 009–011 results existed elsewhere | Extend the existing lightweight index, preserving negative and qualified results |
| Broad listings/searches mix engineering with lore and generated evidence | `docs/` contains both domains; `experiments/` contains large trace archives | Choose mapped directories/files and file globs; exclude trace bodies unless needed |
| Full file reads and oversized tool batches | Long architecture/contracts and generated JSON can exceed output budgets; truncated output hides useful context | Locate headings/symbols first and read bounded ranges; extract archive fields/counts |
| Repeating validation or generating evidence for an audit | README run examples write reports; evaluators and read-only gates have different roles | Use `validate_contract` for protected integrity; writers only for authorized target evidence |
| Successful terminal logs dominate context | Cargo reports each test/build; benchmark tables contain repeated rows | Save logs outside frozen archives; return exit code and concise summary, inspect failures |

Repeated scans/reads are visible in prior work in this chat, but no exhaustive chat
trace or tokenizer dataset was audited. Some repeated reads are warranted after
changes or compaction. Avoid claiming a measured savings percentage from this review.

## Routine work

1. Inspect Git status and applicable instructions; preserve unrelated files.
2. Read the map and experiment index once per task. Select the relevant contract
   clauses, specification/result sections and source rows. Do not read every report.
3. Search symbols in selected paths with `rg -n`; read the enclosing function and
   needed callers/types. Expand scope when references or evidence demand it.
4. Before behavior changes, record hypothesis, alternatives, falsification and
   limitations in the target specification. Reuse established architecture/contracts.
5. During development run the affected integration target (for example
   `cargo test --locked -j 1 --test experiment011`). At completion, run all required
   formatting, Clippy, full tests and the read-only contract gate; targeted tests
   do not substitute for completion checks. Benchmark when runtime/workload changes
   make it relevant, not for documentation-only changes.
6. Keep deterministic evidence under the authorized target; never overwrite frozen
   archives for an integrity audit. Do not rerun writer evaluators for old modes.
7. When module ownership, source paths, experiment status or important conclusions
   change, update the relevant map/index entries. Verify pointers against actual
   repository contents and conclusions against their authorities; leave unrelated
   statuses unchanged. Review the diff, then commit/push per AGENTS.md.

Save verbose command output in a task-specific temporary log. Return command,
exit status, test count and failures; inspect failures fully, without silent
truncation. Final task reports must identify the commit or revision validated,
exact commands, exit statuses, test counts, skipped checks with reasons, and
remaining failures or unresolved issues. A pre-edit check is not validation of
the final revision. Record any pending changes at validation time; rerun affected
checks after later modifications. Reuse passed checks only at an unchanged relevant
revision and state their scope accurately.

For long tasks, carry a concise checkpoint: objective, HEAD, changed files, decisions,
invariants, completed checks, unresolved issues and next action. Link source/report
locations rather than copying entire documents. After compaction verify current
status and changed relevant files; cached understanding is not evidence of current code.

## Required broader inspection

Expand beyond the initially selected files when changes affect:

- Cognitive capacities, memory eviction or persistent state.
- Information locality or policy-visible inputs.
- Historical serialization, replay or audit structure.
- Resource production, transfers, consumption or accounting.
- Experiment defaults or opt-in activation.
- Shared interfaces, architecture ownership or major dependencies.
- Unexpected regressions, integrity failures or stale navigation documents.

Inspect affected callers, contracts, tests and historical reports; use a full
codebase audit when requested or when unknown dependencies make the boundary
uncertain. Record the reason and scope. Selective reading is an efficiency technique,
not a restriction on necessary investigation or a substitute for authoritative evidence.

## Measuring savings prospectively

For each experiment record baseline commit, task scope, validation scope, model and
context settings, input/output and cached tokens if supplied by the client, tool
output bytes, repeated unchanged file/range reads, broad scans, truncations,
reconstruction turns and elapsed time. Keep this task telemetry separate from
deterministic simulation evidence; never store private chat content in the repository.

Compare matched experiment tasks or fixed read-only review tasks before/after this
workflow. Normalize by scope and required gates, use several runs, and report medians
and spread. Savings = `(baseline tokens - guided tokens) / baseline tokens`; report
cached-token costs separately. If actual usage is unavailable, tool-output bytes and
repeat counts are proxies, not token counts. Check correctness alongside savings:
same required gates, findings and preserved counterexamples; no increase in missed
dependencies, rework or stale summaries. Publish measured results only after collecting
data; do not install a tokenizer or add simulation instrumentation for this task.
