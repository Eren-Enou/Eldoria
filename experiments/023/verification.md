# Experiment023 verification

Scientific gate: **PASS**, qualified interpretation in [results](results.md).
No001-023foundation extension or default adoption. All original001-022evidence and
all original023runtime/configuration/archive bytes remain unchanged.

## Actual commands and scope

| Command / gate | Result |
|---|---|
| `cargo fmt --check` | PASS; 0.913s |
| `rustfmt --check --edition 2024 prototypes/continuous-world/src/model.rs prototypes/continuous-world/src/coordinator.rs` | PASS; 0.083s |
| `cargo fmt --manifest-path prototypes/continuous-world/Cargo.toml --check` | PASS; 0.234s |
| `cargo clippy --locked --all-targets -j 1 -- -D warnings` | PASS; 0.683s |
| `cargo clippy --locked --all-targets -j 1 --manifest-path prototypes/continuous-world/Cargo.toml --target-dir target/continuous-world -- -D warnings` | PASS; 9.332s |
| `cargo test --locked -j 1 --manifest-path prototypes/continuous-world/Cargo.toml --target-dir target/continuous-world` | PASS; 24.339s |
| `cargo test --locked -j 1` | PASS; 117.477s |
| `cargo run --release --locked -j 1 --example validate023` | PASS; 23.796s |
| `cargo run --release --locked -j 1 --example validate_contract` | PASS; 1439.006s |

Root tests:217tests across28nonempty groups. Original prototype tests:3passed;
an additional exact16episode-cap regression also passed, with current full
prototype suite4tests/format/Clippy checked after adding it. Generated include
modules require explicit rustfmt checks, recorded above. The final normal
001-022foundation gate took1439.006s and checked361original artifacts, inherited
contracts, exact source/standalone-prefix identities, independent readers and
historical replays. It includes the independent022review anchors/4additional
controls alongside original022's100byte reruns/23corruption controls.

All15legacy protected standalone commands (009-021and020/021foundation wrappers)
passed during GateB; original022standalone passed GateAand its exact shared prefix
ran again in GateB/final foundation. Their library/fixture sources are unchanged.
[GateB executed record](../../docs/development/continuous-world-validation.json).
No historical archive writer was run. The new `validate023` remains separate from
`validate_contract`; root main/default behavior and protected endpoint remain001-022.

`validate023` verifies80pre-run sources,5original validation-helper identities,
original gzip/decoded/member hashes, the42rowregistration, all42independent full
primitive/cognitive reconstructions,19scratch categories and42exact Rust byte
reruns through the isolated host. The source host also reproduced all100original
022worlds before treatment, with the023module, and after formatting. Root source
bindings64unchanged. Runtime policy/physical/cognitive bodies are not rewritten.

Original023gzip SHA256:
`c4bd87738a09bb93c0dc3bbe8214ff312738c8a530b82158968e83f9e22387d4`.
Decoded JSONL SHA256:
`657d6fe488bbaf04f9da2ba37391fd5be7cec06fd9de2508f4374f6daeb8487a`.
42members;75343581decoded bytes;1811033gzip bytes. Member hashes are in
[archive manifest](archive-manifest.json); pre-run source identities in
[source freeze](source-freeze.json). These were not regenerated to remove failures.

## Independent supplemental checks

```powershell
python -B examples/support/slices023.py --verify
python -B examples/support/coverage023.py --verify
python -B examples/support/review023_endpoints.py --verify
python -B examples/support/review023_inquiry.py --verify
python -B examples/support/validate_continuous_demo.py target/first-continuous-world-demo.jsonl
```

All executed successfully. Twenty matched pre-choice prefixes retain identical
inputs/opportunities apart from the registered publicend; exact causal references
are checked. Coverage16/16registered families has actual witnesses. Endpoints
reconstruct all nominal/drain hunger integrals and reject two supplemental scratch
variants: a decision beyond actual finalH, and an incorrect atomic query deadline.
All8executed Direct inquiries have explicit receipt routes/progress audited,
distinguishing null native response from actual provenance delivery. QueryH3
regression spans nominal stop7with actual finish19and final40, no new ordinary
choices after cutoff. No query/claim deadline is skipped to force an effect.

The actual192tickprototype demo independently reconstructs and byte-replays; its
start6/nominal198/final231,89social scenes,10strict crossings and finalfood[1,1,0,0]
are recorded. This custom run is not added to the42original scientific outcomes.

## Integrity and corrections

A fresh final guard confirms all378initial tracked experiment/reinforcement working
files byte-identical and all361protected entries equal under inherited hashing
conventions. Files013-022text use canonicalLF; older inherited conventions remain
unchanged. A first ad-hoc blanketLF guard was wrong for older files; the corrected
per-contract guard passes. The original023source freeze and validation helpers also
match. [Final executed metadata](../../docs/development/continuous-world-final-validation.json).

The prospective architecture said32episodes; actual inherited source cap16was
preserved. Formal preregistration required unchanged cognition. The original loose
<=32regression was insufficient alone; a supplemental exact-cap test now checks
both modes. Only documentation/validation were corrected; no archived output,
registered specification, runtime source, retention or policy was changed.

Initial prototype Clippy rejected constantchunks_exact CLI parsing; usingas_chunks
resolved it before original outcome generation. No behavior formula was retuned.
The first preview test invocation used the wrong discovery directory and ran zero
tests(exit5); corrected discovery in tools/history-explorer executed all4tests
successfully. A browser test caught zero-width checkpoint click targets; visible
rectangular hit regions fixed it. These engineering corrections are separate from
scientific outcomes and are not hidden as passing initial checks.

## Viewing and cost

The main native023adapter preserves the original envelope losslessly, actual
physical-history schema, zero-based member identity, hashes, four captured owned
perspectives, reporting/nominal/final references and typed delivery joins. It imports
no evaluator/scientific reader. Actor checkpoints/atomic interiors are not filled
from observer history; query private response context stays Observer-only.
Full main-viewer, JavaScript, old-preview and browser acceptance details are in
[main023 acceptance](../../docs/development/main023-viewer-acceptance.md).

Separate performance evidence: [samples](performance-samples.jsonl),
[original run process peak](performance-original-run.json),
[replay/reader/archive costs](performance-gates.json), [root throughput](throughput.txt).
The first throughput attempt briefly overlapped viewer discovery; its temporary
log was retained and superseded by the final isolated successful run. No general
optimization or larger-population scalability claim. Live payload/serialized bytes
are labelled separately from process working set and growing observer history.

Current measured toolchain: Rust/Cargo1.99.0, Python3.12.6, Node24.13.0on Windows.
Verbose command logs remain task-specific temporary files; durable JSON records
retain commands, exit statuses, costs and scope. No024or material-production code.
