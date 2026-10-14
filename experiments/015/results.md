# Experiment015: self-initiated encounters and mutual availability

**ACCEPT WITH QUALIFICATION.** Three individuals can independently propose and
respond to contact, generate actual resource/inquiry encounters and retain personal
and material consequences. Refusal, failed contact, contention and remaining separate
also occur. Own history changes whom an individual approaches; it does not establish
better allocation overall. Simple initiation shares major positive outcomes. Ordering
strongly affects results and invitation failure is not learned as a new cognition.

This is a completed research experiment, pending a separate independent scientific
review. No new policy is adopted as default; foundation protection stops at001–014.
No016 was started. [Preregistration](specification.md) governs the prospective plan.

## Staged authority and question

Stage A independently reviewed014 at `4ac0e66`, confirming ACCEPT WITH QUALIFICATION
and actual-time opportunity loss, with reservation/scoring/ecological limits.
[Review](../../docs/experiment-014-review.md) was committed/pushed as `9e3a211`.
Stage B protected168 original artifacts and qualified findings, with a shared
read-only014 gate and both readers; committed/pushed `662de1e`. All mandatory gates
passed before015 specification or implementation. Original001–014 evidence, contracts,
reviews and runtime semantics remain unchanged.

Question: can multiple individuals' own choices determine which encounters happen,
while competing for limited mutual availability and retaining consequences? A
existing mechanisms suffice; B availability matters; C history affects contact;
D contention persists; E simple rules compare favorably; F scheduling/fixture artifacts
explain apparent gains. Narrow A–D are demonstrated; E holds in important controls,
and F remains a strong limit on policy comparisons. These are separate findings.

## Architecture, authority and actual time

Three stationary people share public current co-presence, current availability flags,
remaining horizon and public encounter ordinal. No environment-supplied desired pairs.
New pure `encounters.rs` initiation/response/matching functions use an isolated
`simulation/encounters.rs` adapter and bounded ECS environment (`end`, three commitments).
The fixture provides initial conditions and relative hunger interventions, not a
sequence of interactions. Existing ECS identities,001/002 scarcity negotiation,
005 concerns,006 inquiry,007 acquired evidence,008 Grouped,009 FIFO and012 Contextual
remain intact.013 reconsideration,010/011 projections and native evidence offers are
not enabled. No new persistent actor cognition, social graph, travel or general planner.

Each wave captures all three private proposals before revealing any. Sending spends
one tick; recipients decide in the next tick. These phases can occupy different
actors concurrently: sender-active and receiver-active totals are not added as serial
world elapsed time. A sender receiving an invitation acts in separate phases. Own
state/profile, public sender/type and own outgoing plan determine recipient choice.
Other actors' private scores/plans/food/evidence do not enter that choice.

Acceptance is provisional consent, not a resource Accept or exclusive booking.
Already-sent outgoing consent remains valid when a person also accepts an incoming
invitation. Greedy declared arbitration can honor that outgoing edge instead of the
accepted incoming edge. Every executed pair nevertheless has a real proposal and
the recipient's actual acceptance; no declined recipient is compelled. This is an
important protocol limitation, not proof of optimal preference satisfaction.

Invitations are valid only at their response/arbitration tick. Reciprocal same-kind
requests share one encounter; incompatible accepted edges lose the wave. Ascending,
Descending or globally wave-rotating proposer order uses no private utility. Stable
partner enumeration and incoming ties still favor lower IDs even under Rotating;
none of these controls establishes neutral/fair scheduling. Different resource
opening directions can matter despite the same mutually desired pair.

Public admission reserves handshake2 plus resource6 or Direct13/Evidence31. Actual
old resolver ticks are charged; the unused bound is not charged/refunded. Both
participants occupy the executed interval. With three people every pair overlaps
another; no disjoint work is suppressed by serial resolution. Fresh waves may follow;
every failed non-Wait wave costs2, all-Wait reaches the horizon, and horizon≤128
ensures termination. Mandatory consumption adds three separately recorded ticks.
All commitments in these fixtures complete within a period. Unavailable's initial
prior commitment ends at its first horizon; no hidden release time reaches policy.

