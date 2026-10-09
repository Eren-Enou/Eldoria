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

## Experiment 004 extension (2026-10-07)

`foresight.rs` adds a pure replaceable predictor and optional Foresight ECS resource.
`enable_foresight` validates sensor parameters, enables intentional mode, and stores
per-agent settings and directed partner expectations. It does not reset an existing
foresight resource. `set_prediction_settings` changes future priors/switches without
erasing learned expectations. `set_predictor` supplies an alternate pure function
of TalkInput and Context; the resolver validates unchanged local input/context and
consistent candidate audit entries before applying its choice. In this mode the
predictor replaces the plain TalkPolicy decision path. Other modes are unchanged.

The context contains only own expectations/settings and publicly known evidence
quality/effort, excluding sensor channel, seed, outcome and all partner-private
state. Forecast records link to TalkRecord IDs and separate immediate/anticipated/
combined utility. PredictionError links a selected forecast to the next public
response. Only expected AskEvidence or ProvideEvidence events are learned through
a quarter-error update. Claims too late for a challenge have no feedback sample;
forced closure is censored. No private belief revision is used as a training label.

EvidenceKind::Fallible represents received evidence, not truth. Seeded readings are
keyed by refusal, actor and source. Only after choosing ProvideEvidence does the
resolver sample/release readings. Accurate/Inverted fixture channels isolate
weight/direction in tests; neither is in policy context. Direct readings supersede
unsupported testimony and combine as strongest positive minus strongest negative
weight. Repetition cannot inflate support; a received source is immutable. The
new kind opts into this rule even through the low-level receipt API. Legacy
Disclosure and earlier modes keep their existing rules and serialized schemas.

A disclosure can carry two source readings. Both receipts are audited individually;
the communication record links its final receipt, with all source links in Reading
records. Tick cost is receipt count plus configured effort; other turns cost one
tick. Food stays unchanged. Credibility uses the final combined support, so an
ambiguous pair causes no consistency update. For fallible receipts the reused
verified_claim field denotes evidence consistency, not objective verification;
verifiable stays false. Correctness annotations exist only in observer Reading
records. Original interpretations and saturation-correct trust replay are retained.

New directed expectation maps and audit/error/reading vectors grow without bound.
Existing bounded episodes/beliefs are unchanged. Forecast/evidence searches scan
audit history, and long-run aggregate work can be quadratic. No new persistence,
parallelism or general planning framework was introduced. See
[004 specification](../experiments/004/specification.md) and
[findings](../experiments/004/results.md) for equations, counterfactuals, measurement
boundaries and the finite-horizon loophole revealed by actual experiments.

## Experiment 005 extension (2026-10-07)

`concerns.rs` adds a separate optional `Concerns` ECS resource. `enable_concerns`
requires foresight, is idempotent, and starts its scene cursor at the current
boundary. `set_follow_cost` and `set_concern_policy` change future decisions without
erasing history. The pure creation predicate and follow policy are isolated from
the resolver; the latter is replaceable by a function pointer. No partner component,
truth, sensor seed/channel or sampled reading enters `FollowInput`.

Each concern has stable ID, owner, target, original refusal event/scene, one question
type, saved importance, cached subjective uncertainty, age, attempt/failure counters,
status and last receipt reference. There is no duplicate dialogue or event payload.
Creation follows an actual refusal conversation when importance (current relationship
goal plus hunger/5, capped at 100) is at least 40 and absolute local support is below
60. Unimportant uncertainty is audited but not retained. A concern is not a belief:
it separately records that uncertainty is worth potentially acting on later.

The resource keeps eight entries per owner including terminal entries. At capacity,
evict the oldest terminal entry first, otherwise the least important active entry
(oldest ID breaks ties) with explicit capacity abandonment and eviction transitions.
Uncertainty survives eviction of the corresponding belief/episode. Original event
and receipt archives remain available to the resolver but are not policy inputs.

