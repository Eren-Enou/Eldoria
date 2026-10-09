# Experiment 012: contextual learning self-evaluation

Scientific decision: **ACCEPT WITH QUALIFICATION**. A separately learned estimate of
observed assessment change produces later voluntary decisions beyond unchanged
novelty learning. A small observable context boundary changes cross-concern transfer.
Some changes reach executed persistent resource consequences. This establishes a
narrow capability, not a reason to adopt this heuristic globally. 012 remains
exploratory pending separate scientific review; the protected foundation stays 001–011.

Question: can individuals learn from their own inquiry attempts, distinguishing
concern information from opportunity learning, and use context-sensitive expectations
without privileged information? The answer is yes in these controlled fixtures, with
important failures and limited evidence for the particular contextual rule.

## Existing capabilities and design choice

The [pre-implementation specification](specification.md) distinguishes existing 006
concern/source/method novelty cells from importance, credibility, trust, retained
Grouped support and concern status. Old learning already revises expectations after
silence and repeated information; it is not absent self-evaluation. The tested gap
is transfer of an assessment-change estimate across concerns with similar observable
assessment context. 010/011 score projections do not supply that learned distinction.
Their rules, defaults and accepted interpretations remain unchanged.

Considered: existing learning alone; merely exposing its expectation error; separate
contextual estimates; and a hybrid. The chosen hybrid keeps old novelty learning
unchanged and discounts its gain using bounded observed-change experience. Explicit
error alone would rename the old moving average. A planner or hidden competence
model was unnecessary and rejected. CurrentNeed/StatusValue are not extra controls
here: they answer a different scoring question and do not isolate learned context.

Three modes use the same world and fixed external schedule: exact Unchanged selection
with observation/update capture; GlobalProgress ignoring context; Contextual using
EvidencePresent versus ClaimFallback. Controls are behavioral-use and context ablations,
not different response mechanisms. All start with empty new state. Every first decision
matches; independent reconstruction checks physical/cognitive prefixes up to the first
different voluntary selection. No divergent world is reset.

New live state is at most 32 cells per actor: source, method, context, attempts,
expected change and last-inquiry reference. No event content or autobiographical
summary is retained in a cell. FIFO insertion eviction is audited; updates do not
refresh insertion order. A valid selected reply trains `(old + target)/2`, with
target100 for changed signed support or selected status, otherwise0. Priors55/65
reuse old method values. Invalid replies/Pause do not train. This is a binary-change
heuristic, not a calibrated probability or magnitude forecast.

With a matching cell, candidate gain is `min(old gain, max(expected change, existing
untried public opportunity value))`; importance and costs remain old rules. Without
experience, scores are identical to baseline. Known dependence cannot be bypassed.
Strict greater-than preserves existing candidate order and Pause ties. Policies see
only bounded own state, current retained context and public opportunities. The update
uses the current resolved local result; receipt acquisition comes from that result,
not an observer-history search. The rule was never retuned after outcomes.

## Scope and overall results

15 designed families × three modes give45 controlled trials. Three fixed coupled
parameter/schedule configurations give135 sensitivity trials. Seeds0–7,42,u64 maximum
yield1,800 rows; every full trial is rerun exactly and causally validated. All180
unique families/configurations have seed-invariant query/action/inventory signatures
in this suite. Those repetitions prove reproducibility, not independent statistical
evidence. Quality/content uses accurate/inverted fixtures, not natural source noise.

Counts are actor inquiries during declared training plus limited slots. Changes mean
immediate support OR status change, including misleading changes. Automatic inquiries
after resource execution remain in full-history metrics and are separately labeled;
they are not silently added to scheduled-question denominators.

| Suite / mode | Ask / Pause | Changed / questions | Positive novelty without change | No receipt | Inquiry ticks |
|---|---:|---:|---:|---:|---:|
| Controlled Unchanged |60 /0|8 /60|11|29|1057|
| Controlled GlobalProgress |54 /6|8 /54|11|25|1025|
| Controlled Contextual |60 /0|10 /60|11|30|1146|
| Sensitivity Unchanged |254 /1|27 /254|31|151|4558|
| Sensitivity GlobalProgress |178 /77|12 /178|32|87|3438|
| Sensitivity Contextual |242 /13|30 /242|31|148|4563|

No final selected reply is invalid. The new rule does not eliminate unproductive
questions. GlobalProgress reduces effort and can also suppress useful channels;
Contextual preserves more distinct opportunities, with more effort. No post-hoc
global utility ranks these tradeoffs or equates a transfer with universal improvement.

Training is real cost: controlled training ticks are471/633/633 for
Unchanged/GlobalProgress/Contextual; limited-stage ticks586/392/513. Sensitivity
training2619/2770/2749 and limited1939/668/1814. These exclude other participants'
decisions, announcement and meeting overhead from the actor-query subtotal; full
history retains those costs. There is no free preparation budget concealed as policy
benefit.

