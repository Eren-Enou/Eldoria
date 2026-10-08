# Experiment 007 — provenance-aware testimony and independent corroboration

Executed 2026-10-07 on the established Rust + Bevy ECS headless simulation,
Windows x64 / Rust 1.99.0, rules `experiment-007-v1`.

**Within the implemented local mechanism, individuals distinguish independent
acquisitions from known relays. Hidden attribution still causes overconfidence,
and later disclosure can reverse that confidence and its memory consequences.**
This is a controlled capability demonstration using explicit observation tokens,
not a claim of statistical independence, ideal inference or broad social intelligence.

## Reproduce and evidence

```powershell
cargo run --locked --release --example evaluate007
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo bench --locked --bench throughput
```

The evaluator writes only this directory. Evidence includes the
[pre-implementation hypothesis](specification.md), [full seed-42 trials](seed-42.json),
[readable causal report](report.txt), [128-seed summary](summary.json),
[1,000-agent audit](population.json), [001–006 compatibility](compatibility.json),
[baseline benchmark](baseline-benchmark.csv), [final benchmark](benchmark.csv),
and [test output](test-output.txt). Wall-clock results are separate from deterministic
simulation JSON. No prior evidence files were regenerated or overwritten.

## Research question and architecture

Can another speaker add useful evidence, or merely carry evidence already received?
Experiment 006 could explore another source but did not give that source a legitimate
way to acquire event knowledge. The extension adds three replaceable pure policies
for inspection, exchange and provenance-aware inquiry, behind an opt-in ECS resource.
It preserves the cohesive Agent component, single-threaded resolver, stable IDs,
food accounting, original event history, earlier local policies and old output schemas.
See the [architecture](../../docs/architecture.md) for representations and equations.

The three layers stay distinct:

| Layer | Contents | Available to an investigator's policy? |
| --- | --- | --- |
| Objective causal history | Observation root, actual observer, sensor, relay parent, real depth | No |
| Local knowledge | Received content, communicator, public message handle, disclosed token or missing attribution | Own bounded records only |
| Local interpretation | Believed lineage, independent/shared/conflict classification, support, incremental value and expected usefulness | Yes |

An objective receipt preserves the actual root even when its listener receives no
token. A source policy sees its own acquired testimony; it cannot inspect the
listener's knowledge. Inquiry sees only own concerns/history and public co-presence,
credibility and legitimately received hints. The resolver consults objective origins
only after selection for validation and audit, never to correct hidden-lineage mistakes.

## Observation, relay and attribution

Observation is an explicit **retrospective instrumented inspection**, not a witness
memory inserted after the event. The original refuser voluntarily opens a narrow
record of its own historical scarcity predicate. Up to seven distinct inspectors
choose whether to use it before the resolver samples. Decisions see own Agent,
Profile, public event, quality and effort; the sensor's seed/channel/outcome stay
private. Separate inspectors get separate immutable slots and source-keyed readings.
Inspecting again cannot resample or rewrite a slot, even after forgetting it.

Opening and inspection score `goal - privacy - hunger/10 - effort`; score above zero
selects participation. Sharing score is `goal - privacy - hunger/10 - request_cost/4`.
Share also requires a retained acquisition; Silence is always possible. The harness
supplies co-presence, inspection opportunities, sensor/profile interventions and
attribution availability. It supplies no sampled claims or forced policy choices.

One exchange copies the provider's retained acquisition. Objective parent links
prove where it came from. The listener gets either the provider's known observation
token or an explicitly unattributed account. A provider cannot discover a hidden
token from the objective archive. The cap of three transmission hops permits two
intermediate relays and prevents unrestricted recursive propagation.

`provenance_offer` is a voluntary metadata announcement: quality and optional token,
with no proposition. It neither counts as evidence nor trains realized usefulness.
Exchange records distinguish metadata offers from actual sharing. Public message
handles let a later attributed message replace the assumption attached to the same
provider acquisition; it cannot arbitrarily rewrite unrelated reports.

## Interpretation, credibility and inquiry

Known tokens define local evidence groups. Missing tokens use speaker identity as
an assumption. Within each group use maximum positive/negative quality; across
groups accumulate each side as `total += (100-total)*quality/100`, then subtract
negative support from positive support. This bounded integer heuristic discounts
repetition while allowing corroboration and uncertainty under conflict. It can be
wrong. Quantization and stable token order are implementation assumptions, not
probabilistic calibration.

