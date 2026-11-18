# Eldoria living-world simulation feasibility assessment

Assessment date: 2026-11-17. Inspected baseline: `eb7eec6f12fac7a43d90f40f6053eef8de4a8197`.
Scope: repository architecture, research evidence and product planning. This document
authorizes no implementation, experiment, policy adoption or canon change.
The companion [hybrid intelligence exploration](hybrid-character-intelligence.md)
develops the conventional-planning and optional-inference controller boundary.

## Recommendation and feasibility

Build a separate, versioned Eldoria product runtime and an observer application.
Retain Rust and Bevy ECS, reuse the strongest local decision and accounting
primitives, and replace the fixed research coordinator with a population-independent
scheduler. Use the Rust workbench as the first desktop shell and the History
Explorer as a reference for explaining decisions and tracing consequences.

An interesting eight-person settlement is a credible incremental engineering goal.
A self-sustaining, indefinitely running social and political world is not an
existing capability. Twelve agents are unlikely to make basic arithmetic expensive;
the unresolved challenges are meaningful action selection, economic continuity,
local information, scheduling and history growth. That is an engineering inference,
not a measured twelve-agent performance result.

Prioritize a usable small simulation with short feedback cycles. Each milestone
should deliver something a person can observe and evaluate for interest, clarity
and frustration. Do not require another numbered research experiment before every
product feature. Preserve the research as a read-only reference and regression
boundary, with separate product rules and saves. Product tuning can optimize the
experience without revising what the original experiments established.

## Evidence and inspection boundaries

**Verified implementation** below means source inspection establishes that the
mechanism exists. **Validated scope** means a linked acceptance report establishes
specific tested behavior; it does not establish performance or generality outside
that scope. **Experimental** identifies researched, often protected mechanisms
whose default adoption or general usefulness remains unestablished. **Proposed**
identifies new product architecture or behavior, with no implementation claim.

Navigation started with the [architecture map](../architecture-map.md),
[experiment history](../simulation/EXPERIMENT_HISTORY.md) and
[context workflow](../context-workflow.md). Inspection expanded into root ECS,
cognition and scheduler sources; continuous/material/delayed-work hosts and their
generators; persistence/session sources; workbench integration; and Explorer
documentation. Frozen trace corpora were not loaded or regenerated. This is an
architecture audit, not a new independent scientific review of all 27 experiments.
Completion checks are recorded at the end of this document.

The current protected endpoint is [qualified 001–026](../experiment-contract-001-026.md).
027 is a completed, unadopted, unprotected score-only null: 58 score changes and no
changed choices or execution across 100 worlds. Its failure-bearing eligible
feedback and meal attribution were not exercised; see
[027 results](../../experiments/027/results.md). It supplies no demonstrated solution
to autonomous work allocation and should not become a product prerequisite.

## Existing components and reuse decisions

“Direct reuse” means consume an unchanged public primitive where its semantics fit;
it does not mean modifying a protected module to accommodate product requirements.
Some useful mechanisms are private, embedded in generated hosts or tightly coupled
to historical types; their reuse requires a product-owned implementation or adapter.