| Paired comparison | Equivalent | C: different inquiry, same material outcome | D: different inquiry and persistent material outcome |
|---|---:|---:|---:|
| Controlled Unchanged / GlobalProgress |2|11|2|
| Controlled Unchanged / Contextual |6|8|1|
| Controlled GlobalProgress / Contextual |5|9|1|
| Sensitivity Unchanged / GlobalProgress |1|42|2|
| Sensitivity Unchanged / Contextual |9|36|0|
| Sensitivity GlobalProgress / Contextual |1|42|2|

Pairs overlap. Equivalence is measured query/action/food equivalence, not complete
cognitive identity. B (bounded expectation change) is independently reconstructed;
C is actual later selection, not a score-only probe. D additionally requires executed
events, legal resource accounting and subsequent consumption. No material-only
ambiguous class appears under this operational test. It does not prove universal
mediation for other worlds or quantify every intermediate contribution.

## Strong positive chain and context distinction

`changed_circumstances/core`: actual refusal events1/3/5 establish concerns A/B/C.
Native A20 is retained. At inquiry0 all modes ask A/source1/Direct. Native Claim50
has Stronger novelty50 but does not alter Grouped20 because a reading suppresses
claim fallback. Existing expectation55→52; the separate observed-change estimate55→27.
The observed error is0−55, without declaring the source dishonest or ignorant.

At inquiry2 Unchanged repeats Direct; Contextual and GlobalProgress choose Evidence
because the learned Direct discount is lower. A legitimate new channel1 reading50
changes A20→60, resolves it and raises the Evidence change estimate65→82. The fixed
schedule then makes source1 unwilling, regardless of mode. Unchanged has missed that
earlier opportunity; its later Evidence gets silence. Contextual can instead pursue
B via source2, receiving50 while B stays Partial. Event6/7 executes Offer/Accept for
the two progress modes, versus event6 Leave for Unchanged. Balances4/4 become3/5
versus4/4; later consumption preserves actor/partner1 food2/5 versus3/4. Automatic
later inquiry cannot refund the spent slot or undo material history.

This demonstrates B→C→D for the additional progress signal. **It does not isolate D
as a benefit of context separation:** GlobalProgress also succeeds. Donor cost and
recipient benefit remain distinct.

`context_specific/core` isolates context: after matched A experience, both new modes
have Direct27 and Evidence32. At inquiry4 Contextual leaves those EvidencePresent
estimates separate from B's ClaimFallback and asks B/source1/Direct, legitimately
acquiring50 and learning55→77. GlobalProgress applies its poor A experience across
B and asks A again, then Pauses. Unchanged reaches B only at inquiry6. All Leave.
The context boundary changes the timing and allocation of actual information, but
does not establish a unique executed gain over Unchanged in this case.

## Negative, mistaken and unchanged cases

- `later_informed/core`: source4 first lacks A and silently fails. After a legitimate
  later inspection, GlobalProgress's different allocation happens to revisit A and
  obtains A20→60 at inquiry4, revising Direct27→63 and executing a transfer.
  Contextual and Unchanged spend their limited slots on B, which the provider still
  cannot answer; both Leave. Old poor experience/context separation can miss changed
  circumstances. The actor never sees the acquisition before choosing.
- That apparent advantage reverses in the support40/quality80/four-training variant:
  GlobalProgress Pauses after earlier failures and misses the new acquisition;
  Unchanged/Contextual obtain A88 and transfer two units, persisting actor/partner1
  food1/6 versus3/4. This is real negative adaptation, not a removed outlier.
- `misleading`: the same context-sensitive choice can acquire B−50, or B−80 and
  mistaken resolution in the high-quality variant. The change target rewards it
  as a change. Contextual learns ClaimFallback Direct55→77 despite incorrect content.
  No truth-aware success label repairs the mistake. GlobalProgress may miss both the
  useful and the misleading acquisition. Partial-information counts cannot rank truth.
- `novel_no_progress` and `cost`: the extra estimate changes a later method but the
  response still gives no support/status progress; all Leave. With costly evidence,
  cheap unknowing sources remain attractive. No learned source estimate implies
  universal competence, and no response implies useful concern learning.
- `partial`: legitimate A10→55 occurs earlier for the new modes; it remains unresolved
  and all Leave. Weak new information is real even without resolution or action.
- `repetition`: the repeated Direct receipt does not increase Grouped support.
  New modes obtain A60 earlier, but baseline also resolves before the resource scene;
  all transfer. Earlier information alone is not final trajectory superiority.
- `conflict`: Claim novelty30 leaves retained positive20/negative20 support0. All
  modes make the same scheduled choices. Recognizing conflict does not force certainty.
