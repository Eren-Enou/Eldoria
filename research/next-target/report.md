# Next investigation after Experiment 007

2026-10-07. Diagnosis baseline: `4c4b30f`. Recommendation: **test whether an
individual can maintain a coherent, bounded assessment when native evidence and
third-party testimony concern the same event**. This precedes uncertain attribution,
larger populations, or another general audit refactor. No Experiment 008 is implemented.

## 1. Current capability frontier

The constitution is the ten constraints in [AGENTS.md](../../AGENTS.md): causality,
locality, persistent consequences, individual perspective, resource consistency,
reproducibility, no fixed outcomes, traceability, replaceability and measured cost.
The review covered specifications/results for 001–007, their tests and compatibility
evidence, [architecture](../../docs/architecture.md), the [foundation findings](../../reinforcement/results.md),
and [audit guide](../../docs/history-audit.md). Current code, rather than older
architecture paragraphs describing superseded scans, determines the cost analysis.

| Established mechanism | What the evidence supports | Boundary |
| --- | --- | --- |
| 001 experience | Helpful/refusal histories change scores and later food choices at matched present conditions. | Trust and recent valence both contribute; no general personality inference. |
| 002 interpretation | Received scarcity information revises a retained refusal memory and its trust contribution. | One event-specific proposition; no general belief network. |
| 003 communication | Local preferences select disclosure, silence, requests and motivated false scarcity claims. | Speech follows refusal in a separate bounded phase; privacy harm is a score, not a simulated consequence. |
| 004 anticipation | Expected challenge/answer changes selection; public prediction errors change expectations. | One-step hand-scored consequences, not a model of another mind. |
| 005 unfinished intent | Important uncertainty survives an encounter; cost/history can cause reopening, partial resolution or abandonment. | Eight refusal concerns; opportunities are external. |
| 006 inquiry | Selected source/method outcomes train usefulness; repetition loses value; Pause need not resolve a concern. | Local source/strategy cells, supplied meetings and evidence opportunities. |
| 007 testimony | Known common origins suppress redundant corroboration; hidden origins overcount; later disclosure can reopen uncertainty. | Faithful relay, honest disclosed tokens, instrumented retrospective inspection. |
| Foundation | Exact old reports can be reconstructed from compact query references; derived indexes remove several global scans. | Complete history still grows; compact exports are not resumable saves. |

These are controlled demonstrations. Fixture thresholds and identical outcomes over
many seeds do not establish prevalence, calibrated confidence, psychological validity
or social emergence. The natural 1,000-agent 007 population has **zero provenance
roots/receipts**: it verifies compatibility, not a mixed information society.

An individual remembers bounded episodes, event beliefs, communication episodes,
unfinished concerns, source/method results and acquisitions. It can infer scarcity
support, reinterpret retained blame, update local credibility and predict a challenge
or answer. It can conceal or falsify its own scarcity claim, ask, disclose, relay
retained acquisitions and act through food-choice scores. It cannot derive a new
proposition from multiple premises, generalize a partner's motives across situations,
or seek an encounter by itself. Objective lineage remains outside its perspective.

## 2. Important missing behaviors and a demonstrated limitation

Concrete missing behaviors include: choosing to ask before refusing a trade; seeking
out someone who may know an answer; recalling the gist of an old injury after its
episode is gone; separating inability from unwillingness across several contexts;
tracking a promise or obligation; or reconsidering a belief about present conditions
when the world changes. Current scarcity propositions refer to a fixed historical
refusal. Generosity, caution, honesty and relationship goal are supplied dispositions
or profiles, not learned personal projects. Food is consumed but not produced, and
hunger does not autonomously rise. Longer unreplenished runs can therefore measure
resource exhaustion and repeated procedure rather than richer lives.

Several technically consistent rules are psychologically restrictive, without this
being proof that a particular human model is required:

- A late claim can escape the four-turn challenge window. Concerns permit later
  questioning only if an appropriate later encounter occurs; speakers do not predict
  that delayed challenge in the existing one-step model.
- Reaching absolute support 60 closes a question abruptly; evidence may be wrong.
  Strong negative support resolves the question while preserving blame.
- 005 abandonment is terminal even if later evidence changes uncertainty. In 006/007,
  source/method Pause replaces the older age/failure policy; the old abandonment
  result should not be described as a universal behavior of every opt-in mode.
- Known overlap sets prospective value to zero even if a better report from that
  same origin might help. Distinct tokens are assumed independent despite possible
  shared instrument bias; missing tokens default to speaker grouping.
