# Experiment 006 — adaptive inquiry and expected information gain

Executed 2026-10-07 on the established Rust + Bevy ECS headless simulation,
Windows x64 / Rust 1.99.0. Rules: `experiment-006-v1`. The research question was:
can an individual recognize that asking the same person the same way is unlikely
to teach it anything new, and adapt while retaining the unresolved concern?

The answer is **yes within these controlled mechanisms**. Inquiry history changes
method/source selection and stops unproductive effort; new locally advertised
evidence can restore value. This is a traceable heuristic demonstration, not
calibrated information theory, optimal planning or broad social intelligence.

## Reproduce and inspect

```powershell
cargo run --locked --release --example evaluate006
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo bench --locked --bench throughput
```

The evaluator writes only Experiment 006 evidence. [Specification](specification.md)
was written before implementation. [Readable report](report.txt),
[complete seed-42 trials](seed-42.json), [128-seed summary](summary.json),
[population audit](population.json), [compatibility](compatibility.json),
[baseline timing](baseline-benchmark.csv) and [new timing](benchmark.csv) separate
deterministic simulation data from wall-clock measurement. Prior evidence files
were preserved, including the pre-existing uncommitted 004/005 implementation.

## Architecture and representations

An opt-in Inquiry ECS resource extends Concerns; its independent pure policy sees
only the actor's bounded state and public co-presence/offers. Concern ID, source
ID, strategy, received information, novelty and learned expected usefulness stay
separate. Importance and uncertainty remain on the concern; trust and credibility
keep their earlier meanings. No partner-private state or sampled outcome enters
the inquiry input. The local responder uses the existing TalkInput/Predictor rules.

The minimal actions are Direct, Evidence and Pause. A cell keyed by concern/source/
strategy retains attempts, usefulness expectation, the previous inquiry record and
which advertised offer was attempted. Signatures retain event, provenance, received
proposition, quality and receipt ID. Episodes reference actual queries/outcomes.
There are no inserted retrospective episodes or forced inquiry/response choices.

Local storage caps: 32 estimates, 32 signatures, 16 inquiry episodes, 16 contacts,
32 offers and 32 own evidence capabilities per individual. Eviction is audited.
Existing concern, belief and memory bounds remain. Objective events, notices,
meetings, transitions and full decision archives grow without a global bound.
See [architecture](../../docs/architecture.md) for exact schemas, equations,
legality and accounting.

Only people present in a recorded resource encounter or bounded 2–8-person inquiry
meeting can be candidates. Meetings expose identity and opportunity, not expertise.
A third person cannot inspect the original refusal: with no legitimate event
knowledge their response is Silence. This experiment does not implement evidence
relay, witnesses to past events, global experts, gossip or travel planning.

An experimental capability intervention makes a new immutable evidence provenance
available to the original refuser. At the next meeting the source independently
scores whether to announce the opportunity. Public notices contain quality, effort
and a provenance ID; noise channel, seed and reading remain private to the resolver.
After selection, the source can voluntarily disclose or remain silent. Announcing
is neither a receipt nor proof of the claim. Repeating a notice cannot repeatedly
renew value, including when an earlier offer went unfulfilled.

## Decision and novelty mechanism

Initial expected usefulness: Direct 55, Evidence 65. Valid selected replies update
only their source/strategy cell by `(old + realized value)/2`, truncating to integers.
No unselected response is sampled or used for learning.

| Received information | Novelty / realized useful value |
| --- | --- |
| First weak claim | New, min(quality,40) |
| Same provenance/content, equal or weaker quality | Redundant, 0 |
| New weak origin for the same known proposition | New, min(quality,20) |
| Stronger information supporting the same proposition | Stronger, min(quality - prior maximum + 20,90) |
| Opposing information | Conflict, min(quality,30), without implying certainty |
| Silence, invalid response, or zero-quality information | None, 0; invalid responses do not train cells |

Quality is locally received/advertised quality, not truth. Each receipt in a batch
is classified; the batch value is its maximum, not a sum. Content signatures ignore
new receipt IDs, so redelivery cannot masquerade as new evidence. The cognitive
ledger separately preserves old repeated-evidence and conflicting-support rules.
Original refusal interpretations are never rewritten; live revisions and saturated
trust replay use the existing mechanism. New credibility contributions require a
new legitimate claim/evidence comparison, not repetition of an earlier proof.