| Area and source | Verified capability and qualification | Product disposition |
|---|---|---|
| [Cargo.toml](../../Cargo.toml), [Simulation::new](../../src/simulation.rs), [Agent](../../src/model.rs) | Bevy ECS 0.19.1; agents are components, much cognition is in resources keyed by stable `u32` IDs. The schedule uses a single-thread executor. Root construction accepts a vector of agents, but that does not generalize the later world coordinator. | Reuse ECS and stable-ID discipline directly. Introduce product-owned state types and explicit ID-to-entity reconstruction. Do not serialize ECS handles. |
| [behavior.rs](../../src/behavior.rs) | Pure replaceable scoring, legal actions, deterministic ties and captured decision inputs; concrete resource actions are Accept/Offer/Request/Refuse/Leave. | Reuse unchanged functions for compatible exchanges and as executable examples. Adapt to a broader action protocol; replace assumptions about one-unit scenes and fixed score scales where necessary in product code. |
| [model.rs](../../src/model.rs), [history.rs](../../src/history.rs) | Episode cap 16; directed trust persists separately. Resource policy consults the last four retained partner episodes. Indexed historical contributions support trust correction. Episodic eviction is not removal of every consequence. | Reuse memory/trust semantics selectively. Separate active cognition, sufficient relationship state and archival indexes before long-duration operation; do not recover evicted cognition from an observer log. |
| [cognition.rs](../../src/cognition.rs), [assessment.rs](../../src/assessment.rs), [provenance.rs](../../src/provenance.rs), [retention.rs](../../src/retention.rs) | Event-specific fallible beliefs, revisions, received attribution and bounded assessment basis (32). Receipt identity and locally known origin are distinct. Quality/Salient retention are experimental, not established general winners. | Adapt typed receipts and provenance to product facts. Retain FIFO initially where applicable; broader claims and reputation require a new vocabulary. Keep original algorithms and experiments unchanged. |
| [intentional.rs](../../src/intentional.rs), [simulation.rs](../../src/simulation.rs) | Local `TalkInput`, voluntary silence/explanation/evidence/misleading actions and directed credibility. Automatic conversation opportunities follow resource refusals. | Reuse locality, credibility and consent patterns. Substantially extend the protocol: this is not general conversation, bargaining, promises or political persuasion. |
| [concerns.rs](../../src/concerns.rs), [inquiry.rs](../../src/inquiry.rs), [self_evaluation.rs](../../src/self_evaluation.rs) | Up to eight concerns; the concern question is specifically `RefusalScarcity`. Inquiry has bounded source/strategy estimates and explicit effort. Contextual/progress learning remains experimental. | Reuse bounded intent and evidence-driven update patterns. Replace the domain-specific concern model for ambitions; do not relabel it as a general goal planner. Avoid enabling every research heuristic together by default. |
| [encounters.rs](../../src/encounters.rs), [contact_learning.rs](../../src/contact_learning.rs), [opportunities.rs](../../src/opportunities.rs) | Agents can initiate resource/inquiry invitations, recipients choose separately, and arbitration checks commitments. Fixed ordering and Continue-score coupling have qualified outcomes. | Adapt proposal/consent/reservation stages. Replace population-specific ordering and extend the candidate domain. More invitations are not evidence of better social life. |
| [recurring.rs](../../src/recurring.rs), [acquisition.rs](../../src/acquisition.rs), [environmental_knowledge.rs](../../src/environmental_knowledge.rs) | Shared elapsed hunger pressure, consumption, finite claims with completion-time stock checks, and one owned dated environment slot. Rival depletion does not magically refresh local knowledge. | Reuse arithmetic and transactional accounting where compatible. Adapt to locations, multiple sources and longer runs. Resource renewal, production ecology, mortality and a broad inventory economy are new rules. |
| [concurrent.rs](../../src/simulation/concurrent.rs), [experiment022.rs](../../src/experiment022.rs), [023 coordinator](../../prototypes/continuous-world/src/coordinator.rs), [023 model](../../prototypes/continuous-world/src/model.rs) | Four-person concurrent commitments and shared time; continuous eligibility uses `now + 33`. Arrays, loops, public-context validation and rank arithmetic explicitly assume IDs 0–3. Queries remain atomic and conservatively admitted. Nominal cutoff plus drain is finite. | Replace coordinator in the product. Keep reservation, deadline and post-query reprojection lessons. Do not increase a single population constant or stitch finite trials together and claim continuity. |
| [025 material model](../../prototypes/material-preparation/src/model.rs), [material coordinator](../../prototypes/material-preparation/src/coordinator.rs), [observer2 host](../../prototypes/material-preparation-observer2/src/lib.rs) | Finite raw extraction, escrow, preparation, edible output and waste. Settings and observation offsets assume four actors. Qualified025 demonstrates actual later use, failures and costs, not sustainable production. | Adapt the work lifecycle and accounting to product jobs. Replace fixture observations with spatial sensing and replace finite input supply with a separately specified ecology when that milestone is reached. |
| [026 valuation](../../prototypes/delayed-work-valuation/src/model.rs), [C1 model](../../prototypes/delayed-work-valuation-c1/src/model.rs) | ProjectedNeed changes only eligible Prepare scoring. C1 supplies the qualified missing-member Continue correction. No general planner; adverse and nutritional-null outcomes remain. | Reference or explicitly select a product heuristic after testing its behavior. Do not presume protection means adoption, or combine MealRelief and ProjectedNeed as an already validated product policy. |
| [persistent runtime](../../prototypes/persistent-world/src/runtime.rs), [Session](../../prototypes/persistent-world/src/session.rs), [reliability](../../prototypes/persistent-world/src/reliability.rs), [codec](../../prototypes/persistent-world/src/storage_codec.rs) | Typed complete-state restore, immutable generations, journal binding, writer exclusion, explicit recovery and optional compressed transport. State contains cognition, pending work, counters and history indexes, not merely actors. | Reuse the storage design and compatible utilities; adapt behind a product checkpoint interface. Existing schemas and runtime identities must remain strict. |
| [persistent026 host](../../prototypes/persistent-delayed-work-world/src/lib.rs), [generator](../../prototypes/persistent-delayed-work-world/generate_host.py), [README](../../prototypes/persistent-delayed-work-world/README.md) | Accepted headless continuation for 100 registered worlds, 1,523 full-state splits and 4,905 CLI invocations. Configuration selects the exact registered matrix; it is not an arbitrary-world runtime. Host assembly uses generated source and counted substitutions. | Strong behavioral and persistence reference. Do not grow the generator stack into the product architecture. New authored modules, schema and runtime identity should own the successor. |