- No real exploitation follows disclosure of vulnerability. Adding more elaborate
  privacy forecasts before modeling any observable consequence would mostly add scores.

### Mixed evidence is currently competing updates, not integrated reasoning

[The diagnostic](../../examples/diagnose_frontier.rs) uses the existing public APIs,
007 seed-42 refusal setup, voluntary inspection/sharing and an explicit native
fallible-reading intervention. No new policy or update rule is installed. Native
source 0 reports scarcity with weight 90. A separate inspector supplies an inverted
weight-40 report. Both are fallible structured inputs; the listener never gets the
fixture's truth or inversion flag. Results are in [mixed-evidence.json](mixed-evidence.json).

| Delivery sequence | Support after each receipt | Trust | Concern | Matched-present local policy probe |
| --- | --- | --- | --- | --- |
| Native, provenance, same native again | +90, −40, +90 | 0, −17, 0 | Resolved, Partial, Resolved | Offer, Leave, Offer |
| Provenance, native, same provenance again | −40, +90, −40 | −17, 0, −17 | Partial, Resolved, Partial | Leave, Offer, Leave |

At the second receipt each branch has received the same two content/quality items,
with no eviction. Their beliefs differ with arrival order. Repetition toggles the
assessment even though it adds no new observation. Probe food/hunger/generosity/caution
are held at 4/30/60/80; probes are pure local policy evaluations, **not executed
transfers**. Memory revisions and concern changes are actual resolver effects.

The source explains this precisely. `provenance_receipt` computes support from
retained provenance acquisitions and supplies `Some(evaluation.after)` to
`receive_information_support`. That override replaces the current belief. Native
fallible receipts instead use native positive/negative maxima. Neither combines the
two evidence bases. Repeated provenance can have local incremental value zero while
still replacing a native-supported belief. Ordinary inquiry can enter either reply
path depending on whether the original refuser has a retained acquisition, so this
is a composition risk in intended usage, not merely a reporting discrepancy.

This does not prescribe +50, a Bayesian posterior, or any uniquely correct result.
It establishes that the current output can be governed by channel dispatch rather
than the combined locally available evidence. Compatibility tests deliberately
preserved both paths; they do not establish coherent mixed-channel reasoning. More
uncertain attribution would confound that baseline problem with a new mechanism.

### Bounded cognition: useful mistakes versus capacity artifacts

Memory/beliefs/talk episodes are capped at 16; concerns at eight; inquiry cells and
signatures at 32, episodes/contacts at 16; provenance knowledge/hints/comparison keys
at 32. Source offers and capabilities are also bounded. Directed trust, credibility
and response-expectation maps are not globally bounded.

Existing tests demonstrate specific distortions: 32 repeated deliveries from one
provider evict another acquisition, and receiving the forgotten origin again is
classified as corroboration (`forgetting_is_fifo_and_can_make_an_old_lineage_novel_again`).
Repetition therefore consumes attention capacity even when its immediate information
value is zero. An unresolved concern can outlive its original episode; later strong
evidence resolves the concern but cannot revise the missing episode or its old trust
contribution (`capacity_eviction_is_explicit_and_concerns_outlive_episodes`). These
are real mechanistic discontinuities, not permission to recover forgotten knowledge
from the archive. Whether salience-sensitive retention is better needs a controlled
equal-capacity experiment. Raising all limits would hide rather than explain them.

## 3. Architectural risks and measurements

Reproduction: `cargo run --release --example diagnose_frontier`; five independent
`cargo bench --bench throughput` invocations are retained as `throughput-1.csv` through
`throughput-5.csv`. The diagnostic compiles the unchanged `src/history.rs` by path
for isolated index stress; it does not modify the production index. Timings are
release wall-clock observations on this Windows host, not confidence intervals or
heap measurements. Deterministic state/size data are separate JSON files.

**Directed relationship replay.** A deliberately synthetic zero-valence ledger forces
an early replacement to propagate without saturation. Timings distinguish the first
from last contribution at 1,000/10,000/100,000 entries; see
[diagnostic timings](diagnostic-timings.csv). The early update is linear, while the
last update is effectively constant apart from map lookup. However, the live resolver
only revises a retained resource episode. Since every owner event enters the 16-slot
FIFO, at most 16 of that owner's later contributions can lie in a live revision
suffix under current rules. A huge synthetic early revision is **not a reachable
normal 001–007 behavior**. Rebuild validation still processes full history. Do not
prioritize a segment tree for a stress case that the current cognitive rules exclude.

