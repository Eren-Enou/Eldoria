# Experiment 022 verification

The original 100-world evidence passes deterministic, primitive, conservation,
locality and compatibility validation. This is verification of the registered
research mode, not adoption or protection of 022 into the 001–021 foundation.

## Current checks executed

[Executed check summary](verification-results.json) records all16historical gate
exits and the publication integrity checks. [Supplemental identities](supplemental-source-identity.json)
cover the post-evaluation coverage, physical tests and performance drivers;
they do not replace or expand the original60-source freeze.

| Check | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` | PASS |
| `cargo test --locked -j 1` | PASS:217tests,28groups |
| `cargo run --release --locked -j 1 --example validate_contract` | PASS:001–021,341artifacts |
| Standalone `validate009` through `validate021` | All13PASS |
| `validate020_foundation`, `validate021_foundation` | BothPASS |
| `cargo run --release --locked -j 1 --example validate022` | PASS:100exact original JSON byte reruns |
| Independent original primitive reconstruction | PASS:100worlds |
| Scratch corruptions | PASS:23rejected, originals untouched |
| Complementary physical regression tests | PASS:liveclaim/querymask;scene+claim+meal shared clock |
| `cargo bench --locked -j 1 --bench throughput` | PASS:20legacy workload rows |
| `measure022` and full-suite measurement | PASS:24samples;100-world exact-byte measurement |

All 16 historical gates (normal, 13 standalones, two shared wrappers) ran against
the current changed library, rather than relying on pre-022 passes. Their complete
logs and return codes were captured in the task's `concurrent-program` temporary
directory. The normal gate took 1,873.84 s while another historical pipeline ran;
this is validation throughput under load, not simulation performance.

The auxiliary `validate_history` was excluded before execution because its main
writes `reinforcement/compatibility.json`; it is not a read-only integrity gate.
No historical evaluator/archive writer was invoked. The original working-byte
guard covers 346 files, and original 021 has 21 locked source files. These remain
separate from 022's own source identity and the protected341-artifact inventory.

## What the independent reader checks

[`review022.py`](../../examples/support/review022.py) imports no 022 Rust selector,
evaluator classifications or viewer code. It reuses reviewed independent Python
implementations of unchanged resource/contact/006–012 cognitive arithmetic.

It independently regenerates initial seeded expectations, replays bootstrap
policies, legality, food movements, trust and episodes, checks the declared
physical interventions and voluntary inspection channel, and reconstructs:

- One global timeline, all four need pulses and elapsed hunger areas.
- Every commitment's stable identity, source, participants, reservation, bound
  and exact single release; disjoint work and absence of double booking.
- Each scene's exact participant order, public signal, turn progression, legal
  resource candidates, integer scores/ties, transfer and interpretation.
- Current-stock claim resolution, deterministic ordering, full failed duration,
  meals, ownership and conservation after every operation.
- Owned environmental receipts, sensing cadence, own-take adjustment, depletion
  receipts and retained-only choice inputs; no rival-stock backfill.
- Owned chronological contexts and concern creation/aging/capacity/belief/query
  transitions from actual scene completions and receipts, including prefix times.
- Local inquiry sources, retained knowledge, assessment, learned cells, scores,
  chosen questions, information delivery and actual progress.
- Independent recipient opportunities, scores, stable ties, original response
  references, actual consent and deterministic admission reasons.
- Subsequent captured states and final food, hunger, trust, episodes, cognition
  and material stock; exact correspondence with original references.

The reader derives recorded candidate sets and outcomes from primitive inputs.
Matching a snapshot or a prepared “success” label alone is insufficient.

The complementary [`review022_coverage.py`](../../examples/support/review022_coverage.py)
is a post-evaluation, read-only coverage/interpretation audit (`--verify`), not a
replacement outcome generator. It distinguishes masked local query candidates,
blocked protocol admissions and completed queries, identifies actual mixed-work
intervals, and corrects the fixed-ID diagnostic's interpretation. It also keeps
resource-ledger equality distinct from terminal hunger and whole-history equality.

## Corruptions actually rejected

The 23 in-memory scratch controls include double booking, duplicated transfer,
wrong actor/commitment/source/reply reference, missing release, incorrect deadline,
skipped tick, doubled hunger, false consent, illegal participants, unrecorded grant,
observer/future input, cross-scene references, simultaneous order, wrong completion,
missing event, inconsistent ownership, impossible final state, foreign memory,
missing local choice and a fourth-actor clock error. They call the primitive
reader on altered copies; rejection is not merely an archive hash mismatch.

## Original identities and reproduction

`trials.jsonl.gz` has 100 original zero-based members:

- Compressed bytes:2,286,504.
- Compressed SHA256:`f8d6c5b8de0d955bb316f89ec1dba11a0712311f9d45f49e3fb8e17fb1a0635c`.
- Raw bytes:83,762,712, including one newline per member.
- Raw SHA256:`32955966b1aec83610fc879deeaa1e77704ccdb4baf9e707a6f9d94e99186b0f`.

The [manifest](archive-manifest.json) also covers the four original compressed
analysis/report/comparison/slice files. [Source freeze](source-freeze.json) has
60 canonical-LF source/registration entries, including 15 Python dependencies.
[Preregistration identity](preregistration-source-freeze.json) verifies the prior
design commit. Canonical source newlines are distinct from byte-exact compressed
archive identity and native JSON-member equality.

The runner requires a **new isolated output filename**. Do not write historical
archives in place. `archive022.py` intentionally refuses to recreate existing
originals. The read-only validator streams original members and reruns the fixed
matrix; no prepared verification cache is authoritative.

## Limits of verification

The original independent reader validates comparison-epoch need/sensing, while
prospective architecture/gate prose said absolute boundaries. The final audit
discloses this discrepancy in implementation-notes/results. Verification does
not prove absolute-phase invariance or exact adherence to that sentence; each
capacity comparison does keep the same initial epoch. No archived output changed.

The 100 rows are a bounded deterministic grid with equivalent labels and three
seeds, not a statistical population. Physical-work query exclusion is observed
as a pre-proposal mask; all 31 rejected inquiry admissions are protocol-deadline
cases. This does not validate an interleaved query/claim scheduler. Continuous
192-tick runs demonstrate persistence across passive observation windows, not
reopening closed social budgets. More contact, a received response, one transfer,
or final food-ledger reconvergence is not a universal utility or community result.

Wall-clock evidence is separate in [performance.json](performance.json) and
[throughput CSV](performance-throughput.csv). The full streamed measurement also
reproduced all 83,762,712 original bytes. Peak working set belongs to the measured
process/workload; no matched old implementation baseline or browser-heap estimate
is claimed. Viewer acceptance is documented separately and is not a foundation
validation dependency.
