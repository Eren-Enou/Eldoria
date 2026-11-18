# Eldoria hybrid character intelligence: design exploration

Assessment date: 2026-11-17. Source baseline:
`eb7eec6f12fac7a43d90f40f6053eef8de4a8197`.
Companion to the [living-world feasibility assessment](living-world-feasibility-assessment.md).
This is a product design proposal, not an implementation, experiment registration,
027 retuning or authorization to begin 028. Existing source and protected artifacts
remain unchanged.

## Decision

Use conventional simulation AI as the complete character controller: utility
selection for immediate actions, with a small bounded hierarchical planner for
longer objectives. Both consume the same actor-local state. Optional LLM inference
may propose a strategy expressed through the declared product action vocabulary.
A deterministic evaluator decides whether the proposal is useful; the authoritative
resolver alone changes the world.

Every character must remain functional with inference disabled, unavailable,
exhausted, invalid or late. Societies should arise from resources, commitments,
information, institutions and individual choices. Inference can broaden the
strategies considered at exceptional moments; it must not supply every action,
invent physical rules or decide what other characters consent to do.

The design is technically compatible with the existing Rust/Bevy ECS approach.
It requires a product-owned controller and planning layer, typed action contracts,
an inference gateway and recorded external-input protocol. Those are substantial
new capabilities. Existing forecast `Plan` structures and inquiry “strategies” do
not already provide general planning or reusable social strategies.

## What the repository already supplies

Verified implementation below is distinguished from experimental usefulness and
proposed extensions. Inspection followed the [architecture map](../architecture-map.md)
and [context workflow](../context-workflow.md), then the listed sources. A scoped
symbol search found no GOAP/HTN/LLM integration in the root Rust sources or inspected
persistent/workbench Rust directories; this is not an assertion about unrelated
tools or every historical document.

| Requested role | Existing source and actual capability | Missing integration |
|---|---|---|
| Frequent utility choices | [behavior.rs](../../src/behavior.rs) captures legal alternatives, integer scores, local inputs and deterministic ties. [encounters.rs](../../src/encounters.rs), [inquiry.rs](../../src/inquiry.rs) and [environmental_knowledge.rs](../../src/environmental_knowledge.rs) score contacts, queries and acquisition. | A common product candidate/action vocabulary and score model across needs, goals and social commitments. Historical score scales are not a universal utility function. |
| Longer-term objectives | [concerns.rs](../../src/concerns.rs) has eight bounded unfinished concerns, but the question is `RefusalScarcity`. [foresight.rs](../../src/foresight.rs) predicts a few communication responses and records immediate/anticipated terms. | Explicit general goals, planning operators/methods, contingent steps, interruption and replanning. Shallow forecasts are not a multi-step world planner. |
| Subjective memory and relationships | [model.rs](../../src/model.rs) has 16 episodes and directed trust; [cognition.rs](../../src/cognition.rs) supplies event-specific beliefs; [assessment.rs](../../src/assessment.rs) and [provenance.rs](../../src/provenance.rs) preserve received evidence and locally known attribution. [intentional.rs](../../src/intentional.rs) separates directed credibility from trust. | One versioned owned-view projection shared by utility, planner and inference. Extend fact/goal domains in product code without importing objective roots, hidden inventories or observer history. |
| Experience informing choices | [inquiry.rs](../../src/inquiry.rs) learns source × inquiry-method usefulness. [self_evaluation.rs](../../src/self_evaluation.rs) has up to 32 context-sensitive expected-change cells; [contact_learning.rs](../../src/contact_learning.rs) has narrow feedback mechanisms. These are experimental, qualified mechanisms. | General strategy descriptions, execution monitoring, comparable outcome evidence and bounded retrieval. Existing learning is not a library of arbitrary successful plans. |
| Replaceable decision functions | [simulation.rs](../../src/simulation.rs) has `set_policy`, `set_communication_policy` and `set_predictor` function-pointer seams. Inputs are deliberately local. | A product controller port with persistent planner state and asynchronous proposal input. Network inference must not run inside historical synchronous policy callbacks. |
| Shared legality and mutation | [simulation.rs](../../src/simulation.rs), [concurrent.rs](../../src/simulation/concurrent.rs) and [025 coordinator](../../prototypes/material-preparation/src/coordinator.rs) enforce scenes, reservations, consent, accounting and completion-time checks. | General action admission and phased execution covering all product controllers. A policy's eligibility prediction never substitutes for authoritative legality. |
| Explainability and observation | Decision records contain inputs, terms and selections; [audit.rs](../../src/audit.rs) has immutable referenced contexts. [Explorer](../../tools/history-explorer/README.md) and [recorded reader](../../tools/workbench/src/recorded.rs) distinguish actor decisions and observer consequences. | Plan/proposal provenance, rejected-plan reasons and inference/fallback receipts. Model-written rationales are not authoritative causal explanations. |
| Deterministic continuation | [runtime.rs](../../prototypes/persistent-world/src/runtime.rs), [Session](../../prototypes/persistent-world/src/session.rs) and [persistent026](../../prototypes/persistent-delayed-work-world/README.md) preserve complete typed state with strict identity and explicit recovery. The026 successor is accepted for its finite registered headless scope. | Saved controller/goal/plan state, inference request lifecycle, budget counters and recorded external-result playback. Existing save formats do not cover those new fields or arbitrary populations. |

