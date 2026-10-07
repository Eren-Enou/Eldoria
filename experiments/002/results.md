# Experiment 002 findings

Executed 2026-10-06, Windows x64, Rust 1.99.0, Bevy ECS 0.19.1.
Rules: `experiment-002-v1`. Reproduce with
`cargo run --locked --release --example evaluate002`.

Evidence: [specification](specification.md), [full seed-42 trials](seed-42.json),
[128-seed measurements](summary.json), [1,000-agent trace](population.json),
[benchmark](benchmark.csv), [single population timing](population-benchmark.csv).
Experiment 001 artifacts were not overwritten. Its saved seed-42 complete report
is structurally identical to the regenerated report, including all old metrics.

## Demonstrated result

New event-specific information can change a remembered refusal and a subsequent
choice while physical circumstances are held fixed. This demonstrates the
implemented attribution rule, not general reasoning or consciousness.

Two resolved helpful encounters precede the target refusal in A/B/C/E. Two resolved
refusals precede it in D and the negative-history branches. Actions are chosen by
the policy. All interventions and snapshots are saved. The target refusal has
requester food=0, hunger=90, generosity=20, caution=20; donor hunger=90,
generosity=0, caution=90, food=1 (or 4 for the false-claim controls). Generated
expectations stay fixed within each seed.

Probe: requester food=4, hunger=30, generosity=0, caution=80; partner food=0,
hunger=90, generosity=30, caution=20; same initiator, amount and budget. The
probe deliberately tests a utility boundary; it is not a prevalence estimate.
An earlier candidate probe produced offers in both helpful-history branches.
The final probe lowers generosity and raises caution to measure whether the
information-derived score difference can cross a decision boundary. Unchanged
choices are retained in the negative-history controls.

| Seed-42 condition | Scarcity support | Trust at probe | Recent valence | Choice |
| --- | ---: | ---: | ---: | --- |
| A: no explanation after helpful history | none | 31 | 7 | Leave |
| B: unverified scarcity claim, helpful history | 65 | 48 | 24 | Offer |
| C: claim followed by disclosure | 100 | 48 | 24 | Offer |
| D: false claim, negative history | 25 | -51 | -34 | Leave |
| E: false claim then contradictory disclosure | -100 | 31 | 7 | Leave |
| F: true claim, negative history | 25 | -51 | -34 | Leave |
| F: true claim, helpful history | 65 | 48 | 24 | Offer |
| Delayed disclosure after negative history | 100 | -34 | -17 | Leave |

Original event 5 remains Rejected/-17 in every seed-42 target event record.
B's information scene 0 changes the live memory to PossibleSelfProtection/0;
trust changes 31 -> 48. The probe Offer score changes from
`0 - 30/2 - 80/2 + 31 + 7 = -17` to
`0 - 30/2 - 80/2 + 48 + 24 = 17`. Leave scores zero.
The communication changes neither food nor hunger. Both branches then use the
same consumption phase and identical probe interventions.

C after helpful history adds confidence but no second memory revision or trust
credit. After negative history, testimony is insufficient; later disclosure
revises the target episode but leaves two other refusals intact. This changes
trust -51 -> -34 without changing withdrawal. New knowledge need not cause
forgiveness or cooperation.

E deliberately demonstrates an inaccurate belief: trusted but false testimony
initially earns support 65 and removes blame despite abundant historical food.
The direct disclosure supplies support -100 and restores Rejected/-17 and trust
31. Weak contradictory testimony cannot override stronger evidence. Equal-weight
conflicts preserve the existing belief; confidence is ordinal, not calibrated.

Across seeds 0..127, A versus B differed in choice in 128/128 trials; all B
memories revised. C versus A also differed in 128/128, but C added no further
revision after the already accepted B claim. False claims in E were initially
accepted in 128/128: this exposes trust-based credulity, not truth detection.
All 128 complete suites replayed identically. The tests independently check E's
reversal, weak evidence rejection, negative-history revision without behavioral
change, and matched present states across all these seeds.

## Population and resources

The seed-42 population uses the same policy, resolver, communication and consumption
APIs with 1,000 generated individuals in 500 disjoint pairs: 957 resource events,
171 disclosure scenes, 27 memory revisions, and 843 food units consumed.
Every resource scene terminates; information scenes terminate in one step. Exact
population replay passes. An independent audit replay reconstructs every agent,
checks each policy decision against its local inputs, and applies all revisions
and consumption records to recover the final state.

