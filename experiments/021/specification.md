# Experiment021 — Local Environmental Knowledge and Opportunity Continuity

Prospective specification at `d54df3840b5c6e4942c1a80fd434417126e9241e`,before
treatment implementation or outcome inspection.020 review8e3b574,protection6cdc87e,
main Explorer8e22d0a and phase assessment d54df38 are validated,committed,pushed.
Foundation001–020 is protected. No022,default adoption,automatic021 protection,
production,travel,institutions,language generation or general planner.

## Question and competing explanations

Can bounded,legitimately acquired and potentially outdated environmental information
change voluntary activities and later physical/social histories when other actors
change stock without updating one's knowledge?020 establishes costly acquisition
but supplies fresh stock at every free decision. Known-empty exclusion there is
structural. The [phase assessment](../../docs/development/world-of-individuals-phase-2-assessment.md)
identifies this missing distinction alongside larger supply/scheduling limitations.

H0 retained information is behaviorally inert. H1 last-seen information can enable
legitimate acquisition between observations. H2 stale positive information can waste
time,lose competition or displace help. H3 expiry/absence can miss real opportunities.
H4 fresh-only/current sensing is a sufficient or better simple alternative. H5 effects
reconverge materially while history persists. H6 additional state/audit complexity
has no justified benefit. None presumes memory superiority or intelligent exploration.
Preserve gains,losses,mistakes,nulls and ambiguous mediation.

## Alternatives and selected observation architecture

**A: supplied passive observations at legitimate opportunities.** A stationary cache
can be sensed by a free co-present actor at declared absolute times. This isolates
information continuity without adding a sensing action or valuation formula.
**B: voluntary costly observation action.** This would require an additional action,
admission/effort/resolution and a decision heuristic for unknown versus stale sources.
It is a valuable later question but confounds the smallest present memory test.

Choose A. Shared primary glance slots are epoch+0,8,16,…;held cadence4/16.
Only an actor free at an actual activity-decision boundary on a slot receives the
measurement. Busy actors and atomic-query interiors receive no invented observations.
Slots are fixed independently of mode,seed,needs,results or retained information.
Post-choice availability differences may cause genuinely missed later observations.
No actor receives the future slot schedule as a decision input.

This is controlled intermittent sensing,not a model of general vision or active
attention. Observation is passive and has no added clock cost. Acquisition remains
voluntary;other chosen work can occupy an actor through later glance slots. There
is no active Observe decision to claim as voluntary inquiry. B is deliberately not
combined with memory,new scheduling and new acquisition valuation in one treatment.

## Physical model and exact reuse

Preserve020:one co-present CacheId0,finite stock,no replenishment or accessibility
changes,checked grants,full-cost depleted failures,three individuals,shared018 law
(+5 hunger per4 ticks,cap100),one-unit meal after progression (−40,floor0),old
resource legality/negotiation,005 concerns,006 inquiry,008 Grouped/FIFO and012
Contextual learning. Keep every old capacity/default and current public availability.
Primary acquisition duration4,held2/8;window24 except last_window4. Ordinary horizon144,
ongoing/stranded576. Carry state across windows without resets.

Need/Acquire retain020 ranking and strict ties exactly. No new reward,weight,
future-relevance label,source-learning estimate or acquisition-value formula.
Use the same base contact/meal/Wait candidates. Acquire eligibility replaces the
fresh-stock condition only with an available own positive estimate;own availability,
food0,hunger>0,known access and enough remaining duration are unchanged.

Claims reserve actors,not units. Completion rechecks objective stock independently
of the earlier estimate. Deadline/rank arbitration and resource→claim→meal tick
order remain020. Atomic inquiry cannot skip pending claims;preserve its exclusive
lane mask/provisional rejection qualification. No claim that query concurrency is
generalized. No selection consults truth,private rival state,scenario,seed,future
outcomes,observer history or retained records belonging to another actor.

## Owned record,local inference and eviction

Add at most **one environmental record per actor**,not extra old evidence/episode
capacity. The record contains one immutable last legitimate observation (cache ID,
time,stock,access,public duration),an actor-local receipt sequence and own successful
takes since that observation. Policies receive only that actor's available record,
own legacy input,current public availability/lane and public duration. Unknown is
`None`,never a fabricated zero-stock observation. No archive or history lookup is
available to the selector. Actor-local receipt numbers do not reveal other actors'
observation counts. Growing observer audit is separate from bounded live cognition.

The local estimate is `observed_stock.saturating_sub(own_takes)`:an upper bound under
this finite depletion-only physics,not knowledge of current stock. Own successful
removals update only an existing,unexpired own record. They do not refresh its
observation time. Rival grants never update this record. Completion projects only
own cache/target,own outcome and time;actual cache-before/after quantities and rival
inventory are not passed into cognition.

Depleted is a new legitimate own empty-source experience. It may establish a new
zero record at completion,even after earlier observation expiry. A successful grant
after expiry changes inventory but **cannot restore the old quantity from the
attempt's observer observation or history**. The ongoing commitment legitimately
retains its target identity/time,not an accessible copy of forgotten stock cognition.
This preserves the mode-specific forgetting invariant and separately persistent food,
trust,concerns and learned consequences.

New observations replace the one slot (capacity-one FIFO/overwrite),reset own-take
count and receive a new own receipt number. Re-observation after forgetting is new
local acquisition,not archive recovery. Expiry is checked before decision/result
updates and at final export;actual elapsed time is its only age input. No reading of
past audit to fill an empty slot. Unit tests may use a second synthetic cache ID to
exercise the pure one-slot bound;worlds still contain only the real CacheId0.

## Modes and comparison scope