After resource scenes finish, `run` processes new encounters in stable scene and
participant order, then runs the usual new-refusal conversations. Each owner ages
active concerns once per subsequent encounter and makes one follow decision. Only
concerns about this partner and an earlier scene are legal candidates. Continue
wins ties; concern ID order breaks other ties. Scores are specified in the
[005 hypotheses](../experiments/005/specification.md). Age and failures are utility
penalties, not physical time charges. Counters saturate; utility uses age/failures
capped at 1,000. A chosen request costs 1 + configured pursuit cost ticks; Continue,
Abandon and rejected invalid decisions cost one tick. No food moves.

A real selected Reopen is an evidence request audited in `FollowRecord`, which
links to a response conversation about the earlier event. The shared communication
resolver starts from that request, allows a voluntary evidence-or-silence reply,
then closes within two turns. Existing legal checks, sensor costs, belief support,
memory revision, trust replay and communication records are reused. The request
does not pretend that an earlier Explain action occurred. A retained prior public
claim may be compared to later evidence for credibility; evidence consistency is
fallible. Old modes' four-turn conversations and serialized schemas are unchanged.

Every received receipt refreshes a matching concern from the listener's belief:
absolute support at least 60 means Resolved, otherwise Partial. Resolved concerns
can become Partial after conflicting evidence. Abandoned concerns update their
uncertainty but stay abandoned. Open means no received information. Sequential
conflicting receipts may temporarily resolve and then reopen a concern, with both
transitions recorded. Evicted episodes cannot be reinterpreted by the unchanged
memory mechanism, although their concerns can still resolve.

After follow-up, attempts increment and a failure increments when uncertainty did
not decrease. The public response separately updates the owner's existing answer
expectation through the quarter-error rule if learning is enabled. An answer may
be unhelpful: higher response probability does not imply better information. These
updates and errors live in FollowRecord rather than using invalid TalkRecord IDs
in Experiment 004's prediction-error vector. Food, request time, response time,
receipt IDs and all state transitions can be joined to reconstruct the chain.

Concern scans are bounded (eight; legality checks make the tiny policy scan
quadratic in that fixed cap). Per-partner credibility/expectations/trust, original
events and audit vectors remain unbounded. Each decision and transition copies
bounded local state into an unbounded trace. Existing evidence and revision scans
can dominate long histories. This implementation adds no general planner, meeting
scheduler, persistent storage service, UI or global reputation.
## Experiment 006: expected usefulness of an inquiry

`inquiry.rs` isolates the pure inquiry policy, novelty rules and bounded local
storage. `simulation/adaptive.rs` integrates them behind `enable_inquiry()`; the
existing Concerns prerequisite and old 005 follow-up branch remain intact. No
Agent component, old report schema or dependency changed. New experiment harnesses
live in `experiment006.rs` and `examples/evaluate006.rs`.

New local representations: Cell(Concern ID, Source ID, Strategy, attempts, expected
usefulness, previous record, attempted offer), Signature(event, claim/evidence
provenance, received proposition, quality, receipt), Episode, public Offer and
bounded Contacts. Concern importance/status are never substituted by these values.
Direct and Evidence are structured inquiry actions; Pause leaves concerns untouched.

| Local collection / individual | Capacity |
| --- | ---: |
| Concern/source/strategy estimates | 32 |
| Content/provenance signatures | 32 |
| Inquiry episodes | 16 |
| Locally met contacts | 16 |
| Observed public offers | 32 |
| Source's own configured evidence capabilities | 32 |

FIFO capacity eviction is audited; cell refresh moves it to the back. Episodes
reference stable inquiry/receipt/concern IDs. New bounded beliefs/concerns remain
under their earlier caps. Archive records, notices, meetings and interventions are
unbounded. Forgetting evicted signatures can make old content appear novel again;
the evidence ledger still refuses increased belief support from identical evidence.