## Observer application foundations

| Candidate | Suitable reuse | Limit and decision |
|---|---|---|
| [History Explorer](../../tools/history-explorer/README.md), [architecture](../development/history-explorer-architecture.md), `explorer.py`, `causal.py`/`causal.js`, `perspective.js`, `native-concurrent.js` | Actor/observer separation, timelines, source-qualified causal links, matched comparisons and significant-event navigation. It is a functioning read-only browser application with explicit experiment adapters. | Playback advances a recorded history, not the world. Keep the archive viewer intact. A future live adapter needs a separate versioned stream; the Python archive server should not own simulation commands or live cognition. |
| [Rust workbench manifest](../../tools/workbench/Cargo.toml), [desktop worlds](../../tools/workbench/src/desktop_worlds.rs), [sessions](../../tools/workbench/src/world_sessions.rs), [recorded reader](../../tools/workbench/src/recorded.rs), [portable report](../development/portable-workbench-engineering.md) | egui/eframe shell, external workspace storage, worker execution, notes, recorded inspection and portable Windows packaging. Actual packaged run/reproduce/note/reopen GUI interaction is documented. | Recommend a separate product executable that reuses appropriate shell patterns. Existing session integration links persistent023/024 and explicitly rejects025 persistence; accepted headless026 is not wired into it. Existing recorded lanes and reader bounds are not a generic live feed. |
| [Persistent material](../development/persistent-material-world.md) and [persistent026 final report](../development/persistent-delayed-work-final-report.md) | Real process continuation and recovery provide the best evidence that paused activity and local cognition can survive restart. | Headless acceptance only for those successors. Native Pause/Finalize/persistence release acceptance remains deferred in the [recovery report](../development/persistent-world-recovery.md). Portable recorded-run GUI acceptance does not close that gap. |

Use a simple settlement map and character cards before investing in 3D, animation
or rich natural-language dialogue. A location graph and icons can make distance,
work, isolation and meetings legible at much lower cost. Bevy ECS does not require
the full Bevy renderer. Keep rendering replaceable; there is no present need to
rewrite the core in a browser engine or deploy a remote service.

## Obstacles to 8–12 unscripted agents

1. **Population assumptions cross several layers.** `LocalContexts`, need records,
   environment slots, four-person validation, coordinator loops, observation offsets
   and UI lanes all need a deliberate product representation. With stable IDs,
   runtime slots must be explicit; actor ID must not double as an array offset.
   Generalize scheduling, state serialization and projections together.
2. **Continuous eligibility is not indefinite life.** The023 configuration bounds
   horizon at 1–1024; runtime `complete()` checks a finite end. Persistent026 accepts
   only registered configurations. A product needs uninterrupted logical time,
   safe pause boundaries and an explicit stop policy, rather than rebootstrap at
   report boundaries. Host uptime and offline time advancement are separate
   decisions. Initially, closing the application should pause the world.
3. **Finite food eventually removes meaningful options.** Existing work consumes
   finite inputs; hunger saturation does not provide an ecology or survival model.
   A short finite observer slice is viable, with depletion honestly visible.
   Sustained operation requires explicit replenishment/production rates, access
   costs, stock ceilings and failure behavior. Never invisibly refill inventories
   to keep a story moving.
