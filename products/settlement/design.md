# Settlement v1 rules and prospective implementation contract

This standalone product package implements the first headless settlement milestone.
It is neither experiment028 nor adoption/extension of protected research. Root
Cargo files, research source, historical hosts and scientific artifacts stay exact.
References: [feasibility](../../docs/planning/living-world-feasibility-assessment.md)
and [intelligence](../../docs/planning/hybrid-character-intelligence.md).

Hypothesis: needs, finite food, local knowledge, heterogeneous preferences, voluntary
food transfers and more efficient joint gathering can generate consequential
individual histories without post-initialization scripts. This may produce refusals,
failed work, unequal outcomes and depletion; no cooperative quota or story is imposed.
If representative seeds yield only idle/repetitive/inconsequential behavior, inspect
the decision records and revise product rules with explicit versioning and tests.

## Ownership and phase order

`model` owns serializable domain state. `policy` receives only `OwnedView`; it has no
ECS handles, observer archive or resolver access. `runtime` owns Bevy ECS, stable-ID
index, one clock and a single-thread schedule; `storage` owns immutable complete-state
checkpoints. `observer` returns copied read-only projections. The CLI owns pacing and
files, never actor choices. All mutations use the same resolver, including future
controllers. Planning/proposal interfaces are typed seams only; no planner or LLM.

One step advances logical time from t to t+1: complete due work in commitment-ID
order; apply hunger pressure (3 every 4 ticks, capped 100); refresh local observations
every 8 ticks or on arrival/completion; collect independent invitation responses;
collect all free actors' decisions from the same public phase; admit chosen intents
in seeded rotating roster order. All admission rechecks authoritative legality.
The saved RNG draws one roster rotation per boundary, not hidden policy information.
Sparse stable IDs never serve as array indices. Pause means no step is executed.

There are three connected locations (square↔grove↔orchard). Topology is public;
stocks and private actor state are not. Travel takes 2 ticks and sensing stops while
in transit. Gathering takes 3 ticks and removes 1 edible food at completion. Joint
gathering takes 2 ticks, requires independent consent, reserves both actors, and
removes 2 food atomically, one per actor; insufficient remaining stock causes failure.
No stock reservation means competing work can fail. Eating takes 1 tick, consumes 1
food and relieves 35 hunger. Food transfer takes 1 tick after acceptance; it reserves
both parties and moves 1 food without creating any. Sending an invitation takes 1
tick; declining costs 1 tick; acceptance is included in the accepted job's duration.
Unavailable/invalid invitations expire explicitly.

Only the actor's own inventory/hunger/traits, retained dated site beliefs, bounded
relationships and episodes, co-present availability/signals, incoming messages and
public topology enter a decision. A request is a public signal, not a reading of
private hunger. Memories cap 16, site beliefs 8, directed relationships 32 (insertion
FIFO); trust survives episode eviction. Failed/declined contacts have a 12-tick local
cooldown. Unobserved rival depletion is not inferred from the objective archive.
Scores and exact tie ordering live in `policy.rs` and are recorded for every decision.

## Bounds, continuation and falsification

The horizon is explicit. No new job may finish after it; final steps resolve existing
work without creating ordinary choices at the endpoint. All sources are finite;
there is no renewal, death, political system or sustainability claim. Total source
food + owned food + consumed food must equal the initial total. Each actor has at
most one commitment; invitation offers do not grant recipient consent. Every decision,
admission, completion, refusal and observation has a stable record ID and causal link.

Checkpoints contain every actor, private cognition, pending jobs, counters, logical
clock, RNG state and structured history. They carry a new strict schema/rules version
and checksum; they never infer cognition from the journal. Save creates a new file
and never overwrites a previous generation. Incomplete pending files are not accepted
as committed saves. Resume is explicit; closing a process advances no simulation time.
Ordinary process restart is supported; sudden-power-loss durability and migration
are not claimed. Observer exports are not accepted as checkpoints.

Acceptance must cover 8/12 actors, sparse IDs, independent refusal/cooperation,
competition and food balance, hidden-state twins, impossible intents, every safe
save split in representative runs, cross-process CLI continuation, corruption/version
rejection, and reproducible histories. Measure advance/save/restore/history growth
separately. Keep wall-clock data outside deterministic histories. Run unchanged
research gates and root checks before publishing; do not regenerate frozen evidence.