The policy receives own concerns/cells/signatures, own hunger and relationship
goal, local directed credibility, and currently co-present sources with public
evidence offers. No source private motives/resources/intent, sensor seed or sampling
channel/outcome appears. Public provenance channel IDs 0/1 distinguish readings;
they are not the private `Channel` noise/fixture configuration. The resolver samples
only after selection. Configured capability changes are observer-visible
interventions; voluntary advertisement (goal - privacy - hunger/5 - evidence effort
> 0) exposes only potential quality, effort and provenance, never a reading.

All strategies are considered per active earlier concern and available source.
Prior useful value is Direct=55, Evidence=65. Valid selected replies update their
cell by `(old expectation + realized usefulness)/2`, integer truncation. An
untried source is an exploratory prior, not knowledge that it can answer.

Novelty compares only locally received signatures: exact same provenance/content
with equal or lower quality is Redundant/0; zero quality/no answer is None/0;
an opposite proposition is Conflict/min(quality,30); stronger same-proposition
evidence is Stronger/min(quality - old maximum + 20,90). First information is
New/min(quality,40); a new weak origin for an already known proposition is
New/min(quality,20). Within a multi-reading reply, every receipt gets its own
classification; realized value is the maximum, never their sum. These categories
are heuristics about useful observations, not objective truth or entropy.

An Evidence candidate can benefit from an observed potential new reading:
`opportunity = clamp(offer quality - strongest local quality + 30,0,90)`. It gets
zero if that provenance/quality was already observed or the same offer was already
attempted. Expected gain is `clamp(max(learned/prior, opportunity) + credibility/10,
0,100)`; benefit is `importance * gain / 100`. Score subtracts inquiry effort
(Direct 10, Evidence 28), own hunger/10 and advertised evidence effort. There is
no separate bonus for importance that forces pointless questioning. Pause wins
ties at zero. Concern order, source ID order and Direct before Evidence break
remaining ties. Current relationship goal is recorded; saved concern importance
carries its original contribution. The 005 age/failure utility penalties do not
enter this opt-in policy.

Resource encounters keep old aging/scheduling, replacing only the chosen follow-up
policy. Explicit 2–8 participant meetings record co-presence (one tick), age active
questions, and choose at most one inquiry per participant in sorted order. Agents
do not choose travel or discover people from a global map. Each decision costs
one tick; a query additionally spends its strategy effort. Hunger/10 is a utility
cost, not physical time. A selected query receives one voluntary reply through
the existing local TalkInput/Predictor legality protocol. It may be silence.
Replies cost one tick, plus actual evidence effort and receipt processing ticks.
Announcement decisions cost one tick. Food is unchanged; consumption remains
explicit. Tests sum all old/new audit time components.

Only the original refuser can disclose their narrowly observed historical predicate;
a newly met person with no event knowledge honestly has only Silence available.
Inquiry responses have their own audit/episode stream, not synthetic references
to legacy TalkRecords. Existing predictor scores and earlier talk memories are
used, but inquiry does not train the old binary answer predictor or add source
self-talk episodes to the legacy stream. Inquiry learning records useful value
instead. Original refusal events are immutable; cognitive receipts retain the
existing belief revision and replacement-contribution trust replay. Credibility
updates only from a new legitimately observed claim/evidence comparison; repeated
proof cannot repeatedly award credit. Evidence provenance mismatches fail the
entire bounded response batch before any receipt is applied. Invalid policies
produce audited no-effect closure; run again does not replay opportunities.

This mechanism deliberately has no multi-step planner, global experts, information
network or language model. Future source knowledge acquisition/provenance needs
its own experiment; the present third-source case tests exploration, not successful
relay or independent corroboration.
## Experiment 007: bounded provenance and corroboration

The opt-in `provenance.rs` resource/pure policies and isolated
`simulation/provenance.rs` resolver extend Inquiry. There are no new dependencies,
Agent fields, default behaviors or old serialized fields. Experiment 007 snapshots
wrap 006 snapshots and the new resource. The ten constitutional requirements remain:
recorded causality; local information; retained consequences; three distinct audit
layers; unchanged food; deterministic sorted IDs/integer scores; voluntary decisions;
stable history references; replaceable policies; measured bounded local complexity.

