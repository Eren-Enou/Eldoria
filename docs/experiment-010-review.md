# Experiment 010 scientific review

**ACCEPT WITH QUALIFICATION.** Experiment 010 establishes a legitimate, narrow
existence result: bounded retained differences can alter voluntary inquiry through
unchanged prior mechanisms and leave executed, persistent resource consequences.
Most demonstrated changes in inquiry allocation depend on the experimental
CurrentNeed heuristic. These are separate evidence classes. Neither establishes
selective retention superiority or a calibrated model of inquiry.

Reviewed 2026-10-08 against baseline `ef762a5`, without changing any behavioral
mechanism or original experiment evidence. Scientific acceptance here does **not**
incorporate 010 into the protected 001–009 foundation. That requires a later,
separate integrity pass. Experiment 011 is not started.

## Question and evidence boundary

“Do retention differences established by Experiment 009 survive far enough through
existing individual mechanisms to alter voluntary inquiry, executed resource behavior
and persistent consequences?”

The unchanged path uses 005 concerns, 006 candidate enumeration/learning/offers,
007 local provenance and voluntary responses, 008 Grouped assessment and unchanged
009 retention. Its attention mode captures audit projections but does not apply
them to policy. CurrentNeed applies a new bounded projection to the same mechanism:
`need = 100 - abs(current retained Grouped support)`, then
`benefit = existing benefit * need / 100` with integer truncation. Costs and strict
greater-than stable selection remain unchanged; Pause scores zero.

That confidence discount is a heuristic. It is not calibrated information value,
optimal uncertainty reduction, rational inquiry, human curiosity, objective relevance
or future usefulness. Own persistent concern/relationship/inquiry state has its
established lifetime; the projection neither erases it nor reopens resolved concerns.

Evidence sources are the original [specification](../experiments/010/specification.md),
[results](../experiments/010/results.md), report, comparisons, analysis, full gzip
traces and summary. All remain byte-identical. This review adds classifications and
qualification without silently rewriting the original record.

## Independent reconstruction and classification

`examples/support/review010.py` verifies each frozen gzip/raw size/hash and exact
equality with the current evaluator's target outputs, including the all-seed summary.
It independently reconstructs bounded retained prefixes from receipt/eviction and
received-attribution records, implements the documented ordered Grouped calculation,
and checks checkpoint/attention support. It checks recorded inquiry benefit/cost/tie
selection, actual event legality/accounting, consumption, matched deliveries and
paired future-partner decisions. The existing Rust snapshot validator independently
checks the full prior mechanisms and receipt-time boundaries. Neither reconstruction
feeds information back to live actors.

All 120 unique seed-42 matched groups are classified below. Each entry contains
the controlled case followed by held-out configurations H0/H1/H2/H3 in the exact
order returned by `experiment010::held_out()`. The script emits full configuration,
per-mode inquiry/action/food and classification details to stdout for reproduction.

Classification uses the requested A–F ladder, with two explicitly distinguished
routes inside E:

- **A:** retained relevant basis differs without a measured derived-state effect.
- **B:** concern/current derived state differs, but inquiry and executed actions match.
  This includes differences in uncertainty/receipt-linked state without status change.
- **C:** voluntary inquiry differs, while executed actions match.
- **D:** inquiry changes executed action; qualifying cases also persist and therefore
  appear as E_Q here. There is no isolated nonpersistent D case.
- **E_Q:** full inquiry chain: different inquiry, matching pre-inquiry action readiness,
  different immediate post-reply readiness agreeing with execution, and persistent food.
- **E_A:** material consequence through different assimilation of the *same* inquiry.
  This satisfies the action route of E, but does **not** demonstrate changed allocation
  and is excluded from every full inquiry-chain count.
- **F:** downstream differences exist, but allocation mediation is not isolated:
  readiness already differs or another post-inquiry delivery changes state. This
  classification does not declare the resource event invalid.
- **—:** equivalent measured behavioral/material control; unimportant retained noise
  may still differ. Do not infer universal equality of all internal collections.

