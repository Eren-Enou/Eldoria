# Experiment020 verification and reproduction

Status:all required checks passed,including the protected001–019 foundation,
every standalone009–019 gate,and the separate020 gate. Exact command exit statuses
and engineering durations are in [verification-status.json](verification-status.json).
No020 gate is added to `validate_contract`;001–019 remain the protected foundation.

## Source and evidence identity

Task base:`74c3f5a2c2484a7b83da080ca92dc84721979d0a`.
Viewer prerequisite:`831d8fc39f39326f623751268e727147540aad4d`,committed and pushed.
Preregistration/gate:`6c75e790456ef5d1559a114f9bce2ba6a87c1ded`,committed and pushed
before020 treatment implementation/outcome inspection. New sources are frozen in
[source-freeze.json](source-freeze.json);aggregate parent edits are only module
declarations,exports and order formatting,shown in [shared-extensions.diff](shared-extensions.diff).
No protected recurring/helper body was modified.

[archive-manifest.json](archive-manifest.json) identifies compressed and decompressed
bytes of all164 complete original histories and the independent descriptive analysis.
The new full trace has606846590 bytes including164 newlines,compressed to11207457 bytes.
Uncompressed SHA256:`96f1f225c8c6f53c5bcdd1be7237b45a2095a5f49218f81b9d88bd5d3b7f0433`.
Its manifest permits gzip re-encoding detection as well as original JSON verification.
[frozen-001-019-sha256.json](frozen-001-019-sha256.json) is an unchanged copy of the
existing296-artifact foundation map,not a new020 foundation contract. A separate raw
byte guard captured all296 at task start and has verified no byte changes.

The first valid execution used redundant choice inputs. A schema-only audit capture
change stores the legacy input once,in `activity.decisions[owner]` or
`choices[owner].acquiring_base` when acquisition suppresses a legacy activity. The
pure selector still receives the same complete local Input. Streaming equality of
all164 initial/final histories after reversing that encoding change proved every
decision,record,state and outcome identical. [audit-compaction.json](audit-compaction.json)
records both byte hashes and sizes:944896591→606846590 bytes,35.8% reduction. Initial
scratch output remains in `target/program020-initial`;it is not a discarded failed
behavioral result or replacement of unfavorable evidence. No experimental rules
changed after outcome inspection.

## Independent method

`examples/support/review020.py` imports no Rust selector,evaluator or prepared
classification. It reconstructs new policy arithmetic and strict ties,cache observations,
admission references,actor busy intervals,one shared clock,need pulses,completion
rank,atomic debit/credit,meals,resource execution and query scheduling. It validates
each actual point and reconciles every new event/query/meal/claim exactly once.
Every assessment prefix counter is additionally checked against legitimate receipt
acquisition times,so a future observer receipt cannot silently enter a policy preview.
That read-only guard was added during source review and reran all164 histories/20
corruptions against the same archived analysis with exact equality;no outcomes changed.
Old independently reviewed006–019 Python primitives reconstruct resource decisions,
retained assessment,concern inputs,source/method learning and legitimate query
responses. Reusing those old primitives does not independently re-derive all earlier
theory;the protected foundation gates remain separately required.

Reader development exposed a legacy-helper assumption that every handshake has
three responses.020 deliberately skips busy recipients;the independent020 allocator
therefore matches replies by actual owner ID. This was a reader-only correction,
not a runtime or outcome change. All164 final histories passed reconstruction. Twenty separately corrupted scratch
histories were rejected:cache identity,observation stock,future observation,private
input,choice,claim reference,deadline,duplicate grant,missing debit,false success,
owner,completion order,busy overlap,need progression,meal,resource event,missing
event,missing record,missing completion and elapsed time. These mutation tests
establish targeted sensitivity,not immunity to every possible defect.

The standalone Rust gate checks source/foundation/archive hashes,replays every
registered original trial in exact matrix order,compares complete serialized JSON
bytes and runs the independent reader/corruptions against archived analysis. It is
read-only and cache-free with respect to020 evidence;it does not generate replacement
archives. Reviewer scientific judgement is still a separate future task.

## Completed development checks