Objective Root: immutable ID, event, actual inspector, sampled proposition, quality,
tick and private sensor configuration. Receipt: actual communicator/listener, root,
parent receipt, depth, attribution channel, delivered Knowledge, local Evaluation,
optional native cognition link, food/time and legitimate credibility comparisons.
These objective fields are not included in inquiry or listener inputs. Flattened
root links avoid recursive graph traversal; at most three transmission hops are
legal (two intermediate relays).

Local Knowledge: receipt, event, communicator, public message handle, proposition,
quality and optional disclosed observation token. An unknown lineage is an explicit
assumption keyed by speaker, never a hidden root lookup. Repeating the same provider
acquisition preserves the public handle; later attribution replaces only matching
event/communicator/handle assumptions. Original receipts/interpretations never change.
Hints are voluntary metadata disclosures with quality and optional known token;
they contain no claim. Comparison keys remember prior source/event/proposition and
known evidence token, preventing repeated credibility credit while retained.
Knowledge, hints and comparison keys each have FIFO capacity 32 per individual,
with stable eviction records. Earlier bounds, including 16 inquiry episodes, remain.

Inspection is an explicit retrospective encounter, not an inserted witness memory.
The actual refuser first chooses whether to open the instrumented scarcity predicate
from its own recorded historical circumstances. Up to seven distinct inspectors
choose whether to inspect; decisions receive only own Agent/Profile, public event,
quality and effort. Opening/inspection score is goal - privacy - hunger/10 - effort.
Each inspector/event slot is immutable, including after local eviction. Sampling
occurs in the resolver after selection, using the existing source-keyed deterministic
sensor. One opening decision and each inspection decision cost one tick; selected
inspection pays actual sensor effort. Repeated inspection cannot resample a slot.

A provider's pure ExchangeInput contains own Agent/Profile and one own bounded
acquisition, plus partner/event. Share score is goal - privacy - hunger/10 -
request_cost/4; Share requires an acquisition, otherwise Silence. Resolution
validates unchanged input, depth, immutable content, event/root/parent identity and
attribution availability. Unavailable attribution sends no root/depth/parent to the
listener. An available channel discloses only the token the provider actually knows;
a provider lacking it cannot disclose the true root by consulting the archive.
Metadata offers use the same voluntary selection. Exchange/offer costs one tick;
native belief processing costs one extra tick for the original harmed participant.
Inspection windows, exchange records and query records contain nested time spans;
sum outer operations rather than counting their embedded receipts twice. Food is
recorded before/after and never moves during these actions. Each operation closes
after one decision/response; resource-scene bounds are unchanged.

Local interpretation groups known observation tokens, otherwise remembered speakers.
Within a group retain maximum positive and negative quality. Across ordered groups,
accumulate each side with `total += (100-total)*quality/100`, then support is positive
minus negative. Forgetting is applied before computing final support. This is bounded
heuristic confidence, not Bayesian probability or proof of statistical independence.
First information value min(quality,40); corroboration min(quality,30); independent
conflict min(quality,30); shared lineage min(abs(support change),10), usually zero;
revelation min(abs(support change),40). A new speaker can have zero independence/value.
Transmitted quality is `min(acquired_quality, acquired_quality*(100+credibility)/100)`
with credibility clamped [-40,40]. Credibility can discount a new observation but
cannot transform shared lineage into independent evidence or improve the root.

Strong explicitly attributed evidence (quality >=60) can compare prior locally
received claims from different known tokens. Consistency gives +20*quality/100,
conflict -30*quality/100, directed credibility clamped [-40,40]. Only retained
comparison keys suppress repeated credit; bounded forgetting can lose that guard.
The resolver never compares testimony to objective truth.

Inquiry reuses 006 legal candidates, importance, costs, learned cells, offers and
tie ordering. A known offered/received token already in local knowledge multiplies
expected gain by zero; otherwise the factor is 100%. No hint means an exploratory
prior, not certainty that a source knows something. Each selected valid response
trains `(old+realized value)/2`; Pause preserves concern. The original refuser's
native 006 reply path is retained when it is not relaying a new acquisition. Native
proof signatures/attempted offers are still bounded and learned as before. New
structured acquisitions use the provenance evaluator. They are prospective; no
objective origins are retroactively assigned to native unsupported testimony.
Mixed legacy/provenance confidence aggregation is not established by these trials.