Incremental value: first information at most 40; corroboration at most 30;
independent conflict at most 30; shared lineage at most 10 of actual support change
(zero in the matched relay cases); attribution revelation at most 40 of support
change. A new speaker does not imply new evidence. Transmitted quality is acquired
quality discounted by negative directed credibility; positive credibility cannot
increase it beyond the acquired observation. Credibility never changes lineage.

Strong received evidence with explicitly different known tokens can compare earlier
claims: consistency adds `20*quality/100`, conflict subtracts `30*quality/100`, within
the established [-40,40] bounds. These comparisons use fallible local evidence,
not objective truth. A bounded comparison key prevents repeated proof earning
another contribution while retained.

Inquiry keeps Experiment 006 concern importance, Direct/Evidence costs, priors,
source/strategy learning and Pause. Known overlap multiplies expected gain by zero;
otherwise the factor is 100%, preserving the exploratory prior rather than asserting
that a source certainly knows something. Credibility still enters the base expected
gain separately. Selected valid responses update `(old + realized value)/2`.
Unfulfilled replies lose value; merely announcing a token creates no recurring
opportunity floor. This conservative overlap rule can undervalue a stronger report
from the same lineage; the current trials do not establish an optimal tradeoff.

Locally computed support passes through the existing bounded belief, memory revision,
concern and ordered replacement-contribution trust replay. Original memories in
objective events remain unchanged. The native bridge's speaker identifies the
original refusal subject for trust replay; the new provenance receipt separately
retains the actual communicator. Other listeners acquire testimony without receiving
another individual's episode or concern. Query cells and episodes reference actual
native Inquiry record IDs; separate provenance queries link their factors/receipts.

The native 006 response channel remains active for the original refuser when it is
not relaying a new acquisition. Native testimony is not retroactively given invented
observation roots. Confidence aggregation over mixtures of older native evidence
and new structured acquisitions is not established by this experiment: the new
provenance aggregate concerns its retained acquisitions, while ordinary native
responses keep their earlier belief rules. This is a prospective mechanism rather
than a migration or general evidence-fusion system.

## Controlled scenarios and representative traces

Investigator=0, original refuser=1, A=2, B=3, third source=4 unless specified.
The regular scarcity policy produces the original refusal with requester food0 /
hunger90 and refuser food1 / hunger90 / privacy100. Current agents then have food4 /
hunger30 / privacy20. Concern importance is 78. Inspections use quality50, effort2,
accurate fixtures unless specified. These are recorded experimental interventions.

| Case | Legitimate acquisition / receipt | Investigator result |
| --- | --- | --- |
| A Independent | A and B separately inspect; two actual roots | A support50/value40; B support75/value30; subjective resolution |
| B Known relay | A inspects; A→B→investigator, who also heard A | New speaker B, same known root; support50/value0; unresolved |
| C Three speakers | A→B→source4; all three communicate the same token | Support50 throughout; final two reports value0 |
| D Hidden relay | Same causal inspection/transmissions as B; B's outgoing attribution unavailable | One real root, two believed lineages; support75/value30; overconfidence |
| E Later disclosure | Hidden case, then B voluntarily identifies the earlier message's token | Support75→50, revelation value25; memory/trust revision reverses; concern reopens |
| F Independent conflict | A accurate50, B independently inverted50 | Two roots retained; support50→0; conflict value30; uncertainty remains |
| G Credibility vs independence | Credible2 relays source4; lower-credibility3 independently inspects | Known relay expected gain0; independent source gain53; inquiry selects3 |
| H Alternate independent | A's methods depleted; new B is publicly present, privately has a separate quality40 inspection | Inquiry shifts2→3; value30; support40→64; resolved |
| H Alternate relay | Identical investigator decision input, but B privately acquired A's account | Same source switch; realized value0; support40; unresolved |
| H Alternate empty | Identical investigator decision input; B never inspected or received an account | Same source switch; voluntary Silence; value0; unresolved |
| Independent wrong | Two separate inverted inspections | Two real roots; support -50→-75; wrong subjective resolution |

For B/D, the archived resource events, roots and causal receipt tuples are identical;
only availability of B's public attribution and its subjective consequences change.
This isolates provenance knowledge from objective history. Root0 is not secretly
looked up in D: B's delivered knowledge has `known=None`, while the observer-only
receipt still links root0 through B's parent receipt. E preserves that original
hidden receipt; a new disclosure changes the live knowledge and appends a new
revision. Seed42 trust returns to its pre-information level after the reversal.

