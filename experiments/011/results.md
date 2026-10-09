# Experiment 011: bounded inquiry value under competing concerns

Completed 2026-10-09, baseline `f5695731dd9d06d82e0f3bc3056ddcce235ef892`.
Conclusion: **NO JUSTIFICATION for adopting this StatusValue heuristic**.

StatusValue creates a reproducible local distinction, including near-versus-far
allocation, without future knowledge or new cognitive state. It does not demonstrate
a useful improvement beyond the controls in these fixtures. It amplifies cheap Direct
queries whose expected novelty does not translate into a change of retained Grouped
support or concern status. Preserve the prototype as opt-in research evidence;
retain the existing default. This negative result does not reject all bounded local
inquiry-value mechanisms. No012 was begun and the protected contract is not extended.

## Design and evidence boundary

The [specification](specification.md) fixes the rule before implementation and discloses
fixture corrections after controlled preview. No scoring weights were tuned after
controlled or held-out outcomes. Three actual scarcity refusals establish concerns;
ordinary goals80/65/45 and hunger90 give importance98/83/63. Minor/tie/near/far families
use declared different formation profiles. Actual local deliveries establish current
support. Wrong-confidence intentionally closes one of the initially three concerns;
stronger held-out deliveries can also close concerns before treatment. Such resolved
concerns leave the candidate set through the original rules.

005 supplies bounded concerns and status thresholds, 006 all candidates, learned
expectations/novelty/public offers/cost, 007 local independence and voluntary replies,
008 Grouped assessment, 009 fixed FIFO retention, 010 the exact CurrentNeed function.
Unchanged and CurrentNeed controls are unchanged functions, not retuned copies.
StatusValue uses `d=max(1,60-abs(support))`,
`leverage=min(100,expected_gain*100/d)`,
`value=(expected_gain+leverage)/2`, then importance×value/100 minus existing cost.
Integer rounding, Pause and stable ties are explicit. Distance is a current scalar;
there are no simulated future states, resource-policy lookahead or response sampling
before selection. The equal weighting is an uncalibrated heuristic.

Sixteen controlled families, four held-out combinations and three modes give240
trials per seed. Seeds0–15,42 and u64::MAX give4,320 trials, each with exact complete-state
replay and snapshot validation. A second complete evaluator execution also reproduced
both full seed42 files and the all-seed summary byte-for-byte. Measured inquiry/action/
inventory/status-count outcomes are seed-invariant; these are18 reproducibility
replications, not18 independent samples or a population frequency estimate.

Full traces: [controlled](seed-42.json.gz), [held-out](held-out-seed-42.json.gz),
[all-seed summary](summary.json.gz). The [readable report](report.txt) includes all240
seed42 allocations/configurations and local measurements; [analysis](analysis.json)
contains every matched pair classification. [Manifest](archive-manifest.json) protects
compressed and uncompressed bytes. Evaluators write only target outputs.

## Allocation measurements

These counts use unique seed42 cases, not pooled seed replications. A productive query
changes its selected concern's retained support or status immediately after the reply;
positive 006 realized novelty alone is insufficient. A newly resolved concern was
active before treatment; already-closed wrong confidence is excluded from successes.
Time is recorded query/resolver accounting, including Pause, separate from wall time.
Transfers are a descriptive material event, never the overall objective or a global win.

| Suite / mode | Cases | Questions / Pause | Changed / unproductive questions | Newly resolved | Unresolved remainder | Distinct concerns / repeated target | Accounted time | Transfers |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Controlled Unchanged |16|28 /4|21 /7|20|27|25 /3|760|5|
| Controlled CurrentNeed |16|28 /4|17 /11|15|32|22 /6|667|2|
| Controlled StatusValue |16|32 /0|3 /29|1|46|17 /15|452|1|
| Held-out Unchanged |64|129 /15|20 /109|16|167|70 /59|2136|6|
| Held-out CurrentNeed |64|130 /14|16 /114|12|171|72 /58|2042|6|
| Held-out StatusValue |64|144 /0|4 /140|1|182|71 /73|2036|5|

StatusValue's lower controlled effort comes mainly from cheaper, ineffective questions.
It never Pauses here; that is a failure signature rather than evidence of greater agency.
The held-out suite is difficult for all modes. StatusValue is not rescued by those
variations: it produces fewer changed concerns and no uniquely useful resolution
demonstration. No truth-accuracy score or global utility ranking is used.

## Controlled cases, including mistakes

A/B/C = concerns0/1/2; the digit is source ID; D/E = Direct/Evidence. Entries show
the two selected inquiries followed by the actual resource result. All future partners
are1 except minor_later and mirrored_future, whose partner is2. Offer/Accept transfers
one unit with actor cost and recipient benefit; Leave transfers zero.