Locally computed support passes through the established native receipt/revision
pipeline for the original harmed participant. The native receipt's speaker is the
original refusal subject for historical trust replay; the provenance receipt retains
the actual communicator. This explicit bridge preserves old schemas and correctly
attributes the memory/trust consequence to the refusal subject. Other listeners
retain acquisitions without receiving someone else's memory or concern. Native
beliefs remain bounded at 16; evicted memories are not recreated. A false confidence
closure can reopen when disclosure lowers support below 60. Trust replays ordered
replacement contributions with original saturation semantics.

Queries append native Inquiry records/episodes with real stable IDs; Cells.last_record
always references that namespace. Provenance Query separately links its decision,
factors and structured receipt to the Inquiry record. Native responses remain linked
directly when used. The foundation pass now stores this extension by reference;
legacy reports materialize the old shape. Observer snapshots do not become
agent-accessible state. See [history and audit ownership](history-audit.md).

Local decisions scan at most eight concerns, seven available partners, 32 acquisitions,
32 hints and 32 cells. Knowledge grouping is O(32 log32), comparisons O(32*32).
Parent/root lookup is O(1), with no chain walk. The foundation pass indexes inspection
slots and received evidence, and caches clamped relationship prefixes. Revision can
still traverse the affected relationship suffix. Objective roots, windows, receipts, exchanges, queries, evictions and
existing audits remain unbounded; relationship maps retain the earlier sparse,
unbounded design. No persistence service or large-population optimization is added.

## Experiment 008: coherent local assessment across channels

The opt-in `Assessment` and `Assessor` function resources unify the received local
evidence basis, preserving the acquisition, inquiry, time, memory, trust and history
infrastructure. Enable after provenance while idle; method is fixed for a run and
same-method enable is idempotent. Enable imports no prior or forgotten evidence.

Each owner retains at most 32 delivered Items across events: explicit receipt namespace,
event, communicator, optional public message handle, local Origin, content and received
quality. Origins distinguish unsupported self-claim, historical disclosure, native
reading (speaker/event-local channel), disclosed observation token and unknown-speaker
assumption. Actual hidden roots, private sensors and truth are excluded. Received
quality is frozen; subsequent credibility changes do not rescore retained receipts.

Native and provenance cognitive bridges submit a local Item before updating belief.
Group event evidence by local origin, using sign-wise maxima within groups. Unsupported
claims are a fallback when no positive-quality evidence remains; a zero-quality item
does not erase that fallback. Max takes strongest positive minus strongest negative
across groups. Grouped uses 007 residual combination in stable origin order on each
sign before subtracting. Independent corroboration distinguishes these two small
rules; no Bayesian model or general belief network is introduced.

FIFO eviction occurs before assessment. Repeats occupy attention capacity but add
no evidential weight below capacity. Attribution updates only retained Unknown items
with matching event/communicator/message; incoming audit records remain immutable.
Observer indexes validate source immutability but their historical evidence maxima
never supply 008 support. Forgetting remains consequential.

`native_acquired_exchange` is an 008-only adapter over the same validated voluntary
exchange. It requires a retained source acquisition and original harmed recipient,
samples nothing, preserves public known/unknown origin, parent, decision, quality
and time, and uses a native cognitive receipt namespace. Both adapters keep objective
transmission records. There is no caller-provided origin override or hidden mapping
from anonymous native readings to observation tokens. Distinct acquired origins stay
distinct unless a legitimately delivered public relationship says otherwise.

Unified support enters the unchanged bounded belief/memory/concern and saturated
trust-revision pipeline. The provenance bridge agrees with applied cognitive support.
007 novelty/inquiry-learning rules remain separate to isolate assessment; coherent
cross-channel usefulness calibration is not claimed. Other parties retain acquisitions
without receiving someone else's injury memory. Matched-present probes record local
policy choices without executing transfers.