The [feasibility assessment](living-world-feasibility-assessment.md) documents the
four-actor assumptions, finite horizons and growing history costs. They remain
bottlenecks regardless of the intelligence architecture. Optional inference does
not solve population generalization, renewable resources, spatial sensing or UI
integration. Protected mechanisms remain unadopted unless separately chosen for a
product rule set; protection preserves scientific findings, including harms/nulls.

## Conventional reasoning architecture

Define a product `OwnedView`: own physical state, retained memories/beliefs,
directed relationships, active goals, received commitments and legitimate current
observations. Facts distinguish known, uncertain and unknown, with receipt and
observation dates. Utility, planning and inference receive projections of this
same view. The resolver may inspect authoritative state to execute actions; it
cannot leak its private checks back into the candidate generator as advance truth.

**Utility selection** runs whenever an idle character has an opportunity, or when
an interrupt requires reconsideration. Candidates include immediate needs, ongoing
plan steps, social responses and waiting. Terms can include urgency, goal progress,
locally estimated costs, relationship preference, risk and commitment continuity.
Specify bounded integer arithmetic and ties. Preserve urgent alternatives even
when a plan exists; do not make plan completion override eating or voluntary exit.

**Planning** provides candidate sequences, not a second authority that bypasses
utility. Start with a small HTN-style method library: a goal such as establishing
a food reserve decomposes into known acquisition/preparation, retaining stock or
requesting an exchange. Methods are general rules with preconditions, expected
effects, effort and failure branches, not scripts for named agents. Utility chooses
which goal/next step currently deserves execution.

| Approach | Fit | Recommendation |
|---|---|---|
| Utility alone | Cheap, explainable frequent choices; existing implementation is a strong starting point. Limited explicit representation of delayed dependencies. | Retain as the always-available fallback and immediate selector. |
| Bounded GOAP-style search | Composes typed operators to reach goals in an actor's believed state. Flexible combinations; search and inaccurate hypothetical effects can grow rapidly. Social consent cannot be treated as a guaranteed operator effect. | Consider after action semantics stabilize; use explicit depth/node caps and uncertain effects. Do not begin with unrestricted search. |
| Small hierarchical methods | Predictable search, readable decomposition and designer control of legal strategy vocabulary. Coverage is limited by authored methods. | Recommended first planner. Allow alternative methods and failure recovery, not fixed narratives. |
| LLM proposal assistance | Can suggest another decomposition or conditional strategy within the declared domain. Novel wording alone has no simulation effect. | Optional candidate source after conventional planning works. No direct policy override. |

All planning occurs over an explicitly limited belief model. A predicted transfer
is conditional on the other actor agreeing; an unobserved source may be depleted.
An agent may choose an information-gathering step rather than assuming an unknown
precondition. Execution rechecks conditions, delivers legitimate failure feedback
and replans as needed. Do not run hypothetical plans through the true ECS world
and expose resulting hidden-state outcomes to the planner.

Maintain only a bounded plan prefix and a few alternatives. Record why a plan
started, which goal it serves, what it assumes, when it becomes invalid and which
receipt established progress. Reconsider on relevant local observations, completion,
failure, need interrupts or a declared expiry, rather than rebuilding every actor's
plans every tick. Persistence restores the actual plan and its assumptions; it
does not recreate a more informed plan from historical exports.

## Optional inference as a proposal service

Place the gateway outside the ECS worker. The core emits an immutable request
containing actor/branch/request IDs, logical boundary, schema/rules identity, the
bounded permitted context and a finite goal/operator catalog. The adapter manages
provider access and returns data. Provider secrets, transport retries and wall-clock
telemetry stay outside actor state. Exact operational API choices are deferred.