| Family | Unchanged: core, H0, H1, H2, H3 | CurrentNeed: core, H0, H1, H2, H3 |
|---|---|---|
| Protected weak concern | B, —, B, B, B | E_Q, —, C, E_Q, C |
| Minor late item | B, B, B, B, B | E_Q, C, E_Q, E_Q, E_Q |
| Unequal importance / mirrored future | B, —, B, E_A, A | E_Q, —, C, E_Q, E_Q |
| Costly salience / inverted replies | B, —, B, E_A, A | C, —, C, E_Q, E_Q |
| Repeated weak traffic | B, A, B, B, B | E_Q, C, C, E_Q, C |
| Conflict | E_A, —, B, B, B | E_A, —, B, B, B |
| Hidden shared origin | F, B, F, B, F | F, F, F, C, F |
| Late attribution | F, B, B, B, B | F, F, C, C, C |
| Legitimate redelivery | F, B, F, E_Q, F | F, E_Q, F, E_Q, F |
| Equivalent relevant state | —, —, —, —, — | —, —, —, —, — |
| No mode preserves | —, —, B, —, B | —, —, B, —, B |
| Quality-protected evidence | B, A, B, B, B | C, C, C, E_Q, C |

H0: goal40/weak20/traffic90/response60/effort30/count32, source2 only,
future partner2 before family mirroring, one-unit stakes.
H1: goal60/weak40/traffic20/response80/effort10/count40, reversed deliveries,
both sources, future partner1 before mirroring, one-unit stakes.
H2: goal80/weak55/traffic70/response90/effort0/count48, both sources,
future partner2 before mirroring, one-unit stakes.
H3: goal60/weak40/traffic90/response80/effort10/count40, reversed deliveries,
both sources, future partner1 before mirroring, two-unit stakes.
Minor/all-forget and quality families retain their disclosed isolation overrides.

| Path/suite | Groups | Inquiry differs | Executed actions differ | Full inquiry chain E_Q | Inquiry differs, action matches |
|---|---:|---:|---:|---:|---:|
| Unchanged, controlled | 12 | 1 | 4 | 0 | 0 |
| Unchanged, held-out | 48 | 3 | 7 | 1 | 0 |
| CurrentNeed, controlled | 12 | 9 | 8 | 4 | 2 |
| CurrentNeed, held-out | 48 | 32 | 18 | 12 | 15 |

All action differences also change persistent food here. Counts describe selected
case/configuration groups, not population prevalence or independent observations.

## Strongest unchanged-path result

The full chain is **redelivery/H2/Unchanged**, in the original
`held-out-seed-42.json.gz`, with targets event1/3, concerns0/1, and future partner2.
It is independently verified from the trace, not accepted merely from the comparison
flag. The new regression protects this particular material accepted invariant.

| Trace step | FIFO | Quality | Salient |
|---|---|---|---|
| Assessment0: Native0, A quality55 | support55 | support55 | support55 |
| After 48 traffic deliveries | A basis0 | A basis0 | A basis55 |
| Assessment49: new Provenance97, A quality55 | before0 →55 | before0 →0; incoming rejected | before55 →79 |
| Before query0 | A Partial, B Open | A Partial, B Open | A Resolved, B Open |
| Query0 / Inquiry0 | A/source1/Direct, score44 | A/source1/Direct, score44 | B/source2/Evidence, score23 |
| Assessment50 from selected response | A Claim1: support55 | A Claim1: support90 | B native reading: support90 |
| Immediate B relationship input | trust−17, valence−17 | trust−17, valence−17 | trust0, valence0 |
| Actual event16 | Leave, Offer score−29 | Leave, Offer score−29 | Offer, score5 |
| Actual event17 | absent | absent | Accept, score34; transfer1 |
| After consumption: actor/partner2 | 3/4 | 3/4 | 2/5 |

Grouped support for Salient combines distinct locally available origin groups:
55 + floor(45×55/100) =79. This meets the existing absolute-support60 concern
resolution threshold. FIFO has forgotten Native0; Quality rejects the new lower
quality item against its full stronger basis. Neither can rebuild Native0 from
observer history. Provenance97 is legitimately delivered new evidence in all modes,
with message96 and received Known48; it is not a recovered observer acquisition.

Before inquiry all modes have food4, the same B memory and trust−17, and a Leave
diagnostic for B. The receipt changes A status in Salient; A is legally removed from
its inquiry candidates. The pre-existing mechanism selects B rather than being
instructed to ask B. In FIFO/Quality, A Direct scores44 versus B Evidence23; Salient
has only B active, whose Evidence23 beats Direct21. Public notices can consequently
differ after A resolves; they are endogenous voluntary consequences of that state,
not different pre-treatment capabilities or an imposed schedule.