| Check | Observed result |
|---|---|
| Initial targeted acquisition tests | 3 passed;scalar overflow,last-unit race,exact019 control |
| Final `cargo test --locked -j 1` | exit0;202 tests across25 result blocks,including5 acquisition tests |
| `cargo fmt --check` | exit0 |
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` | exit0 |
| Final `evaluate020 -- --output target/program020-final` | exit0;164 exact duplicate executions |
| Independent final reader | exit0;164 histories/20 corruption categories |
| `cargo run --release --locked -j 1 --example validate020` | exit0;164 exact original byte replays plus independent reconstruction |
| `cargo bench --locked -j 1 --bench throughput` | exit0;[separate output](throughput.txt) |
| `cargo run --release --locked -j 1 --example measure020` | exit0;[separate output](observer-cost.txt) |
|020 demo projection test | exit0;every frame/action/ledger in10 displayed original histories |
| Actual020 browser | pending/failed claims,sharing/consumption,filters,576-tick depletion;no warning/error logs |
| Protected foundation + standalone009–019 | all12 commands exit0;full foundation plus each standalone,read-only |

The five new Rust regressions cover scalar conservation/overflow,three stale last-unit
claims,exact019 ongoing control,hidden recipient mirror and no retroactive consent
when a two-tick claim finishes after the earlier response opportunity.
The full viewer suite passed25 Python tests,including the020 projection;the final
projection test was repeated after adding contact dispositions and fixing actual
same-tick order. The preceding main-viewer milestone also passed602-original-source traversal,
actual JS filter/order/owned-state tests and real historical browser cases. New020
presentation has a separate [acceptance record](viewer-acceptance.md);it does not
silently extend the old ingestion schema.

## Engineering cost

Wall-clock numbers are engineering observations from this Windows machine with
other validation potentially running,not deterministic simulation evidence or
statistical performance estimates. Compilation/waiting time is excluded from the
evaluator's internal timers. No optimization pass or heap profiler was performed.

The final164-world evaluator summed1.062s execution,1.173s replay and1.761s
serialization. Maximum individual JSON record is15329115 bytes. Audit capture
and serialization remain substantial relative to simulation work. The independent
final reconstruction/corruption pass took81.55s;the archived read-only gate's
separate independent pass took73.06s. Neither is a controlled before/after benchmark.

`measure020` separately reported10000 pure candidate evaluations in13553µs.
Fifty ongoing trials took749853µs Owned,800566µs Need and772633µs Acquire,
versus345107µs for019 Baseline. The exact-trace Owned control therefore cost about
2.17× this019 workload in this single measurement. It includes new cache/claim/choice
audit work and repeated invariant checks;it does not establish a general slowdown.

Ongoing JSON is6780650 bytes Owned and8419864 Need;Need has689 captured points,
461 activity steps and12 claims. Their one-shot serialization took13.3/19.7ms.
Ten observer demo projections use2477179 bytes. Complete records are retained
despite their cost;no outcome-dependent deletion or audit-infrastructure refactor
was used. Live new state is one scalar cache and at most three pending claims.
Completed claim records grow only in observer history;no new cognitive collection
or memory-capacity increase exists.

[audit-cost.json](audit-cost.json) compares original bytes at the same ongoing
control:019 C85 is zero-based archive ordinal84,6296760 bytes;020 Owned is6780650,
7.7% more.020 Need is8419864,33.7% more,but also has a different executed history.
Do not attribute that entire difference to cache-record overhead.020 record numbers
in reports/demo are zero-based ordinals;the older main viewer's C/H labels are one-based.

## Reproduction

Files changed are confined to the new020 implementation (`src/acquisition.rs`,
`src/simulation/acquisition.rs`,`src/experiment020.rs`),module declarations in two parent files,
five acquisition regressions,new evaluator/validator/measurement and independent
reader/report/archive tooling,the020 evidence directory,and the explicit observer
demo/projection test. README,AGENTS,architecture/context/history indexes,viewer
guides and the working checkpoint document ownership and research status. No old
experiment archive,contract,policy body or default was changed.

Read-only verification:

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -j 1 -- -D warnings
cargo test --locked -j 1
cargo run --release --locked -j 1 --example validate_contract
# Run each standalone validate009 through validate019 as well.
cargo run --release --locked -j 1 --example validate020
python -B tools/history-explorer/tests/test_finite_demo.py
```

New authorized scratch execution and analysis:

```powershell
cargo run --release --locked -j 1 --example evaluate020 -- --output target/experiment020-new
python -B examples/support/review020.py --directory target/experiment020-new --corruptions --output target/experiment020-new/analysis.json
cargo run --release --locked -j 1 --example measure020
cargo bench --locked -j 1 --bench throughput
```

Use a genuinely fresh evaluator directory;existing trial destinations are rejected.
Do not run archive generation to refresh historical evidence in place. The authorized
020 archive writer itself refuses an existing full-trace destination. Demo generation
is a deterministic read-only projection of the archive,not a new scientific outcome.
