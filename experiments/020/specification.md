# Experiment020: Finite Environmental Resource Acquisition

Prospective specification, written before treatment implementation and matrix outcome
inspection. Baseline source: `831d8fc39f39326f623751268e727147540aad4d`; its viewer Gate B
was validated,committed and pushed before this design. Foundation001–019 is protected.
This specification authorizes only opt-in020 research,not default adoption,protection
or021. [Design gate](design-gate.md) must pass before runtime code.

## Question and hypotheses

Can independent pursuit of finite,costly,locally observed food alter later inventory,
consumption,cooperation and persistent histories without creating food or selecting
beneficiaries by observer knowledge? Does access enable useful allocation or relocate
unproductive activity? Distinguish additional supply from access to the same supply.

H1 acquisition sometimes replaces another activity;H2 grants change later voluntary
consumption;H3 grants change actual sharing/negotiation;H4 contention denies a valid
claim;H5 failure costs opportunities;H6 finite access can delay but cannot prevent
exhaustion;H7 repeated pursuit of exhausted sources;H8 access can harm an individual,
miss cooperation or strand food;H9 score/admission/order dominate;H10 effects are inert.
Preserve contrary and null outcomes. H7 is **structurally constrained** here: current
visible zero is ineligible,so repeated knowingly-empty attempts cannot occur. Stale
same-cohort observations can fail. Absence of further empty attempts is not learned
avoidance or empirical evidence about memoryful stale-source policies.

## Physical state,identity and conservation

One stationary co-present cache,stable domain CacheId0,nonnegative u32 stock and fixed
initial capacity equal to initial stock. No ECS handle is serialized. All actors can
currently observe stock; a trial's access flag declares whether taking is permitted.
No travel,production,replenishment,destruction,markets,food-value changes or new cognition.
Availability,access,acquisition,consumption and interpersonal redistribution differ.

At every captured state:

`initial agent food + initial cache food = current agent food + current cache food + consumed`.

Acquisition atomically debits one existing cache unit and credits one owned unit.
Failure changes neither balance. Use checked inventory addition,never wrap/negative
stock or duplicate grants. Existing legal meals destroy one unit and reduce hunger
by40,after progression,floor0. Interpersonal actions retain original legality/accounting.
Trace cache stock and all owned food through every operation,not only final totals.

## Actor-local projection and pure policies

Input contains the established own Agent/profile,bounded local inquiry projection,
now/end and public availability,plus:

* own current cache observation:CacheId,observed-at,current stock,accessibility and
  public attempt duration;
* current public atomic-inquiry-lane availability,reflecting the declared exclusive
  legacy atomic-query mechanism,not anyone's private cognition or future actions.

No scenario label,objective inquiry truth,observer history,rival food/hunger/traits,
hidden provenance,rival intended choice,future stock,claim outcome or beneficiary
rank reaches policy. A new current observation is a legitimate local channel,not
recovery from an archive. No persistent cache belief or contact-feedback cell is added.

Acquire eligibility:actor free,own food0,H>0,observed access true,observed stock>0,
and `now+duration<=window end`. Current visible zero always excludes it. Food0 is a
one-meal inventory heuristic,not a capacity increase or an optimality assertion.
The same underlying policy evaluates every eligible actor independently before any
new acquisition is reserved. Actor IDs and arbitration order do not enter its score.

Reuse019 Baseline's unchanged018/017 resource/inquiry ranking and Consume/Wait values.
When the public atomic inquiry lane is occupied,mask only Inquiry contact eligibility
and reselect among the already scored contact candidates using the same strict ties;
do not modify006/007/008/009/012 learning or assessment. The eligibility mask is a new
020 protocol constraint,not a new expected-value formula. Without acquisitions it
does not apply,so the historical control path remains exact.

Two preregistered acquisition selectors:

1. **Need:** append eligible Acquire at own H after Wait,Consume,Invite,and choose
   strict first maximum. It intentionally exposes the inherited incomparable score
   scales; a trusted contact can suppress available food.
2. **Acquire:** same candidates,choose eligible Acquire first;otherwise historical
   maximum. This is a transparent empty-inventory priority heuristic,not calibrated
   value or mandatory success. It may waste time or lose useful contact.

The **Ignored** policy records acquisition eligibility but chooses historical behavior
without taking. It isolates adding an option from using it. No post-outcome formula,
threshold,trait,weight,cost,capacity or ordering retuning is allowed.

## Cohort admission,time and contention

The cache baseline duration is **4 logical ticks**,engineering-scale equal to one
default need interval,not calibrated effort. Held-out durations **2 and8** bracket
that interval. Fixed shared018 law:epoch-based +5 per4 ticks,saturating100. Acquiring
or negotiating never suspends another individual's physical pressure progression.

Each cohort captures current inputs and independent choices for free actors. Then:

1. Commit every legally selected Acquire at the current tick,without reserving a
   particular unit. Record unique AttemptId,CacheId,actor,observation,input/choice,
   start,end and cohort/wave. At most three active claims;claimants become busy.