Resource resolution retains old negotiation, balances, episodes/trust, concern
capture and termination, while excluding automatic communication/inquiry. Selected
inquiry retains old voluntary concern/source/method selection, provenance delivery
and learning. No guaranteed food/evidence follows meeting acceptance. Every post-setup
event/query is checked inside a charged span; arbitrary direct legacy API calls are
outside this opt-in scheduling protocol.

## Policies and non-isolated comparisons

Simple gives eligible candidates1; History uses014 own resource readiness+hunger/2
or old query score, minus handshake2. NoHistory masks only trust/episodes for resource
initiation ranking. Actual resource behavior and all query cognition keep history.
Wait0/resource partners in ID order/selected inquiry and strict ties remain fixed.
Simple is deliberately resource-first on equal1 scores, not a neutral activity
baseline. Its zero inquiries are a property of that rule, not a discovery about curiosity.

Common response policy compares own readiness for incoming resource contact, own
relationship goal−privacy for incoming inquiry, and own outgoing score for Continue.
Incoming precedes Continue on equal scores. These are heuristics, not calibrated
cross-person or cross-activity values. In particular Simple's outgoing1 is a priority
label, while incoming scores use larger readiness values. **Simple versus History
therefore changes effective Continue valuation as well as initiation.** NoHistory's
masked outgoing score likewise propagates into its owner's response choice. This
is an explicit causal path, not an isolated recipient-policy ablation or evidence
that richer cognition alone improves social behavior.
NoHistory masks initiation while incoming response values keep real history;
that phase asymmetry can itself increase contention. Lower contention under History
is not independent proof of better prediction or coordination.

## Strong positive executed history

Mutual/core seed42, History: actor0 has food0/hunger90. Its resource score96 favors
inviting visible1;1 independently invites0 (98),2 waits. Neither inviter sees the
other's inventory. Both recipients accept the other's invitation on a local tie
against their own plan. At tick2 arbitration selects0→1 and merges the reciprocal.
Actual events0/1/2 at ticks3/4/5 are Request/Offer/Accept; one food unit transfers.
This is neither a probe nor a scripted accepted offer.

Further independently chosen waves transfer additional stock. After first consumption,
0 has food3/hunger50 (initially0/90),1 has food0/hunger0, with directed trust96/32.
These states carry into subsequent decisions with the explicit relative hunger
intervention; there is no reset. The same beneficial first chain occurs under Simple
and NoHistory, so History is not uniquely responsible. Twelve total transfers across
six periods circulate four initial units; only four units are consumed. Counts do
not establish twelve produced units or global welfare. The finite system eventually
leaves0 hungry again.

Contended adds another hungry actor2. It independently seeks0, while0 accepts1 and
1 accepts0. Actor2's invitation is declined and it receives no food in the first
period, despite compatible-looking local need. Its first hunger stays90 while0
reaches50. ID ties and private inventory ignorance contribute; no oracle redirects2
to the actual donor. The unit contention test separately constructs conflicting
legal accepted edges and verifies one execution, explicit order reversal and no
double booking.
Across all six core Contended periods,2 executes zero encounters while0/1 share27;
2 finishes hunger100. This is observed contact starvation under the declared rules,
not friendship, hierarchy or an institution.

## History changes contact and persistent distribution

History/core starts from014's genuine two refusals by1. For actor0, current resource
initiation scores are13 toward1 and78 toward2. History invites2. NoHistory masks
only its own ranking history, ties at78 and invites1. Starting worlds otherwise
match. Actual first History encounter is Offer/Refuse (events4/5, ticks44/45), not
the cooperation it hoped for. The new refusal generates real episodes/trust/concerns;
at first consumption0's trust toward2 is−44. First inventories are3/4/4 versus
NoHistory0/7/4. NoHistory's first encounter is actual Leave (event4/tick44), followed
by other voluntary waves, not the same fabricated counterfactual response.