The selected Salient reply then revises the still-retained B refusal episode and
replaces its trust contribution under the old rules. B readiness changes from
Leave to Offer. There is no independent later delivery in this family between reply
and execution. The resource scene legally executes Offer/Accept with balances
[4,4]→[3,5]. FIFO/Quality execute Leave. Subsequent automatic inquiry can resolve B
for Quality **after** its Leave, without retroactively transferring food. Consumption
uses the existing operation: one unit is consumed, preserving the inventory gap.

This establishes possibility through unchanged mechanisms, conditional on this
controlled redelivery/importance/opportunity configuration. It establishes neither
frequency nor generality; redelivery is a contributing legitimate cause, so retention
alone is not sufficient. The other unchanged inquiry differences are redelivery/core,
H1 and H3 only; each is F because pre-inquiry readiness differs. Unchanged conflict/core,
importance/H2 and costly-salience/H2 are E_A, not changed-inquiry evidence.

## Strongest CurrentNeed result and preserved mistakes

Protected/core is an unambiguous but costly E_Q. FIFO/Quality lose A40 while Salient
retains it with importance78. All have the same candidate opportunities before the
projection: A Evidence benefit65/cost31, score34; B Evidence54/31, score23. Salient's
A benefit becomes39, score8, and A Direct scores14, so B Evidence23 wins. FIFO/Quality
ask A/source1/Evidence; Salient asks B/source2/Evidence. Legitimate response receipt41
supports the selected event at90. Before inquiry all three are ready to Leave with
future partner1; after reply only FIFO/Quality are ready to Offer. Actual Offer/Accept
versus Leave persists as actor/partner1 food2/5 versus3/4 after consumption.

The importance/core paired fixture changes only the later resource partner to2.
The same inquiry allocations now let Salient transfer while FIFO/Quality Leave.
The independent review checks identical first inquiry decisions in this paired
control for both paths and all four held-out configurations as well as the core.
No actor sees which of these futures will occur when allocating inquiry.

Other preserved negative/positive findings:

- Minor/core: FIFO retains20 and discounts A, asks B, and realizes the partner2
  opportunity; selective modes reject20, ask A and Leave. This FIFO advantage reverses
  in the mirrored held-out configuration. A retained distinction can be useful without
  making the retention rule truth-aware or globally preferable.
- Repeated/core: Quality and Salient protect A40, but their B allocation misses
  the fixed A opportunity. H2 reverses the realized opportunity. Quality/H2 permits
  both selective modes to realize B while FIFO pursues A. Quality/core changes inquiry
  without changing action. Current salience/quality can help or waste attention.
- Costly-salience/core: different inquiry, uniformly Leave after inverted replies
  (C). H2 Salient misses an A transfer that FIFO/Quality realize; H3 can favor its
  allocation instead. Strong wrong retained evidence is not tuned away.
- Conflict/core: all ask A Direct. Retained opposing evidence blocks selective
  assimilation while FIFO's loss of conflict permits positive revision and transfer.
  This is E_A, not evidence that uncertainty chose a different inquiry.
- Hidden/core: locally Unknown reports sharing an undisclosed root create mistaken
  negative confidence; later correction also changes state. Late-attribution/core
  includes separate new provider disclosures. Both remain F for allocation mediation.
  Salient's later attribution changes support−35→−10 after the slot was spent; it
  neither restores the slot nor rewrites a missed transfer. Query totals do not change
  during that correction. Stronger local evidence need not remove residual conflict.
- Equivalent and all-forget controls do not produce inquiry/action/food differences.
  All-forget can be compensated by a later legitimate inquiry. Some derived metadata
  differs in its held-out cases, hence B rather than a claim of identical cognition.

Offer/Accept realizes a controlled cooperative opportunity with donor cost and
recipient benefit. It is not universally better than Leave. No global utility ranking
is defined, and no retention method wins universally.

## Locality, matching and resources

The runtime adapter receives the existing actor-local 007 decision and at most eight
own support triples, projected from the actor's live at-most32 assessment items.
It receives no future partner/event/action, objective truth, hidden root, scenario,
seed, observer history or partner-private state. `simulation::provenance_follow`
selects items by owner and passes only own concerns. Observer assessment cursors are
recorded for validation; the runtime does not replay archives to construct the input.
Public quality/effort offers follow existing voluntary announcement rules; private
capabilities and sensor outcomes stay in the resolver until a chosen response.