- `changing_source`: provider4 genuinely acquires useful A, then loses it when36
  legitimate relay receipts overflow its32-acquisition FIFO store. All modes share
  core choices; A was already resolved there, so its disappearance does not create
  an active A retest. Lower-quality cases keep A unresolved but may still investigate
  other concerns. This validates the changed provider circumstance and memory limit;
  it does not establish robust detection/recovery of lost competence.
- `ignorance` versus `withholding`: provider4 respectively lacks an acquisition or
  retains one but declines to share. Actor-local decisions, learned outcomes and
  silence match. The individual cannot tell which hidden cause occurred. Existing
  channels have no explicit verbal “I do not know” message; honest inability is not
  automatically labeled dishonesty, and the experiment adds no fictional motive fact.
- `no_information`: with empty new cells, all initial candidates/choices match.
  After real silent encounters GlobalProgress generalizes failure and eventually
  Pauses; Contextual/Unchanged continue testing other concerns. No informative
  contextual distinction is conjured before experience. This family intentionally
  overlaps the silent control and is not a new independent sample.
- `forgetting`:36 legitimate unrelated receipts evict A's assessment basis. At the
  next opportunity A support is0 and context ClaimFallback; separately learned
  EvidencePresent estimates survive without storing A's claim. A new Direct receipt
  can give0→50 even with Redundant novelty0; it is fresh delivery, not archive recovery.
  Baseline receives a reading20 instead. All Leave. Cell-capacity testing separately
  proves an evicted estimate returns to its prior rather than replaying history.
- `mirrored_future`: inquiry phase matches `context_specific` exactly while the later
  partner changes. In the support40 variant, Unchanged/Contextual's B80 enables an
  executed transfer to partner2 that GlobalProgress misses. Their corresponding
  future with partner1 leaves all modes withdrawing. The fixed future changes whether
  the knowledge matters; it does not select the inquiry.

## Independent verification and corrections

`examples/support/review012.py` is a separately implemented read-only reconstruction,
not an independent human scientific review. It reuses reviewed Python Grouped,
integer, old-score and native-novelty primitives, and independently implements 012
context, cells, FIFO update and discount. It reconstructs local provenance prefixes,
native/relay receipts, current retained bases, all candidate order/scores/ties,
learning, time, physical matched prefixes, resource selection and conservation.
It does not call Rust treatment scoring or assume the archive's interpretation.
Corrupted context, update and outcome self-tests are rejected.

Three fixture repairs are disclosed in the specification: contradictory capability
overwrite was rejected; the ambiguity source was made genuinely informed before
withholding; and the default fallback sensor was aligned with already received
channel quality. The last defect was discovered by independent verification after
the first1,800-row execution. The pre-repair summary is preserved in
`development-summary.json.gz`; it is not evidence of ordinary source failure.
No negative outcome was tuned away and no scoring/update/capacity rule changed.
The final sensitivity suite is therefore not a blind untouched holdout.

A reporting repair also separated automatic post-resource inquiry updates from
scheduled-question denominators. Both sets remain in `analysis.json`; the table
above uses scheduled outcomes consistently. Compile-time adapter/test-field repairs
did not change the hypothesis. No protected implementation defect was found or fixed.

## Complexity and performance

Existing memory/belief/concern/evidence/old-estimate capacities remain unchanged.
New cells cap at32, with no new cognitive content summary. Captured query records
reference existing inquiry IDs and bounded context triples; they do not duplicate
full state every query. Observer history still grows, including outcomes and evictions.
Point snapshots are experiment exports, not a resumable-memory channel.

Across all final rows, actor cell occupancy is2–7; all new live-cell maps serialize
to209–795 bytes. New observer records serialize to3,498–7,513 bytes per trial; total
trial snapshots117,361–450,048 bytes. Serialized maximum-occupancy microbenchmark
lists are3,247 Global /3,631 Contextual bytes. These are JSON sizes, not resident RAM;
unit tests exercise32-cell eviction, while ordinary fixtures do not reach that cap.

Five repeats of50,000 evaluations on the same19-candidate input give median cost
including the same decision clone: Unchanged768 ns (empty) /761 ns (32 supplied
cells), Global821 /865 ns, Contextual817 /958 ns. The32-cell Contextual range is945–968 ns.
This isolates the scorer on a fixed input, not projection, learning, ECS dispatch or
the full simulation. It shows a modest measured cost for this bounded workload,
not general scalability. See `policy-performance.csv`.

Whole-trial construction/export/causal-validation medians are503–522 μs controlled,
578–616 μs sensitivity, depending on mode. The trials diverge in behavior and history,
so these timings are not isolated algorithm-speed comparisons. `performance.csv`
keeps wall-clock measurements out of deterministic JSON. The unchanged throughput
benchmark is separately recorded; no general optimization was performed.