4. **Most social reasoning concerns a narrow event type.** Refusal explanations,
   scarcity beliefs and source learning do not create friendship projects, civic
   disagreements, bargaining or mutual obligations. Those require new action
   opportunities and persistent state with consequences.
5. **Current shared context is spatially thin.** Public availability and a common
   source do not establish travel, audibility or a local social network. Add locations,
   timed travel and explicit visibility. Candidate partners come from legitimately
   known or co-present individuals, not a global scan of hidden actor state.
6. **Atomic inquiry limits scheduling.** Present admission avoids crossing pending
   deadlines and can require an exclusive lane. At twelve agents this may suppress
   useful concurrency. Model product conversations as bounded scheduled steps with
   actor reservations; first retain conservative semantics and measure blockage.
7. **Autonomy exists but interesting autonomy is unproven.** Invitation choice already
   removes the need to schedule every meeting manually. However, fixed score scales,
   scarcity and ties can produce repetitive Request→Leave loops, starvation of
   activities and population-order effects. Record chosen alternatives, failed
   attempts and time use; tune product rules against those symptoms.
8. **Long-duration storage is not bounded by memory caps.** Episodes can be bounded
   while journal records, trust attribution indexes, context chronology and complete
   snapshots grow. A world that resumes correctly in a finite campaign still needs
   a history/storage design for days of use.

Unscripted means initial conditions and general rules generate opportunities and
choices. It permits authored geography, traits and resource rules. It excludes
timed instructions that force named agents to meet, cooperate or elect a particular
leader. A fixed seed is reproducibility, not a prewritten plot.

## Proposed social and political rules

Introduce these in stages, starting with two or three ambitions and one consequential
institution. They are proposed extensions, not capabilities demonstrated by trust
or concern experiments.

| Mechanism | Minimum operative rule | Observable consequence and failure possibility |
|---|---|---|
| Ambitions | An actor owns a bounded set of goals with target, priority, locally assessed progress, expiry/abandon conditions and references to known opportunities. Candidate actions score needs, relationships, goal progress, effort and uncertainty on documented scales. Start with securing a reserve, maintaining a partnership and contributing to a common project. | A goal changes an actual choice or commitment and can be delayed, abandoned or fail. A label on a biography is insufficient. No omniscient future payoff or full-world planner is needed. |
| Social groups | Explicit membership proposals and acceptance; an identifiable shared project, contribution ledger, membership exit and rules for access to shared assets. Begin with one work association and allow refusal or departure. | Members sacrifice time/resources and may receive benefits. Free riding, exclusion, internal disagreement and project failure remain legal outcomes. Shared accounts cannot reveal private member beliefs. |
| Reputation | Each listener retains bounded, sourced claims about a subject and domain, derived from witnessing or explicit testimony. Repeated/relayed claims retain known attribution and cannot manufacture independent support. Keep trust, credibility and reputation separate. | Different observers can disagree; mistaken reputations affect invitations, voting or access. No universal reputation scalar appears in every actor's input. |
| Competing interests | Scarce work slots, contribution burdens and rival allocations create alternatives with real opportunity costs. Actors propose, negotiate, accept or refuse using their own goals and beliefs. | Disputes change work, transfers or project completion. Do not enforce consensus or a target amount of conflict. |
| Political influence | A small association proposes a shared-store rule; eligible members consent to a charter and later select a steward through a declared voting procedure. Steward authority is a limited permission to allocate the shared store within that charter, with term expiry and replacement. | Support and persuasion can alter ballots; ballots grant limited authority; allocations alter real stock and future preferences. Rejection and deadlock are valid. A title without executable permissions is cosmetic. |

For example, two agents may favor storing food and two may favor immediate common
meals. Contributions establish who can vote under the declared charter; ballots
can choose a steward; the steward allocates only association-owned food. Witnesses
and recipients learn about actual allocations and may later withdraw support.
This is a possible causal chain to implement and observe, not a promised narrative.
Eight agents can support local institutional politics, not evidence of a kingdom's
social realism. Diplomacy, warfare, dynasties and magic should wait for a working
small community and specific product needs.

## Recommended runtime and application boundaries

Proposed ownership, with names illustrative rather than created paths:

| Boundary | Owns | Must not own |
|---|---|---|
| Product core | Stable IDs, authoritative physical state, local cognition, rules, legality, reservations, accounting and one logical clock | Widgets, wall-clock pacing, observer notes, archive readers or global knowledge in policy inputs |
| Policy/controller port | Actor-local observations and legal intents; autonomous policy or future human controller | Direct ECS access, inventory mutation, recipient consent or resolver authority |
| Session host | Single writer, deterministic command ordering, advance/pause, durable generations, command log and crash recovery | Reconstructing private state from viewer exports or silently accepting incompatible saves |
| Projection service | Immutable observer snapshots, actor-perspective views, branch-qualified event references and paged history | Changing decisions or using UI filters to change simulated sensing |
| Desktop application | Map/cards/timeline, run speed, step, notes, recovery prompts and eventual player intent entry | Owning the simulation clock or blocking the core while rendering |
| Research compatibility tools | Existing archive gates and explicitly versioned adapters | Depending on the new product runtime or treating product saves as original evidence |

One worker should own the ECS world and command queue initially. At each safe
boundary, resolve due work in a documented stable order, apply elapsed needs and
environment rules, deliver legitimate observations, collect actor intents, arbitrate
reservations and record resulting transitions. Specify the exact phase ordering
before implementation: it affects whether someone can eat, hear a claim or reserve
a source on the same tick. Keep rendering asynchronous through bounded snapshot
delivery. Dropping superseded display frames is acceptable; dropping authoritative
events is not. Slowing real-time pacing must not change logical outcomes.

The same session host can eventually run as a headless process while observers
disconnect and reconnect. Always-on operation additionally needs process supervision,
storage limits and an explicit failure/restart policy; it is separate from the first
desktop release's pause-on-close behavior. Reopening after downtime must not invent
unexecuted history or silently fast-forward the world.

Use an initial tick-based scheduler for simplicity. A deterministic due-event queue
can later skip empty intervals if profiling warrants it, with exact elapsed-need
and sensing semantics preserved. Avoid parallel policy execution until needed;
ECS availability is not a reason to introduce nondeterministic resolution.

Split persistence into complete authoritative checkpoints and an append-only stream
of commands/events. Checkpoint RNG state, counters, pending commitments, controller
ownership, goals, membership, local beliefs and any policy-relevant summaries.
Event history is for audit/replay, not an alternate source of forgotten knowledge.
Define checkpoint cadence, segmented history, indexes and explicit archival policy
before calling the runtime long-lived. Archive immutable segments; do not silently
delete attribution needed by future belief revision. Any compaction needs a proved
sufficient summary or an explicit product rule limiting revisability.

Reuse strict runtime/schema identities and explicit recovery into a fresh branch.
Do not loosen old readers to load product saves. Checksum integrity is not hostile
tamper authentication, and current sync/rename tests do not establish sudden-power-
loss durability. Initial support can promise ordinary process restart only.

## Future player participation

Put autonomy behind a controller interface from the beginning. A controller receives
the actor's local view and returns an intent; the same resolver validates and executes
it for either a person or an autonomous policy. Every actor keeps the same hunger,
inventory, action durations, cognitive limits, permissions and exposure to failure.
Switching controllers must not reset those states or grant access to other actors'
private information. Autonomous recipients still decide whether to cooperate.

The observer is an external role with broader inspection privileges. Entering a
character should switch to that actor's allowed projection; diagnostic observer
controls should remain explicitly separate. A person may remember information
outside the simulation, so interface separation cannot guarantee that a player
forgets previously inspected secrets. Decide the product's spoiler policy explicitly.

Start with pause-to-submit intents at safe boundaries and deterministic timeouts
that wait or delegate back to autonomy. Record commands, actor, intended boundary,
sequence and accepted/rejected result for replay. Human input must not hold the
entire world hostage indefinitely. Multiplayer, network authentication and remote
hosting are later decisions; this design does not require them for the first slice.

## Minimal playable observer simulation

The first observer release should be a small named settlement with eight agents
(twelve as a supported stress case), three connected locations, limited travel,
food collection/preparation/consumption, voluntary sharing and a few local goals.
Use distinct initial traits, inventories and ambitions, with no scheduled meetings.
Begin with a finite resource budget and an honestly labeled scenario endpoint;
add explicit renewable supply before promising sustained community life.

