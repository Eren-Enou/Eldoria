# Finite material preparation — architecture investigation

Stage C investigation for attachmentf602f11c. **Prospective design, not runtime or
outcome evidence.** Implementation remains conditional on published001–024 Gate B
and the separate025physical/cognitive/time gate. Numeric policy and matrix choices
belong in the subsequent fixed registration, not inferred from this document.

## Existing ownership and smallest seam

`src/simulation/concurrent.rs` owns private State, four-person availability,
reservations, recipient admission, atomic query exclusion, clock/need sync, physical
completion and conservation. `src/experiment022.rs` owns serialized work/commitment
types. `src/environmental_knowledge.rs` owns the one-slot021cell and pure retained
acquisition projection. `src/acquisition.rs` owns finite ready-food grants/ranking.
The root modules and all historical adapters are source locked.

`prototypes/continuous-world` generates a host with unchanged parent bodies and
an appended023child coordinator/bootstrap. `prototypes/activity-valuation` appends
the024ordinary-scoring coordinator, reverses its declared substitutions and checks
exact parent text. Its selector and physical scheduler are distinct owners.
The current main Explorer consumes original histories; it is not a runtime seam.

A delayed process needs integration with actual occupancy, deadline resolution and
conservation. An external batch calculator or a second independent clock cannot
answer the question. Merely adding another Acquire cache cannot represent raw
escrow, delayed conversion, waste or unfinished work.

## Alternatives

| Design | Assessment |
|---|---|
| Versioned successor host with explicit generated substitutions | Preferred boundary. Preserve old files and paths; generate only the successor's type/scheduler seams, reverse substitutions, compile and independently test them. |
| External opt-in adapter around existing primitives | Useful for policy/observation projection, insufficient alone: existing Claim grants immediately edible food and has one completion deadline. Do not mislabel preparation as an ordinary Claim or fabricate finished food outside physical accounting. |
| Shared physical-commitment abstraction | Could reduce future duplication, but requires refactoring frozen paths and broad compatibility work. Not necessary for this study; reject that scope. |

Use an isolated025workspace. Reuse frozen old policy/consent/negotiation/cognitive
functions. A generated025-only work-type extension can introduce Prepare while
keeping original work variants and serialization stable. The new coordinator must
capture its actual selected five-way action; do not record Wait or Claim when the
actor actually selected Prepare. A versioned025boundary/history type can reuse
old field types without changing frozen022–024record definitions.

The generated successor will need a tightly enumerated physical extension, unlike
024's score-only seam. Mechanical reversal proves the undeclared text stayed
unchanged; it does not independently prove new execution correct. Both disabled
compatibility and primitive reconstruction remain mandatory.

### Bounded implementation seam investigation

Scoped source inspection finds only one explicit concurrent namespace path in
`src/experiment.rs`, and old Work matching concentrated in the private scheduler.
A generated successor can replace that namespace path with a025-only copy of the
record types that adds Prepare, leaving old files unchanged. Retain original
module/include paths by explicit absolute generated bindings and reverse-check
every non-additive substitution.

| Successor-only seam | Required guard |
|---|---|
| New work variant/type binding | Old variants unchanged; every unrelated type body and serialization preserved; no protected module written. |
| Old completion filter excludes Prepare | New phase resolver owns both deadlines; old paths cannot release it prematurely as an ordinary default work item. |
| Phase resolution after inherited same-tick work | Extract, convert and release in the declared order before ordinary selection; check conservation after each operation. |
| Extended physical total | Add initial raw to the conserved total and current raw/escrow/waste to compartments; keep old conservation/occupancy assertions, not a bypass. |
| New coordinator and actual choice type | Reuse old response/admission/legacy ranking; explicitly capture five-way actual selection rather than an old Wait/Claim placeholder. |
| Tagged environmental projection | Cache observation and raw observation are a disjoint union, audited on overwrite; old acquisition sees only current cache tag. |

Globally exclusive inquiry already rejects admission whenever any active work
exists, so using the same Prepare reservation protects extraction as well as final
completion without a new query coroutine. Occupancy/busy assertions already apply
to every non-resource-scene work item; preserve them. The ordinary boundary must
reproject public time after any accepted atomic inquiry as023already does.