**Forecast lookup.** `observe_prediction` searches `forecasts.iter().find(...)` from
the beginning, usually for a recent turn. Actual repeated refusal conversations
produce 400/4,000/16,000 forecasts at 100/1,000/4,000 encounters. The measured tail
lookup and full-conversation curves are in the CSV. This is reachable O(history)
per feedback and potentially quadratic cumulative work, unlike the synthetic suffix
case. A future long-history experiment can justify a narrow talk-record→forecast
reference; no general planner or history framework is required.

**Audit growth.** At 100 agents and 3/30/100 encounters, compact export grows
980,096 → 8,601,939 → 27,720,148 bytes. Retained native inquiry inputs alone occupy
67,701 → 1,057,516 → 3,971,748 bytes across 300/3,000/10,000 inquiries. These are
necessary historical input versions in the current representation; their total
volume is not all removable duplication. Full clone/serialization timings are in
the CSV. The 100-encounter population has 5,414 resource events and still pays for
every inquiry audit, including Pause. Small agent count does not imply small audit.

**Checkpoint prefixes.** Across 007's seed-42 controlled trials, eight selected
append-only archive arrays occupy 883,247 bytes across all initial/checkpoint/final
snapshots versus 360,938 bytes in final snapshots alone: 522,309 bytes of repeated
prefix representation (including repeated array delimiters). The arrays cover events,
information, talk records, forecasts, native inquiries, roots, receipts and legacy
queries. [storage.json](storage.json) gives per-trial/per-array counts. This is a
lower-scope measurement, not a claimed percentage of the entire report. Mutable
cognitive checkpoints cannot simply be omitted; references to shared immutable
archives could eventually preserve them without repeating every prefix.

Selected timings from [the first complete diagnostic run](diagnostic-timings-1.csv)
are below; the replay run is retained separately as `diagnostic-timings.csv`.

| Operation | Small case | Larger case | Interpretation |
| --- | --- | --- | --- |
| 200 first-entry revisions | 0.403 ms / 1,000 entries | 32.786 ms / 100,000 | Synthetic full-suffix cost; not normal live memory revision. |
| 200 last-entry revisions | 0.0038 ms / 1,000 | 0.0098 ms / 100,000 | No long suffix. |
| 10,000 latest-forecast searches | 1.794 ms / 400 forecasts | 136.453 ms / 16,000 | Real lookup predicate on actual generated forecasts; microbenchmark excludes cloning. |
| Full repeated conversations | 2.262 ms / 100 encounters | 173.967 ms / 4,000 | All normal runtime work; does not isolate lookup alone. |
| Compact snapshot construction, 100 agents | 0.362 ms / 3 encounters | 21.364 ms / 100 | Entire observer copy. |
| Compact serialization, 100 agents | 2.033 ms / 3 encounters | 74.119 ms / 100 | Separate from construction and execution. |
| 1,000 Agent clones | 0.091 ms / 10 partners | 78.729 ms / 10,000 | Synthetic degree stress, same actual Agent type. |

**Other inspected costs.** Every resource step clones both `Agent`s and writes clones
back; `Agent` includes its whole directed trust map. Talk/inspection/exchange inputs
also capture own Agent. Degree therefore matters even with 16 episodes. The
`agent_clone_partners` diagnostic varies map size without inventing a social network.
World snapshots clone every agent and archive: O(agents + relationship entries +
retained audit payload), followed by serialization of the same payload. Calling
`events()` repeatedly in a diagnostic loop would itself clone history; the probes
avoid that in timed encounter loops.

Admission uses ordered busy/active sets; advance scales with active scenes, not all
completed scenes. New-conversation and follow-up cursors process only new ranges.
Contacts and candidate source scans are bounded by current meetings/collections.
Root/parent lookup is direct. Compact validation is observer work, and some local
attribution checks search receipt history; it should not be inserted into every
agent decision. Indexes and archives remain O(history) memory. No RSS or large-world
feasibility conclusion is supported.

### Revisit Experiment 001 regression