Use deterministic local triggers: repeated locally recorded plan failure, an
unfamiliar combination of retained facts, persistent goal blockage, or a consequential
choice visible to this actor under declared rules. A global hidden crisis flag,
future event or observer assessment cannot secretly choose whom to assist. A low
utility-score margin is a possible trigger, not proof that inference has value.

Return a typed strategy proposal: goal ID, known actor/source references, bounded
steps or method selections, conditions, expected outcomes, and optional references
to supplied receipts. Unknown IDs, unsupported operators, oversized responses,
cycles and fabricated receipts are rejected. Free text is explanatory display data;
it cannot define executable effects, install code, call tools, grant authority or
create obligations for another character.

The deterministic pipeline is:

1. Check request binding, schema, bounds and current relevance.
2. Ground references against the actor's currently usable knowledge and the allowed
   vocabulary. A response cannot revive context evicted while inference ran.
3. Compile the proposed method/sequence into the same bounded plan representation
   used by the conventional planner. Estimate it with the same local model.
4. Score feasible next steps with the same utility evaluator. Preserve the best
   conventional candidate and its safe fallback. The model does not supply a
   privileged utility score or automatic tie advantage.
5. Admit the chosen intent through common legality, reservations and recipient
   consent; resolve phases and mutate state through the authoritative resolver.
6. Learn only from legitimately experienced outcomes. A persuasive prediction is
   not an experience, belief receipt or success record.

The model can introduce a new combination of existing steps, not a new world action
or an invented game mechanic. A strategy such as persuading a steward is unavailable
until the product has actual proposals, permissions and consent rules for it.
If the action catalog cannot express a proposed strategy, reject or defer it;
natural language must not become a hidden simulation engine.

## Determinism, latency and replay

Live inference is an external input. A seed and a low sampling setting do not make
fresh remote inference a deterministic simulation primitive. Exact replay means
replaying the recorded external-input stream with the pinned runtime, rules and
configuration; it does not mean reproducing provider inference from a seed.

Record request ID, branch/actor IDs, request context/hash, relevant policy versions,
provider/model identifier as returned, requested inference settings, raw response,
parsed proposal, validation status, admitted/discarded boundary, budget decision and
fallback outcome. Preserve validation and selected-action records separately from
the provider's explanation. Do not claim the model's internal reasoning was recorded.
Usage, monetary cost and network latency belong in operational telemetry; only
recorded outcomes needed to reproduce decisions enter the authoritative input log.

In the first comparison, pause logical time at a declared decision boundary, acquire
or fail the optional proposal, record the result and then resume. Timeout produces
a recorded failure/fallback event. This isolates strategy quality from asynchronous
scheduling. It is a small-test protocol, not the recommended always-on UX.

For a later live host, conventional actions continue while inference is pending.
The host admits response events only at safe boundaries, records their actual
ordering, expires stale requests and discards late responses explicitly. Two live
runs with different network timing can diverge even with the same seed; their own
recorded streams must each replay exactly. If identical timing-independent live
outcomes are required, a barrier or precomputed inputs are necessary, with a latency
tradeoff. Sorting responses cannot remove timing-dependent availability by itself.

Save pending request identities, consumed/expired status, plans and budget counters.
After restore, never automatically reissue an already consumed request or silently
substitute a fresh response. Unresolved inference can be explicitly abandoned to
the conventional fallback and logged. Replay makes no network calls; a missing or
misbound required response is a replay integrity failure, not permission to ask the
provider again. A separately selected inference-off run remains fully executable.

## Budgets, fallback and population scaling

Budgets must be enforceable configuration, not instructions in a prompt. Start
with inference disabled by default in the product prototype. Suggested controls:

| Budget | Deterministic control and consequence |
|---|---|
| Calls per actor and simulation interval | Logical-time window, cooldown and saved counters. Denial selects the normal conventional candidate. |
| Whole-world quota | Stable arbitration over eligible actor requests, with a declared rotating priority or other explicit rule. Quota access can affect outcomes and must be visible in the audit. |
| Context/output size | Fixed fact/receipt counts, byte limits and output bounds; deterministic truncation respects retention and relevance. Never retrieve evicted context from observer storage. |
| Planner work | Node/depth/candidate caps, deterministic expansion order and a retained best conventional result on exhaustion. |
| Network concurrency/retries | Bounded queue and attempts in the host; no ECS blocking. Record terminal errors and late-result rejection. |
| Monetary limits | Reserve against a configured upper cost estimate before sending; reconcile actual usage externally. Changes in budget availability enter the run as recorded host configuration/input decisions. No silent paid overrun. |