Compressed final data total approximately1.55 MB for30.89 MB of full seed42 state
and5.64 MB of ten-seed summaries; the retained pre-repair summary adds0.11 MB.
The manifest protects exact compressed/raw lengths and SHA-256. Historical001–011
outputs,120 protected hashes, contracts and scientific reviews remain unchanged.

## Interpretation and remaining question

Demonstrated: bounded local updates distinct from novelty; context-dependent transfer;
real changed inquiry, useful/weak/misleading receipts, persistent executed divergence,
observational ambiguity and explicit forgetting without observer recovery.
Mechanism-dependent: binary change target, two-context boundary, priors, moving average,
conservative discount, offer renewal and stable ties. Neither progress nor cost is
calibrated inquiry utility.
Ambiguous: longer-term usefulness of failures, optimal exploration under changed
sources, topic expertise beyond this assessment boundary, and whether the extra state
earns its complexity in naturally arising encounters. Aggregate resolution/inventory
does not resolve these tradeoffs.
This study does not prove that32 extra cells are the uniquely smallest solution.
A transient context-aware scorer using only currently retained items, or inference
from still-retained episodes, might reproduce some effects without new cells; neither
is an evaluated control here. Such current-state inference would also need to avoid
mistaking present marginal contribution for historical progress after later evidence
or eviction. New cells are justified for the tested prospective cross-concern
history-transfer hypothesis, not as a proven minimal architecture for every task.
Unsupported: consciousness, human introspection, general intelligence, truthful
source classification, optimal inquiry, global winner, natural-world prevalence,
statistical significance or broad emergence.

H1/H2 have narrow positive support; H3/H4 failures remain real. The “merely observer
explanation” and “all behavior equals baseline” explanations are falsified here.
Privileged-information explanations are excluded at the tested input boundary,
not across hypothetical future implementations. Broad adoption is not justified.

Strongest remaining question: how can a bounded individual revise an appropriately
learned poor expectation when circumstances genuinely change, without either a
hidden competence oracle or unlimited exploration? Context separation sometimes
preserves that opportunity and sometimes delays it. This experiment stops there;
it does not invent a replacement planner or begin013.

## Reproduction and validation

```text
cargo run --release --locked -j 1 --example evaluate012 -- --output target/experiment012-final
python -B examples/support/review012.py --self-test --replay-dir target/experiment012-final
cargo run --release --locked -j 1 --example validate012
cargo run --release --locked -j 1 --example measure012
```

The evaluator reruns every full trial; standalone012 compares both seed42 archives
and the complete1,800-row summary byte-for-byte, then independently reconstructs180
trials without generated caches or writes. It deliberately stays outside the001–011
foundation gate. The archive writer is authorized012 tooling only; use the retained
development source as its explicit prerequisite rather than erasing that evidence.

Final formatting, Clippy, full tests, all foundation/009/010/011/012 gates and throughput
verification all passed at the final source revision (baseline `5b9a5c6` plus this
task's changes). Formatting/Clippy/full tests were rerun after the runtime receipt
flag boundary was hardened; all144 tests passed, with no remaining failures.
The final typed `Experience` boundary excludes partner-private response inputs as
well as observer-history access; the same checks/replays were rerun after that repair.

| Required verification | Exit / evidence |
|---|---|
| `cargo fmt --check` |0|
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` |0|
| `cargo test --locked -j 1` |0;144 tests|
| `cargo run --release --locked -j 1 --example validate_contract` |0; entire protected001–011 foundation|
| Same command with `validate009`, `validate010`, `validate011` |0 each; unchanged archive/replay paths|
| Same command with `validate012` |0;1,800 exact full reruns, raw archive/summary equality and180 independent reconstructions|
| `review012.py --self-test --replay-dir target/experiment012-final` |0; all archive/raw checks, corruption rejection and1,800-row/ten-seed summary audit|
| `cargo bench --locked -j 1 --bench throughput` |0; old-mode workload, separate wall-clock output|
| 012 evaluator / fixed-input measurement |0 each; no outcome retuning|

All120 protected SHA-256 values match. A complete baseline tracked-file guard also
confirms only the ten intended integration/navigation files changed among old files;
all older contracts, scientific reviews, tests and archived outputs remain byte-identical.
Verbose logs are in `target/experiment012-session/`; benchmark output is also archived
as `throughput.txt`. No historical writer was invoked. Standalone012 is read-only and
requires Python standard-library gzip verification, with no prepared target cache.

Files added: new policy/state, simulation adapter and012 fixture; evaluator, separate
gate, measurement and independent reader; tests; specification/results/report/analysis,
comparisons, deterministic archives/manifests and performance evidence; task checkpoint.
Integration modifies module declarations, two guarded provenance hooks and exclusion
of combined010/011/012 projections. Navigation entries are updated. No old archive,
contract, accepted review or default is changed; no foundation extension to012 or013.
