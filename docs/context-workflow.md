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

Experiments001–020 are protected by the
[foundation contract](experiment-contract-001-020.md) and its inherited historical
maps. Experiment 011's [scientific review](experiment-011-review.md) is confirmed with
qualification: its evidence is protected, while StatusValue remains unadopted.
Experiment012's [scientific review](experiment-012-review.md) establishes ACCEPT
WITH QUALIFICATION; Contextual and GlobalProgress remain opt-in research mechanisms,
neither adopted as default. Its progress and context findings remain distinct.
Experiment013's unchanged [scientific review](experiment-013-review.md) establishes
ACCEPT WITH QUALIFICATION; Pulse/Ungated remain optional research, not defaults.
Preserve C64 versus B64 and retained episodes outside the last-four partner scoring
window. Its shared read-only gate invokes both independent readers.
The [integrity report](integrity-audit-001-020.md) records the latest protection pass;
the [research handoff](research-handoff-001-020.md) preserves open valuation/timing questions.
017 is protected with qualification; its188 contact-only nulls and narrow7/188 material
effect remain distinct. No default adoption or retroactive recurring hunger.
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
| Rebuilding architecture from chronological history | `architecture.md` contains 001–012 sections; contracts and history ownership are separate | Use `architecture-map.md` for source ownership, then affected sections |
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

Experiment014 is protected with qualification; [review](experiment-014-review.md).
Its actual-time capability and negative outcomes are distinct from unadopted activity
heuristics. Standalone014 and the foundation share both independent readers. See
[handoff](research-handoff-001-014.md) for the historical encounter-initiation boundary.

Experiment015 is protected with qualification: [review](experiment-015-review.md),
[contract](experiment-contract-001-015.md). Preserve independent initiation, provisional
consent, real execution, asymmetric order and Continue coupling, missing invitation
feedback and negative cases. Both readers and standalone/shared015 gate protect the
same evidence. No default adoption;016 was separately preregistered after015 protection.

Experiment016 is protected ACCEPT WITH QUALIFICATION; Typed adoption NO JUSTIFICATION.
[Review](experiment-016-review.md),[contract](experiment-contract-001-016.md),
[results](../experiments/016/results.md). Shared foundation/standalone016 preserve207
artifacts and three readers, including nine complementary corruption categories.
No default adoption;Gate B was validated/committed/pushed e968c34 before017.

Experiment017 is protected with qualification: [review](experiment-017-review.md),
[contract](experiment-contract-001-017.md),[results](../experiments/017/results.md).
Its resource-step/free-person consumption chain is qualified by contact-only null,
rare material differences,atomic inquiry,three-person topology,ordering and no
metabolism. Shared/standalone validate017 and all three independent readers protect
228 original artifacts. No policy adoption;018 requires its own later independent
review/protection and must not be silently incorporated into this historical contract.

Experiment018 is independently reviewed ACCEPT WITH QUALIFICATION:
[review](experiment-018-review.md), [results](../experiments/018/results.md),
[specification](../experiments/018/specification.md). Follow the recurring source row
in the architecture map. Shared/standalone `validate018` reconstructs need progression,
actual activities and negative consequences,including the complementary primitive
reader. [001–018 protection](experiment-contract-001-018.md) freezes269 original
artifacts;preserve the invalid initial integration candidate and valid costly/null
outcomes. No default adoption.019 is separately authorized only after Gate B passes
and is committed/pushed;no automatic019 protection or020.

Experiment019 is independently reviewed and protected with qualification:
[specification](../experiments/019/specification.md),[results](../experiments/019/results.md).
Shared/standalone019 enforces296 original artifacts and four readers. Distinguish
small-score endpoint stability from actual choice/target changes,Deadline versus its
EatFirst ablation,and timing-induced meal efficiency from successful cooperation.
Preserve private-cooperation/seed reversals,conservative exclusions and full nulls.
Environmental acquisition020 is now separately authorized prospective research. History Explorer remains separately authorized
read-only engineering: [guide](development/history-explorer-user-guide.md). It consumes
verified original018/019 archives and never becomes an input to scientific gates or
actor cognition. Viewer-only edits need appropriate data/projection/browser checks;
reuse simulation checks only at an unchanged relevant Rust/fixture revision. Do not
rebuild old evidence to adapt a viewer schema or recover missing cognition.

Experiment020 is unprotected,opt-in research:[specification](../experiments/020/specification.md),
[results](../experiments/020/results.md),[verification](../experiments/020/verification.md).
Follow its architecture-map row for cache/claim/time code and independent reader.
Use standalone validate020 alongside protected gates;do not add it to001–019 or
rewrite old evidence. Keep current-stock legality separate from learned avoidance,
initial-placement controls separate from access,and per-actor costs separate from
global welfare.020's observer demo is an explicit separate schema adapter. No021.
