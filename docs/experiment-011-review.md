# Experiment 011 scientific review

**CONFIRM WITH QUALIFICATION.** StatusValue creates a genuine actor-local allocation
distinction, but the tested limited-slot outcomes do not justify adopting it. The
strongest failure is an expected-novelty/retained-assessment mismatch amplified by
leverage saturation, cheaper Direct inquiries and stable ties. No implementation bug
was found. There are useful partial updates absent from the controls, and failures
of source availability affect every mode. The result is narrower than “StatusValue
adds no useful behavior” or “an unchanged concern means no learning occurred.”

Reviewed 2026-10-09 against `86f4fee`; unrelated navigation commit `55536d9` landed
during review and was preserved. This review changes no simulation behavior,
formula, capacity, default, protected 001–010 contract or original 011 evidence.
It does not freeze 011 into the foundation or begin 012.

## Question and independent evidence

The specification asks: “can limited inquiry be allocated using own importance,
expected answer usefulness and retained state, without giving uncertainty intrinsic
value or knowing future relevance?” The reviewed claim is that StatusValue adds
a locally grounded distinction but frequently chooses cheap inquiries that fail to
change retained assessment or substantive concern state enough to justify adoption.

Reviewed AGENTS, the constitution, protected contract/audit, 011 specification,
results, report, analysis/comparisons, archive scripts/manifests, evaluator and nine
tests; 010's scientific review; relevant 006–009 findings; and the actual selection,
response, learning, assessment and fixture implementations. Historical evidence is
the original [011 archive](../experiments/011/results.md), not new replacement trials.

New [read-only reconstruction](../examples/support/review011.py) imports no runtime,
evaluator or prior analyzer scoring function. It reconstructs all 240 unique seed42
trials (48 controlled, 192 held-out), including both controls. It recomputes
FIFO prefixes, ordered Grouped support, all treatment candidate sets/gains/costs/
scores/ties, local projection timing, actual native receipts, 006 novelty and learning
for every recorded inquiry including preparation and later automatic inquiries,
attempted offers, time, concern counters/transitions, actual balances and consumption.
It matches full first actor-local inputs, pre-treatment inquiry history and deliveries
across modes, and checks the paired future. It reads support for resolved concerns
from retained items rather than treating disappearance from the active projection as
support becoming zero. Its output lists every treatment step and pair classification.
The full traces contain2,316 post-enable query captures and3,336 inquiry records
including preparation, other participants and subsequent automatic inquiries.

The existing archive-only reconstruction was also run, separately. Current execution
replays all 4,320 trials over 18 seeds exactly; the two full seed42 outputs and entire
all-seed summary match frozen raw bytes. Deterministic replication is reproducibility,
not independent statistical evidence. Independent arithmetic/outcome reconstruction
supports the decision; identical snapshots alone would not establish it.

## Scoring and decisions

006 supplies candidates and learned source × method expected usefulness, public
quality/effort offers, attempts, costs and Pause. 007 applies only legitimately known
overlap. 008 supplies Grouped support; FIFO/32 from 009 is fixed. CurrentNeed calls
010's unchanged function. StatusValue changes only the final benefit calculation:

```text
g = existing expected_gain
d = max(1, 60 - abs(current retained support))
leverage = min(100, g * 100 / d)
value = (g + leverage) / 2
benefit = importance * value / 100
score = benefit - existing cost
```

All divisions truncate. Unchanged uses `importance*g/100-cost`; CurrentNeed discounts
that already-truncated benefit by `(100-abs(support))/100` before subtracting cost.
Strict greater-than preserves the first candidate on ties, with Pause0 first,
concern order, source order, Direct then Evidence. Source lists are sorted by stable
IDs; reversing their fixture listing does not reverse ties. Integer magnitudes are
small and reconstructed exactly. There is no overflow, wrong divisor, changed
candidate legality, unexpected tie implementation or response-before-selection.

For `same_uncertainty/core`, A is concern0/event1, importance98, retained support20.
First inquiry0 has these local alternatives, before any response exists:

| Mode | A/source1/Direct | A/source1/Evidence | Choice |
|---|---|---|---|
| Unchanged | g55; benefit53; cost13; score40 | g90; benefit88; cost31; score57 | Evidence |
| CurrentNeed | benefit42; cost13; score29 | benefit70; cost31; score39 | Evidence |
| StatusValue | d40; leverage100; value77; benefit75; cost13; score62 | d40; leverage100; value95; benefit93; cost31; score62 | Direct, stable tie |

