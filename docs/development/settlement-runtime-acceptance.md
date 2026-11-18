# First Eldoria settlement: engineering acceptance

Date: 2026-11-18. Research baseline:
`63ebead5aee6c86da44260d16c8d286cc9203277`.
Product runtime source/lock fingerprint:
`80a54fc216ef8417d9564da77bc8dc2d82a7b199ac3ee784db5b3070c023237a`.
This report concerns a new isolated product, not a numbered scientific experiment,
research-policy adoption or change to protected 001–026.

[Run guide and architecture](../../products/settlement/README.md),
[prospective rules](../../products/settlement/design.md),
[actual release CLI receipt](settlement-runtime-validation.json),
[performance receipt](settlement-runtime-performance.json).

## Requirement audit

| Requirement | Current implementation and direct evidence |
|---|---|
| Separate Rust/Bevy product | `products/settlement` has its own workspace, manifest, lockfile, source and checkpoint schema. No research source/host or root Cargo file changed; only navigation summaries link the new product. |
| Autonomous 8/12 characters | Normal initialization followed only by generic `advance` calls. Domain tests exercise 8/12 with seeds 1, 7, 42, 99 through 512 ticks. Release CLI acceptance independently runs both populations at seed 42/horizon 512. No post-setup person-specific events or policy interventions. |
| Stable identities and population independence | Stable `u64` character IDs, reconstructed ID→ECS entity map, dynamic rosters and commitments; no fixed four-person arrays. Tests include sparse/custom IDs, reversed ID input order, one-person operation and 12-person worlds. Configuration bounds are explicit; large populations are unmeasured. |
| Shared clock and independent activity | One ECS exclusive stepping system on a single-thread schedule. Simultaneous independent jobs and pair commitments resolve in documented phases; trait-dependent local decisions are collected before deterministic seeded admission. Tests check invariants every debug step and reconstruct all recorded choices. Actual release histories reach 8/12 concurrent commitments across 3 locations without double booking. |
| Multiple places and local travel | Three connected places, timed travel, no sensing in transit. Captured policy views contain own retained site beliefs and current co-presence, not remote stocks or other characters' private state. |
| Food, cooperation, refusal and competition | Common resolver implements consumption, solo gathering, joint gathering, voluntary requests/offers and accepted transfers. Autonomous outcome tests require actual meals, travel, solo/joint work, transfers, explicit refusals and depletion failures. Separate tests exercise consent against recipient priorities, impossible intents and completion-time stock races. |
| Needs, beliefs, relationships | Utility terms depend on own hunger/inventory/preferences, dated retained site beliefs and directed trust. Experiences update bounded memories/relationships. Tests establish unknown versus observed-zero and separately persistent trust after episode eviction. |
| Planner/proposal seams without inference | Pure `Controller` and `ProposalSource` interfaces in `policy.rs`; only `Utility` is installed. All actual choices use common authoritative validation. No HTN planner, provider SDK, inference request, faction, generation system or renderer. |
| Pause/advance/resume | `advance(0)` and read-only inspection are inert; stepping is capped by the finite endpoint. CLI commands commit and return at safe boundaries. Real run/resume commands execute in separate processes. |
| Complete-state continuation | 130 safe-prefix checks (every split 0–64 for both 8/12) compare resumed with uninterrupted state/bytes. Additional autonomous histories cover saved Eat/Gather/Travel/Send/Transfer/JointGather/Reply phases. They preserve RNG stream, pending work, cognition, source stocks, counters and the full executed prefix. |
| Determinism and randomness | Seeded SplitMix64 setup and one saved stream draw per boundary; canonical IDs/order and integer utility/ties. Same-seed reruns and differently chunked/reopened runs produce exact checkpoint bytes. No wall-clock measurements enter simulation records. |
| Save integrity | New schema/rules/source identity, SHA256 payload binding, complete-state/history validation and fresh-file publication. Tests reject corrupted/truncated/mismatched saves, invalid state/counters/deadlines, actual valid `.pending` files, overwrite attempts, unsafe archive paths and malformed CLI options. No migrations or power-loss guarantee. |
| Structured observation | Typed records capture local decision inputs, candidate score terms, selected intent, admission/rejection, commitments, messages, consent/refusal and actual outcomes with record/commitment references. Read-only snapshots and bounded history pages are exercised via the real CLI. Mutating a copied snapshot cannot change the world; exports cannot load as saves. |
| Information locality | Hidden-state twin test changes foreign inventory, hunger, traits, cognition and unobserved stocks while the actor's view and decision stay exact. CLI actor projections expose no foreign food/hunger. No observer history is accepted by a controller. |
| Research preservation | Root formatting/Clippy/217 tests pass at unchanged research source. Full 026 and separate 027 read-only status are recorded below; no historical evidence is regenerated. |
| Measured costs and next interface | Six release-mode benchmark points and real CLI continuation receipts below; README specifies the next observer dashboard boundary and deferred functionality. |

## What actual runs do

The release CLI used one process per command: init→run 64→save bookmark→resume 448,
and a separate uninterrupted run 512. Paused/bookmark bytes match; final resumed and
uninterrupted bytes match. Both inspect variants and paginated records were read.

