# Experiment017: Endogenous Opportunities and Changing Social Availability

Preregistered after Gate A review `5904f9a9e7efffbb22a52ddb922488f09fac9adc`
and Gate B protected/pushed `e968c34b8492a7f6692da9b521ca0f032a07c4ea`.
This specification precedes treatment implementation and outcome evaluation.
001–016 and207 original evidence files remain unchanged. No default adoption,
017 protection or018. Scientific acceptance and mechanism adoption are separate.

## Question and competing hypotheses

Can individuals' actual activities, commitments and persistent circumstances change
other individuals' legitimate interaction opportunities without laboratory-assigned
busy/willingness changes? A old mechanisms suffice; B a selected encounter prevents
another current contact; C resources/relationships alter later activity/availability;
D occupation loses an opportunity independently of learned unwillingness; E simple
commitments explain effects without cognition; F old local history matters; G order,
starting assignments or artificial timing dominate; H more independent activity can
reduce cooperation, worsen someone's outcome or merely exhaust finite resources.
Do not assume benefit or that richer scheduling is warranted.

## Causal classification and authority

External: initial people, food, traits, legitimate prior episodes/evidence/concerns,
stationary public three-person co-presence, horizon, physical rules, seed and declared
ordering. Actor-chosen: invitation, response, old negotiation/inquiry, Wait and new
narrow opportunity to select existing explicit food consumption. Endogenous: actual
occupied pairs, elapsed time, consumed/transferred food, hunger reduction, received
information, real refusal/trust/concern changes and subsequent choices. Observer-only:
full history, hidden partner state, scenario names and future outcomes.

An availability change requires a reference to a real selected/accepted activity.
No post-setup profile changes, scripted busy intervals, food refills or hunger+20
interventions. Consumption reduces hunger by40 as established; there is no metabolism
or hunger increase with time. Finite depletion and eventual inactivity are valid.

## Existing boundaries and selected architecture

014 uses externally supplied windows.015/016 capture simultaneous private proposals,
two handshake phases, provisional independent consent and greedy matching, then run
one resolver atomically in a three-person room.016 feedback does not make broader
availability endogenous. Actual resource `advance` already resolves one event per
tick and maintains ECS active/busy indexes; old consume accounts one food/40 hunger
in one tick. Reuse these rules rather than a new resource system or planner.

Select a new isolated opt-in adapter: resource encounters may remain active across
subsequent logical ticks. Their two participants cannot initiate/respond elsewhere;
the third receives current public availability and can Wait or consume own food.
At completion they become available from the real resolver's termination, without
advance disclosure of result/duration. Reuse old015 input/prepare, pure initiation,
016 Continue0 response control, arbitration, old resource policy/legality/memory,
005 concerns,006/007 inquiry,008 Grouped,009 FIFO and012 Contextual. No016 retry learning.

Alternatives: unchanged waves alone remain a required control; four-person concurrent
pairs would require broader input/consent changes and are not necessary for the narrow
occupied-pair/free-person test; a general event scheduler or ECS rewrite is excessive;
new work/fatigue/travel would add unsupported systems. Three people deliberately
cannot execute disjoint meetings. Inquiry remains an atomic existing resolver in all
modes: no independent idle-person decisions during its internal ticks. This is a
resource-commitment experiment, not uniform asynchronous execution or a claim of
complete endogenous social scheduling. All recipients retain decision authority.

## Fixed modes and actor-local policy

Five modes, no global default:

- Serial: current-wave availability; old History initiation/Continue0 recipients;
  resource resolver runs to completion; no voluntary within-period consumption.
- Committed: same contact policy, stepped resource commitments, no voluntary consumption.
  This isolates whether commitments alone change material behavior with three people.
- SerialActive: Serial plus actor-selected consumption under the common rule below.
- Active: Committed plus that same consumption rule. Compare to SerialActive to
  isolate whether decisions during actual occupation change later opportunities.
- NoHistory: Active, but old NoHistory masks own trust/episodes only for outgoing
  resource ranking. Actual memories/trust, incoming valuation, inquiry and updates
  persist. Continue0 prevents an outgoing score change rescaling recipient Continue.