The tie is consequential, but its preference is the documented rule, not a bug.
Saturation compresses the gain distinction while preserving the 18-point method
cost difference. Fractional rounding contributes to the exact tie; it does not
explain the entire pattern of wrong-source inquiries and suppressed claims.

In `near_resolution/core`, after seven shared Unchanged preparation meetings,
StatusValue's A Direct g13/d5 scores22 versus A Evidence g32 scoring10. In far,
A support5 instead makes B/source1/Evidence score9 beat A Evidence8. That source
cannot answer B; learning lowers its expected Evidence value32→16. At the next
opportunity A Evidence8 wins and obtains a real partial update. Both controls Pause
in near and far. This verifies a near/far distinction without assuming it is useful
in every case or attributing its shared learned estimates to StatusValue.

## Strongest demonstrated failure and persistent consequence

`same_uncertainty/core` isolates the full executed chain:

1. Actual refusals events1/3/5 create three concerns. Native0/1/2 give support20 each.
   All modes start with identical agents, food, trust, memory and first local inputs.
2. Unchanged/CurrentNeed choose A/source1/Evidence. Native3 is a legitimate reading90;
   Grouped A becomes `20 + floor(80*90/100) = 92`, resolving A and revising its retained
   refusal's relationship contribution. StatusValue chooses tied Direct instead.
3. StatusValue also receives Native3, but it is a Claim of received quality50.
   Retained reading20 suppresses claim fallback. Support remains20/Partial, while
   006 novelty is Stronger50 and Direct expectation becomes `(55+50)/2 = 52`.
   Positive novelty thus understates this method's ineffectiveness for Grouped state.
4. Next actor inquiry4: the controls choose B/source2/Evidence and resolve B.
   StatusValue chooses A/source2/Direct using the fresh source's55 prior. Source2
   has no A acquisition or native self-knowledge: no information arrives; its
   expectation becomes27. Learning changes allocation but cannot refund the slot.
5. Before inquiries all are ready to Leave with future partner1 (trust−17, recent
   valence−17, Offer score−29). Immediately afterward the controls have Offer score5,
   trust/valence0; StatusValue still chooses Leave. No separate intervening delivery
   is responsible for the opening difference.
6. Actual event6/7 executes Offer/Accept under both controls: food4/4→3/5. StatusValue
   event6 executes Leave: food4/4 stays4/4. Subsequent consumption leaves actor/partner1
   food2/5 versus3/4. Later automatic inquiry cannot undo that material history.

This is E_Q, a changed-inquiry route to persistent executed consequences. It is a
missed controlled cooperative opportunity with donor cost and recipient benefit;
it is not a proof that transferring food is globally preferable to keeping it.

## Whole-suite findings and benefits

Counts below are unique seed42 fixtures; changed means immediate selected-concern
support **or status** changes. Silence and attempts still count against the budget.

| Suite/mode | Questions / Pause | Direct / Evidence | Changed / no change | Newly resolved | Accounted time |
|---|---|---|---|---|---|
| Controlled Unchanged |28 /4|6 /22|21 /7|20|760|
| Controlled CurrentNeed |28 /4|11 /17|17 /11|15|667|
| Controlled StatusValue |32 /0|29 /3|3 /29|1|452|
| Held-out Unchanged |129 /15|101 /28|20 /109|16|2136|
| Held-out CurrentNeed |130 /14|108 /22|16 /114|12|2042|
| Held-out StatusValue |144 /0|129 /15|4 /140|1|2036|

These are not population frequencies or a global utility score. Resolution can be
wrong. The larger held-out failure counts affect every mode, especially absent or
unknowing sources; they cannot all be blamed on StatusValue's formula.

| StatusValue no-change inquiries | Controlled | Held-out |
|---|---|---|
| Direct with no support/status change |28 of29 Direct|126 of129 Direct|
| Received Claim suppressed by retained non-Claim evidence |11|34|
| No receipt acquired |18|106|
| Positive 006 novelty despite no support/status change |10|31|
| Selected leverage saturates at100, across all questions |26 of32|126 of144|
| Positive top-score ties, across all questions |25 of32|98 of144|

The received-suppressed and no-receipt rows partition the 29/140 no-change questions.
All 32/144 StatusValue questions update a learning record **and its expected value**.
They also update attempt/failure metadata; received claims occupy new receipt slots.
“Unproductive” in the original results is therefore a narrow operational label,
not absence of cognition, information, learning or every concern-state change.
Negative information about an unhelpful source may be useful later; these short
fixtures do not establish that later benefit. Cheap inquiries explain a dominant
failure path but are neither inherently worthless nor the sole cause.