| Seed 42, horizon 512 | 8 people | 12 people |
|---|---:|---:|
| Completed meals |97|145|
| Solo gathering completions |64|110|
| Joint gathering completions |12|11|
| Completed one-food transfers |26|41|
| Explicit recipient refusals |78|170|
| Source-depletion work failures |0|3|
| Completed travel |13|20|
| Peak concurrent commitments / work locations |8 /3|12 /3|
| Structured records |6,889|10,872|
| Initial / consumed / final source+owned food |97 /97 /0|145 /145 /0|
| Terminal individual hunger range |19–81|18–81|

These counts are executed events, not proposed actions or a cooperation target.
Transfers may circulate the same food; they do not create supply. Both worlds
exhaust all finite food. Denied admissions, unavailable recipients and refusals
remain visible; equal terminal inventory does not imply equal individual experience.
The twelve-person run has 149 invitation admission losses from contention/time,
49 invitations not answered as the selected priority, and 2 proposer-unavailable
acceptance losses. These are scheduling outcomes, not evidence of fairness or
general social intelligence. The interface can trace them through local choices.

The first development scoring revision produced only joint work in the tested
social domain: the outcome tests failed for missing transfers/refusals. Product
scores were corrected before publication so caution can favor solo work and local
requests can compete with gathering. A later coverage assertion expected a transfer
within the first 64 ticks; parity already passed there, but transfers occurred later.
The final tests retain all early splits and additionally verify each work type when
autonomously reached in longer histories. These are development corrections, not
changes to historical research or a claim that every seed must exhibit every event.

## Costs and history growth

Windows x86_64, rustc 1.99.0; one release process, one sample per point, seed 42 and
fixed horizon 512. Advancement includes first schedule initialization. The inherited
research gate was running concurrently, so these are indicative observations rather
than isolated throughput estimates or statistical comparisons. File sizes are raw
logical bytes; memory peaks and browser/UI rendering were not measured.

|People|Tick|Advance ms|Encode + validate ms|Publish ms|File load + restore ms|Checkpoint bytes|Records|
|---:|---:|---:|---:|---:|---:|---:|---:|
|8|64|3.106|4.575|11.773|5.681|634,185|846|
|8|256|8.565|19.564|16.246|26.921|3,178,728|3,124|
|8|512|19.410|49.535|21.546|66.956|7,484,714|6,889|
|12|64|2.937|6.173|12.901|9.337|1,041,741|1,283|
|12|256|13.341|31.359|18.005|43.458|5,140,469|4,697|
|12|512|30.111|81.908|27.262|101.564|12,105,543|10,872|

Publication uses already encoded bytes; total save also includes encode/validation.
The receipt separately reports in-memory restore/validation. History and full-state
copies dominate storage; bounded cognition does not bound audit cost. Four immutable
generations duplicate history. Runtime is adequate for this finite scenario in the
measured sample; long-running service storage, compaction, large-population costs,
GUI responsiveness and sustainable economy remain unproven.

## Completed checks and scope

Commands from repository root, on the product fingerprint above with pending product
and navigation/document changes. Verbose logs are outside protected archives in
`%TEMP%/eldoria-settlement-20261118/`.

| Command | Result |
|---|---|
| `cargo fmt --check --manifest-path products/settlement/Cargo.toml` | Exit 0 |
| `cargo clippy --locked --all-targets --manifest-path products/settlement/Cargo.toml --target-dir target/settlement -- -D warnings` | Exit 0 |
| `cargo test --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement` | Exit 0; 12 domain/continuation tests + 2 real CLI tests, 0 failed |
| `cargo bench --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement --bench settlement` | Exit 0; six measured points; all encoded/restored and published/loaded states agree |
| `cargo build --release --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement` + real CLI campaign | Exit 0; both 8/12 exact resumed/uninterrupted checkpoints |
| `cargo fmt --check` | Exit 0 |
| `cargo clippy --all-targets -- -D warnings` | Exit 0 |
| `cargo test` | Exit 0 ; 217 passed, 0 failed |
| `cargo run --release --locked -j 1 --example validate026_foundation` | Exit 0; includes unchanged `validate_contract` (001–024), separate 025 foundation and isolated 026 C1 checks. 495 portable/512 raw originals, 93 custody bindings and 3 layout fault rejections pass. |
| `python -B examples/support/validate027.py` | Exit 0; source/publication, 100 inherited C1 + 50 Fixed parity, 100 exact native replays, 100 independent reconstructions and 17 scratch fault rejections pass; no protection extension. |

No root throughput benchmark is needed: root runtime/workload are unchanged; the
new product has its own relevant benchmark. No GUI/browser smoke check is applicable
because no viewer/renderer/adapter was modified. No GUI delivery, LLM behavior or
hybrid-planner efficacy is claimed.

## Next milestone

A separate observer application should visualize the three locations and current
commitments, follow a person, switch explicitly between actor and observer views,
page causal history and expose run/pause/step plus complete-state save/reopen.
Keep simulation on one worker and send bounded copied projections; do not attach
the research Explorer to this schema implicitly. A user should be able to trace a
request through consent or refusal to actual transfer/work and later needs/trust.
Demonstrate that workflow in the real UI before claiming an enjoyable observer
release. Politics, generational history, renewability and inference remain later
independently scoped product work.
