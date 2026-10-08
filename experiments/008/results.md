# Experiment 008 — Coherent bounded assessment

Completed 2026-10-08. Rules: `experiment-008-v1`. The experiment completes the
preliminary opt-in implementation in the repository. Earlier experiment reports
and defaults remain intact. The motivating diagnosis is
[the completed frontier investigation](../../research/next-target/report.md);
the hypothesis and completion controls were recorded in [specification.md](specification.md).

## Finding and mechanism

One bounded local basis now supplies belief support for both native information
and provenance testimony. With unchanged retained evidence, received quality and
credibility, the tested delivery orders and repetitions give the same final
support, concern status, replacement trust and matched-present action. Prefixes
can differ: an individual acts on what has arrived so far. This is coherence of
local assessment, not a truth guarantee or a statistical independence claim.

The basis retains at most 32 receipt items per individual, FIFO, with frozen
received quality and public origin. Ordinary native claims, disclosures and
readings retain their existing local source identities; explicitly attributed
acquisitions use their delivered observation token. Unknown attribution retains
the existing speaker assumption. No policy receives objective roots, relay depth,
sensor outcome, seed or a global graph. Enabling is prospective: old receipts are
not imported from history. Zero-quality evidence cannot displace claim fallback.

Two replaceable rules were compared. **Max** subtracts the strongest opposing
support from the strongest favorable support. **Grouped**, the selected rule,
first collapses each locally known origin to its strongest evidence on each side,
then combines distinct groups using the established integer residual rule:
`side += (100 - side) * quality / 100`; support is positive minus negative.
Unsupported native claims are fallback when no positive-quality evidence is
retained. Ordered origin iteration makes rounding reproducible. Max solves the
channel overwrite but cannot distinguish equal-quality corroboration from relay;
Grouped preserves that distinction without a new probabilistic framework.

The bridge continues existing belief revisions, memory replacement, saturated
trust replay and concern updates. Original interpretations remain immutable.
Audit records retain before/after support, attribution replacements, evictions
and retained receipt references. Their replay validates history; it never restores
forgotten knowledge into live assessment. Existing memory, belief, concern,
provenance and inquiry caps were not increased.

## Distinguishing results

Seed 42 and seeds 0–127 give these controlled outcomes. Support is on the existing
integer scale −100 to +100. Actions below are the pure scarcity-policy probe with
matched current food, hunger, generosity and caution; probes do not transfer food.
The receipts, revisions, concern updates and time costs are actual simulation work.

| Control | Legacy | Max | Grouped |
|---|---|---|---|
| Native +90, testimony −40, repeat native | +90, −40, +90 | +90, +50, +50 | +90, +50, +50 |
| Testimony −40, native +90, repeat testimony | −40, +90, −40 | −40, +50, +50 | −40, +50, +50 |
| Equal opposing evidence, either order | Channel-dependent prefix/final | 0 | 0 |
| Independent native/provenance favorable readings, 50 each | +50 | +50 | +75 |
| Known relay of one favorable reading | +50 | +50 | +50 |
| Hidden relay assumed distinct locally | +75 | +50 | +75 |
| Attribution subsequently reveals shared origin | +75 → +50 | +50 → +50 | +75 → +50 |
| Two independent native/provenance misleading unfavorable readings | −50 | −50 | −75 |
| Unfavorable 40, favorable native 90, new independent favorable 50 | Channel-dependent | −40, +50, +50 | −40, +50, +55 |

In the mixed controls both shared rules finish at +50, Partial concern and Leave,
instead of the legacy Offer/Leave alternation. Independent +75 resolves the concern
and yields Offer, whereas known relay +50 remains Partial and Leave. Hidden relay
can therefore still produce overconfidence; later explicit attribution replaces
only retained matching local assumptions and reverses the result. This is a real
local-information difference, not an order-invariance violation.

The same already acquired report was additionally delivered as provenance/native,
native/provenance, provenance/provenance and native/native. Both rules retain one
public origin and support +50 after each delivery. The narrow native acquisition
adapter uses the same voluntary source policy, retained acquisition, immutable
parent chain, quality and accounting. It neither samples again nor lets callers
assign origins. Anonymous native readings cannot be matched to hidden roots.

Repeated unsupported claims cannot displace received evidence. The initial native
claim strength varies with seeded expectation (30–42); subsequent unfavorable
evidence fixes support at −40 in both orders and on repetition. Repetition does
not increase support while the relevant basis remains retained.

## Corrections, forgetting and surprises

Coherence preserves fallibility: two genuinely separate misleading readings give
−75, enough to resolve a concern while maintaining blame and Leave. A legitimate
later favorable reading in the correction control raises mixed support only from
+50 to +55; it leaves the concern Partial. We did not change thresholds or discard
the contrary evidence to manufacture closure. The voluntary inquiry control
likewise finishes at +55 after a real new-source response.