| Family | Unchanged | CurrentNeed | StatusValue |
|---|---|---|---|
| uncertain_minor | A1E → C3E; transfer | A1D → A2D; Leave | A1D → A2D; Leave |
| poor_answerability | B2E → A1D; Leave | B2E → C3E; Leave | A1D → A2D; Leave |
| near_resolution | Pause → Pause; Leave | Pause → Pause; Leave | A1D → A1D; Leave |
| far_resolution | Pause → Pause; Leave | Pause → Pause; Leave | B1E → A1E; Leave |
| expensive | B2E → A1D; Leave | B2E → C3E; Leave | A1D → A2D; Leave |
| learned_poor | A1E → B2E; transfer | B2E → C3E; Leave | A2D → A3D; Leave |
| new_opportunity | B2E → A1E; transfer | A2D → A1E; transfer | A2D → A1E; transfer |
| resolved | B2E → A1E; transfer | B2E → C3E; Leave | A1D → A2D; Leave |
| wrong_confidence | B2E → C3E; Leave | B2E → C3E; Leave | B1D → B2D; Leave |
| minor_later | A1E → C3E; Leave | A1D → A2D; Leave | A1D → A2D; Leave |
| mirrored_future | A1E → C3E; Leave | A1D → A2D; Leave | A1D → A2D; Leave |
| equal_tie | A1D → A2D; Leave | A1D → A2D; Leave | A1D → A2D; Leave |
| partial_answers | A1E → A2E; Leave | A1E → B2E; Leave | C1D → C2D; Leave |
| silent | A1D → A2D; Leave | B1D → B2D; Leave | A1D → A2D; Leave |
| same_uncertainty | A1E → B2E; transfer | A1E → B2E; transfer | A1D → A2D; Leave |
| same_importance | B2E → C3E; Leave | B2E → C3E; Leave | A1D → A2D; Leave |

Specific distinctions and counterexamples:

- Unchanged already handles new public evidence, removal of resolved concerns,
  high importance, public costs and answer usefulness in several cases. In
  poor_answerability/expensive it nevertheless spends its second slot on A Direct
  without a concern change, while CurrentNeed resolves C. Thus Unchanged has a worse
  local allocation too; its greater total resolution count does not make it universal.
- CurrentNeed is worse in uncertain_minor: its uncertainty discount favors A Direct
  over A Evidence; Unchanged obtains the useful reading and later executes a transfer.
  It does not overweight the highly uncertain minor B enough to select B in this
  particular case. The negative method-choice result is reported instead of pretending
  the intended uncertainty contrast forced a different target.
- Only CurrentNeed reprioritizes usefully in partial_answers/core. First A receives20
  on a distinct retained origin, support10→28 without resolution; expected Evidence
  usefulness65→42. The second fixed public source is2. B's lower confidence now wins,
  and B receives20, support20→36 without resolution. A and B remain Partial. Unchanged
  instead asks source2 about A, which the source cannot substantiate. StatusValue asks
  both sources about C with no retained change. No material divergence follows; this
  is an inquiry/knowledge distinction, not a resource trajectory claim.
- StatusValue uniquely separates near from far after the same shared history:
  near A support55/importance63 is queried twice Direct; far A support5 makes B/E
  then A/E preferable. The controls Pause in both. The near small public reading20
  could legitimately cross60, but the learned expectations/costs do not select it.
  StatusValue's cheaper Direct replies leave evidence-based support55 unchanged.
  Its new distinction is interpretable but not demonstrated useful.
- new_opportunity gives A a genuinely new channel0 quality90 after the first slot
  (previous offer channel1 quality10). Both projections reconsider A and obtain the
  reading; Unchanged does too. StatusValue's recovery is not uniquely added value.
  Before then CurrentNeed/StatusValue waste a slot with source2 about A; Unchanged
  uses it to resolve B. No archive recovery or imposed target occurs.
- same_uncertainty favors legitimate importance in both old controls, while the new
  equal-score Direct tie wastes attention. same_importance gives B/C higher public
  expected gains, yet StatusValue chooses cheap A inquiries. The new rule overvalues
  leverage saturation relative to actual source/method effectiveness. That first
  same_importance Direct reply does legitimately raise A support0→25: a cheaper partial
  gain on a concern the controls leave untouched while resolving B/C. This positive
  local change is retained in the evidence. It does not resolve A or alter executed
  behavior here; no global utility has been specified to rank that tradeoff. The
  conclusion concerns adoption under the tested limited-slot criteria, not a claim
  that every cheap partial answer is worthless.