G obtains credibility legitimately on an earlier refusal: source2 reports an accurate
quality50 inspection, source3 an inverted50 inspection, and source4 supplies separate
quality80 evidence. Local comparison yields source2 +16 and source3 -24. On a new
actual refusal, source2 relays source4's already received root; source3 independently
inspects and voluntarily advertises that token. Both are locally present. At importance
78, Direct costs13: the known relay has benefit0/score-13; source3 has gain53,
benefit41/score28 and delivers discounted quality38, raising support50→69.
One earlier exploratory query to the original refuser remains in the trace;
it was not removed to make the final example cleaner.

H uses seven real opportunities with A under temporarily provenance-disabled
valuation to isolate learned depletion even when overlap is known. Its selected
queries are Direct, Evidence, Direct, then four Pauses. Both method cells have
decreased expectations (13 and32). Provenance valuation is restored before B
arrives. No B metadata is supplied before selection, and all three investigator
inputs are exactly equal despite different B acquisitions. Direct to B scores29;
both known-overlap A candidates score below zero. B's independent/relay/silent
response then changes realized usefulness. This is exploration under uncertainty,
not an investigator knowing B's expertise in advance.

## Ablations

With provenance disabled, B's known relay becomes a separate speaker group:
support50→75 and value30 instead of support50/value0. This duplicates D's mistaken
evaluation despite retaining the token in the stored message.

Disabling speaker identity **for evidence grouping** preserves support75 for two
explicit observation tokens and support50 for the known shared token. Communicator
IDs remain in audit and directed relationships; this ablation does not erase all
identity from the world. The result demonstrates that explicit provenance can
distinguish origins without using speaker count as its independence rule.

Known vs unavailable attribution holds the actual causal world fixed. Independent
vs relay cases hold proposition and quality50 fixed, replacing the second causal
inspection with a real A→B receipt. Their incremental values differ by30. These
comparisons test mechanisms, not population effect sizes.

## Reproducibility, compatibility and population

Fourteen variants were evaluated over seeds 0–127: 1,792 trials plus exact replays.
Every classified support/value/action sequence was invariant across this controlled
range. Generated expectations and detailed memories still vary. Identical outcomes
under intentionally matched intervention state do not estimate prevalence or social
emergence. Byte-stable JSON uses ordered maps, sorted participants and fixed ticks.

All Experiment 001–006 archived seed42 reports compare structurally equal to fresh
runs, including their original defaults and metrics. Old files are unchanged.
The natural 1,000-agent check uses the existing three disjoint-pair encounter rounds,
consumption after each and no replenishment. It exactly preserves Experiment 006
resource events, final agents and native cognition: 43 native information receipts,
3,000 follow decisions and 165 unresolved concerns. No inspection, structured relay,
shared-lineage detection, corroboration, hidden relay or independent conflict occurs;
new provenance occupancy/evictions and source switches are zero. These zeros describe
the scheduler's lack of those opportunities, not a failure rate. No rumor scheduler
or synthetic population gossip statistic was introduced.

## Storage, performance and complexity

Knowledge, hints and comparison keys each cap at32 per individual; previous belief16,
memory16, concern8, inquiry episode16/cell32/signature32/contact16 bounds remain.
Controlled capacity tests create 40 actual refusals and inspections and verify
deterministic eviction in all three new collections. An evicted provider cannot
recover an old acquisition from the root archive. Another test forgets A's token
after 32 B reports (a metadata hint alone does not count as retained evidence);
hearing A again is classified as corroboration and support rises
50→75. The auditor retains the older receipt unchanged. This demonstrates bounded
cognition, not a safeguard that keeps agents permanently correct.

The compact seed42 population audit is 11,952,361 bytes versus 8,468,468 for006,
an increase of41.14%. The new resource alone occupies 3,483,413 serialized bytes
despite empty live provenance state, because query inputs/decisions are fully audited.
Pretty population JSON occupies28,534,720 bytes; full seed42 controlled traces occupy
3,976,194 bytes. Compact bytes are measured
from serialized snapshots, not a heap-memory estimate. Audit duplication is a measured
cost and a future streaming/reference opportunity, not optimized in this experiment.

Timing uses the existing release benchmark: 100 seeds each at populations100 and1000,
initialization separately from three-encounter simulation/consumption, with final
audit clones/serialization outside the timed run. Both006 and007 rows are measured
in the same final invocation. No inspection/relay workload is inferred from those
ordinary pair timings. Final paired timings:

| Population × 100 seeds | 006 init ms | 007 init ms | 006 run ms | 007 run ms | Run increase |
| --- | ---: | ---: | ---: | ---: | ---: |
| 100 | 18.678 | 21.431 | 176.327 | 226.660 | 28.55% |
| 1,000 | 86.930 | 101.932 | 2,195.467 | 2,892.500 | 31.75% |

Record counts are equal within each pair: 82,328 and821,305 respectively. The extra
cost includes local projection, factor evaluation and duplicated full query audit
records. The final invocation ran after tests finished; the pre-change baseline is
also archived separately. These are single aggregate timings, without confidence
intervals, heap profiling or long-history workloads. Earlier 006 timings differ
substantially across invocations, so this is a paired overhead observation, not a
stable throughput prediction.

Local interpretation is bounded O(32 log32); inquiry scans bounded local collections
and currently present sources. Credibility comparisons are at most O(32²). Root and
parent resolution are direct vector lookups with no recursive lineage traversal.
Checking immutable inspection slots scans the growing root archive. Native evidence
searches, memory-revision replacement collection and saturated trust replay continue
to scale with history length. Objective roots/receipts/windows/exchanges/query traces,
eviction logs, original audit and earlier sparse relationships remain unbounded.
Long histories can dominate runtime; no million-agent or persistent-world scaling
claim follows from these short checks.

## Verification and unexpected behavior

Formatting, warning-denied Clippy across all targets, the full83-test suite, release
evaluation and benchmark pass. Twelve new integration tests cover independent/relay/
hidden/revealed/conflicting evidence; three speakers; credibility; source locality;
private sensor/provenance isolation; voluntary sampling/sharing/silence; immutable
slots; invalid attribution/input/provenance; depth limits; FIFO capacities/forgetting;
repeated proof; full seeded replay; stable audit/cognitive/query references; resource
and time accounting; archived006 and native population compatibility. Earlier tests
continue to cover original saturated trust replay and all older rule sets.

Review caught two integration problems before final evidence: a source without a new
acquisition initially silenced the original refuser's native response, and learning
cells initially used a separate query-ID namespace. Native replies were restored and
queries now append actual Inquiry records/episodes with explicit provenance links.
Both have regression coverage. Compilation/Clippy issues found during development
were corrected; final checks have no failures.

Additional observed effects were retained: independent conflict can outweigh a
stronger reading enough to keep the concern unresolved; G's credibility-conditioning
event remains uncertain even after quality80 evidence because the opposite quality50
reading is retained. A method can return briefly to Direct before pausing. Hidden
relay can close a concern, later attribution can reopen it, and two separately
inverted inspections can close it incorrectly. Repeated receipts consume capacity
and time even when their information value is zero. The heuristic was not tuned to
remove these outcomes.

## Demonstrated, heuristic and unsupported

**Demonstrated:** bounded legitimate acquisition and relay, local recognition of
shared provenance, useful corroboration, hidden-lineage overconfidence, later
confidence/memory/concern revision, independently retained conflict, separate
credibility and independence, and useful or failed alternative-source exploration.
Old modes replay exactly; structured knowledge and observer-only origins remain
auditable and distinct.

**Heuristic:** residual support accumulation, speaker-based unknown-lineage
assumptions, novelty caps, zero/100 overlap factor, quality discounts, credibility
increments, voluntary utility scores and the existing support60 closure threshold.
Distinct sensor acquisitions do not establish statistical independence. Inspectors
share an instrumented historical record; correlated sensor bias remains possible.

**Unsupported:** natural witness discovery, probabilistically calibrated attribution,
forged observation tokens, lying about origins in the new relay channel, recursive
gossip networks, rumor prevalence, global reputation, historical consensus,
institutional knowledge, culture, broad social intelligence, general legacy evidence
fusion, persistence services or large-world scaling. Old intentional deception still
exists, but new structured relay is a faithful-copy mechanism with an honest available
attribution channel. Metadata does not prove that the next reply will be delivered.
Evicted memories are not restored from observer history. No consciousness claim is made.

## Single recommendation for Experiment 008

**Uncertain and potentially misleading source attribution.** Preserve this bounded
relay mechanism and test whether a listener can revise confidence in an attribution
claim using legitimately obtained contradictory lineage evidence, while keeping
speaker credibility, content credibility and confidence in provenance distinct.
007 makes honest disclosed tokens reliable by construction; removing that assumption
would most directly test whether its apparent epistemic competence survives uncertain
origins. Experiment 008 has not been implemented.