The user can start a seed, run/pause/step/change speed, follow a person, inspect a
local decision, trace a transfer or refusal to its effects, bookmark a moment and
save/reopen the actual world. Display location, current activity, relevant need,
goal progress and recent relationships in plain language. Show attempted versus
completed work, uncertain versus observed facts and observer versus actor views.
Avoid presenting raw research matrices as the main interaction.

An enjoyable session should allow a user to identify individuals, anticipate a
choice, be surprised by a consequence and explain it from visible evidence.
Evaluate that through short observation sessions, not only event counts. No minimum
quota of cooperation, betrayal or leadership should force outcomes. Group politics
is a later extension of this slice, not a prerequisite for first useful observation.

## Incremental delivery milestones

| Milestone | Deliverable | Exit evidence |
|---|---|---|
| 0. Separate product boundary | Standalone authored runtime package and written phase/action/checkpoint contracts; all protected files remain unchanged | Build independence, path/identity separation, clean research integrity gates |
| 1. Eight-agent autonomous core | Dynamic stable-ID roster, local observation, timed resource work, voluntary contacts, deterministic arbitration and finite honest stop | 8/12-agent runs without post-setup scripts; same seed/commands reproduce; noncontiguous IDs work; no double booking or food imbalance |
| 2. Observer vertical slice | Settlement display, individual follow, decisions/consequences, pause/step and real save/reopen | Actual desktop workflow; resumed and uninterrupted state agree, including active work and local cognition; a user can explain observed choices |
| 3. Sustained local life | Locations/travel completed, explicit renewable inputs and bounded operational history/storage | Long soak with measured memory/storage slopes, local sensing checks and transparent depletion/recovery; no silent reset at report boundaries |
| 4. Purpose and relationships | Bounded ambitions, project contributions, promises with explicit terms and sourced reputations | Goals alter actual behavior; refusal, failure and misinformation remain possible; observation sessions find consequences understandable and worth following |
| 5. Small-group politics | Consent-based association, shared-store charter, limited stewardship and replaceable authority | A complete proposal→support→authority→allocation→reaction chain, plus rejection/deadlock and membership-exit cases |
| 6. First player role | Controller switch and local intent UI using the same action path | Same legality/accounting for human and autonomous intents; refusal respected; timeout and command replay work |

Milestones may overlap in UI prototyping, but sustained life depends on explicit
resource and persistence rules. After each visible slice, prefer correcting confusing
or repetitive behavior over adding another cognitive subsystem. Dates and staffing
estimates require a measured first slice; this assessment does not invent them.

## Performance and technical risks

Existing data indicates observer overhead deserves early attention. The
[persistent026 report, section G](../development/persistent-delayed-work-final-report.md)
records Source43 native create+advance at a warm median 50.972 ms, checkpoint
serialization/validation 245.628 ms and restore 311.657 ms. Its final raw commit is
10,620,334 bytes and plain journal 14,210,483 bytes. These are finite four-agent
Windows samples, not frame-time or twelve-agent forecasts. The same report records
growing retained generations and a native child peak 894,115,840 bytes during
independent verification; that is not actor cognition size.

| Risk | Mitigation and measurement |
|---|---|
| Whole-history copies and repeated checkpoint hashing dominate | Separate active state and audit storage; page projections; sample checkpoint cadence. Measure advance, capture, save and restore independently, with increasing elapsed history. |
| Candidate/relationship growth | At 12 agents there are 132 directed pairs versus 12 at 4. Simple all-pairs work is still modest, but costs can grow with history and claims. Measure candidates, retained facts and per-tick allocation; use local co-presence before premature indexing. |
| UI retains research-sized documents | Existing recorded reader limits include 64 MiB input and 65,536 scanned items. Add bounded live snapshots, history pagination and backpressure; measure browser/desktop heap separately from core memory. |
| Deadlock, priority starvation or excessive query exclusion | Test conflicting commitments, expiry, cancellation, ties and scarce resources. Measure actor idle time, blocked reasons and opportunity counts; rotating ordering is a rule choice, not proof of fairness. |
| Repetitive equilibrium or collapse | Provide a small set of meaningful alternatives and honest failure states; measure activity time shares and goal progress, then inspect observed play. Avoid repairing dullness with forced named-agent encounters. |
| Integration changes locality or retention | Policy ports expose owned state and legitimate observations only; test invariance to hidden partner changes and unavailable archive data. Preserve separately persistent consequences after episode eviction. |
| Save compatibility and crash behavior | New product schema/runtime identity; exact interrupted/uninterrupted state comparisons; explicit recovery branches; test corrupted checkpoints and unsaved tails. Do not claim arbitrary migration. |
| Source-generator coupling | Keep historical generators untouched; author product code directly with documented provenance and a narrow dependency graph. Avoid requiring frozen-host text substitutions for each feature. |