Pure replaceable selection receives only current old `encounters::Input` and mode.
Its old contact candidate set/rank/order/reservations remains visible for audit.
Activity candidates are Wait0, Consume (own hunger score, eligible only in the three
Active variants with own food>0,hunger>0 and one tick available), then the selected
old Invite at its old score. Strict greater-than ties preserve the earlier candidate:
Wait,Consume,Invite. This is a declared local heuristic combining existing scores,
not calibrated utility. No future usefulness, outcome, label, hidden inventory or
observer provenance enters it. Every actor uses the same rule. Busy actors have no
new decision; their existing selected resource actions continue voluntarily.

Only an actual selected Invite grants outgoing permission. Consume/Wait mask outgoing
permission to Wait even if the old contact preview ranked Invite. A person can consume
during send phase and independently accept an incoming request after consumption has
finished. Incoming old valuations and strict stable-ID ties remain; Continue0 is the
same unoptimized control in all five modes. No failed invitation fabricates old learning.

## Time, commitments and accounting

Global time is shared; active person time is not added as serial elapsed time.
At an idle cohort boundary prepare old concerns/signatures exactly as015, then capture
all independent activities before responses. Send occupies one tick; simultaneous
selected consumption completes at that tick using existing one-unit/40-hunger rules.
If invitations exist, response occupies another tick, with current recipient state,
then old arbitration at exact expiry. Failed invitations cost these two ticks.
Without invitation, selected consumption costs one tick; all-Wait/no active commitment
ends the horizon (no unknown future wake-up oracle).

Old admission bounds remain: Resource8 including handshake; Direct15; Evidence33.
Book a resource scene only with valid proposal,actual recipient Accept,current free
participants and enough horizon. The old resolver advances one event/tick, at most6.
Serial variants run it atomically. Stepped variants expose one free-person decision
per resource tick; selected consumption and the disjoint pair's actual resource event
share that tick. Consumption never overlaps that consumer's own meeting or response.
Use the same food/hunger record semantics, no clock rewind or hidden refund.

Actual resource termination releases both participants. No exact future release is
an input, no arbitrary post-completion busy extension, no cancellation/rollback or
separate consent-withdrawal protocol. Existing resource Leave remains legal. Inquiry
runs its existing12/13 or bounded31 ticks atomically after valid admission; the third
person has no interior opportunities in this minimal design. This limit is common
across modes and must be preserved in interpretation, not disguised as concurrency.

At most one resource commitment and three invitations exist. No two pairs can be
disjoint with three people. Current busy state derives from actual active scenes,
not a fixture deadline. Safe admission means commitments finish by each period end;
none crosses periods. Mandatory established consumption then charges three serial
ticks (including unsuccessful consumption), across every mode. Period boundaries
are external; people, time, stocks, knowledge, trust and all bounded cognition persist.

## Controlled families and matching

Use13 families: mutual,contended,competing_eater,history,information,misleading,
refusal,unilateral,separate,private_mirror,formerly_useful,longitudinal,missed.
Reuse015 initial histories/settings; longitudinal uses formerly_useful's legitimate
provider eviction and continues24 periods. No later willingness changes. missed has
horizon2. competing_eater begins from contended,with actor2 food2,hunger60,generosity100,
caution0. This permits meaningful own consumption while other people may meet; it
does not prescribe who meets or whether anyone benefits. Other circumstances unchanged.

These cover competing requests and starvation (contended), need/transfer/release
(mutual), resource history (history), acquired/resolved/misleading/redundant evidence
(information,misleading,formerly_useful), unproductive commitments (refusal), delayed
availability/missed mutual access (mutual/contended), unilateral interest, no contact,
hidden future recipient responses (private_mirror versus information's identical
focal input), finite exhaustion and long uninterrupted history. Multiple requirements
may share a family; do not count them as independent replications. Whether a desired
case actually realizes an effect is an outcome, not a fixture success assumption.

All modes share initial state, profiles, source knowledge, co-presence, horizon,
resources, seeds and external rules. Only scheduling/activity/history mode differs.
No resets after divergence. Partner/incoming ties remain ID-ordered. Arbitration
Ascending/Descending/Rotating uses old formula and idle cohort-wave ordinal (not
interior resource ticks). Declare later phase changes rather than claim fair matching.

