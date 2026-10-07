# Experiment 001 architecture proposal

Written before implementation, 2026-10-06.

One library plus a CLI in one crate keeps the experimental model inspectable. Use
Bevy ECS without rendering: agents are entities with one cohesive component;
scenes, stable-ID index, clock, policy, and objective history are resources. A
single-threaded schedule advances each active scene one action per tick in stable
scene order. An agent may join only one active scene. Stable domain identifiers
are serialized instead of Bevy entity handles.

Pipeline: construct a local observation -> enumerate legal actions -> score using
only the actor's state and observation -> select deterministically -> validate and
resolve -> record objective event -> interpret independently for each participant
-> update bounded memories and relationships. The policy is a replaceable function
behind a resource; it never receives the World or the other agent's component.

The first domain contains food requests, offers, acceptance, refusal, and leaving.
An offer reserves nothing; acceptance validates the current donor balance and
transfers atomically. No food is created or consumed during scenes. Experimental
resource resets are explicit interventions in reports, outside scene accounting.
There is no trading, theft, debt, locomotion, metabolism, or civilization model.

Agents have food, hunger, generosity, caution, expectations of others, sparse
pairwise trust, and bounded structured episodic memory. Their own state is known;
the partner's inventory, traits, hunger, and memories are private. Offers and
requests are public signals, not proof of private state. Aggregate trust survives
episodic eviction; therefore bounded episodes do not erase every consequence.
Relationships are sparse but not globally bounded in this first experiment.

Scores use integers and explicit tie ordering. Generation uses a documented small
deterministic PRNG; decisions do not consume randomness. Fixed ticks, sorted IDs,
ordered maps, and serial resolution avoid dependence on wall clock or ECS query
iteration. Cross-version replay requires the lockfile and rules version. No claim
of determinism across future rule changes is made.

Every decision retains all candidate scores, relevant local inputs, and memory
event references. Every event records action, balances before/after, result,
participants, scene and tick. Subjective memories retain event IDs, observed
action, interpretation, and valence. These records describe implemented causes;
they do not establish psychological explanations.

Scenes have a finite turn budget and explicit agreement/refusal/withdrawal/timeout
outcomes. Requests permit a response; offers permit acceptance/refusal. No fixed
story is encoded. The evaluator and interpretation rules are intentionally small,
testable hypotheses, not a complete personality model.

Population experiments reuse the same mechanism in disjoint pairs. This sacrifices
parallelism for a clear reproducibility baseline. Benchmarks measure initialization
separately from scene throughput. Full audit history is retained for experiments;
production persistence/streaming and relationship pruning remain future work.

Controlled altered-history experiments generate both agents, produce actual past
interactions under explicitly intervened traits, then restore identical present
traits/resources/hunger while retaining learned trust and episodes. A history-free
counterfactual removes learned state at the same present. This isolates whether
implemented history changes choices without pretending interventions are natural
world events. Experimental fixtures do not bypass the interaction resolver.

## Implemented rules and review

Verified Bevy ECS 0.19.1 (Rust minimum 1.95) through `cargo info` and
[versioned Bevy API documentation](https://docs.rs/bevy_ecs/0.19.1/bevy_ecs/schedule/struct.SingleThreadedExecutor.html).
The installed compiler is Rust 1.99.0. Cargo.lock pins transitive versions; release
candidates were excluded. The single-threaded Schedule API compiled successfully
against the selected release.

Policy inputs: own food/hunger/dispositions, known partner ID, last public action,
amount and turn budget, own learned trust and last four partner episodes. Only
food eligibility uses inventory in this model. The interpreter also receives only
own state, public signal and observed transfer/failure; it cannot inspect another
actor's privileged decision trace. Full objective audit reports are an observer
facility, not agent knowledge.

Let H=hunger, G=generosity, C=caution, T=pairwise trust, E=recent episodic valence.
All are integers, divisions truncate toward zero. E sums the last four partner
episodes and clamps to [-40,40]; T clamps to [-100,100].

| Action | Utility |
| --- | --- |
| Accept | H + 20 + T/4 - C/5 |
| Offer | G - H/2 - C/2 + T + E |
| Request | H - 35 + T/4 - C/4 |
| Refuse | H/2 + C/2 - G/2 - T/2 |
| Leave | 0 |

Ties prefer Accept, Offer, Request, Refuse, Leave in that order, considering only
legal actions. Initial choices permit request/leave, plus offer if own balance is
enough. After a request: offer/refuse/leave; after an offer: accept/refuse/leave.
Accept is unavailable if own balance would overflow. Legality is checked again in
the resolver, including donor balance. Invalid replacement-policy actions terminate
with Inability and cause no transfer. No ECS mutation can happen inside a policy.

Interpretation: received help adds 24 + own expectation/10; giving help adds
8 - own hunger/10; receiving refusal adds -(8 + own expectation/4 + own hunger/10);
refusing is interpreted as protecting own needs with zero valence. Neutral public
signals and withdrawal have zero valence. Every event creates an episode for each
participant and updates trust by its valence. These deliberate simple rules create
perspective and learning; they are not calibrated psychology. Expectation here is
a minimal prior about assistance, trust is learned evaluation, and signals are
episodic knowledge. There is no inferred partner-inventory belief yet.

Historical causality is explicit: event -> interpretation/valence -> trust and
episode -> future candidate score -> selected action -> balances/outcome. The
same experience contributes to trust and recency, a deliberate aggregate/episodic
combination; the experiment does not separate their causal effects.

Review: an exclusive resolver and per-step agent clones make deterministic
validation straightforward but prevent parallel execution. Active-scene admission
scans existing scenes (quadratic setup over many disjoint pairs); populations of
1,000 are measured, larger claims are unsupported. Report construction clones
full snapshots and traces, so benchmark throughput excludes reporting and replay.
Memory episodes are bounded; relationship maps, scenes and audit records are not.
Constructor rejects preexisting episodes rather than accepting dangling event
references; save/load is a future boundary. Dispositions stay stable during scenes,
while trust and episodes change. Policy and interpreter live in one isolated module
and can be replaced independently of the scene/accounting/reporting machinery.

Experiment 002 should add explicit resource consumption and communicated scarcity
with uncertain partner beliefs. Compare identical refusal signals with and without
legitimate evidence of need; evaluate belief calibration and trust revision without
leaking private state. This tests a demonstrated missing capability before scaling.