The foundation observed 153.341 → 179.360 ms for 100 seeds × 1,000 agents (+17%).
Five current runs instead range 200.137–305.533 ms (median 248.448); 100-agent runs
range 21.515–32.975 ms. Every record count remains identical. This large drift
without source changes means that **17% is not an isolated causal effect estimate**.
The diagnostic also replays the exact index append implementation on real 001
events separately from simulation, giving a direct maintenance-cost measurement.
At 1,000 agents × 100 seeds, replaying index insertion took 56.930 ms beside a
245.598 ms complete simulation run; at 100 agents the corresponding values were
5.794 and 29.332 ms. These are separate workloads: **do not subtract them to claim
an index-free runtime**. This is not an index-disabled A/B: cache state, allocation
and scheduling differ.

Index maintenance is a plausible, real tax on a mode that never revises beliefs.
Ordered-map insertion adds history-dependent lookup/allocation cost, but no new
full-history scan to each 001 step. For the next small behavioral experiment this
is an acceptable measured tradeoff; it is not evidence for an urgent architecture
rewrite. Before promising throughput or choosing lazy per-mode indexing, run an
interleaved index-enabled/disabled build comparison on an otherwise quiet host,
with unchanged event/state output. That causal attribution remains unmeasured here.

## 4. Candidate next investigations

### A. Coherent assessment across evidence channels — recommended

**Question:** Is coherent bounded evidence assessment sufficient to remove
channel-driven belief/concern/action reversals without suppressing genuine conflict?
**Why:** The current mixed-channel probe already changes an individual's behavior;
every later testimony mechanism would inherit this ambiguity.
**Smallest experiment:** one refusal, one native source, two locally informed
providers; matched content/quality, permuted arrival order, repeats, explicit shared
versus distinct origin and a later legitimate correction. Stay below capacity first,
then add one explicit eviction control. Compare old dispatch with a single local
assessment mechanism; keep costs, opportunities and resource state fixed.
**Required systems:** existing receipts, bounded local evidence summaries, attribution,
belief/memory/concern/inquiry pipeline and causal audit. Native source identities
need explicit semantics, not invented historical roots or observer graph lookup.
**Complexity:** medium; behavioral semantics and migration boundaries matter more
than storage. Keep old modes unchanged through an opt-in experiment.
**Constitutional risks:** recovering forgotten evidence, treating sources as independent
without local justification, changing old rules silently, or using truth to resolve
conflict. All new support must be reconstructible from the actor's retained inputs.
**Falsification:** with the same retained evidence and no intervening learning/eviction,
channel/order alone still changes final support or action; repeated evidence changes
support without a specified new reason; or apparent success requires hidden origins.
If a small explicit arbitration rule matches a more elaborate mechanism on all
distinguishing cases, reject the extra complexity.

### B. Attention and retention under repeated information

**Question:** Does salience-sensitive retention preserve useful distinctions better
than FIFO at exactly the same capacity, while still producing legitimate forgetting?
**Why:** Duplicate traffic can erase an unrelated consequential acquisition.
**Smallest experiment:** two unresolved events, repeated low-value receipts for one,
then a probe about the other; cross relevance/recency and include a condition where
old information really becomes less useful. **Systems:** bounded acquisition/concern
retention and eviction audit. **Complexity:** medium. **Risks:** secretly retaining
an unlimited summary, tuning retention to future truth, or disguising perfect recall.
**Falsification:** no advantage over FIFO on held-out relevance conditions at the
same byte/item budget, or benefit only when the rule knows future queries. Defer
until mixed-channel retention semantics are explicit.

### C. Uncertain or misleading attribution

**Question:** Can local contradictory lineage information revise confidence in an
origin claim without conflating speaker credibility and content evidence?
**Why:** 007 disclosed tokens are honest by construction; faithful relay cannot lie
about its origins. **Smallest experiment:** one actual root, two incompatible origin
claims, one later locally verifiable link, with truthful/false/unknown attribution
controls. **Systems:** attribution beliefs distinct from objective links and content
support, a motivated attribution action, bounded revision. **Complexity:** medium–high.
**Risks:** truth-oracle verification, fabricated roots entering policy, omniscient
fraud detection. **Falsification:** no correction without observer information, or
false and uncertain attribution are always treated identically despite accessible
discriminating evidence. This remains valuable, but current channel inconsistency
would make its outcome harder to interpret than A.

### D. Choosing when to seek information