## Primary parameters, held-out sensitivity and freezing

Core: budget24,6 periods,long24,Ascending. Four deterministic seeds0,1,42,u64-max.
Held-out: Descending and Rotating over all13 families; plus budget16/periods8/long32
and budget48/periods8/long32 for competing_eater,history,information,longitudinal.
65 core+170 sensitivity=235 histories/seed;940 worlds,each exact rerun. Full seed42
histories archived; all seeds summarized. Timing/horizon/period variants are coupled
parameter sensitivity, not external transfer. Seeds are replay controls, not independent
statistical samples. Freeze the three new policy/adapter/fixture sources before final
treatment outcome inspection. Disclose syntax/tooling repairs and any deviations.
Do not retune activity scores, old inquiry, retention, matching or scenarios for gains.

## Causal audit and independent verification

Capture current own inputs/activity/contact candidates, actual public availability,
send/response states, consent/allocations, admitted commitment reference, each active
resource tick, actual completion/release, selected consumption, old resource/query
events, Grouped evidence/concern learning, later inputs and persistent balances.
Store immutable observer point references for shared before/after states rather than
duplicate full states at each equal boundary. Actor policy never reads that table.

A read-only Python verifier independent of evaluator classifications reconstructs
pure selection/ties/local inputs, busy from actual scene intervals, consent/arbitration,
no double booking, exact shared time, resource events/consumption, query learning and
retained cognitive prefixes, matched starts, private response twins and replay evidence.
Reuse already independently checked old arithmetic/cognitive helpers, document that
dependency. Reject scratch corruptions of commitment,release time,public observation,
food balance and selected action. Test that changing observer audit storage cannot
restore information or influence policy. Exact replay alone is not independent science.

Identify full chains from initial conditions→selected activity→occupation→other local
observation→voluntary choice→real/missed interaction→knowledge/relationship→later
behavior→persistent material state where demonstrated. Separate externally configured,
actor-chosen,endogenously changed,altered-choice,executed/missed and persistent classes.
Do not give an unexecuted meeting a fabricated answer or global utility label.

## Falsification, cost, validation and non-goals

Reject autonomy interpretation if busy state is fixture-scripted, consent forced,
future release/result leaks, observer history restores cognition, initial treatment
mismatch explains an effect, resource/time accounting fails or only a probe differs.
If Committed only adds public busy observations and otherwise matches Serial, report
that null. If effects require consumption/timing/history/order, attribute them there.
Preserve costly eating, ignored inquiries, failed negotiations, exhaustion, mistaken
beliefs, contact inequality, no-effect cases and order reversals. No universal welfare
ordering from transfers, meetings, consumption or final food alone.

Measure bounded live environment/commitments separately from cognition and growing
observer points/decisions/old histories, serialization and overall simulation time.
No new cognitive cells are planned. Benchmark incremental stepping/policy/serialization
and old throughput separately from deterministic evidence. Avoid generalized audit
refactoring. Existing caps/defaults stay exact. Expected added cost is current free
actor evaluation per resource tick and extra audit rows, not a larger planner.

Run fmt,locked all-target Clippy/full tests,normal001–016 gate and separate009–016,
new evaluator/exact replay/read-only reconstruction/corruptions,relevant throughput
measurement,207 inventory/raw guards. Archive only017 outputs in its own directory.
Conclude ACCEPT/ACCEPT WITH QUALIFICATION/NO JUSTIFICATION/INCONCLUSIVE based on traces.
Unsupported: naturally emerging society,calibrated motivation,friendship,consciousness,
institutions,optimal cooperation,sustainable economy,large-population scalability or
general intelligence. No factions,reputation networks,gossip,travel,production,
language generation,health/fatigue/labor system,planner or018. No017 default/protection.

## Development disclosure before final evaluation

Initial compilation used a nonexistent snapshot wrapper; corrected to the established
Snapshot construction, without policy/fixture changes. Five initial targeted tests
passed. Added independent legality checks for replaceable invitation policies and
two control tests; all seven targeted tests and all-target Clippy pass. No final
comparative outcomes were inspected before this source freeze. No score,capacity,
retention,inquiry,fixture or success criterion was retuned.