2. Non-acquirers' actual outgoing invitations retain their selected target/type.
   An acquiring actor sends no contact invitation and cannot eat or respond while
   reserved. Consuming non-acquirers eat at the next tick. A free actor may receive
   and independently accept an invitation after its meal,as in019.
3. Existing two-tick handshake/recipient policy and deterministic arbitration apply
   to interpersonal proposals. A busy recipient has **no response opportunity**;
   record that omission explicitly. Availability is rechecked at arbitration; if
   a short claim has just ended,the missing earlier acceptance still cannot become
   consent retroactively. Do not fabricate Wait/Accept decisions for busy people.
4. Resource scenes continue one tick at a time with the existing resolver. The
   uncommitted third actor may acquire while the pair negotiates. Runtime busy
   reservations prevent any actor occupying both. Do not charge parallel time twice.
5. Each elapsed tick advances the unchanged physical clock,executes the existing
   scheduled resource action if any,then completes due claims in deterministic
   order,then performs meals already chosen by other free actors for that tick.
   Resource actors and acquirers cannot be among those eaters.
6. Completion checks actual CacheId,current stock and checked inventory credit.
   A depleted loser pays the **full same duration**,gets no unit,no refund and no
   automatic fallback action. Simultaneous completions use Ascending,Descending or
   the existing Rotating proposer rank based on the recorded submission wave.
   Earlier completion times precede later ones. Resolve sequential debits atomically.
7. Active attempts end at their declared tick; do not delay them until another
   activity happens to finish. The next actor decision is at the next recorded
   cohort boundary after release. State carries across windows without resets.

**Atomic inquiry restriction:** legacy inquiries advance a shared atomic interval;
their internals expose no general concurrent clock callback. Therefore an inquiry
cannot be admitted while any cache claim remains active. Existing claims mask its
public lane at selection. If a newly selected acquisition closes the lane in the
same cohort,record an explicit atomic-overlap rejection after provisional consent;
do not execute a query or learn from its preview. Do not replace it silently with
another allocation. Once claims finish,real inquiries remain possible with their
existing sources,heuristics,costs,receipts and uncertainty. A query admitted with
no claims blocks interior new decisions as in018/019. This narrow exclusivity is
an architectural limitation to measure,not suppression of inquiry for a favorable
result. No query duration can skip a pending acquisition deadline.

Control prototype implementation may locally duplicate frozen recurring adapter
bookkeeping where private helpers prevent composition. Do not alter frozen helper
bodies or design a general scheduler. Use a private020 world wrapper so callers
cannot invoke unrelated public Simulation actions during a pending claim. New
resource/attempt actions are admitted only through the bounded020 lifecycle.

## Treatments and matched total supply

Five primary modes,all same individuals,traits,prepared cognition,needs,external
windows,ordering and total potential food within each family:

| Mode | Initial cache/owned placement | Access | Selector |
|---|---|---|---|
| Owned | Move the family's cache units into starting agents,one per cyclic ID0,1,2;cache0 | No meaningful stock | Historical/Ignored |
| Closed | Family's specified private inventories and cache stock | False | Historical/Ignored |
| Ignored | Same physical state as Closed | True | Historical/Ignored |
| Need | Same physical state as Closed | True | Need |
| Acquire | Same physical state as Closed | True | Acquire |

Closed/Need/Acquire are access/use comparisons with identical starting private
inventories and total stock. Ignored/Need/Acquire isolate voluntary ranking under
identical physical access. Owned is an explicit ownership-placement intervention,
**not** a matched-agent-inventory pure-access effect. Closed/Ignored should preserve
the complete legacy outcome when access is ignored. The ongoing/Owned control
starts12 units per actor and is an exact historical019 Baseline ongoing control.

## Registered families

All unlisted setups use019's ongoing preparation with stock0,then the following
explicit before-epoch circumstances. Arrays are actor0,1,2. Generosity/caution and
initial owned food are experimental construction,not later scripted behavior.
Profiles and seeded expectations retain the established preparation. All three
are co-present; no future meetings or successful responses are scheduled.

| Family | Owned food | Initial H | Generosity / caution | Cache | Purpose |
|---|---|---|---|---:|---|
| single_start | [0,0,0] | [80,0,0] | [0,0,0]/[100,100,100] | 3 | One initially eligible hungry claimant;others may later develop needs. |
| contact_bias | [0,0,0] | [100,100,100] | [60,60,60]/[20,20,20] | 2 | Eligible acquisition may lose to contact under Need. |
| empty | [0,0,0] | [80,80,80] | [60,60,60]/[20,20,20] | 0 | No grant can occur. |
| stale_pair | [0,0,0] | [80,80,0] | [0,0,0]/[100,100,100] | 1 | Two legitimate observations;late completion may be stale. |
| contested | [0,0,0] | [80,80,80] | [0,0,0]/[100,100,100] | 1 | Multiple claims for final unit;real losers. |
| lost_contact | [0,1,0] | [80,20,80] | [60,100,0]/[20,0,100] | 1 | Descending primary order;acquisition may cost cooperation with an independent potential donor. |
| sharing | [0,0,0] | [20,80,0] | [100,0,0]/[0,100,100] | 1 | Acquirer may voluntarily offer newly owned food;no forced donation. |
| stranded | [0,0,0] | [100,100,100] | [60,60,60]/[20,20,20] | 2 | Use019 history preparation;retained relationships can compete with acquisition. |
| ongoing | [8,8,8] | [20,20,20] | [60,60,60]/[20,20,20] | 12 | Total36,longitudinal exhaustion,comparison with historical owned-only world. |
| information | [4,4,2] | [20,20,20] | [60,60,60]/[20,20,20] | 2 | Use019 information preparation;concerns/inquiry compete with taking. |
| private_helpful | [0,1,0] | [80,20,0] | [60,100,0]/[20,0,100] | 1 | Matched focal input versus private_cautious. |
| private_cautious | [0,1,0] | [80,20,0] | [60,0,0]/[20,100,100] | 1 | Only partner1's private G/C differ;not an isolated trait estimate. |
| null | [0,0,0] | [0,0,0] | [0,0,0]/[100,100,100] | 0 | Policy/access changes can have no executed effect. |
| last_window | [0,0,0] | [80,80,0] | [0,0,0]/[100,100,100] | 1 | Window4;duration8 cannot fit;preserve stranded food. |