- silent spends both slots without information, lowers selected existing estimates
  (55→27 for fresh Direct attempts), and leaves unresolved concerns. wrong_confidence
  retains A's mistaken negative85/Resolved state, so A is absent from every inquiry
  candidate set. The later missed resource opportunity remains a real consequence.
- equal_tie is behaviorally equivalent across all modes. All preserve stable order,
  including a second inquiry to a different source with no useful answer. minor_later
  is a shared failure: none investigates the minor B before the fixed B opportunity.
  No policy knows that future. No mechanism has enough local future-relevance information
  to guarantee useful allocation, and adding a score does not create that information.

## Executed and persistent causality

Pairwise classifications count16 controlled or64 held-out matched groups per pair.
They overlap across pairs and must not be added as independent findings. E_Q requires
equal pre-inquiry readiness, changed allocation, different post-inquiry readiness
matching actual opening actions, accounting and post-consumption inventory divergence.
C means inquiry differs but executed actions match. Equivalent means measured behavior,
not identical scores or every internal field. E_A (same inquiry/different assimilation)
and F (ambiguous mediation) remain distinct in the classifier; neither occurs here.

| Suite / pair | Equivalent | C | E_Q | E_A / F |
|---|---:|---:|---:|---:|
| Controlled Unchanged / CurrentNeed |6|7|3|0 /0|
| Controlled Unchanged / StatusValue |2|10|4|0 /0|
| Controlled CurrentNeed / StatusValue |5|10|1|0 /0|
| Held-out Unchanged / CurrentNeed |34|30|0|0 /0|
| Held-out Unchanged / StatusValue |45|18|1|0 /0|
| Held-out CurrentNeed / StatusValue |22|41|1|0 /0|

The four controlled Unchanged/StatusValue E_Q families are uncertain_minor,
learned_poor, resolved and same_uncertainty. All show a missed transfer under
StatusValue. The held-out E_Q is same_uncertainty/H2, also a missed transfer under
StatusValue. Other inquiry differences often leave execution unchanged.

same_uncertainty/core gives a complete audited negative chain:

| Stage | Unchanged | StatusValue |
|---|---|---|
| Actual prior refusals | events1/3/5 create A/B/C | identical |
| Initial retained Native0/1/2 | support20/20/20, importance98/83/63 | identical |
| Inquiry0 candidates for A/source1 | Evidence g90, benefit88, cost31, score57 | Direct g55, leverage100, value77, benefit75, cost13, score62; Evidence value95, benefit93, cost31, score62 |
| Choice | A/source1/Evidence | A/source1/Direct, wins stable tie |
| New Native3 receipt | reading90; support20→92; A Resolved | Claim fallback is excluded by retained reading20; A stays Partial |
| Inquiry4 at next public meeting | B/source2/Evidence; B resolves | A/source2/Direct; source lacks A information |
| Initial A resource readiness | Leave; trust−17 and recent valence−17 | identical Leave |
| Immediate post-inquiry readiness | Offer score5; revised trust/valence0 | Leave; no revision of A20 |
| Actual event6 /7 | Offer/Accept, balances4/4→3/5 | Leave, balances4/4 unchanged |
| Subsequent consumption | actor2, partner1=5 | actor3, partner1=4 |

The current retained reading suppresses unsupported claim fallback; even positive
006 novelty can leave Grouped support unchanged. The actor does not see objective
truth to correct this mismatch. Existing later automatic inquiry is preserved after
the executed scene and cannot undo the recorded missed transfer. This is a material
consequence with donor cost/recipient benefit, not a universal assertion that Offer
is preferable. No later independent delivery is inserted before the measured resource
opening, and no state is reset after the first treatment choice.

## Held-out sensitivity and locality

H0: goals−10, support−15, response35, effort35, sources2/3, two slots, future2, stakes1.
H1: goals+10, support+10, response65, effort12, reversed listings/deliveries, two slots,
future3, stakes1. H2: goals unchanged, support+15, declared response95 capped at90,
effort0, all sources, three slots, future1, stakes1. H3: goals+10, unshifted support,
response65/effort12, reversed order, two slots, future3, stakes2. Near/far and partial
source schedules are disclosed family overrides. Native evidence caps at90; stronger
held-out initial evidence can resolve a concern before inquiry.

The same policy runs every combination. Goals, support, public quality/effort,
source availability, opportunity count, order and fixed future material partner/stakes
vary together; this is parameter/schedule sensitivity, not an external dataset or
isolated identification of each factor. Reported sensor reliability varies; the
accurate fixture channel fixes response content rather than sampling an error rate.
Initial wrong-confidence evidence remains inverted. This does not measure robustness
under naturally unreliable sources. New distinctions persist, but added usefulness
does not. Controls are often already sufficient or equally uninformed. In H2's
same_uncertainty condition Unchanged resolves all three concerns, CurrentNeed resolves
A after two ineffective questions, and StatusValue asks A Direct from three different
sources without any support/status change. Its three slots do not cure the failure.