Proposed initial performance budgets, to validate rather than promise: at 12 agents,
p95 core advancement below 10 ms per logical tick, responsive UI updates at 10 Hz or
better, pause acknowledgment below 250 ms at supported safe boundaries, and a declared
disk budget for a 24-hour soak. Measure warm/cold cases and maximum indivisible work.
Report history growth in bytes per simulated hour and heap after warm-up; acceptance
must explain any growth rather than masking it with an episode cap. Do not tune
against expensive forensic validation as though it were the live frame loop.

## Proposed first implementation task

**Create an isolated population-independent headless product kernel with an
eight-agent autonomous resource scenario.** This is the next proposed task, not work
authorized or performed by this assessment. It is product engineering, not028,
retuning027 or adoption of a protected policy.

Before coding, specify a small product rule sheet: phase order, legal actions,
reservation/consent semantics, finite source accounting, actor-local visibility,
score/tie rules and complete-state ownership. Create a new standalone package with
its own namespace and outputs outside scientific archives. Reuse public pure
primitives only where they fit unchanged; port necessary scheduling concepts into
authored product modules with source references. Keep a neutral rules baseline;
do not activate the entire experimental policy stack.

The task should end with a CLI that initializes 8 or 12 agents, advances a bounded
number of logical ticks, outputs compact decisions/consequences and can serialize
and restore complete state at safe boundaries. It should exercise voluntary
invitations, independent recipient choice, food collection/consumption and failures,
using initial conditions rather than post-setup scripts. A fixed finite run budget
is a test control, not an assertion of indefinite ecology. No groups, voting, LLM,
map renderer, renewable supply or live server is needed for this first task.

Acceptance: exact replay of the same seed/commands; uninterrupted versus restored
state equality including active reservations; IDs with gaps; no overlapping actor
commitments; conservation including consumed units; completion-time stock checks;
hidden-state independence of local choices; terminal/depleted behavior; and compact
performance samples at 8 and 12 agents. Character-specific outcomes need not be
prescribed. Then build the observer workflow on that runtime, rather than another
large batch-only research interface.

## Preservation and assessment completion record

Only this assessment and its new hybrid-intelligence companion are in scope. No
source, historical contract, architecture map, experiment index, original evidence
or existing prototype is to be edited. Existing ownership and experiment statuses
are unchanged, so no map/index
update is required; links here provide the new assessment's navigation. Future
implementation requires a separately authorized task and must retain the
[001–026 gate](../experiment-contract-001-026.md), all historical entrypoints and
separate native026 target directories. Product success never rewrites a null,
failed candidate or qualified scientific conclusion.

Rust checks cover the baseline source/fixture revision above, unchanged by this
assessment. Document link and diff checks cover the added Markdown. Verbose logs
are outside the repository in
`%TEMP%/eldoria-feasibility-20261117/`.

| Check | Result |
|---|---|
| `cargo fmt --check` | Exit 0 |
| `cargo clippy --all-targets -- -D warnings` | Exit 0 |
| `cargo test` | Exit 0; 217 passed, 0 failed, 0 ignored across 28 result summaries, including empty suites |
| Local Markdown targets | Checked in both new planning documents; 0 missing |
| `cargo run --release --locked -j 1 --example validate_contract` | Exit 0; protected 001–024 archive/replay and 408-artifact foundation checks pass |
| `cargo run --release --locked -j 1 --example validate026_foundation` | Exit 0; 495 portable/512 raw originals, 93 custody bindings and 3 layout corruption rejections pass |

No throughput benchmark is needed for a documentation-only change; performance
figures above are explicitly attributed historical observations. No GUI/browser
smoke test is needed because no viewer, projection or runtime changes are made.