**Question:** Can a retained concern make an individual initiate one costly inquiry
opportunity instead of waiting for an experiment-supplied encounter?
**Why:** Current intent has no agency over contact timing. **Smallest experiment:**
two publicly available partners and one time slot; choose ordinary interaction,
inquiry or neither, without geography. **Systems:** a bounded opportunity decision
using existing costs, concerns and known sources. **Complexity:** medium. **Risks:**
global expert search, knowledge of partner inventory, predetermined meetings or a
general planner. **Falsification:** concern-only ablation never changes selection,
or success depends on knowing which partner privately has evidence. Defer until
expected information value rests on coherent evidence outcomes.

### E. Longer historical execution and reporting

**Question:** Can existing behavior sustain longer personal histories without
forecast feedback or snapshot production dominating measured execution?
**Why:** Forecast scan growth is reachable, and 100-agent audits already reach tens
of MB. **Smallest experiment:** matched repeated conversations and bounded partner
rotation, separately timing feedback, clone/serialization and checkpoint export;
only then compare a narrow forecast reference or shared immutable report archive.
**Systems:** existing simulation and measurement harness; no behavioral feature.
**Complexity:** low–medium. **Risks:** deletion, hidden cognitive access, altered
ordering or opaque compression. **Falsification:** isolated measurements do not
show the suspected operation dominating at the intended history depth, or the
optimization changes any audit/decision. This is a targeted engineering follow-up,
not the most informative next cognition experiment.

## 5. Recommended target and falsifiable Experiment 008 question

Choose **A**. It distinguishes a concrete existing composition failure from an
intentional model of reconsideration. The evidence is already consequential for
trust, concern closure and a later local choice. B asks how to forget evidence;
C asks whether its origin can be doubted; D asks whether to pursue it; all are
harder to interpret when the current aggregate depends on the last channel used.
E can remove an identified cost, but the measured small experiments can already
run; it cannot answer the missing behavioral question. Prioritize depth and coherence
of one individual's history over population count.

**Proposed question:** *When direct evidence and third-party testimony concern the
same refusal, can a bounded local assessment mechanism make final beliefs, concern
status and decisions depend on retained content, reliability and legitimately known
dependence rather than delivery channel or repetition, while remaining uncertain
under conflict and revisable after genuinely new evidence?*

Pre-register two competing explanations: current reversals are an integration
artifact, versus an explicit recency/attention mechanism is needed. First use no
eviction, equal speaker credibility and fixed evidence quality, then separately vary
credibility, correction and capacity. Permute delivery order without intervening
decisions that change evidence weights; exact equality is the relevant null control.
Do not demand order invariance where delivery legitimately changes credibility or
what survives eviction. Include same-source repetition, known dependence, unknown
dependence, strong/weak conflict, false-but-consistent evidence and later correction.
Measure support, evidence basis, information value, concern transitions, memory/trust
and a matched local action; retain failed/unchanged cases. Success is not knowing
truth. A coherent incorrect belief is still possible with misleading local evidence.

## 6. Stop conditions

Do not build uncertain attribution, autonomous gossip, reputation networks, factions,
institutions, geography, a general planner, language dialogue, an Observer UI or a
world database now. Revisit attribution after mixed-channel controls are coherent;
contact choice after locally predicted value can be evaluated; broader social
mechanisms after repeated multi-person histories demonstrate a missing interaction
that pairwise mechanisms cannot explain. Revisit scale only with an explicit target
history depth, degree and latency/storage budget, plus measurements showing failure.

Do not optimize the synthetic long trust suffix before a legitimate behavior can
reach it. Do not raise memory caps or recover old acquisitions to pass cognition
tests. Do not delete causal history or expose observer indexes to policies. Revisit
report sharing only when export is a material workload; preserve mutable checkpoint
state and immutable record accessibility. Stop Experiment 008's proposed scope once
the competing explanations above can be distinguished; extra social features would
reduce interpretability. This report itself makes no behavioral implementation.

## 7. Evidence and verification boundary

The added Rust example is diagnostic tooling only. It calls existing APIs and compiles
the exact index source for synthetic measurement. Production `src/`, old tests,
benchmarks and historical experiment outputs are unchanged. The new evidence lives
only here. Five benchmark repetitions expose timing drift rather than hiding it.
The existing full test suite and Clippy are rerun; deterministic diagnostic outputs
are replay-checked separately from wall-clock CSVs. See [tests](tests.txt) and the
[verification record](verification.json) alongside the final measurements.

This is a source-and-experiment diagnosis, not external psychology research. Claims
about plausibility identify modeling limits; none assert a calibrated human theory.
Unmeasured: production heap growth, index-disabled causal timing, broad rotating
social networks, held-out psychological validity and population-scale feasibility.
