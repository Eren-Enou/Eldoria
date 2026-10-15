# Experiment016: Learning From Failed Contact

Preregistered2026-10-14 after Gate A review16bf660 and Gate B protected/pushed
`98aef63c92e156082cbbe4ea5cf46b1602279d51`. Foundation001–015 and all186 artifacts
remain unchanged. This specification precedes treatment implementation and runs.
No016 protection/default adoption; no017.

## Question and competing explanations

Can own legitimately observed invitation outcomes adapt later contact, reducing
some repeated failures while preserving independent consent, uncertain availability
and costly/mistaken avoidance? A existing cognition/current availability suffices;
B missing retained feedback causes waste; C bounded feedback preserves opportunity;
D a simpler short-lived record suffices; E changed circumstances make avoidance
harmful; F matching/Continue/reduced attempts/fixture artifacts explain apparent gains;
G decline,nonexecution and actual unsuccessful negotiation are different signals.
No hypothesis or overall utility winner is assumed.

## Reuse, candidates and selected minimal extension

Keep015 stationary three-person waves,current availability,independent proposals and
responses,expiry,declared arbitration,actual resolver time,resource/inquiry/consumption,
005 concerns,006 novelty,007 provenance,008 Grouped,009 FIFO,012 Contextual unchanged.
No010/011/013 alternatives. Existing episodes/trust/estimates do not encode invitation
failure: old015 failed waves only spend time. An observer log is not actor experience.

Considered: unchanged; cycling partners without remembering outcome (cannot identify
failed contact); a single recent outcome; a bounded retry ledger; permanent willingness
estimates (unjustified/private motives/longer lifetime). Select finite retry delays,
not a calibrated reliability estimate. A stored cross-wave deadline is cognition,
not stateless. No new query-content,trust or reputation representation.

Five modes:

- Unchanged: exact015 History initiation and original recipient response; no feedback
  cognition. A historical reference, not the learning comparison's recipient control.
- Control: unchanged History initiation; common prospective recipient control below;
  no feedback cognition. This isolates that control against Unchanged.
- Recent: Control plus one most recent failed outgoing contact; defer same partner/type
  for6 ticks. Any new outcome replaces the one record; Met clears it.
- Typed: Control plus at most2 insertion-FIFO own retry records keyed partner/type.
  Declined defers12 ticks; AcceptedUnmet/Expired/Unavailable defer2. Existing-key update
  stays in insertion position. Met removes that key. Full new-key insertion evicts first.
- Untyped: same2 cells/update as Typed, but every non-Met outcome defers12 ticks.
  This is the minimal outcome-specific ablation; it does not erase real old cognition.

At each decision expire entries with until<=now, without archive recovery. Only eligible
old candidates matching a live record become ineligible/score−1; other old scores,
reservation,enumeration and ties stay exact. Wait0 may end the period. No future-value
signal or retuning. Received failure at final wave tick establishes deadline now+delay;
actual elapsed time,including query/consumption,can expire it before next period.
Success revises/removes discouragement; nothing discourages permanently.

Pure replaceable initiation receives old own Input plus own at-most2 records/mode;
update receives only actor's signal and current own records. All actors independently
use the same treatment. Recipient control is identical across Control/Recent/Typed/
Untyped: original015 incoming valuations/ties, but Continue score0 (Wait0 precedes it).
Recipients thus choose positive incoming readiness or Wait independently of treatment
outgoing score. Provisional outgoing permission still exists; incoming acceptance
never cancels it. This is an explicit prospective response-control intervention, not
an invisible015 correction or claim of optimal consent. Never attribute Control versus
Unchanged differences to contact learning. Incoming sets may differ causally later.

## Actor-visible feedback and chronology

One signal per actual own outgoing invitation, delivered only after response/arbitration
and any actual meeting have finished. Contains own stableID,partner,contact kind,sent
and received ticks,and:

- Declined: own invitation did not receive consent. Does not reveal whether recipient
  preferred an unobserved invitation,Wait or Continue,or their scores/private motive.
- AcceptedUnmet: consent to own invitation but no compatible own meeting; no competing
  edge,private arbitration detail or reason is exposed.
- Met: own edge Execute or compatible Reciprocal. This is not food/evidence success.
- Expired: own known request deadline passed; no motive inferred.
- Unavailable: current public availability prevents execution; not permanent unwillingness.

Only own result and legitimate public response/availability are projected. Do not give
raw observer allocation,another actor's invitation,score,private food/knowledge,truth,
hidden provenance,future release/answer or scenario label to cognition. Original resolver
may audit precise dispositions; learning does not see them. Historical waves generally
avoid Unavailable/Expired through admission; unit boundary tests cover them without
manufacturing rejections. An unavailable visible partner can yield no invitation at all.

Decline/nonexecution creates no resource event/Refuse,trust,memory,concern resolution,
006 novelty or012 progress update. Real resource refusal and selected valid inquiry
retain their independent consequences. Met can accompany refusal,silence,redundancy
or misleading evidence. Both invitation and interaction stages are recorded separately.
Only outgoing proposer learns own result; incoming-only participant gains no invented
outgoing feedback. Dropped/expired entries cannot be rebuilt from audit.

## Fixtures and prospective matrix

