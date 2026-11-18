# Eldoria settlement

A working headless, finite settlement simulation in a separate Rust/Bevy ECS
workspace. Eight characters are the default; twelve is the tested stress case.
There are no scripted events after initialization, LLM calls, graphics, factions,
renewable resources or changes to the research simulator.

## Run from the repository root

Requires Rust 1.95 or later and Cargo. Dependencies are locked independently of the
root research package. First build may download crates; execution needs no network.

```powershell
cargo build --release --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement
$bin = (Resolve-Path target/settlement/release/eldoria-settlement.exe).Path
New-Item -ItemType Directory -Path target/my-settlement
& $bin init target/my-settlement/initial.json --population 8 --seed 42 --horizon 512
& $bin run target/my-settlement/initial.json --ticks 64 --out target/my-settlement/paused.json
& $bin inspect target/my-settlement/paused.json
& $bin inspect target/my-settlement/paused.json --actor 1001
& $bin events target/my-settlement/paused.json --from 0 --limit 12
& $bin save target/my-settlement/paused.json --out target/my-settlement/bookmark.json
& $bin resume target/my-settlement/paused.json --ticks 448 --out target/my-settlement/final.json
```

Use a fresh directory/filename when repeating these commands. `save` makes another
identical checkpoint, without stepping. Every `run`/`resume` stops at a safe boundary
and saves actual state; no background process continues ticking. Stop advancing to
pause; closing/reopening the program advances no time. `--ticks 0` is inert. Requests
past the finite horizon stop at the endpoint; they never restart the scenario.
For twelve characters, initialize another directory with `--population 12`.

On Unix use `target/settlement/release/eldoria-settlement` directly. The code uses
portable Rust APIs, but acceptance was performed on Windows, not a Unix host.

`inspect` is explicitly omniscient observer data. `inspect --actor ID` is the
character's own retained knowledge plus current co-presence, not private partner
inventories, hunger or beliefs. `events` returns up to 1000 records per page. Character
IDs are stable unsigned 64-bit values (default 1001, 1018,…), never ECS handles. Future
JavaScript consumers must preserve these integer IDs losslessly rather than round
large custom IDs through a JavaScript `Number`.

## What happens

People begin in Lantern Square, Bramble Grove or the Old Orchard with different
food, hunger, generosity, caution and reserve preferences. They can eat, gather,
travel to adjacent places, request/offer one food, or invite someone to gather
together. Joint gathering takes two work ticks after sending an invitation and
yields one food each; recipients may prefer their own needs or solitary work.
Public requests can attract offers. Relationships change
through experienced help/refusal; retained episodes also impose a contact cooldown.

Finite sources can be exhausted. Jobs contend for stock at completion, so successful
admission does not guarantee food. People may refuse, fail, wait or remain hungry.
No rule forces a particular amount of cooperation or equal outcomes. See
[acceptance and measured examples](../../docs/development/settlement-runtime-acceptance.md)
for observed behaviors, counts and negative results.

## Architecture

| Module | Responsibility |
|---|---|
| [model.rs](src/model.rs) | Serializable characters, dated site beliefs, bounded relationships/episodes, typed actions, commitments, decisions, events and saved RNG |
| [policy.rs](src/policy.rs) | Pure local utility choices; named integer score terms and deterministic ties; future `Controller`/`ProposalSource` interfaces |
| [runtime.rs](src/runtime.rs) | Bevy ECS components, recreated ID index, single-thread schedule, one clock, independent recipient decisions, common legality, reservations and authoritative consequences |
| [storage.rs](src/storage.rs) | Checksummed complete-state generations, strict schema/rules/source identity, no-overwrite publication and restoration |
| [observer.rs](src/observer.rs) | Copied observer snapshot and bounded read-only history pages; never policy input |
| [main.rs](src/main.rs) | `init`, `run`, `resume`, `inspect`, `events`, `save`; bounded stepping and fresh output files |