Consumption uses one available unit when hunger is positive and reduces hunger
by 40, floored at zero. Empty inventory leaves need unsatisfied. Transfer totals
are conserved and total initial food equals final food plus consumed food when
there are no fixture interventions. Tests cover zero inventory, satiation, small
hunger, maximum inventory, and invalid IDs. A hungry last-unit donor receives an
Offer cost of 60; a matched abundant donor does not. The controlled policy test
shows Leave versus Offer under identical other inputs. There is no hunger growth,
death, agriculture, or production.

## Verification and cost

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
and `cargo bench --bench throughput` pass. There are 21 integration tests: all
11 original tests plus 10 new tests. Baseline checks passed before changes.
Original tests retain the 256-seed conservation/reconstruction sweep and
128-seed history comparison. New tests cover evidence locality, false claims,
legality and atomic rejection, historical disclosure after present-state changes,
repeated evidence, confidence precedence, saturated trust, eviction, replay,
behavioral consequences and 1,000-agent audit reconstruction. No failing checks
remain. Testing is finite and limited to this Windows toolchain.

| Mode | Population | Rounds | Initialization ms | Run ms | Records |
| --- | ---: | ---: | ---: | ---: | ---: |
| Experiment 001 | 100 | 100 | 9.777 | 15.274 | 9,640 |
| Experiment 002 | 100 | 100 | 7.358 | 21.074 | 21,341 |
| Experiment 001 | 1,000 | 100 | 49.479 | 117.114 | 95,578 |
| Experiment 002 | 1,000 | 100 | 46.120 | 185.249 | 212,751 |

Extended run time includes disclosures after every refusal, consumption for all
agents, and the event snapshot used to schedule disclosures. Its records include
resource, information and consumption records, so records/sec is not a like-for-like
speedup. At 1,000 agents the extended workload takes about 58% more run time.
Serialization, evidence report construction and replay checks are excluded from
the throughput benchmark. Single-run population timing includes construction and
returned snapshots and is stored separately. These are local timings without
confidence intervals; initialization differences should not be overinterpreted.

The compact population audit JSON is 1,284,547 bytes. This measures serialized
trace size, not resident memory. Beliefs and episodes each cap at 16 per agent.
The internal circumstance archive adds two (food,hunger) pairs per resource event
(16 bytes of element payload, excluding vector overhead/capacity). Initial trust
is copied once for ledger replay. Audit vectors, relationship maps and completed
scenes remain unbounded. Revision replays the event ledger, and admission/idle
checks scan scenes; long histories can incur quadratic aggregate work. No heap
allocation profile or long-run memory bound is claimed.

## Limitations and next experiment

Communication is a controlled, explicit follow-up scene, rather than a policy's
spontaneous speech choice or a new branch in the alternating resource protocol.
Only original refusal participants can communicate about that event. Disclosure
is a voluntary release of an instrumented historical scarcity predicate: the
listener receives that one observation, never unrestricted private components.
This experimental sensor is stronger than ordinary testimony and depends on an
authentic retained record; it is not a solution to discovering private motives.
The policy sees revised memories/trust, not a general-purpose belief reasoner.

Beliefs concern scarcity at a specific refusal, not current inventory or character.
No probabilistic calibration is established. Confidence weights and thresholds
are hand chosen; there is no automatic learning of a speaker's honesty. Credibility
is frozen at the first information scene concerning an event so the claim cannot
increase its own credibility through its trust effect. Repetition does not add
independent evidence. Equal-strength conflict remains unresolved. Evicted episodes
cannot be reinterpreted even if later evidence forms a belief. Audit replay is an
accounting mechanism, not access to other people's memories. Snapshot loading and
persistent storage services remain unimplemented.

For Experiment 003, test policy-selected communication and costly, contemporaneous
observation with false claims and explicitly limited access. Compare confidence
calibration and unresolved equal-quality evidence, and separate trust from recency
in ablations. First preserve event-specific provenance and consumption accounting;
consider indexed relationship ledgers only if longer-history measurements justify
them. Experiment 003 has not been started.