At six-period end, History leaves food2/hunger0 with2; Simple/NoHistory instead leave
food2/hunger0 with1. Actor0 ends food0/hunger100 in all three. Each mode totals15
transfers and10 consumed units. This is persistent allocation and relationship
divergence despite identical aggregates, not demonstrated overall improvement.
The continuing policy intervention affects later proposals/responses too; no isolated
claim assigns the final two units exclusively to the very first refusal.

Early resource refusals also change concern targets and hence eligible inquiry sources.
The conservative third-party filter excludes anyone targeted by any retained own
concern, including unrelated/resolved matters. It can remove otherwise interesting
informants. History makes no inquiry in this core family; NoHistory makes two and
obtains a retained change. This is a restricted-protocol path, not calibrated source
value or evidence that having less history is generally better.

## Information, refusal and harmful/equal results

- Information:0 voluntarily invites2 for inquiry at41. Handshake ends43; selected
  Direct inquiry ends56 and legitimately changes support20→60/Partial→Resolved.
  Two executed inquiries across the core history produce one receipt/change; Simple
  asks none. All three still have zero transfers and identical final food/hunger
  here. Changed knowledge is not a demonstrated material advantage.
- Misleading: the same kind of inquiry receives inverted evidence, support20→−30.
  Progress learns from a real change, not truth. It is not tuned away or called useful
  merely because a response/positive update occurs.
- Private mirror:0's initial input/decision equals Information in every mode/config,
  while hidden source privacy changes the real response. First41→43 wave has all
  three invitations declined; no question executes. History issues144 invitations,
  has108 declines and executes no inquiry across core. Invitation refusal alone does
  not train006/012, so it keeps trying. This is costly failed autonomy, not discovery
  of the recipient's motives. Resource invitations to0 can be refused while it
  continues seeking information.
  Decline is a structured public protocol result, but no new actor memory/input
  carries it into the next wave. Repetition partly reflects that missing feedback
  representation; it does not show deliberate ignoring of remembered refusals or
  prove a new persistent cognitive map is necessary.
- Formerly-useful: a real setup inquiry changes10→28. Source A is later FIFO-evicted
  while the archive preserves it. The actual later inquiry selects B and gets a
  redundant receipt, support10→10; no meaningful assessment change. A targeted old
  exchange cannot redeliver evicted A. The original test's predicted silence was wrong;
  the redundant case remains evidence, not a replaced outcome.
- Unilateral: first0→1 invitation is declined,0 spends2 ticks, and no first-period
  encounter or food appears. Later needs can provoke meetings without transfers.
  Meeting agreement and negotiation cooperation must remain separate.
- Refusal: invitations can be accepted but old resource policy actually refuses;
  all modes generate executed refusal history/concerns with zero core transfers.
  Invitation declines themselves add no invented resource event/trust contribution.
- Missed: horizon2 prevents admission despite co-presence; all modes make no contact.
  It is a timing control, not observed rejection. Conservative caps can reject fast
  feasible branches; unexecuted counterfactual outcomes remain unknown.
- Separate: History/NoHistory initially Wait with no forced meeting. Later external
  hunger increases can produce unsuccessful contact. Simple's extra invitations
  do not create food. No lasting preference for separation is established.
- Longitudinal24–32 periods: same people, food, trust, episodes, concerns, knowledge,
  estimates and elapsed time persist. Core Simple executes127 encounters versus119
  for History/NoHistory, all with zero transfers. They finish food0/hunger100 for all.
  More contact fails to prevent exhaustion; no production or forced useful reunion.

## Complete matrix and ordering reversals