Records preserve immutable incoming Items, prior belief, basis-before, support,
attribution changes, eviction and retained receipt references. An 008 snapshot wraps
the existing compact export plus Assessment without extending older export schemas.
Observer validation replays the local stream, checks ownership/time/consequences and
validates native acquired origins against delivered public payloads, never hidden
objective roots. It cannot restore cognition.

The new basis adds at most 32 Items per active owner; all earlier caps remain unchanged.
It duplicates some data because assessment combines channels while provider acquisitions
must still support sharing. Assessment records and their bounded reference lists add
unbounded audit storage. Grouping is O(32 log 32), retention/attribution scans O(32),
without runtime observer-history searches. Existing snapshot validation, forecasts
and history costs are unchanged; there is no general performance refactor.

## Experiment 010: local survival-to-action projection

`attention.rs` is an opt-in extension after Grouped assessment. Existing006 already
enumerates competing concerns/sources/methods, so no new planner or target selector
is needed. Unchanged captures a control; CurrentNeed projects at most eight active
own concerns' support from at most32 currently retained008 items. The replaceable
pure function discounts each existing007 candidate benefit by
`(100-abs(support))/100`, leaves priors, costs and independence untouched, then
reselects with existing stable ties and Pause0. This heuristic is new010 behavior,
not a retroactive change to006 or009. It never changes retention weights or status.

The input contains only the existing actor-local decision and support triples.
Historical assessment records provide an observer-only prefix cursor; they do not
compute runtime projections. No future partner, private source holdings, actual
roots, sensor outcomes or objective truth enter selection. The control runs the old
policy unchanged. Without explicit enable, all earlier schemas/defaults are unchanged.

Actual native/provenance reply, inquiry learning, concern transition, revision and
trust replay follow the established resolver. Fixed resource encounters execute
existing legality/accounting and may provide later inquiry; subsequent consumption
retains consequences. Nothing resets divergent worlds. Assessment eviction leaves
separately persistent consequences intact under the mode-specific contract.

Attention adds mode, prospective query cursor, function pointer and growing observer
records referencing existing query/inquiry IDs and assessment boundaries, with <=8
support triples per record. No new persistent cognitive collection is added. Full
010 snapshots wrap009, and observer validation checks available prefixes and fixed
built-in scores. Alternative replacement functions need their own score replay
validator; the experiments use only the registered built-in function.
## Experiment 011 inquiry value

`inquiry_value::policy` is an explicitly opt-in transient scorer, separate from the
protected010 projection. `enable_inquiry_value` requires Grouped assessment, is
prospective/fixed for a run. Combining010 and011 projections is rejected in either
enablement order; the additional guard applies only when the new011 resource exists.
The011 comparison path calls existing provenance policy unchanged, exact010
CurrentNeed, or the new pure StatusValue function. Its replacement setter affects
only StatusValue. No older default, resolution, retention, resource or capacity rule
changes. This research result does not justify adopting the prototype as a default.

The scorer receives only the already local decision plus at most8 active own support
triples from live32-item assessment. It uses importance, existing expected_gain,
existing cost and current distance from the existing60 threshold; no future state
is simulated. See the frozen [formula and boundary](../experiments/011/specification.md).
No persistent cognitive collection is added. `ValueAudit` is observer configuration
and references using the existing010 `Record` schema; the runtime never reads those
records to recover knowledge. Capture is conditional on the011 resource.

The011 Snapshot independently validates retained prefixes/timestamps and built-in
score selection, with the009 base validator. Checkpoints cover sequential inquiry,
actual executed scenes and subsequent consumption. The [results](../experiments/011/results.md)
keep unchanged/current-need/StatusValue, query effects and E_Q/E_A/F separate. A positive
novelty estimate need not change Grouped support: positive-quality retained evidence
can suppress claim fallback. This mismatch causes preserved allocation mistakes.