Expected gain is the greater of learned/prior usefulness and a still-untried new
public evidence opportunity, adjusted by local credibility/10 and bounded 0–100.
Benefit = concern importance × expected gain /100. Cost includes strategy effort
(Direct 10, Evidence 28), own hunger/10 and publicly advertised disclosure effort.
Pause scores zero and wins ties. Importance multiplies information value; it does
not independently reward futile questions. The old 005 age/failure penalties are
not part of this opt-in policy. A method can stop while the concern stays Open or
Partial. Neither Pause nor a low usefulness estimate asserts resolution.

Meeting co-presence costs one tick, each decision one tick, selected queries spend
their strategy effort, announcements one tick, and valid replies spend one tick
plus receipt processing and actual disclosure effort. Hunger is a utility cost,
not an extra time charge. Every meeting gives at most one inquiry per participant
and one voluntary bounded reply. Food does not move during inquiry; explicit
consumption is preserved. Invalid actions/provenance mismatches terminate with no
information or learning effects; time and failure are audited.

## Controlled results and ablations

Eight variants × seeds 0–127 = 1,024 trials, each fully replayed: 1,024 matching
replays. Their controlled physical states/preferences are intentionally matched;
identical outcomes across these seeds demonstrate capability, not prevalence in
diverse populations. The first real refusal creates concern 0 at importance 78
(98 in the high-importance trial). Later public meetings supply opportunities.

| Required case | Actual result in all 128 seeds |
| --- | --- |
| A: Repeated low novelty | Two Direct requests receive the same unsupported claim. Realized value 40→0; expected Direct usefulness 55→47→23. |
| B: Change strategy | Third query chooses Evidence. Direct gain 23, score 4; untried Evidence gain 65, score 19 despite higher cost. |
| C: New evidence restores value | After five queries and two pauses, the concern remains Partial. A voluntary quality-80 new-provenance offer restores Evidence; a stronger receipt resolves the concern subjectively. |
| D: Alternative source | A's methods are depleted. A recorded meeting introduces Source B; B's untried Direct prior wins. The same concern is pursued through B, who gives no answer because they lack knowledge. |
| E: Importance versus usefulness | Importance stays 98. Three unanswered queries (Direct, Evidence, Direct) exhaust useful expectations; four subsequent opportunities choose Pause. Concern stays Open, uncertainty 100. |
| F: Useful repetition | After a repeated Evidence response scored Redundant/0, new quality-80 provenance makes the same Evidence strategy useful. The next Evidence receipt scores Stronger/50 and resolves the concern. |
| History disabled | Identical priors, costs, physical conditions and response rules: all seven opportunities repeat Direct. |
| Novelty disabled | Redundant content is credited as useful: seven queries, no pause, rather than five queries then two pauses. |

Pure-input ablations additionally remove only estimate cells from each recorded
context. At the third opportunity Evidence→Direct; at the alternative-source
opportunity Source B→Source A. Removing only B's public availability instead yields
Pause. These comparisons isolate recorded inquiry history/available options rather
than unrelated utility retuning. They are local policy counterfactuals, not claims
about otherwise identical full life trajectories.

## Representative causal trace

Seed 42, owner 0, concern importance 78, hunger 30:

1. Direct gain 55, benefit 42, cost 13, score 29; Evidence score 19. Direct wins.
   Receipt 0 is a quality-50 unsupported claim: New/value40, cell becomes 47.
2. Direct gain 47, score 23 still beats Evidence19. Receipt 1 repeats the claim:
   Redundant/value0, cell becomes 23. Belief support does not increase.
3. Direct score4 versus Evidence19: Evidence wins. Receipt 2 is a new quality-40
   instrumented origin, weaker than the claim: New/value20, Evidence cell becomes42.
   Concern remains Partial with support40 under the authoritative old belief rules.
4. Cheap Direct still scores4 versus Evidence1, so it is retried once: redundant,
   Direct expectation23→11. Then Evidence scores1 and is retried: redundant,
   expectation42→21.
5. Direct score-5, Evidence-15: Pause wins at subsequent opportunities. Importance
   stays78; no concern is abandoned or declared certain.
6. New public quality80/channel1 offer is not an old observation or attempted offer.
   Opportunity value = 80 - prior strongest quality50 +30 =60. Evidence benefit46,
   cost31, score15; it wins. Receipt5 is Stronger/value50, support becomes80 and
   the concern resolves. Trust revision remains traceable to the original event.

Candidate records reference earlier inquiry record IDs and notices; response
records reference actual information receipts. Objective proof correctness stays
separate from what the individual received. Tests show equally advertised inverted
evidence produces the same initial inquiry decision but wrong support -80 and a
mistaken subjective resolution. Better inquiry selection does not establish truth.

