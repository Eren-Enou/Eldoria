# Four-person concurrency architecture (prospective)

Written after gates A–C at `8ac1548`, before any 022 runtime or outcomes.
The protected endpoint is 001–021. This is an opt-in research adapter, not a
replacement scheduler for historical experiments.

## Ownership and existing limits

`Simulation` owns one ECS world: authoritative `Agent` components, a single
`Runtime.tick`, stable scene/event vectors, an active-scene BTreeSet and busy
participant set. `advance` already advances every disjoint active scene once per
global tick, in scene-ID order. Resource legality, scarcity policy, transfer,
interpretation and bounded episode/trust updates remain those primitives.

`simulation/encounters.rs` admits only IDs 0–2. Its local input construction and
arbitration policy are otherwise separable from that fixture. `recurring::Clock`
and need records use three-element arrays. The 019/020/021 adapters hold a single
social commitment; their completion code scans a global event suffix. That suffix
must not be used for concurrent scene completion. Acquisition uses a vector of
claims, but its rank function assumes three people. Environmental knowledge has
one slot per person in three-element arrays. Atomic inquiry advances time inside
adaptive/provenance resolution and cannot be interleaved with pending deadlines.

020 and 021 duplicate physical ticking, need sync, claim completion, participant
release, meal and protocol phases. Isolation preserved frozen evidence; extracting
these now would risk changing old ordering. No such refactor is proposed.

## Alternatives and decision

A: extend the single-pair 021 adapter. Rejected: frozen source, fixed arrays and
unsafe global completion suffix. B: a bounded opt-in registry around the existing
multi-scene core. Selected. C: extract a shared commitment/time kernel. Deferred:
no need to route protected experiments through new scheduling semantics.

021 source-locks `lib.rs` and `simulation.rs`. Additive module declarations in
unfrozen `experiment.rs` and `simulation/adaptive.rs` provide a public research
namespace and private ECS adapter without changing those frozen parents. The
private wrapper must not expose a mutable Simulation during live commitments.
The namespace is `experiment::concurrent`; it does not adopt a global mode.

## Registry and clock contract

At most four physical commitments exist: Send, Reply, Resource, Claim or Meal.
Each has monotonic ID, actual participants, start, deadline/bound, source and
completion record. An actor belongs to at most one. A pending invitation is a
bounded nonoccupying protocol token, at most one per proposer, with expiry at
send start+2. It may become invalid while its proposer does independent work.

Each ordinary boundary advances exactly one global tick. Apply +5 hunger per
absolute four-tick pulse (cap100), then advance every active resource scene once,
then resolve due claims in (deadline, declared rank, ID) order, meals, and releases.
Only the completed commitment releases its participants. Resource completion
filters events by exact scene ID; concerns age only for actual participants.
No retroactive action occurs on release: the next decision boundary uses the new
state. All mutations retain their actual world tick and original source IDs.

Free recipients see incoming invitations at start+1, independently run the old
Continue-zero recipient policy, and spend one Reply tick. Busy recipients have no
invented response. At expiry, current availability, exact acceptance, reciprocity,
contention, social capacity and atomic compatibility determine admission.
All free ordinary choices at a boundary use the same pre-reservation public
availability. Pending proposers cannot send duplicate invitations; this is a
protocol legality mask, not a utility change. They may eat or acquire instead.
Same-boundary admitted social work is reserved before new environmental work.

Four-ID rank is ascending ID, descending 3-ID, or (ID+4-tick%4)%4. Concurrent
permits two social commitments; Serialized permits one. Neither ranks by truth,
future usefulness or global welfare. Consent does not guarantee agreement.

## Local cognition and atomic inquiry

Reuse 006/007 inquiry, Grouped008, FIFO009, Contextual012 and 021 LastSeen with
one dated owned slot per person. Public availability is observable; partner food,
traits, motives, private information and objective stock between glimpses are not.
Inputs contain an owner-local encounter count, not other people's scene count.
Observation occurs only when free at an absolute cadence boundary. Successful own
takes adjust the slot; depletion is new local evidence. Audit cannot restore it.

Atomic query stays exclusive with every physical commitment and pending protocol
deadline. It rechecks the currently selected question against the invited one;
changed question cancels rather than silently substituting consent. No interior
decisions or glimpses are invented. New resource-guarded four-person need-sync
hooks after existing adaptive/provenance time writes are no-ops in old worlds.
The old three-person Clock is never installed. Tests and all historical replays
must establish that these additive hooks leave old behavior unchanged.

## Evidence, safety and cost

Record pre-boundary state, local choices and scores, independent responses,
admission reasons, starts/releases, exact core events, claims, meals, owned slot
changes, need pulses and final cognition. Observer capture is never a policy
input. Conservation is initial owned food+cache = current owned food+cache+actual
consumption; no regeneration. Stable IDs are domain IDs, never ECS handles.

Tests must cover double booking, different and same-tick completion, release
isolation, pulse accounting including atomic jumps, current-stock grants, consent,
deterministic ordering, stale memory and exact replay. An independent reader must
derive execution from primitives and reject corruptions, not trust summary labels.
Four-person candidate evaluation is small (at most three partners). Audit storage
grows with time and is measured separately from bounded live cognition/registry.
Avoid a generic event queue and full audit-state duplication at every boundary.

If query time cannot be synchronized without changing protected semantics, stop
before runtime adoption and report that blocker. Do not weaken any source lock.