Fallback order: finish a still-valid current step if appropriate; otherwise select
the best conventional candidate; if none is admissible, wait or take the declared
safe action. Inference-off and quota-zero must match the conventional controller's
choices and world/actor state exactly. Configuration and inference-audit metadata
can differ across modes; complete-state restore equality is tested within the same
configuration. A model refusal, malformed reply, outage or exhausted budget must not
prevent needs, work, communication or institutions from operating.

At 8–12 agents, bound planner expansion before trying parallel policies. Inference
volume scales with eligible situations and context size, not just population; one
call per actor per tick would defeat the architecture. A global quota keeps provider
load bounded, but can allocate less assistance per actor as population grows.
Measure trigger rate, queue occupancy, denied requests and opportunity distribution.
Maintain deterministic single-writer resolution even if requests run concurrently.

Do not cache solely by textual prompt or goal name. Any reuse key must include
permitted context, rules/catalog version and relevant assumptions. Exact cached
inference is an external-result optimization; a character remembering a strategy
is a separate simulated cognitive mechanism. Shared infrastructure caches must not
reveal one actor's private facts or successful private tactics to another.

## Explainability and risks

The authoritative explanation should show: actor-local facts/receipts; goal and
method; candidate next actions; score terms and assumptions; proposal provenance;
budget/validation decisions; final selection; recipient consent; execution and
observed consequences. A concise UI can say “saved effort under this actor's
estimate; source availability remained uncertain,” with drill-down references.
It must not present model prose as the cause when the actual selector rejected the
proposal or a resolver prevented execution.

| Risk | Design response |
|---|---|
| Omniscience through prompts or planning | One owned-view boundary; hidden-state twin tests; separate observer exports. Audit whether merely triggering inference leaks privileged information. |
| Hallucinated capabilities or guaranteed cooperation | Typed catalog, grounded references, conditional plans and common resolver. Revalidate at execution; reject unsupported actions. |
| Prompt injection in memories or social messages | Treat all quoted content as untrusted data. The gateway has no game-mutation tools; enforce schemas and capabilities outside the model. A fictional message cannot change host budgets or policy. |
| Plausible but inaccurate explanations | Derive causes from structured scores and execution records. Label supplied rationales as model claims. No chain-of-thought requirement or claim. |
| Compute bias in the comparison | Match catalogs, context, planning caps and final candidate caps; retain the best conventional candidate. Report additional inference cost explicitly. |
| Goal oscillation, starvation and plan lock-in | Interrupt rules, bounded commitment preference, expiry, failed-precondition receipts and deterministic replanning. |
| Population-order and quota effects | Stable documented arbitration; report per-actor assistance and conventional activity shares. Do not infer fairness from determinism. |
| External data disclosure | Send only the explicitly permitted actor view through a separately configured adapter; keep credentials and human notes out of prompts. Provider configuration is an implementation decision for a later authorized task. |
| Strategy spread homogenizes society | Local acquisition, bounded retention, explicit transmission costs, diverse goals and fallible transfer. Do not install a globally best policy after one observed success. |
| Research-contract contamination | New product package/schema/rules; protected hosts/readers untouched; optional inference never runs inside historical gates. |

## Remembering, reusing and transmitting strategies

Later introduce a bounded actor-owned strategy memory, separate from episodic
events and source × inquiry-method expectations. A retained strategy contains a
typed template, goal/context signature, required capabilities, conditional steps,
observed attempts/outcomes, effort, provenance and last-used/expiry information.
Record failures and harmful side effects as well as successes. “Successful” means
the actor legitimately observed progress on a defined goal; it does not imply
global welfare, causal proof or knowledge of hidden results.

Learn a template only after real execution, whether the original proposal came
from conventional planning, inference or another actor. Retrieve it using currently
retained context and recheck assumptions. Use bounded capacity and explicit
replacement; do not silently recover forgotten templates from audit history.
Separate a retained strategy from the individual episodes that once supported it,
with a stated rule for what can persist when those episodes expire.

Social transmission must be an actual action: a sender chooses a retained strategy
to teach or recommend; a recipient can refuse, spend time, interpret the claim and
store it with sender/known attribution and uncertainty. A reported success is
testimony, not the recipient's own demonstrated competence. Costs, prerequisites,
missing context, deception and misapplication can prevent success. Preserve known
relay lineage so repetition does not manufacture independent support.

This is a future path to local traditions and institutions, not a claim that the
current provenance or learning mechanisms already create culture. Start with
individual reuse; add transmission only after its failure and forgetting semantics
are clear. Never use the provider's persistent session or a shared vector store as
an undocumented character memory.

## Smallest useful comparison

