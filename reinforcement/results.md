# History foundation reinforcement results

2026-10-07. Experiments 001–007 are frozen; Experiment 008 was not started.
The [design](design.md) records findings and tradeoffs before the refactor.
The [developer guide](../docs/history-audit.md) describes ownership, references,
reconstruction, indexing and rules for future extensions.

## Findings and changes

The baseline 1,000-agent export was 11,952,309 bytes. Its provenance resource used
3,483,413 bytes despite zero live provenance knowledge. Queries accounted for
3,441,387 bytes. Repeated base inputs, native decisions and native responses alone
used 2,525,955 bytes; native inquiry already owned those values. Other repeated
fields included result, timestamps and learning cells.

The runtime now stores provenance query extensions using typed references to native
inquiry records and immutable, exactly interned local contexts. Invalid policy
attempts remain reconstructible. A separately versioned compact export validates
references and expands losslessly to the old public format. Old reports remain
unchanged, including their full inputs. No objective record was deleted.

Ordered active-scene and busy-participant sets eliminate scans of completed scenes.
Resolver indexes replace full receipt scans for evidence maxima, first-contact trust
and immutable source identity, and replace inspection-slot scans. Directed trust
prefixes preserve every historical clamp and recompute only an affected suffix,
stopping at convergence. Rebuild checks compare caches against authoritative history.
Policies cannot access these indexes. Knowledge limits and forgetting remain intact.

## Storage and occupancy

Sizes use compact JSON, not pretty printing or compression. The measurement label
differs from the archived 007 population label, explaining a 52-byte difference.

| Population, three encounters | Legacy before and after | New compact export | Reduction |
| --- | ---: | ---: | ---: |
| 100 | 1,305,044 | 980,102 | 24.90% |
| 1,000 | 11,952,309 | 8,930,554 | 25.28% |

At 1,000 agents the compact query extensions use 419,506 bytes, versus 3,441,387
for legacy queries (87.81% less). This population shares one empty local overlay;
nonempty controlled contexts also reconstruct exactly, but their size savings are
not inferred from this favorable population case.

Record counts and live occupancy are unchanged: 2,432 events, 43 information scenes,
3,000 queries, 184 concerns, 225 inquiry cells and 30 signatures at 1,000 agents.
There are zero provenance roots, receipts and live acquisitions in that population.
Controlled scenarios exercise nonempty provenance separately. New derived indexes
contain 1,000 directed relationships, 4,864 contributions and event positions, 27
evidence questions and 26 source entries. No active scenes remain at export.
See [before](baseline-structure.json) and [after](after-structure.json).

## Runtime and export measurements

Release builds on the same Windows host. Population rows below are medians of five
seed-42 runs; milliseconds. Initialization, execution, snapshot construction and
serialization are timed separately. Execution includes audit insertion; insertion
alone was not instrumented. Snapshot construction measures the observer audit copy,
not just record append cost. Heap allocation/RSS was not measured.

| Population / operation | Before | After |
| --- | ---: | ---: |
| 100 initialization | 0.2682 | 0.2290 |
| 100 execution | 2.1801 | 1.9008 |
| 100 legacy snapshot | 0.4800 | 0.4133 |
| 100 legacy serialization | 2.8269 | 2.2474 |
| 100 compact snapshot | — | 0.2195 |
| 100 compact serialization | — | 1.5031 |
| 1,000 initialization | 1.0649 | 0.7058 |
| 1,000 execution | 21.7931 | 16.6571 |
| 1,000 legacy snapshot | 8.1990 | 4.5448 |
| 1,000 legacy serialization | 24.6575 | 21.1003 |
| 1,000 compact snapshot | — | 3.7125 |
| 1,000 compact serialization | — | 16.6879 |
| 1,000 compact expansion/validation/equality | — | 10.4947 |

Long-history workloads perform genuine refusal/disclosure and trust revision with
two agents. Single observations, with exact unchanged legacy sizes:

| Encounters | Before execution ms | After execution ms | Legacy bytes |
| --- | ---: | ---: | ---: |
| 100 | 0.8772 | 0.6328 | 187,469 |
| 1,000 | 65.1434 | 4.9278 | 1,862,624 |
| 4,000 | 1,034.8994 | 19.5284 | 7,486,641 |

The 4,000-encounter run is about 53 times faster in this measurement. It exercises
the removed global scans; it does not prove constant-time arbitrary old revisions.

The established throughput benchmark also passes. Its 100-seed, 1,000-agent 007
three-encounter execution changes from 2,550.678 to 1,824.095 ms; 006 changes from
2,154.562 to 1,430.603 ms. Record counts agree in every mode. Experiment 001
regresses from 153.341 to 179.360 ms (about 17%); its 100-agent row also regresses.
Index maintenance has a cost even when no revision occurs. Other rows improve in
this run. Host timing noise is material: an earlier post-change population run had
a 1,000-agent median near 21.5 ms rather than the final 16.7 ms. Treat short-run
percentages as observations, not stable speedup guarantees. No large-world claim
follows. Raw [before](baseline-measurements.csv), [after](after-measurements.csv),
[baseline throughput](baseline-throughput.csv) and [final throughput](after-throughput.csv)
remain available rather than selectively reporting only improvements.

## Validation

Baseline: 83 tests. Final: 93 tests, zero failures; formatting check, warning-denied
all-target Clippy, release compatibility evaluator and throughput benchmark pass.
The initial parallel test build hit Windows paging-file exhaustion (OS 1455 / LLVM
out of memory). Rebuilding with Cargo `-j 1` passed; no tests or checks were weakened.

The new tests cover compact round trips, exact old archives, immutable contexts,
hash collision safety, malformed policy preservation, dangling/incorrect references,
hidden attribution substitution, saturated signed trust revisions, index rebuilding,
and empty/single/odd/1,000-agent populations. Existing locality, eviction, accounting,
termination and seeded replay tests remain unchanged. All seven archived seed-42
outputs compare structurally equal. The evaluator additionally expands 162 controlled
snapshots across seeds 0, 42 and u64::MAX exactly. See [compatibility](compatibility.json)
and [test output](test-output.txt). Earlier experiment evidence was not rewritten.

## Limits, rejected alternatives and next use

Objective events, completed scenes, communication/inquiry/provenance records,
revisions, evictions and distinct audit contexts remain unbounded. Directed trust,
credibility and expectation maps retain their prior growth characteristics. Indexes
add O(history) observer memory; count measurements are not heap-size measurements.

Remaining work includes worst-case relationship-suffix replay, older forecast/history
searches, full native decision inputs, cloning and serialization of whole snapshots,
and duplicated archive prefixes across report checkpoints. Bounded cognitive scans
were retained deliberately. The existing distinction between native evidence and
provenance evidence remains; this pass does not change their behavioral integration.
Legacy numeric ID namespaces are not universally converted to newtypes. Compact
validation is a targeted causal checker, not a complete forensic verifier or save
file loader. These are explicit limits rather than promised future fixes.

Rejected changes include arbitrary history deletion, restoring forgotten knowledge,
naive trust summation, global context normalization, base-plus-delta reconstruction
for all cognitive state, generic recursive JSON compression, and a database or
general event framework. They either violate existing contracts or introduce more
complexity than the measured duplication justifies.

Future systems can extend an existing decision by reference, capture only genuinely
new local context, and validate/reconstruct it without another parallel copy of the
same inquiry. That is an architectural capability, not evidence of future behavioral
or population scalability. Stop here: further normalization should follow a new
measured need. Experiment 008 remains outside this task.
