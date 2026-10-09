# Selective context and experiment workflow

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
7. Review the diff, update affected map/index rows, then commit/push per AGENTS.md.

Save verbose command output in a task-specific temporary log. Return command,
exit status, test count and failures; inspect relevant log ranges when needed.
Do not truncate failures silently. Reuse a passed check at the same code/fixture
revision unless new changes or unresolved concerns justify rerunning it.

For long tasks, carry a concise checkpoint: objective, HEAD, changed files, decisions,
invariants, completed checks, unresolved issues and next action. Link source/report
locations rather than copying entire documents. After compaction verify current
status and changed relevant files; cached understanding is not evidence of current code.

## Full audit remains available

Expand to a complete inventory, source inspection and affected historical reports
when requested, when changing shared interfaces/ownership/locality/persistence, or
when integrity failures, unexplained regressions, stale maps or unknown dependencies
make the boundary uncertain. Record the reason and audited scope. These guides are
navigation aids, not an inspection quota or a substitute for authoritative evidence.

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