The [rule sheet](design.md) specifies phase order, durations, sensing, capacities,
food balance and the finite endpoint. The design adapts the research's principles
of local utility, bounded memory, trust, voluntary consent, stable identity and
resource accounting. It deliberately does not link protected research modules or
their source-generating hosts: their action domains, fixed populations and schemas
do not fit this product. Bevy ECS/Serde and the pinned SplitMix64 arithmetic are
reused as infrastructure/patterns; no historical policy-adoption claim follows.

Only `OwnedView` reaches a controller. Both recipient and initiating choices are
recorded before authoritative admission. Every candidate has named score terms;
the history distinguishes proposal, admission, rejection, completion and refusal.
Trace `Record.cause` to the deciding record and `Commitment.cause` to the decision
that started work. Invitation views link to the sender's completed message. Shared
outcomes affect both participants even when the event's subject is one participant.

## Saves, limits and reproducibility

Saves contain logical time, seed configuration, the live RNG stream, all characters
and retained cognition, unfinished commitments, counters, source stocks, consumed
food and structured history. ECS entities/schedules are recreated, not serialized.
No sensing, bootstrap or replay into cognition occurs on load. Same seed, roster,
runtime and advance sequence produce identical checkpoint bytes; splitting advances
at safe boundaries preserves that identity.

The envelope has its own `eldoria-settlement-checkpoint/v1` schema, product rules
identity and SHA256 fingerprint of runtime sources/manifest/lockfile. Unsupported
runtime identities are rejected; there is no migration or fallback to a research
reader. Changing implementation source deliberately invalidates earlier development
saves. Checksums detect accidental corruption; they are not hostile-tamper signatures.

Writes sync/read back a new `.pending` file, then use hard-link publication that
cannot overwrite an existing generation, and remove the pending name. Pending paths
are refused by `load`; they are never automatically promoted. This requires a
filesystem supporting hard links (tested on local Windows storage). Failed
publication retains pending evidence. Ordinary process restart is tested;
power-loss/filesystem-directory durability is not claimed. Parent directories must
exist; unsafe archive, traversal and linked output paths are rejected.

Configuration bounds: 1–128 characters, 8–4096 ticks and at most 32768 actor-ticks;
the acceptance focus is 8/12, not large-population performance. Per character:
episodes 16, site beliefs 8, relationships 32, insertion FIFO. Episode eviction does
not erase separately retained trust. The current map has only three places.
Hunger saturates; there is no mortality or indefinitely sustainable economy.

History grows with decisions/observations, and checkpoints currently retain that
history. Save/load validates the complete prefix, so their time and memory grow.
File loads are bounded at 256 MiB. This is a finite observable foundation, not an
always-on production service. Do not feed observer exports into character cognition
or regard them as loadable checkpoints.

## Verify and measure

```powershell
cargo fmt --check --manifest-path products/settlement/Cargo.toml
cargo clippy --locked --all-targets --manifest-path products/settlement/Cargo.toml --target-dir target/settlement -- -D warnings
cargo test --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement
cargo bench --locked --manifest-path products/settlement/Cargo.toml --target-dir target/settlement --bench settlement
```

The benchmark prints JSON to stdout; timing stays outside deterministic world
records. It measures stepping, encoding/validation, publication, restore/validation,
history records and checkpoint bytes for 8/12 people at 64/256/512 ticks. Tests cover
autonomy, locality twins, consent/refusal, depletion, sparse identities, every safe
boundary in representative histories, every work type and actual multi-process CLI
resume. The acceptance report records the exact completed checks and scope.

## Next observer milestone

Build a separate read-only desktop dashboard over `observer::snapshot`,
`Settlement::owned_view` and paged records. Start with a three-node map, people and
activities, run/pause/step, actor/observer perspective switching, food balances and
decision→consent→work→consequence links. Keep the ECS world on one worker and publish
bounded snapshots to the UI; presentation speed must not alter logical decisions.
Do not load full history into each frame or treat a selected timeline position as
restored cognition. Prove actual save/reopen and pending-work inspection in the UI.
No renderer, viewer integration or live network service is claimed by this milestone.