All mode comparisons have identical first local input and delivered state. The paired
uncertain_minor/mirrored_future configurations change only the later partner and match
both entire inquiry-phase checkpoints and first decisions for all18 seeds and all five
configurations. Private food0 versus9 on unmet future partner4 also leaves first inquiry
identical in explicit runtime tests. Legitimate importance changes can alter selection,
and actual silent replies update old learned cells. The pure scorer receives no future
partner, truth, scenario, seed, observer record, private capability or hidden provenance.
The offline resource probes use a future observation solely to diagnose mediation.

Rust validates local projections against legitimate assessment-time prefixes; independent
Python reconstructs all240 seed42 trials, native retained bases, 006/007 expected gains,
costs, each scoring rule/tie, status/candidate boundaries, resource readiness, actual
events, conservation and consumption. No output of this reconstruction enters a live
actor. The checks exclude the identified leakage/mismatch explanations within these
fixtures; they do not prove locality for every possible future extension.

## Complexity and performance

No new persistent cognitive state. Evidence32, concerns8 and all prior capacities
remain. The pure score uses existing candidates and at most8 own support triples;
projection is bounded by existing Grouped work over32 own items. Observer capture
stores query/inquiry/assessment-boundary references plus those triples, not another
copy of full local inputs. Observer records/checkpoints/histories still grow.

Seven interleaved samples of the same controlled fixture gave these noisy medians:

| Mode | Run + validate ms | Serialize ms | Trial bytes | Value audit bytes / records | Identical-input clone + score ns |
|---|---:|---:|---:|---:|---:|
| Unchanged |0.400|0.193|111,069|804 /10|663.50|
| CurrentNeed |0.380|0.192|110,878|917 /10|773.42|
| StatusValue |0.399|0.193|114,552|917 /10|808.92|

The last column uses19 candidates/three basis triples,10,000 iterations per sample,
including the same decision-clone cost. StatusValue adds about4.6% to CurrentNeed's
clone-plus-score median here; this is neither isolated arithmetic CPU nor end-to-end
overhead estimation. Whole-trial timing also includes different voluntary actions,
responses and audit sizes. No resident-memory claim or general optimization.
The three JSON archives total117,745,636 raw bytes, compressed to3,561,272 bytes.
Summary duplication for analysis is offline evidence, not actor cognition. See
[whole-suite timing](benchmark.csv), [focused timing](performance.csv),
[identical-input scoring](scoring.csv), and [legacy throughput](throughput.txt).
Wall clocks never enter deterministic JSON or policy. Legacy throughput completed;
concurrent workload and single-host noise limit comparisons with historical timings.

## Interpretation, validation and remaining question

Universal inertness is falsified: allocation can change retained information,
executed actions and persistent inventory. Equivalence to CurrentNeed is falsified:
StatusValue creates different near/far, method and target choices in held-out too.
The stronger hypothesis that this threshold-distance heuristic adds demonstrated
useful allocation beyond the controls is unsupported. Its saturation and inexpensive
method preference create predictable errors, and the unchanged mechanism explains
the main useful cases. A/B remain viable explanations of sufficiency in particular
conditions; no single mode is sufficient universally. There is no observed basis to
promote StatusValue, claim optimal attention, human reasoning, calibrated usefulness,
significance, social emergence, or natural-world prevalence.

All required validation passed: fmt; locked all-target Clippy with warnings denied;
135 tests including nine011 tests; protected validate_contract, standalone validate009
and validate010; two full011 evaluator executions with exact reruns; independent target
and archive-only reconstruction; throughput; all106 frozen001–010 hashes unchanged.
Current010 full traces still match frozen raw bytes, with34-seed replay and E_Q/E_A/F
review;009 retains129-seed archive/replay checks. No historical evaluator or archive
writer was used to regenerate earlier evidence. Only011 evidence is archived here.

Changed files: new scorer/API/hook,011 harness/evaluator/tests, specification/results,
offline analysis/archival/evidence and architecture/history/README notes. Prior
attention, retention, concern/inquiry/resource rules, capacities/defaults, protected
contracts and every archived001–010 byte remain unchanged.

Strongest unresolved question: can existing local source/method usefulness be related
to **changes in the retained assessment**, rather than novelty or proximity alone,
well enough to avoid cheap ineffective questions without future knowledge or a planner?
This experiment identifies the mismatch; it does not implement its solution. Stop here.