Strongest positive distinctions:

- `far_resolution/core`: only StatusValue obtains A reading20 on its second inquiry,
  support5→24, after its first wrong-source query. Both controls Pause. A remains
  Partial, execution is Leave and no material divergence occurs. This is genuine
  added local information at a cost (61 ticks for two questions versus2 for two Pauses).
- `same_importance/core`: StatusValue's first inexpensive A Direct reply raises
  support0→**50**, remaining Partial, at13 ticks. Both controls instead resolve B/C
  with readings90 at62 total ticks, leaving A untouched. StatusValue's second query
  obtains nothing. All Leave. Its cheaper partial gain is a real tradeoff without
  a specified utility that ranks A versus B/C. This A gain also occurs in H1/H2/H3.
- `new_opportunity/core` and H2: StatusValue notices a new public channel, receives
  A90 and resolves it; controls do too. It is legitimate recovery of opportunity,
  not uniquely added value, remembered truth or observer recovery.

Two prose numbers in the original results are incorrect: `same_importance` says
0→25, whereas its trace is0→50; `partial_answers` says first A Evidence65→42,
whereas Stronger30 gives65→47. The later B New20 gives65→42. These are reporting
errors, not defects in scoring or simulation. The archives remain unchanged and
these corrections are recorded here. “No useful improvement beyond controls” must
also allow the far partial gain and the unranked same-importance tradeoff.

## Comparisons, negative controls and ambiguity

Independent pair counts reproduce the original scientific classifications:

| Suite/pair | Equivalent measured behavior | C: different inquiry, same execution | E_Q | E_A / F |
|---|---|---|---|---|
| Controlled Unchanged/CurrentNeed |6|7|3|0 /0|
| Controlled Unchanged/StatusValue |2|10|4|0 /0|
| Controlled CurrentNeed/StatusValue |5|10|1|0 /0|
| Held-out Unchanged/CurrentNeed |34|30|0|0 /0|
| Held-out Unchanged/StatusValue |45|18|1|0 /0|
| Held-out CurrentNeed/StatusValue |22|41|1|0 /0|

Pairs overlap and must not be pooled as independent effects. Equivalent concerns
measured query/action/food outcomes, not identical scores or complete cognition.
E_A and ambiguous F remain distinct possibilities; neither is observed in 011.
The 010 evidence for those classes remains protected and separate.

Unchanged/StatusValue controlled E_Q cases are uncertain_minor, learned_poor,
resolved and same_uncertainty; held-out is same_uncertainty/H2. All transfer differences
favor the controls' realization of a cooperative opportunity, not a universal utility.
CurrentNeed/StatusValue E_Q is same_uncertainty in core/H2. Retained-support changes
often stop at cognition: ten controlled C comparisons for each StatusValue pair.

CurrentNeed improves B/C coverage over Unchanged in poor_answerability/expensive,
and in partial_answers gets A10→28 then B20→36 while Unchanged's second source cannot
answer A and StatusValue's chosen sources cannot answer C. Both concerns remain Partial;
all resource actions match. Conversely Unchanged obtains a later transfer that
CurrentNeed misses in uncertain_minor. No old control is universally sufficient.

All three choose identically in equal_tie, including the wasted second query. All
miss the minor B's later opportunity in minor_later. Wrong-confidence A−85 is already
Resolved and legally absent from every candidate set: an important failure created
by earlier mistaken evidence, not treatment resolution. Silent replies lower learned
expectations without providing evidence. These failures were not removed.

Uncertain_minor/mirrored_future preserves the complete inquiry phase while changing
the later partner. Core Unchanged's transfer advantage becomes uniformly Leave;
every held-out pair uniformly Leaves. **011 does not demonstrate a reversed material
winner in these paired futures.** 010 contains such reversals, but cannot supply a
missing 011 result. This pairing demonstrates future ignorance and contingency of
the measured advantage, not StatusValue superiority in another future.

## Causal explanation and experimental limits

The strongest explanation is a mismatch of units/semantics: `expected_gain` estimates
006 signature novelty, not expected Grouped support change. StatusValue treats it
as threshold leverage without considering that Claim fallback is suppressed by an
existing reading. Default Direct55 and many retained supports saturate leverage,
so Direct approaches Evidence benefit at lower cost. Stable ties favor earlier
Direct/source IDs. Positive novelty can maintain that expectation even when the
support change is zero; fresh-source priors can keep an unanswered concern attractive.
Response availability and learned expectations then explain repeated wrong-source
queries. None is an arithmetic implementation error.

