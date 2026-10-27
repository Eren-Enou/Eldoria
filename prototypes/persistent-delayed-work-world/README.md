# Persistent delayed-work world (headless, opt-in)

Engineering successor for the exact accepted026 C1 mechanism. This is not027,
policy adoption, a new simulator, a GUI release or a horizon extension. Current
acceptance is recorded in docs/development/persistent-delayed-work-world.md and
the continuation checkpoint; the presence of these commands alone is not a pass.

Build from repository root:

```powershell
cargo build --release --locked -j 1 --manifest-path prototypes/persistent-delayed-work-world/Cargo.toml --target-dir target/persistent-delayed-work-world
$bin = (Resolve-Path target/persistent-delayed-work-world/release/woi-persistent-delayed-work-world.exe).Path
& $bin identity
& $bin parity experiments/026/continuation-c1/trials.jsonl.gz
```

The package and target are distinct from both frozen native026 producers. Never
reuse their shared old target. `identity` reports the engineering runtime and the
counted source seams; `parity` checks all100 full typed original archived C1 histories.

## Explicit configuration and actual continuation

`config SOURCE025 Baseline|ProjectedNeed NEW-OUTPUT` selects one of the exact100
registered configurations. Both modes require an explicit argument; there is no
implicit mode or arbitrary new world configuration. Source025 is0–49, not026 row
ordinal. The outer mode is delayed-work valuation; inherited024 valuation remains
Baseline, as in original026. Existing MealRelief and original hosts stay unchanged.

Use a fresh absolute directory outside scientific archives. Example after build:

```powershell
$root = Join-Path (Get-Location).Path 'target/persistent-C1-demo'
New-Item -ItemType Directory -Path $root
$config = Join-Path $root 'source23-ProjectedNeed.json'
$world = Join-Path $root 'world-23'
& $bin config 23 ProjectedNeed $config
& $bin create $world $config
& $bin advance $world 5
& $bin events $world 0
& $bin save $world
& $bin status $world
& $bin advance $world 5 --codec zlib
& $bin storage $world
$status = & $bin status $world | ConvertFrom-Json
& $bin verify $world --generation $status.latest_committed_generation
& $bin advance $world 2048
& $bin finalize $world
```

Each invocation is a separate process. `advance` executes bounded scheduler
iterations and saves. Steps are not always ticks: an atomic inquiry remains
indivisible. `save` without changed state is a no-op, so always inspect the actual
generation instead of assuming an increment. Exiting does not finalize work or
advance logical time. Nominal cutoff and the charged33-tick drain stay original.

`status` shows explicit mode, runtime/policy identity, current logical time,
terminal state, actual commitments, inventories, raw stock, owned escrow, waste,
checkpoint generation and journal size. This is an observer view. Actual source
stock shown here does not enter an individual's scoring. `events DIR BOUNDARY`
returns an executed journal record; a boundary ordinal need not equal its tick.
`state DIR NEW-OUTPUT` writes the complete typed checkpoint envelope for comparison.

## Storage and explicit recovery

```text
check ABSOLUTE-WORLD-DIR
storage ABSOLUTE-WORLD-DIR
verify ABSOLUTE-WORLD-DIR --generation N
interrupt ABSOLUTE-WORLD-DIR STEPS
recover ORIGINAL-WORLD FRESH-BRANCH-ROOT/SAME-WORLD-ID --generation N
advance FRESH-BRANCH-ROOT/SAME-WORLD-ID STEPS
finalize FRESH-BRANCH-ROOT/SAME-WORLD-ID
```

Create the fresh branch root first; the final directory must not exist. Recovery
selection is mandatory. It preserves parent bytes, separately retains excluded
verified committed history and unverified suffix, and resumes the selected actual
state. It does not replay a tail into cognition. `interrupt` intentionally leaves
synced journal progress without a new checkpoint; ordinary reopen must refuse that
unsaved endpoint. It is not a power-loss test. Storage inspections never promote
.pending files, silently roll back or delete generations. Raw/zlib are transport
choices; typed state is identical. No automatic cross-runtime migration exists.

## Evidence ownership and validation

Checkpoints use `woi-delayed-work-world-checkpoint/v1`, with explicit registered
mode, policy identity and strict engineering runtime identity. The existing
material host and its schema are untouched. Final history uses an engineering
wrapper containing actual runtime identity and `c1_projection`. The projection is
the exact canonical accepted C1 format; its source-freeze value identifies that
frozen mechanism, not a claim that the engineering binary was the original producer.
Original scientific archives are read-only and never replaced by these exports.

Run the real-process campaign into a NEW absolute ignored output directory:

```text
python -B examples/support/validate_persistent_delayed_work.py --binary ABSOLUTE-EXE --output NEW-ABSOLUTE-TARGET-DIRECTORY
```

The campaign begins with uninterrupted100-row parity, then independent processes,
full-state hashes, terminal equality, independent resumed-history reconstruction,
recovery/storage faults and separate wall-clock measurements. See the final
engineering receipt for actual completed counts and any failed/incomplete gate.
No GUI/computer control, default change, canon, renewal or population mechanism.
