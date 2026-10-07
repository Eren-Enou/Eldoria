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

## Experiment 002 extension (2026-10-06)

The original Experiment 001 policy, CLI, report schema and evidence remain usable.
`scarcity_policy` is selected explicitly by Experiment 002 fixtures and adds an
own-inventory opportunity cost for giving the last food unit while hungry.
`cognition.rs` defines event-specific beliefs, evidence provenance, revisions,
one-turn information scenes and consumption records. `experiment002.rs` supplies
controlled fixtures and population evaluation; `examples/evaluate002.rs` saves
reproducible evidence separately from timings.

`Simulation::communicate` validates the participants and original refusal before
changing state. Testimony is never checked against objective truth. Disclosure
explicitly releases a narrow predicate from an instrumented historical circumstance
record. This is a deliberate observation capability, not access available to the
policy. `revise_belief` is a pure replaceable evidence rule. The listener's trust
at first information contact determines testimony reliability; repeated claims
cannot bootstrap credibility. Evidence of strictly greater absolute strength
replaces a prior belief. Support >=60 changes a retained rejection to possible
self-protection with zero valence; contrary evidence restores the original memory.

Cognition is a separate ECS resource keyed by stable individual IDs. Beliefs are
private per-individual state; the resolver accesses only the recipient's beliefs.
Policies still accept only an Agent and Observation, and experience revised
beliefs indirectly through their consequences for that individual's memories and
trust. Information records preserve before/after beliefs, provenance, memory
revisions and trust. Join decision memory event IDs to information records by
listener/event for the complete interpretation history. IDs are typed by their
record collection (resource event vs information scene); the shared tick orders
records across collections. Original objective Events are never modified.

Trust revision replays the individual's relationship ledger with latest revised
valences replacing original contributions, preserving clamp order. Applying a
simple refund would be wrong after saturation. Only retained episodes may change;
previous revisions continue affecting aggregate trust after memory eviction.
Beliefs and episodes are independently capped at 16. Full event, circumstance,
information and consumption histories, scene vectors and relationship maps can
grow. Replaying a ledger is linear in recorded history per revision; long-run
persistence, indexing and pruning require later measurement.

`Simulation::consume` is a between-scene phase with explicit balance/hunger/sink
records. One unit reduces positive hunger by 40; empty inventory leaves hunger
unchanged. Both information and consumption reject calls while resource scenes
are active. Valid information scenes always finish in one turn; invalid calls
return an error before modifying agents, clock or audit. These narrow APIs avoid
expanding the resource protocol while enabling controlled information experiments.
They do not yet schedule spontaneous communication or autonomous metabolism.

See [Experiment 002 findings](../experiments/002/results.md) for hypotheses,
controlled comparisons, sensor limitations, reproducibility and measured costs.

## Experiment 003 extension (2026-10-07)

`intentional.rs` defines a pure replaceable communication policy, local inputs,
legality, profiles, bounded communication memories, credibility, conversation and
turn records. An optional ECS resource is installed by `enable_intentional()`.
It initializes profiles from own caution/generosity/expectation, installs the
policy and marks the existing event cursor. `set_communication_profile` and
`set_communication_policy` operate only at idle boundaries. Learned state is
retained when profiles change; calling enable repeatedly does not reset it.

After the resource schedule finishes, `run` visits new refusals in event order.
Each conversation has at most four decisions, alternating original refuser and
listener. The first silence permits a question; listener silence closes. Requests
require a remaining response slot. Invalid policy output consumes one time slot,
records an invalid turn, terminates and changes no food, beliefs or memories.
Valid actions always preserve food; consuming remains an explicit separate phase.
The public protocol uses Explain for both truthful and deceptive claims. Only the
observer record and speaker's own memory carry Mislead intent. The listener gets
neither truth annotations nor another agent's component/profile/credibility.

Scarcity is the typed-by-context proposition `scarce(original_refusal_event)`;
the bool payload has no other meaning. Evidence kind distinguishes unsupported
testimony and authenticated disclosure. The refuser receives only their own
historical scarcity bit; the listener receives it only through the existing
narrow disclosure channel. Objective circumstances, claims, evidence, beliefs,
interpretations and decisions remain distinct. Joined IDs identify the full chain:
resource event -> conversation/turn -> information receipt -> memory revision ->
next local decision. IDs are collection-scoped stable integers, not ECS handles.

The old public `communicate` path keeps Experiment 002's trust-based testimony
weight. A private shared receipt implementation accepts the intentional mode's
credibility weight while reusing belief revision, immutable audit and saturation-
correct trust replay. Credibility only updates when evidence compares a prior
claim in the same conversation. Trust measures experience valence; credibility
measures local observed claim consistency. No global reputation exists.

Policy equations and pre-implementation tradeoffs are in
[003 specification](../experiments/003/specification.md). Tie order follows legal
candidate order: Silence, ProvideEvidence, Explain, Mislead for speakers, and
Silence before the single available request for listeners. Every score input is
recorded. The profile domains are 0..100. Four turns is a deliberate bound, not a
claim that unanswered uncertainty is resolved. Closing silence can be forced by
that budget and must not be counted as voluntary concealment.

Communication episodes cap at 16 per individual; credibility maps are sparse but
unbounded per directed partner, as are trust maps. Full audits/scenes and decision
snapshots grow. Existing ledger scans/revision replay can be quadratic over long
histories. Experiment 003 measures short 100/1000-agent runs and serialized trace
size, not heap allocations or long-term scale. The architecture intentionally
keeps the original resource negotiation and post-refusal communication policies
separate; unifying their action space is not required for these controlled tests.