This is a heuristic failure under the documented assessment semantics, rather than
a requirement to change Direct, Evidence or the old learning rule. Direct can help
when no evidence is retained, as same_importance demonstrates. Evidence also fails
when the chosen source cannot answer. Near/core's retained55 plus a separate reading20
could reach64; StatusValue chooses Direct instead. The60 threshold makes small
updates sometimes action-insensitive; neither every support change nor every reply
should be promoted to executed success.

Limits of the experiment, distinct from those heuristic mechanisms:

- Sixteen designed families reuse the same three refusal/resource scaffold, and
  some families overlap. Seven/nine preparation meetings deliberately depress
  expectations; preparation is matched but excluded from the limited treatment
  budget. Source schedules and partial-answer coverage were corrected after a
  controlled preview. This is not an untouched blind preregistration.
- H0–H3 couple importance, initial support, advertised quality/effort, source subset,
  ordering, slots and later partner/stakes. They test parameter/schedule sensitivity,
  not isolated factor effects or external transfer. Near/far and partial have disclosed
  source/quality overrides. Reversed listings preserve sorted ties; reversed deliveries
  can change legitimate prefixes. Declared95 quality is capped at90.
- AccurateFixture fixes favorable response content; reliability variation is received
  quality, not sampled response error. Wrong-confidence retains initially inverted
  evidence. Naturally unreliable sources, hidden-provenance competition and long-run
  recovery are not independently tested here.
- Original 006 priors do not encode topic expertise; unknowing sources remain legal
  candidates. This construction makes all modes vulnerable, particularly H0. It
  does not script the selected target or change opportunities by mode, but limits
  how widely the failure frequency can be generalized.
- The seed42 traces retain at most six assessment items and have no assessment
  eviction. The mechanism obeys its32-item bound, but 011 does not independently
  test allocation during capacity pressure or forgetting. Those findings belong
  to earlier experiments, not this negative-value result.
- The scorer receives no remaining-slot budget and is myopic. Two/three slots can
  punish exploration whose learned benefit would arrive later. Short-horizon support/
  status metrics omit that potential benefit, credible-source learning, effort savings
  and the unranked importance of partial knowledge. No global utility or quantitative
  adoption/complexity threshold was preregistered; the specification's qualitative
  distinction/failure criteria do not uniquely determine an adoption decision.

Consequently the evidence supports withholding adoption under these limited-slot
criteria. It does not establish general dominance, a formal cost-benefit optimum,
or that every locally reasonable cheaper choice was irrational.

## Locality, legality and complexity

Source inspection confirms selection receives only the existing own 006/007 input
plus at most eight own active concern/support triples from live at-most32 Grouped
items. Public co-presence and voluntary advertised offers are legitimate. There is
no future partner/action/relevance, sampled answer, objective truth, hidden provenance,
scenario label, seed, observer record or partner-private component in that input.
Responder inputs contain their own state/self-knowledge only after selection.
The matched first inputs and paired future/private-inventory regression corroborate
these boundaries; they are not exhaustive proofs for every possible fixture.

Runtime projection reads live items, not assessment audit records. Existing own
beliefs, concern counters, source-method cells and trust have separate established
lifetimes; using those current retained effects is not archive recovery. Observer
prefix replay here validates past inputs and never feeds a live actor. 009 retention
is fixed FIFO, assessment/concern bounds stay32/8, and prior inquiry bounds remain.

Valid sequential replies obey original native-self-knowledge or voluntary exchange
rules. Pause costs1; Ask costs1 plus Direct10/Evidence28 plus actual response work
and inspection effort. Candidate hunger costs are selection heuristics, not a second
ledger charge. Reconstruction checks receipt work, effort, attempted-offer learning,
no inquiry food transfer, exact donor/receiver balances and later consumption.
External opportunities match until voluntary divergence; endogenous responses/time
may then diverge, with no reset. Initial circumstances/capabilities are declared
interventions, not natural resource production or autonomous opportunity emergence.

No new persistent cognition is added. The prototype adds a bounded projection/scoring
rule and unbounded observer references/checkpoints. Original serialized archives total
117,745,636 raw bytes, compressed to3,561,272. Original seven-sample clone-plus-score
medians are663.50/773.42/808.92ns for Unchanged/CurrentNeed/StatusValue; StatusValue is
about4.6% above CurrentNeed in that specific 19-candidate/three-summary micro-measure,
which includes cloning. Whole-trial timings mix different actions, validation and
serialization; they do not show isolated CPU cost or resident memory. No material
runtime cost claim motivates the scientific decision. This review adds offline
verification only; no runtime throughput optimization or new behavioral benchmark
is needed. The evaluator's repeat timing output stays outside deterministic evidence.

