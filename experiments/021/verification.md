# Experiment021 verification and engineering cost

Research status:unprotected,unadopted. The normal protected gate remains001–020.
No historical archive was regenerated in place. Original021 outputs were written
once from a fresh target directory after preregistration and source hashing.

## Scientific and compatibility checks

- `cargo fmt --check`:PASS.
- `cargo clippy --locked --all-targets -j 1 -- -D warnings`:PASS.
- `cargo test --locked -j 1`:211 passed,0 failed across26 result groups,including9
  new targeted tests. These cover all28 fresh-primary old020 execution controls,
  all14 Unobserved/oldIgnored controls,and ownership/future/absence/expiry/overwrite/
  new-receipt/depleted/own-result/no-recovery semantics.
- Registered evaluator:214 trials,each executed twice with exact Trial equality.
- Independent `review021.py`:214 full reconstructions,28 corruption categories
  rejected,78.52s in the initial engineering run. No Rust scorer/evaluator imported;
  only reviewed old cognitive/resource primitives are reused.
- `compare021.py`:57 matched groups,157 contrasts;old initial/profile matching and
  unchanged local inputs at first selected-activity divergence asserted.
- `findings021.py`:useful,mixed,costly,null,inert,expiry and reconvergence anchors PASS.
- Additional read-only semantic trust extraction:missing entries normalize to0.
  [Derived contrasts](trust-comparisons.json) preserve all differing pairs;LastSeen10,
  Expire12 6,FreshReference4,Unobserved15. Original raw-map comparison161/163 differs
  only in zero-valued entries;it is not counted as changed trust. No original
  analysis,comparison archive or simulation outcome was rewritten.
- Working-byte guard:all326 original001–020 files unchanged after021 execution,
  including the5 viewer-only original020 files outside its scientific inventory.
  [Starting-checkout hashes](frozen-001-020-working-sha256.json) preserve the actual
  Windows working bytes. They are an in-session preservation proof,not a new
  cross-platform line-ending contract;the canonical321-file foundation gate remains
  authoritative for portable validation.

Final [command/status record](validation-run.json):all17 gates PASS. This includes
normal001–020 foundation (1,414.52s including compilation),standalone009–020,
the shared protected020 wrapper,separate021 original byte replays/independent
reconstruction/corruptions,selector/control measurement and throughput. Final raw
checks confirm all326 old working files unchanged,all21 source locks intact and all
three021 archive envelopes unchanged. Canonical Git diff is empty. No gate writes
historical outcomes or promotes a default.

## Evidence and sources

`archive-manifest.json` records compressed and decompressed SHA-256 and sizes.
Original trials are751,189,037 bytes including214 LF separators,compressed to
14,880,167 bytes. Independent analysis is7,998,566 bytes,compressed300,965.
Comparisons separate raw audit equality from physical action/material projections.
All histories,including failures and nulls,are preserved. `causal-slices.json` is
bounded presentation material derived from actual originals,not replacement trials.

`preregistration-source-freeze.json` precedes final outcomes.
`format-only-after-registration.json` records one later whitespace-only rustfmt
change:the pre-format hash was reconstructed and whitespace-stripped equality
verified. No mechanism or matrix change followed outcomes. Final `source-freeze.json`
checks21 source/specification/test/presentation inputs (the20 original locks plus
one later read-only semantic-trust verifier;all20 hashes remain unchanged). It is a separate research
source lock,not an extension of the protected001–020 contract.

Replay is read-only:

```powershell
cargo run --release --locked -j 1 --example validate021
python -B examples/support/review021.py --corruptions --expected experiments/021/analysis.json.gz
python -B examples/support/compare021.py --analysis experiments/021/analysis.json.gz --expected experiments/021/comparisons.json.gz
python -B examples/support/findings021.py
python -B examples/support/review021_trust.py
```

The evaluator refuses an existing destination. To generate a new comparison run,
choose a fresh target directory;do not run historical archive writers.

## Runtime and observer storage

Wall-clock measurements are engineering observations from this Windows workspace,
not deterministic simulation evidence or statistical performance comparisons.
`performance-run.csv` and `evaluator-performance.json` are separate from archive
hashes:214 first executions totaled0.997s (median3.727ms,p95 13.195ms,max17.435ms),
replays1.054s and serialization plus buffered writes1.746s. Serialized trial median2,822,714 bytes,
maximum15,611,353. Compilation and independent-reader time are separate.

Cognitive storage caps at one environmental Cell per actor. The214 worlds contain
25,112 growing observer memory-change records. Full old cognitive inputs,point
histories and causal audit remain the larger storage cost. The new Choice references
its existing base once rather than copying the full old decision twice. Audit
before/after cells are observer state and cannot become policy memory. No general
optimization or frozen-infrastructure refactor was performed.

Offline archive creation currently materializes the751MB raw trial file for gzip;
comparison analysis retains compact paired inputs across the matrix. Peak process
RSS was not measured. These are additional tooling/storage limitations,not bounded
by the one-cell cognitive cap. Larger studies should measure them before expansion.

[Measurements](measurement-summary.json),[complete engineering output](performance.txt):
10,000 contested-input selector evaluations take13.796ms (about1.38µs each).
Twenty ongoing FreshReference worlds take294.172ms versus matched020 Need273.111ms,
about7.7% additional execution/capture time in one ordered run. Their raw traces are
8,800,775 versus8,419,864 bytes,+4.5%;new memory audit accounts for300,783 bytes/900
records. This is store/capture overhead on matched behavior,not a calibrated general
cost prediction. CPU order/cache/load variation and peak RSS were not controlled.

CurrentOnly/LastSeen/Expire12/Unobserved20-world timings are275.957/282.990/278.473/
284.105ms,with402/203/217/0 memory records. Different modes execute different
histories;their runtimes cannot isolate memory cost. The reported serialization
interval in measure021 includes both full Trial serialization and the audit-byte
tally;it is not pure serializer timing. All20 existing throughput workloads complete
(100/1000 actors,100 rounds). They exercise001–008 compatibility throughput,not
021 population scaling. No general optimization was performed.

## Limits and known failures

No known behavioral/locality/accounting defect was found by these checks. Initial
Clippy rejected an unnecessary lazy `then` closure;`then_some` fixed it before
source registration/outcomes. Later rustfmt changed whitespace only. The first
comparison draft counted observation-bearing claim-record inequality as executed
inequality;the final reader explicitly separates those projections. Original worlds
were unchanged. This correction prevents audit-only differences being called action.

Fixed passive observation,one stationary nonrenewing cache,three actors,old Need/
Acquire scores,prepared traits/history,fixed windows and capped hunger constrain
interpretation. Endpoint convergence does not prove equal histories or welfare.
No negative-to-positive physical reopening exists here. Atomic provisional-query
blocking remains unwitnessed. Duplicate private lifecycle code risks future drift.
The next session should independently review021 before any protection/adoption.