FIFO flooding deliberately evicts the strong native receipt: +90 → +50 → −40.
A fresh native acquisition then gives +50 again. These are different retained
bases, so invariance is neither expected nor claimed. After the original injury
episode has been evicted, a later +90 belief may resolve the concern, but cannot
recreate the episode or revise its old trust contribution; the action remains Leave.

The first completion preview failed because eviction-scene churn legitimately
received native evidence, after which the fixture tried to overwrite its immutable
source slot. The revised control uses a recorded privacy intervention allowing
voluntary silence during churn. The resolver's immutability checks remain intact.

An intentional limit remains: Experiment 007 novelty/inquiry usefulness learning
is unchanged. A new receipt can have a novelty value different from its change in
joint assessed support (the inquiry control adds only five support points).
Credibility remains separate from origin grouping and changes only through its
existing legitimate comparisons. The primary invariance controls freeze received
weights; no claim is made across real credibility changes or forgetting.

## Validation and artifacts

`cargo run --release --locked -j 1 --example evaluate008` completed 56 variants
per seed over 128 seeds: **7,168 controlled trials, each replayed exactly**.
There are 16 legacy variants and 20 for each new rule; the four acquisition-adapter
controls require the opt-in mode. Population runs at 100 and 1,000 also replayed
exactly. The compatibility evaluator reproduces all seven archived seed-42 JSON
values exactly, without rewriting any earlier evidence.

All **106 tests** pass, including 13 new tests covering order/repetition, adapters,
known/hidden origins, correction, FIFO forgetting, prospective enable, invalid
receipts, policy locality, immutable causal links, voluntary inquiry, native-only
population compatibility and source decisions. Snapshot and derived history-index
validation pass. Formatting, Clippy with warnings denied and throughput benchmark
pass. Single-job Cargo avoids the previously documented Windows parallel-build
paging limitation.

- [seed-42.json](seed-42.json): complete deterministic snapshots and assessment audit.
- [report.txt](report.txt): human-readable receipt sequences, origins, revisions,
  support, evictions, concern/action probes and acquisition links.
- [summary.json](summary.json): 128-seed outcome counts and storage metrics.
- [compatibility.json](compatibility.json): all 001–007 comparisons true.
- `population-{100,1000}-{Legacy,Max,Grouped}.json`: deterministic population audit.
- [population-benchmark.csv](population-benchmark.csv) and
  [throughput.csv](throughput.csv): wall-clock measurements, separate from JSON.
- [test-output.txt](test-output.txt): test transcript.

## Performance and storage

Three population samples use the same seed 42. The evaluator's build/run/snapshot
measurement includes construction, execution, history-index validation and snapshot
validation; it is not an isolated execution benchmark. Serialization is separate.
The byte counts below use compact serialization; archived population files are
pretty-printed and therefore larger.

| Population | Legacy median ms | Max median ms | Grouped median ms | Grouped serialization median ms | Legacy / Grouped JSON bytes |
|---|---:|---:|---:|---:|---:|
| 100 | 2.314 | 2.002 | 2.067 | 1.485 | 980,120 / 981,068 |
| 1,000 | 23.243 | 23.329 | 23.547 | 17.892 | 8,930,572 / 8,949,773 |

The natural population workloads produce only 2 and 43 assessment records and no
manufactured relay opportunities. These measurements therefore do not establish
high-volume corroboration performance. The throughput CSV adds an Experiment 008
three-encounter workload alongside 007, with the same base-record denominator;
extra assessment records are not counted as extra throughput. Short timings vary.

In that single throughput run, 100 repetitions at population 100 took 163.287 ms
for 007 and 167.507 ms for 008 (2.6% higher); population 1,000 took 1,961.026 ms
and 1,896.585 ms (3.3% lower). Both count 82,328 and 821,305 base records,
respectively. The opposite timing directions do not establish a speed improvement
or a stable overhead estimate; the CSV preserves initialization separately.

Live assessment is bounded to 32 items per individual and an update groups at most
32 items (`O(32 log 32)`). This adds local storage; it does not remove existing
unbounded observer archives. Assessment audit grows by one record per accepted
receipt with up to 32 references, and exports include this history. The seed-42
suite JSON is 4,955,663 bytes. No heap profile or long-horizon scaling claim is made.

## Scope and remaining questions

This result supports coherent assessment for retained, equally weighted local
information and distinguishes known corroboration from known relay. It does not
prove independence from origin IDs, establish consciousness or broad emergence,
or promise stable action after memory/trust history differs. Non-injured recipients
still retain acquisitions without gaining a newly invented injury belief. The
tests deliberately use fixed source qualities and matched current states; they
are mechanism controls, not prevalence estimates.

Unresolved questions are how novelty estimates should relate to joint support,
how frozen received weights should behave after legitimate credibility changes,
and how bounded forgetting affects longer histories. They remain questions here;
no subsequent experiment, attribution model or unrelated world expansion was
designed or implemented.