The full archived seed-42 groups independently match initial agents/resources,
concerns, incoming deliveries through the limited opportunity, configurations,
future partner and capacities across retention modes. Retention's method field is
the intended treatment; retained items and own consequences may then diverge.
Sources have identical configured capabilities; public announcements can differ
legitimately when prior concern status differs. Stable ID sorting makes reversed
meeting-list order invariant; reversing evidence delivery order is a real variation.
Endogenous time costs can differ; external opportunities occur in the same fixed
sequence rather than on a claim of identical absolute ticks after divergence.

Qualifying events contain actual selected decisions, observations, memory causes,
trust and episodic inputs, legal transfer outcomes and balances. All recorded event
totals conserve food; consumption subtracts the recorded consumed amount, and other
agents' food stays fixed through consumption. Source inspection costs use existing
accounting. Explicit initial fixture injections are interventions, not endogenous
resource production. Source inspection, meetings and later scenes do not reset
resources/profiles after divergence. Cognitive convergence preserves material history.

These source boundaries, paired controls, prefix reconstruction and compatibility
tests exclude the identified leakage/mismatch explanations in these fixtures. They
do not prove that every hypothetical future fixture or adapter is free of leakage.
The original results' broad statement that artifact explanation E is excluded should
be read with this scope. Designed fixed opportunities remain an external-validity
limitation, even though the inquiry target and executed outcome are voluntary.

## Interpretation and limits

Twelve designed families, four reused parameter/schedule combinations, and 34 exact
seed replications yield 12,240 trials. All 360 case/config/path/method measured outcome
groups are seed-invariant. Replication demonstrates deterministic reproducibility,
not independent statistical evidence. Held-out configurations are parameter/schedule
sensitivity with several factors varied together, not an independent external test
set or isolated estimation of each factor's effect. The specification discloses
fixture construction corrections and diagnostics added after previews; this is not
an untouched blind preregistration.

Universal behavioral inertness is contradicted by the verified unchanged E_Q case.
CurrentNeed can legitimately transmit retained support into allocation and material
consequences. Selective superiority, necessary action improvement after inquiry
change, and universal benefit of preserving strong evidence are unsupported and
contradicted by counterexamples. No significance, optimality, human likeness,
consciousness, natural-world prevalence or general planner claim is supported.

F cases remain ambiguous for inquiry mediation rather than being promoted to E_Q.
Readiness diagnostics are useful controlled evidence, not a claim of identifying
every counterfactual path in arbitrary schedules. The strongest unresolved question
is how often unchanged concern/status pathways matter under naturally arising
competing opportunities, and whether a better-founded local inquiry-value estimate
would preserve these effects without CurrentNeed's confidence-driven misallocation.
This review does not implement such a mechanism.

## Changes and validation

Changed files: this review, `examples/support/review010.py` (read-only reconstruction),
and `tests/experiment010.rs` (one unchanged-path full-chain regression). Original
results/specification/report/analysis/comparisons and all archived evidence remain
unchanged. No runtime source, score, heuristic, capacity, default or protected
001–009 contract changes. No new benchmark is necessary for documentation/validation
changes; original timing claims remain noisy whole-pipeline observations, not policy
CPU or resident-memory measurements. Review reconstruction is offline audit work.

Reproduction, with outputs restricted to target/stdout:

```text
cargo fmt --check
cargo clippy --locked --all-targets -j 1 -- -D warnings
cargo test --locked -j 1
cargo run --release --locked -j 1 --example validate_contract
cargo run --release --locked -j 1 --example validate009
cargo run --release --locked -j 1 --example evaluate010
python examples/support/review010.py
```

The evaluator performs 12,240 exact reruns and snapshot validations, writing only
target outputs. The review script matches all three archived raw JSON files exactly,
including their manifest hashes, and independently reconstructs 120 matched groups.
Do not run the archival writers/analyzer in place for this review.

Result: fmt, locked all-target Clippy with warnings denied, all 126 tests (seven
010 tests), both read-only foundation/009 gates, the full 010 evaluator and independent
review reconstruction passed. Both 010 full traces and the all-seed summary matched
the frozen raw JSON byte-for-byte. The 90 protected 001–009 hashes and all original
010 archive files remained unchanged. A broader before/after guard confirmed 139
evidence/runtime/foundation files unchanged. The sole tracked edit is the additional
010 regression; the review and read-only Python reconstruction are new files.