Fixed same seeds0,1,42,u64-max; exact rerun every world. Modes match initial people,
food,traits,profiles,knowledge,concerns,capabilities and external rules until voluntary
divergence. No resetting thereafter. Core budget24,6 periods,long24,Ascending; shift7,
brief willing width10.14 families (setup from015,never edits its source):

1 decline: private-mirror; repeated invitation decline before inquiry.
2 late_willing: private-mirror; source2 becomes privately willing at period-relative
  shift,independently of mode,per period. No notification to proposer.
3 brief_willing: willing only in [shift,shift+10) per period; may be missed.
4 alternative: unilateral setup but person2 is possible donor (food4,hunger0,g100,c0).
  Both legitimate alternatives compete; no promised useful target.
5 unavailable: mutual;1 publicly busy through first period. No hidden release input.
6 contended:015 contended; accepted-edge losses may follow later evolution.
7 refusal:015 refusal; contact may succeed while negotiation fails.
8 information:015 information; legitimate inquiry may change assessment.
9 misleading:015 misleading; valid receipt may worsen belief.
10 redundant:015 formerly-useful; provider evidence loss and redundant query remain.
11 separate:015 separate; no forced contact or reunion.
12 mutual:015 mutual; simpler/unchanged controls may share all benefit.
13 longitudinal:formerly-useful setup with persistent24–32 periods;2 private willingness
  alternates by period (unwilling for period%3==0); current prior commitment of2 through
  shift on even periods. Knowledge loss already established legitimately,finite food,
  failed/redundant contact,hunger and all later learning/history persist without reset.
14 missed:horizon2; no admissible contact,not a remembered rejection.

Willingness interventions modify only source2's own communication profile; all agents'
food/traits/trust/history remain. Apply fixed period-relative rules at wave boundaries
before capture; a running old resolver is atomic,so an intervening change is applied
at next choice boundary and cannot leak a future answer. Record actual application
and source profile. Relative hunger+20clamp100 between periods,consumption3ticks,all
stocks conserved; no new food. Mode-specific elapsed-time changes are causal.

Held-out: all14 families/five modes under Descending and Rotating with all other core
parameters fixed. Additional four families(late_willing,brief_willing,unavailable,
longitudinal) under Ascending budget16/periods8/long32/shift3 and budget48/periods8/
long32/shift11. These coupled parameter sensitivities are not independent factor effects
or transfer.70core+180held=250 seed42 histories,1000 worlds across four replay seeds.
No final outcome inspection before freezing policy/fixture source definitions.

## Outcomes, falsification and validation

Record proposals,candidates,own retained retry state before/after expiration,legitimate
signal/update/eviction,recipient choices,matching order/dispositions,actual executed
spans,resource events,query receipts/Grouped status/006/012 learning,consumption,next
inputs and final state. Keep failed waves,time,per-actor contact distribution,repeat
intervals,missed alternatives,declines,accepted-unmet,unavailable,expiry,reciprocal,
meetings,meaningful versus redundant/misleading inquiry,trust/concerns and material
separate. Fewer attempts/more meetings/transfers are not utility or welfare.

Primary comparisons: Control versus Recent/Typed/Untyped at fixed scenario/order.
Typed versus Untyped isolates outcome-specific duration; Recent versus Typed couples
capacity,lifetime and classification,so cannot establish minimum-state necessity.
Unchanged versus Control exposes response intervention separately. Preserve no-effect,
short-lived/simple wins,missed contact,changed-information-only,costly avoidance,
order reversals and finite-food exhaustion. No predetermined positive chain required.

Reject interpretation if future/private/observer knowledge leaks,feedback precedes
response,recipient forced,old cognition altered,failed invitation trains query cells,
new learning reads evicted audit,mode changes external rules,unaccounted time/food,
or merely different scores are called executed gain. Independent read-only Python
reconstructs old cognition and policies,memory/update/expiry,own signal projection,
consent/matching/time/resource accounting. Corrupt feedback,eligibility and duration
must be rejected. Exact replay alone is not independent scientific interpretation.

Run fmt,locked all-target Clippy,full tests,shared001–015 gate and standalone009–015;
new evaluator/validator/independent reader,186 hashes plus raw-byte guard. Never write
frozen evidence in place. Separate wall-clock policy/update/simulation/serialization
and observer-storage cost; reuse015 capture without duplicating extra full agent state.
At most2 new tiny cells/actor (Recent1,controls0); audit grows with waves. No performance
refactor unless necessary. Verdict ACCEPT/ACCEPT WITH QUALIFICATION/NO JUSTIFICATION/
INCONCLUSIVE; scientific finding is not adoption or protection. Unsupported: calibrated
willingness,optimal frequency,welfare,human motives/psychology,friendship,institutions,
planning,travel,production,language,networks or civilization. No017.

## Development disclosure

Eight initial targeted tests passed. Initial all-target Clippy rejected a nested
if; it was collapsed to an equivalent let-chain before inspecting final outcomes.
The source hash freeze was refreshed for this syntax-only repair, with no numerical,
policy,fixture,capacity or criterion change. Exact archived replay must verify identity.

After the final1000-world run and first250-history independent reconstruction,
all-seed summaries exposed two narrow Recent consumption gains in seed1/decline
(Ascending and Rotating). Extract all five modes in both already-preregistered
configurations as ten supplementary full traces for independent checking. This is
post-evaluation selection for causal audit,not held-out evidence,new scenarios,policy
retuning or replacement of negative archives. Primary250 histories remain unchanged.