## Population, performance and audit growth

The 1,000-agent seeded population uses three encounters per fixed pair and food
consumption after each, with no replenishment or capability interventions. Replay
matches exactly. It records 2,432 resource actions, 922 legacy talk turns,
3,000 inquiry decisions and 43 information receipts. Inquiry pursued 158 distinct
concerns in 229 questions: 4 consecutive same-source/same-strategy repetitions,
67 strategy changes, zero source changes (fixed-pair scheduling), 3 low-value
pauses involving 3 unresolved concerns. At the end 165 concerns remain active and
19 resolved. Local state has 225 cells, 30 signatures and 229 retained episodes
distributed across the population.

Source switching and renewed availability are established by controlled scenarios;
this short fixed-pair population cannot demonstrate either naturally. Population
policies differ from old 005, so resource/talk outcomes can also differ causally.

Matched release benchmark rows use 100 seeds, populations 100/1,000 and three
encounters. Generation/ECS/admission are timed separately; execution includes later
admission, resource interactions, communication/inquiry and consumption once at the
end. Audit snapshot cloning/counting and serialization are excluded. This differs
from the population evidence, which consumes after each encounter.

| Population × 100 runs | 005 run ms | 006 run ms | Relative increase |
| --- | ---: | ---: | ---: |
| 100 | 105.967 | 115.671 | 9.2% |
| 1,000 | 1,308.289 | 1,433.450 | 9.6% |

The old-mode 005 three-encounter row in the pre-change baseline was 1,648.959 ms
at 1,000 agents; the post-change timing was 1,308.289 ms. Timing variability/load
prevents treating that difference as a demonstrated speed improvement. These are
single local measurements without confidence intervals. Records/sec across modes
is not a like-for-like efficiency score: inquiry replies are embedded in inquiry
records whereas legacy talk replies are separate records.

Compact deterministic population JSON grows from old 005's 8,341,078 bytes to
8,468,468 bytes (+127,390, 1.53%). This measures serialized audit volume, not heap
allocations or resident memory. Local collections are bounded, but full input
snapshots repeat bounded payloads per decision. Long archives, evidence searches,
trust replay, scene admission and contact history audits remain scaling risks.
No additional allocation profiler, long-run memory bound or million-agent claim
is made. No premature optimization was introduced.

## Tests, unexpected behavior and limits

Formatting, warning-denied Clippy for all targets, all 71 tests (70 integration plus
one storage unit test), and the release benchmark pass. The 57 earlier integration
tests remain active. New tests cover 128-seed full replay, input-only ablations,
private state/noise isolation, introduced-source legality, five novelty categories,
bounded storage/eviction, query/response validation, immutable provenance, repeated
and unfulfilled offers, cell reconstruction, valid event/receipt references,
original-event preservation, food conservation and every tick. Empty, single,
odd and 1,000-agent populations are checked. Compatibility 001–005 is archived
under 006 and matches structurally, including the entire saved 005 suite.

Observed unexpected behavior: separate method expectations briefly oscillate
Direct→Evidence→Direct→Evidence before pausing, because Direct is cheaper. This
was retained and reported rather than forcing a monotone strategy ladder. Review
also identified the unfulfilled-offer renewal trap; attempted-offer bookkeeping
and a regression test prevent recurring public promises from resetting the score.
Third-source exploration failed to discover information, as legitimate knowledge
was absent. Neither case is presented as successful inference.

The numeric usefulness priors, weights, thresholds, novelty categories and averaging
rate are heuristics. The model does not calculate Shannon entropy, fit predictive
probabilities or model question phrasing. Only refusal scarcity exists. Importance
does not evolve during these probes. Meetings are external opportunities; agents
do not choose travel destinations. New evidence is a controlled instrumented
capability, not emergent discovery. Providers retain legacy communication memories;
inquiry episodes track the investigator's inquiry outcomes rather than adding
provider self-talk to the old stream. Evidence can be wrong; exhaustion is local
to retained source/strategy history and forgetting may restore inappropriate optimism.
No general planning, autonomous search, institutions, language generation, LLM
decision calls or successful third-party evidence relay is implemented.

## Single most informative next experiment

Experiment 007 should test **provenance-aware comparison of locally informed
alternative sources**: give a second individual knowledge through an explicit
bounded observation channel, compare genuinely independent evidence with a relay
of the same earlier evidence, and test whether inquiry choices and uncertainty
respond to that distinction. Experiment 006 demonstrated switching but its alternate
source had no knowledge; this directly tests the missing benefit without adding a
global information network. Experiment 007 has not been implemented.