Old snapshot validation primarily checks retained cognition/receipts and learned
updates; it is not a substitute for the new complete material ledger. New histories
must keep old cognitive validation and add independent preparation accounting.
These observations support feasibility, not a claim of compiled correctness.

## Minimal new state and common-unit ontology

One stationary source with a stable SourceId distinct from CacheId0, finite raw
stock and declared public access/work/yield. Raw contains an already present edible
fraction. The candidate2raw→1edible+1inert waste law uses one common conserved mass
unit, not two unrelated resource counts. No energy or matter is created.

At most one active batch per actor, with stable commitment/owner/source IDs,
admission tick, extraction deadline, completion deadline, explicit phase and owned
raw escrow. Physical process state is bounded bookkeeping, not remembered event
content or a new planner. Waste and cumulative consumed material persist as physical
ledgers. Original histories remain growing observer records, separately measured.

At every primitive operation:

`M_initial = raw_source + sum(raw_escrow) + ready_cache + sum(owned_food) + waste + consumed`

Use checked integer arithmetic, bounded validated initial quantities and explicit
output room. Failure cannot debit nonexistent raw, credit food/waste, overfill or
discard remainder. Existing Offer/Accept and meals operate on finished food only.
The inherited40-point hunger relief is an abstract need law, not a realistic
metabolic energy model. Finite preparation does not establish renewable production.

## Reservation and physical phase ownership

One Prepare admission reserves its actor for both positive-duration phases. Use
the same active-participant authority and Runtime busy set as other commitments.
At most four occupied actors and one incompatible commitment each. Admission
validates legitimate local eligibility, public access/time and output room, but
must not disclose/revalidate hidden stock as a policy condition.

Actual extraction revalidates the identified physical source after real effort.
Success atomically transfers the batch into the owner's escrow. Insufficient stock
creates a failed attempt without material; it still pays the prospectively declared
full attempt duration. Another person's take does not update the loser's memory.
Same-tick contention uses the existing deterministic actor/order convention and
commitment ID, declared before outcomes. No stock is assigned at selection time.

At real completion, success converts actual owned escrow into finished food and
waste before release. Failure produces none. Release occurs exactly once after
the recorded physical outcome. No later meal/gift follows automatically. Existing
policies independently choose after release, including continued contact or Wait.

The exact same-tick ordering must be registered and reflected in code/reader. A
minimal successor can preserve old needs→resource events→ready claims→meals→old
protocol releases, then resolve raw extractions and preparation completions in
declared stable order, before any next ordinary selection. No decision or reply
may occur in the middle of an atomic physical operation. All compartments must
conserve before and after each step, not only at export.

## Clock, inquiry and end-of-run accounting

Keep the existing one-tick progression and global time. Need is charged once for
actual elapsed intervals, including processing and failures; disjoint unrelated
work continues concurrently. Atomic inquiries remain disallowed whenever any
incompatible live work or pending protocol exists, including preparation. Both
extraction and completion deadlines must be protected; a query cannot jump over
either or emit a receipt when admission is blocked.

Report checkpoints annotate original work IDs and physical phase/escrow as of that
tick. They neither terminate work nor restart phases or refresh memory. A checkpoint
at an endpoint is distinct from a strict interior crossing.

Prefer a total extraction+preparation duration within existing33 public reservation
and drain bound. New ordinary work stops at nominal T; admitted work completes
with actual charged time through T+33. Capture nominal unfinished extraction and
post-extraction escrow exactly. If evaluating/exporting mid-process, preserve raw
escrow as raw; no premature finished output or silent deletion. Any larger bound
would require explicit versioning and a separate horizon factor; it is unnecessary
for a bounded first study if all declared phases fit33.

## Environmental information and one-slot limit

The actor's logical environmental slot is an owned tagged record: ReadyCache or
RawSource. It includes source/owner identity, own receipt, observed tick, last-seen
stock, declared public access/durations/yield and own removal count. A newer
observation can overwrite either tag. Unknown is distinct from observed zero.
The physical source can deplete without updating other actors' retained records.
Own removal changes an existing matching record without refreshing its date;
failure can deliver new local evidence. Completion bookkeeping cannot reconstruct
an overwritten source observation.