Primary seed42,five modes ×14 families =70 worlds. Default window24,gap1,interval4,
duration4,Ascending order;lost_contact uses Descending,last_window window4. All
ordinary horizons144 ticks from the captured epoch;ongoing andstranded576. No
post-start inventory/profile/hunger changes except legitimate resolution/clock.

Held-out parameter sensitivity,not independent transfer or statistical replication:

* Durations2/8 for contested,lost_contact,information,ongoing,last_window,each Closed/
  Need/Acquire:30 worlds.
* The two non-primary orders for contested,sharing,lost_contact,ongoing,each Closed/
  Need/Acquire:24 worlds. Every order comparison retains matched starts.
* Seeds1 and1729 for contested,sharing,private_helpful,ongoing,all five modes:40 worlds.

Total **164** complete deterministic histories before replay. No policy changes
between variants. Cases cover required F1–F16 through these explicit purposes;
stale_pair/contested test both stale observations and final-unit competition,
lost_contact tests opportunity cost/harm,and the two sustained families test depletion.
If a hypothesized sharing/harm/stranding chain fails,report that null;do not tune it
or substitute an unregistered winning case. Extra diagnostics must be clearly
separate from registered evidence and never replace unfavorable histories.

## Measurement,independence and falsification

Capture each local input/candidate/choice,source availability,claim admission/start/
finish,arbitration order,cache/owned balances,time/need changes,interpersonal intent/
consent/allocation/execution,actual meals/inquiries,trust and later state. Distinguish
attempts,grants,failures,contested claims,per-person acquisition/consumption,remaining
stock,waiting,missed social admissions,actual transfers,queries,empty-cache eligibility,
exhaustion time,pressure trajectories and material/full-state reconvergence. Never
combine these into welfare. A lower pressure-time for one actor can harm another.

An independent Python reader must not call the020 evaluator or selector. Reconstruct
activity legality/selection,public observation,submission/completion ordering,clock,
busy periods,stock/inventories,meals and transfers from primitive records. Reuse
reviewed legacy pure cognitive arithmetic only with explicit attribution; it does
not independently establish new020 legality. Scratch corruption tests must reject
wrong CacheId/stock/ownership,duplicate or undecremented grants,false success,future
stock,private-input leakage,choice/order/overlap/time/need/meal/reference corruption
and missing/forged events. Exact Rust reruns and source/archive manifests complement,
not replace,independent reconstruction. Preserve complete original histories.

Falsify or stop for any conservation,locality,deadline or busy-overlap defect,scripted
inquiry/donor/beneficiary,future-aware selection,unmatched primary supply,unreported
physical change,unsupported replay or observer recovery of cognition. Report a
material implementation defect before interpreting outcomes;fix rules to the frozen
specification,retain invalid candidates and disclosure,never retune parameters.
No effect from identical executed histories is a valid null. No social benefit can
be claimed from merely acquiring food or simulating a hypothetical later offer.

## Complexity,validation and stop

One cache,three active claims maximum,no new cognitive state. Original cognitive
caps/retention lifetimes persist. Observer histories grow;measure new audit bytes,
candidate/step cost,complete evaluator and replay time separately from deterministic
records. Use standard JSON/gzip manifests and a source freeze. No general optimization.

Run fmt,locked all-target Clippy(-D warnings),locked tests,foundation001–019,standalone
protected validators009–019,complete020 evaluator/exact replay,independent reconstruction/
corruptions and relevant throughput measurements. Verify296 protected artifacts remain
byte-identical. Add explicit020 viewer semantics only after inspecting final actual
schema;defer a large frontend rewrite,but provide an authentic read-only demonstration.

Conclude ACCEPT/ACCEPT WITH QUALIFICATION/NO JUSTIFICATION/INCONCLUSIVE from observed
evidence,not intended success. Stop at a defensible reproducible result or a precise
design/implementation gate blocker. Independent scientific review and protection
are future separate tasks. Do not begin021 or promote acquisition to a default.
