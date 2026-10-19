# First Continuous World prototype (opt-in Experiment023)

Run from the Eldoria repository root. Tested on Windows with Rust/Cargo1.99.0,
Python3.12.6 and Node24.13.0 (Node is needed for viewer tests, not simulation).
Cargo manifests retain the repository's Rust1.95minimum and pinned dependencies.
Python is needed by the source-preserving build generator. No API keys/services.

```powershell
cargo run --release --locked -j 1 --manifest-path prototypes/continuous-world/Cargo.toml --target-dir target/continuous-world --bin continuous-world -- run target/my-continuous-world.jsonl --case paired --ticks 192 --period 11 --seed 42
python -B tools/history-explorer/explorer.py --continuous experiments/023 --continuous target/my-continuous-world.jsonl
```

The output filename must be NEW. Choose another filename for a repeat; existing
files and protected001-022/reinforcement output locations are rejected. If8765is
already in use, add `--port 8769` to the viewer command. Select **023 - Open actual
prototype run** (the button uses a middle dot in the interface), then Observer
Perspective. Browser playback reads an already executed history; it does not
advance or control the world. `--continuous` is an explicit viewer source opt-in.
The ordinary viewer still defaults to its historical019example.

`--ticks 192` gives192ticks of ordinary decision opportunities after the controlled
bootstrap. The SAME world then drains already proposed/admitted work for33real
ticks; no new ordinary choices occur during drain. For this example: start6,
nominal stop198, final231. Needs still advance. Final silence is a testing endpoint,
not a voluntary decision to stop living. Reporting period11does not reset anything.
This command can produce severe hunger and repeatedly useless contact.

For the already executed demonstration used in the program:

```powershell
python -B tools/history-explorer/explorer.py --continuous experiments/023 --continuous target/first-continuous-world-demo.jsonl
```

`target` is ignored and must be recreated after a fresh clone using the run command
with that new filename. The tracked originals are always available through
`--continuous experiments/023`. The demonstration is a custom run, clearly labelled
outside the registered scientific matrix; it is not substituted for original data.

Read-only validation and replay:

```powershell
cargo run --release --locked -j 1 --example validate023
python -B examples/support/validate_continuous_demo.py target/my-continuous-world.jsonl
cargo run --release --locked -j 1 --manifest-path prototypes/continuous-world/Cargo.toml --target-dir target/continuous-world --bin continuous-world -- replay target/my-continuous-world.jsonl
```

Supported bounded configuration: `--ticks`1-1024, `--period`1-128, `--seed`u64,
`--claim`2/4/8, `--order`Ascending/Descending/Rotating, `--phase`Epoch/Absolute,
`--mode`Continuous/Windowed. `--case` must be one of the registered023initialization
families. Four actors, capacity2, cadence8, policy scores, retention and physical
laws are fixed. A nonregistered parameter combination is a custom run, not new
accepted evidence. No source edits are needed. Use `matrix NEW.jsonl` only for a
new isolated rerun; never regenerate scientific archives in place.

Implementation: a separate Cargo workspace compiles exact frozen root modules by
source path. Generated module-path wrappers and an appended private023child reuse
existing clock, physical/cognitive rules and bootstrap. All100original022worlds
reproduced exact bytes before treatment. The80file original source freeze remains
unchanged. Root001-022entrypoints/defaults are untouched. Build-generation coupling
and cold duplicate compilation are limitations; only this Windows/ASCII-workspace
configuration was tested. This is not a general event engine or planner.

Inherited episodic memory cap is16; retained evidence/provenance/learning cap32,
concerns8per actor, environmental slot1per actor. Legacy trust/concern/expectation
consequences can outlive evicted episodes under their established contracts.
Observer records never become a replacement cognition source.

See [walkthrough](../../docs/development/first-continuous-world-walkthrough.md),
[results](../../experiments/023/results.md), [verification](../../experiments/023/verification.md)
and [readiness](../../docs/development/continuous-world-readiness.md). No global
adoption or023foundation protection; no production or024implementation.