## Classification and conclusions for a later integrity pass

**Demonstrated:** matched local allocation differences; exact scores/ties; cheap
Direct predominance; suppressed-Claim/novelty mismatch; partial information benefits;
failed/no-response learning; wrong-confidence exclusion; genuine resource events and
persistent E_Q consequences; unchanged bytes and deterministic replay.

**Mechanism-dependent:** saturation and near-threshold bonus, strict order on ties,
claim fallback suppression, priors/learning migration to fresh unknowing sources,
and support60 status/action sensitivity. These explain this heuristic and these
established mechanisms, not a universal theory of curiosity.

**Ambiguous:** global value of partial support versus multiple resolutions, value
of negative learning beyond the tested horizon, independent effects of each held-out
factor, the adoption tradeoff without a utility/complexity threshold, and natural-world
prevalence. No claim of reversed material advantage in 011 is supported.

**Unsupported:** optimality, calibrated information value, general intelligence,
human curiosity/psychological realism, consciousness, broad emergence, statistical
significance from seed replication, or performance in naturally arising social worlds.

Precise conclusions worth protecting later: StatusValue remains opt-in and unadopted;
its formula/ties/controls remain fixed; the same_uncertainty E_Q negative chain and
0→50 /5→24 partial benefits coexist; positive novelty or a reply is insufficient
evidence of support/status productivity; no-change questions can still train cells;
equivalence, C, E_Q, E_A and F must remain distinct; missed opportunities are not
undone by later cognition; mirrored advantage disappearance is not reversal. Protect
all original negative and positive evidence and the reporting corrections together.
This review does not itself extend a protected contract.

Strongest remaining question: using only bounded local information, how much of an
inquiry's eventual usefulness depends on changing the retained assessment, and how
much on learning which source/method will answer later? The present short-horizon
fixtures reveal a mismatch but do not determine that tradeoff or a replacement rule.

## Verification, integrity and changed files

Passed formatting, locked all-target Clippy with warnings denied, all135 Rust tests
(including nine011 tests), integrated read-only validate_contract and standalone
validate009/validate010. The old gates retain129-seed009 and34-seed010 replay, causal
validation and frozen-byte checks. Existing011 archive reconstruction passed.

The existing release evaluator ran from isolated `target/review011`, writing only
its nested `target/experiment011` output. All4,320 complete-state trials reran exactly.
The new reconstruction compared all three deterministic files byte-for-byte with
the frozen raw archives. No archival writer was run. All106 protected hashes and all
14 original files under011, including prose, scripts and timing evidence, remained
unchanged. The initial262-file guard detected four documentation changes belonging
to the unrelated navigation commit, with all other baseline files unchanged. A
second guard at that commit checks all264 tracked files, allowing only this review's
two navigation-row edits. No runtime/evidence/contract change is hidden by that exception.
An offline audit hook rejected file writes and target-cache reads while archive-only
reconstruction passed; four scratch corruptions (score, learning, support and food
balance) were rejected. These checks change no archived trial or executed outcome.

Reproduce (normal Python, not `-O`):

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -j 1 -- -D warnings
cargo test --locked -j 1
cargo run --release --locked -j 1 --example validate_contract
cargo run --release --locked -j 1 --example validate009
cargo run --release --locked -j 1 --example validate010
cargo build --release --locked -j 1 --example evaluate011
New-Item -ItemType Directory -Force target/review011 | Out-Null
Push-Location target/review011
& ../../target/release/examples/evaluate011.exe
Pop-Location
python -B examples/support/analyze011.py --archive-only
python -B examples/support/review011.py --replay-dir target/review011/target/experiment011
```

Changed files: this review, `examples/support/review011.py`, and review links/qualified
011 summaries in `docs/architecture-map.md` and `docs/simulation/EXPERIMENT_HISTORY.md`.
Existing regressions
already protect the strongest tie/executed-negative chain, partial reprioritization,
locality, prospective controls and corrupted audit rejection; no runtime or Rust test
changes are necessary. New offline assertions additionally protect the factual positive
and negative trace anchors. No simulation defect or failing required validation remains.
Original011 prose errors are documented here, without rewriting it.