| Mode | Legitimate channel / retention |
|---|---|
| FreshReference | Fresh measurement at every free decision,then same020 selector. Additional sensing frequency is a reference,not a pure memory comparison. |
| CurrentOnly | Same declared glance slots as LastSeen;use a new current receipt only at that current decision,then discard it. Own current depleted experience is also legitimate at its receipt time. |
| LastSeen | Same glance slots;keep the latest record with own-take updates until a new receipt replaces it. |
| Expire12 | LastSeen,but unavailable at age≥12 ticks;drop before use or result update. Own-take changes do not reset age. |
| Unobserved | No acquired cache record/channel;no acquisition of an unknown source. This is a channel ablation,not voluntary forgetting or a pure retention contrast. |

CurrentOnly/LastSeen/Expire12 isolate retention under identical external sensing
slots and physical starts until voluntary divergence. FreshReference tests the
simpler020 current-sensor alternative but supplies more information. Unobserved
isolates acquired knowledge,not retention. All modes share potential supply,
traits,prepared cognition,needs,source availability,resource rules and external
windows. Never reset after divergence or interpret unequal sensing as memory gain.

An outdated negative becoming positive is **not realizable** in one monotonically
depleting cache with fixed access. Do not add a shutter,second stock or renewal to
manufacture that case. Missing a still-positive source after expiry is distinct.
No conclusion about learning renewed or newly accessible sources is supported.

## Registered cases and held-out plan

Reuse all14 archived020 initial families unchanged:single_start,contact_bias,empty,
stale_pair,contested,lost_contact,sharing,stranded,ongoing,information,
private_helpful,private_cautious,null,last_window. Reuse exact declared setup food,
hunger,traits/profiles,prepared old cognition and case-specific default order/window.
These are existing diagnostic fixtures,not natural-community samples.

The [registered matrix](registered-matrix.json) contains exactly214 unique rows:

* Primary140:14 families ×Need/Acquire ×five knowledge modes,seed42,cadence8.
* Cadence24:4/16 ×single_start/contact_bias/lost_contact/ongoing ×Need ×
  CurrentOnly/LastSeen/Expire12.
* Duration16:2/8 ×contested/sharing/lost_contact/last_window ×Need ×
  CurrentOnly/LastSeen.
* Order10:Descending/Rotating ×sharing/lost_contact/contact_bias ×Need ×
  CurrentOnly/LastSeen,excluding the already-primary lost_contact/Descending pair.
* Seeds24:1/1729 ×sharing/private_helpful/private_cautious/single_start ×Need ×
  CurrentOnly/LastSeen/Expire12.

These are deterministic parameter sensitivities,not independent statistical samples
or external transfer. No post-matrix retuning of cadence,lifetime,weights,traits,
capacity,opportunities or physics. Selected qualitative chains must be accompanied
by all primary contrasts,held-out reversals,nulls and failed/misleading cases.

## Causal evidence and falsification

Audit actual source observation/time/owner,new receipt,one-slot before/after,
overwrite/expiry/discard,own-result update,retained local input and estimate,
all candidate scores/eligibility/strict ties,selected action,admission,deadline,
busy interval,physical grant/failure,consumption,real resource/query execution,
later state/history and old retained cognition. Keep exact source/step/attempt/event
references;do not infer sole causation from timestamp adjacency.

Measure first knowledge/choice/execution difference with matched prefixes,successful
and failed claims,missed actual contacts,food/meal distribution,hunger-area per actor,
cache-inclusive material equality,trust,retained-state equality versus observer-export
equality,and persistence/reconvergence across later captures/windows. A new receipt
alone does not prove a useful decision;more transfers are not universally preferable.

Reject a retention-effect interpretation if the first choice changes from unrelated
fixture mismatch,hidden/future/current stock without a receipt,private rival input,
observer recovery,mode-dependent external opportunities or a retuned scorer. If
retained state differs but action/execution does not,report inertness. If current
sensing explains all gains,do not infer special memory competence. If only timing
or history differs,do not call it sustained survival or material superiority.

## Implementation,verification and cost boundary

Use new opt-in `environmental_knowledge` policy/state and isolated simulation adapter,
new021 fixture/evaluator/reader/tests. Protected020 sources cannot be edited. Private
legacy helpers may require local lifecycle duplication;document drift/maintenance
cost rather than refactoring frozen systems. Keep a private world wrapper while
claims exist. New source/final-original hashes and explicit archive manifest are
required;writers refuse existing output locations. Original001–020 evidence stays
byte-identical,including the five unprotected but preserved original020 viewer files.

Required:targeted tests,fmt,locked all-target Clippy denied warnings,full Rust tests,
read-only foundation/standalone009–020,exact214 original serialized replays,
independent primitive reconstruction and corruption rejection. Independent reader
may reuse reviewed old cognitive/resource primitives,but must derive new sensing,
retention,own-result logic and decisions without importing Rust scorer/evaluator or
prepared classifications. Include unknown/expiry/overwrite/redelivery,foreign/future
inputs,stock conservation,full-cost stale failure,old fresh/ignored controls and
the successful-after-expiry no-archive-recovery regression.

Measure pure selection,matched fresh-control execution/serialization,new audit bytes,
longer histories and compression;run relevant throughput workload. Keep wall-clock
measurements outside deterministic histories. One tiny record per actor does not
justify unbounded audit duplication;reference existing legacy inputs once as020.

Conclude ACCEPT,ACCEPT WITH QUALIFICATION,NO JUSTIFICATION or INCONCLUSIVE from
actual positive/negative/null evidence. No adoption or automatic protection.
The strongest limitation remains scheduled passive sensing at one stationary,
nonrenewing source. Active exploration,spatial knowledge,multiple sources,renewal,
psychological realism,optimality and autonomous-community capability remain untested.