Recommend a separate bounded product-controller comparison after the conventional
action/planning seam exists. Do not retrofit inference into the frozen026 host,
retune027 or register028 through this document. The earlier assessment's dynamic
eight-agent kernel remains the first world-building task; this smaller four-agent
comparison can test the intelligence seam without waiting for a political society.

**Question:** Does one selectively requested typed strategy proposal improve an
actor's realized, locally defined goal progress beyond the same utility + bounded
planner, after accounting for effort, failures, inference latency and cost?

Use four agents, one finite work/resource environment and a small existing product
action catalog. One focal agent has an observable reserve/contribution objective;
other agents use the same conventional controllers in all arms. Give all actors
initial conditions only, with independent invitations and consent during execution.
The optional model may suggest combinations of legal work, waiting, exchange or
inquiry; no free-form dialogue or invented political actions are needed.

Use three fixed scenario families with four matched seeds each: a familiar path,
an unfamiliar combination of known preconditions, and a locally visible social
blockage where another actor may refuse. This yields 12 matched pairs/24 finite
runs. Fix the seeds, horizon, goals, trigger, prompt schema, model configuration,
budgets and measures before obtaining responses. This is a diagnostic sample, not
a population-level statistical or social-emergence claim. Social strategy diversity
may be too restricted to expose an effect; preserve that negative result rather
than changing rules mid-run to make assistance win.

| Arm | Controller |
|---|---|
| A: conventional | Identical utility selector and bounded hierarchical planner; no external inference. |
| B: selective assistance | Same controller plus at most one request per run at the first eligible local blockage; at most two typed plan proposals, each at most four steps. Same final evaluator and resolver. |

Suggested initial shared bounds are 200 deterministic planner expansions and eight
final candidates per decision. Both arms retain the conventional best candidate;
B may fill up to two of the other slots with grounded model proposals. Log which
conventional alternatives were displaced. These numbers are proposed test limits,
not measured optimal capacities. Both arms see the same permitted context and
operators. LLM inference adds compute and information processing, not extra world
facts or higher action permissions. Future conventional search with a larger budget
is a useful follow-up if B finds a benefit; this first comparison alone cannot prove
that an LLM is better than every conventional planner.

Acquire B's response at a paused logical decision boundary, as described above.
Model unavailability or invalid output uses the fallback and remains in the paired
results; do not reroll until a useful answer appears. Replay B from its recorded
response without a network call. Service-latency effects on an always-on world are
outside this initial comparison and require a later asynchronous test.

Report every pair, including no-trigger/null/harm cases:

- Goal completion/progress by the fixed horizon, elapsed actor work and consumed
  resources; hunger trajectory and commitments missed, separately per actor.
- Proposed, schema-valid, grounded, selected and executed strategies as separate
  counts; changed scores are not changed choices, and changed choices are not wins.
- Common-rule violations, refused requests and failed assumptions; exact replay
  and interrupted/restored complete-state equality.
- Calls, prompt/output sizes, operational latency and monetary usage separately
  from deterministic world records; conventional planner expansion counts.
- Whether an observer can trace the selected strategy to local evidence, utility
  terms, consent and actual consequences without relying on generated prose.

Add recorded fault controls for malformed proposals, unknown/private references,
unsupported actions, exhausted quota, timeout and missing/mismatched replay receipt.
These controls need not make paid model calls. Inference-off/quota-zero must be
exactly conventional; missing replay input must fail closed as a replay error.

Do not adopt assistance merely because the model writes a plausible plan. A useful
result needs an executed difference with an interpretable benefit/cost tradeoff,
without locality/accounting/replay violations. If B never changes an executed
choice, or its costs dominate the observed benefit, keep the conventional system
and record the result. One successful narrow comparison would justify another
bounded product evaluation, not population-wide LLM dependence or global strategy
sharing.

## Completion and preservation

Only new planning documents are changed. No inference calls, feature implementation,
new experimental evidence, protected-source edits or historical replay modifications
were performed. Existing scientific contracts remain authoritative.

Repository checks at the unchanged source/fixture baseline: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo test` (217 passed, 0 failed),
`cargo run --release --locked -j 1 --example validate_contract` and
`cargo run --release --locked -j 1 --example validate026_foundation` all exit 0.
The full026 gate confirms 495 portable/512 raw originals and 93 custody bindings.
Verbose logs: `%TEMP%/eldoria-feasibility-20261117/`. These results protect the
baseline; they do not validate an unimplemented hybrid controller. Final Markdown
link/diff checks apply to both new documents. No new performance benchmark or GUI
test is warranted for these documentation-only additions.