If implementation needs an old-cache projection, it must be derived from the one
current tagged record. A raw record projects None to old ready acquisition. Never
retain an old ready cell alongside a raw cell as a hidden second usable slot. Any
split backing representation requires an enforced disjoint union and explicit
single-slot export, not two independent memory stores. New audit records are not
an input channel.

The pure Prepare candidate may use own food/hunger, retained dated stock upper
bound, public access, known effort/yield and public time/availability. It receives
neither actual hidden stock nor rival escrow/inventory, future partner, success,
scenario/mode/outcome label or observer history. Preserve old Baseline/MealRelief
scores and ties. Register any appended tie position and exact integer arithmetic
before runtime; do not fit coefficients to produce work.

## Compatibility controls and a design distinction

Distinguish **feature off** from **raw access closed**. Feature-off compatibility
must reproduce all old024inputs/bodies/checkpoints after only the declared schema
projection, with no new observation affecting the old slot. Raw mass can remain
in a separate inert physical compartment without affecting old actors.

The primary025comparison instead has the same raw source and observation schedule
in both worlds, with public access closed/enabled. A legitimate raw observation
can overwrite a ready-cache observation even in the closed-access control. Such a
control need not equal an old024world: cognition about its environment differs
legitimately. Do not claim old-world byte parity for that distinct comparison.
Register this distinction explicitly and verify the primary pair differs initially
only in declared access and its legitimate local projection. If the prospective
gate requires every closed-access observation to preserve old024behavior exactly,
the tagged overwrite requirement conflicts with that demand; report the conflict
rather than silently adding memory or suppressing observations in just one arm.

Old policies remain unchanged; changed local opportunity information is part of
the new physical environment. A ready-food placement control is a separate physical
counterfactual. Access effects must be compared within a fixed old valuation policy,
never conflated with switching Baseline to MealRelief.

## Traceability, risks and rejected scope

Capture actual local input/retained tag, five candidate scores/eligibility/ties,
selected action, admission, stable work identity and phase deadlines. Link actual
source debit→owned escrow→finished food/waste→release→later independently selected
meal/contact to primitive original references. Track per-operation conservation,
occupied intervals, unrelated concurrent work, missed actual opportunities and
later hunger/social/material history. Do not claim unique provenance of a food unit
after it enters a fungible inventory without a separate identity mechanism.

Risks: accumulated host generators/type adaptation, phase-resolution ordering,
old snapshot assumptions about interleaved material changes, availability versus
busy consistency, one-slot projection mistakes, output overflow and nominal-drain
confounding. Inspect all callers affected by generated extensions; assert reversal
and disabled parity before treatment. Independent readers must rebuild physics
without importing the new evaluator, and reject scratch stock/escrow/food/waste,
time, release, cognition and consent corruption.

### Read-only observer integration boundary

Current ownership is `tools/history-explorer/valuation_history.py` for024source
identity/envelope, `concurrent_history.py` for shared interval/actor primitives,
`explorer.py` for explicit sources/export/cache routing and `native-concurrent.js`
for captured actor versus observer views. A new025source must remain explicit;
do not relabel preparation records as024or021or use the viewer as a scientific reader.

The old observer food-point integrator knows only events/ready grants/meals and
would omit preparation output between completion and the next old food event.
025therefore needs a material-aware observer timeline, including raw/escrow/waste,
not only an extra colored lane. Likewise old causal chains join only Claim/Meal
outside resource invitations: add actual Prepare choice→work→extraction→conversion
→release references and later own-choice context. Preserve source/member IDs and
lossless original reconstruction. Do not assert unique provenance of fungible food.

Actor rendering must show only the captured tagged source receipt and local terms,
including unknown/stale knowledge. Actual source stock, rival escrow, later work
success and counterfactual classifications belong only to observer views. Keep
explicit schema/version routing, source freezes, member hashes, bounded caches,
read-only routes and poisoned-cache rejection; no live simulation/tuning controls.
Original025data and actual browser verification are still future work.

No generalized production framework, planner, queue, jobs, farms, tools, markets,
currency, property law, institutions, travel, extra actors, renewables or canon
change is needed. No025law/matrix or runtime is registered by this investigation.