13 families ×3 modes give39 core and156 prospective sensitivity combinations.
Four seeds give780 worlds, each exactly rerun. Seed42's195 complete histories yield
195 overlapping within-config comparisons, plus78 matched ordering contrasts.
Seeds are replay controls, not independent statistical samples. Budget16/48 variations
also change period horizons; ordering-only variants hold budget24 and all first
proposal/response inputs fixed. No retuning followed preview or sensitivity.

Core seed42 aggregates96 periods per mode (2172 activity ticks each):

| Mode | Invitations | Encounters | Declines | True contention | Reciprocal | Q ticks / changes | Transfers / consumed |
|---|---:|---:|---:|---:|---:|---:|---:|
| Simple |1434|471|504|154|305|0/0|48/98|
| History |1340|412|523|0|405|88/3|48/98|
| NoHistory |1319|405|518|153|243|113/4|48/98|

Core39 pair classes:13 equal choices/trajectory/final material;9 changed choices
with equal material/relationship trajectories;12 changed choices and relationship
trajectories but no final material change;3 changed trajectories that catch up in
material/trust;2 persistent final material/trust differences. Across all195 pairs,
25 change final food/hunger and110 change final trust. Choices, encounters, transient
trajectories, full final agents and final material are separate CSV columns.

| Budget24 order | Simple transfers / consumed | History | NoHistory |
|---|---:|---:|---:|
| Ascending |48/98|48/98|48/98|
| Descending |56/99|107/93|56/99|
| Rotating |56/99|131/98|49/98|

Large transfer counts can be repeated circulation and coexist with lower consumption.
Under Descending, History's Private-mirror case transfers21 versus Ascending0 but
consumes10 versus12 and leaves1 with two food units/hunger0 while2 is food0/hunger60.
It is not a universal improvement. History/Rotating in the History family ends focal
hunger20 versus Simple0, while others' stocks/needs differ. Simple can be better on
a particular person's dimension without a global ranking. Matching order and who
opens resource negotiation materially shape these histories.

All15 aggregate rows and all comparisons remain in `report.txt`, `comparisons.csv`
and `order-comparisons.csv`; `analysis.json` contains independently reconstructed
per-query outcomes, contact distribution, timelines and references. Do not treat
these designed overlapping comparisons as195 independent causal trials.

## Independent reconstruction and integrity

`review015.py` imports no Rust evaluator flags. It reconstructs each pure score,
strict tie, source eligibility, own concern prefix, retained Grouped basis, own012
cells, locally known provenance and selected old query/learning. It checks all
rejected/unused previews, recipient inputs and Continue values, provisional acceptance,
arbitration, commitments, actual spans, old resource scores/interpretation, saturated
trust replacement, food/hunger and next-period continuity. Every post-setup event/query
must be charged. All first proposal/response captures match across ordering-only
variants; all15 Information/private-mirror proposer comparisons match before reply.
Three in-memory score/time/invitation corruptions fail. Existing privacy/eviction and
matching regressions provide complementary controls. Observer data is used only for
verification, never to furnish actor cognition.

No decision policy receives objective event truth, future partners/responses,
partner-private holdings, hidden roots or observer archives. Own physical state and
legitimately received readings remain locally available. Current availability hides future release time; unit twins
with different release times produce identical current inputs. The provider-eviction
regression confirms historical evidence cannot restore unavailable acquisition.
Separate legacy trust, belief and concern lifetimes remain valid; no universal erasure
rule is imposed.

## Cost, limitations and scientific classification

Environment/current pending storage caps at3; old memory16/concerns8/evidence32 and
other established cognitive limits remain. No new persistent cognitive collection.
Across780 worlds complete serialized trial sizes are45,239–26,791,918 bytes. Core/
sensitivity seed42 raw archives total534,639,584 bytes, compressed10,656,254; original
preview adds1,466,072 compressed. Compact independent analysis is3,203,055 bytes.
These are serialized observer costs, not resident memory or live actor storage.
Repeated bounded projections/outgoing-input copies dominate new capture size; no
general audit refactor/optimization was performed. Large captures constrain scaling.

Five20,000-call fixed-input batch averages give median initiation costs Simple817ns,
History1296ns, NoHistory1221ns; common response879ns. They include capture/cloning but
exclude resolution, changing histories and full-world serialization. Whole-suite
timings in `performance.csv` measure first construction/causal validation, excluding
replay/output; `measurements.csv` and separate throughput output are nondeterministic
wall-clock evidence, not simulation state or a calibrated comparison with014 timings.

Demonstrated: independently initiated/accepted/declined contact, locally changed
partner choice, valid inquiry/real negotiation, lost requests, actual transfers,
trust/concern updates and persistent distribution differences under conserved time/food.
Mechanism-dependent: fixed waves, current public room/ordinal, one provisional acceptance,
heuristic score scales, expiry, conservative caps, three-person matching and ID ties.
Ambiguous: value of history sophistication, Continue scale, optimal consent protocol,
single-choice mediation, scheduling neutrality, ecological prevalence and necessity
of new invitation-learning state. Unsupported: human realism, friendship, conscious
intention, natural institutions, optimal cooperation, general intelligence or society.

The strongest open question is whether individuals can reduce repeated failed contact
using justified local experience while respecting separate invitation/query outcomes,
without replacing independent consent with an optimizer or merely exploiting matching
order. This experiment motivates that question; it implements no new learning to
answer it.015 needs an independent review before any protection pass. No016.

## Verification and changes

All completion checks exited0 at baseline `662de1e` plus this015 source/integration
and documentation tree, identified exactly by the three behavior hashes:

| Command/check | Result |
|---|---|
| `cargo fmt --check` |0 |
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` |0; includes new evaluator/validator/measurement and nine tests |
| `cargo test --locked -j 1` |0;165 passed,0 failed |
| `cargo run --release --locked -j 1 --example validate_contract` |0; protected001–014 gates and independent readers |
| Same Cargo invocation for `validate009`, `validate010`, `validate011`, `validate012`, `validate013`, `validate014` |0 each |
| `evaluate015 -- --output target/experiment015-final` |0;780 worlds, each exactly replayed |
| `validate015` |0;780 exact reruns, strict raw archive/summary equality, envelopes/source/inherited hashes,195 independent reconstructions/195 pairs |
| `python -B examples/support/review015.py --self-test` |0; all previews, cognition, scoring, matching, actual execution and three rejected corruptions |
| Newly built015 executable from empty working directory |0; strengthened all15 mirror/78 first-order-input controls,0 files created; no prepared cache |
| `measure015` |0;20 fixed-input batch timing rows |
| `cargo bench --locked -j 1 --bench throughput` |0; unchanged legacy compatibility workload, separate `throughput.txt` |
| Original168 hashes and17 review-start raw014 guards |0; original evidence unchanged |
|015 source freeze / document links / diff whitespace |0 |

Logs are `%TEMP%/eldoria-014-review-015/C-*.log`. The full Clippy/test/runtime gates
cover the final frozen behavior revision. Later changes only add documentary analysis
and strengthen read-only assertions; the empty-directory015 run includes those guards.
No passing runtime check was rerun for a documentation-only edit. No old archive writer
ran. All required gate failures are resolved; no material compatibility defect remains.

Development corrections are disclosed in the specification. The initial7/8 failure
was a wrong scientific expectation, not repaired behavior. All39 original preview
proposals/responses/executed histories match after removing only audit fields. Nine
targeted tests pass. Three behavior files were hash-frozen before full evaluation;
there was no policy retuning, cognitive-capacity increase or removed failed case.
New source/adapter/fixture, tests, evaluator/validator/measurement, independent reader,
reports/archives and navigation comprise015 changes. Shared source changes only add
module declarations. Final commit/push is reported in task delivery.
