# 026 build isolation — post-run I1 verification transport

The first full C1 wrapper failed while checking the preserved v1 failure. Both
workspaces had the same Cargo package name and used `target/delayed-work-valuation`.
Their generated host output directories collided. Rebuilding v1 reused C1 host
output, producing an invalid100-row v1-labelled artifact instead of the required
43-row panic. Its source-input digest alone did not prove artifact lineage.
The failure log/checksums are preserved separately; those outputs are not science
archives and never replace either v1's failure prefix or C1's registered dataset.

No protected source, original16-file v1 freeze,16-file C1 freeze, matrix, scoring,
reader algorithm, assertion or archived trial was changed to accept this result.

The post-run I1 gate executes a counted/reversible copy of the frozen C1 verifier:
only its v1 build target, C1 build target and failure-helper path differ. The
failure helper is itself a counted copy with only its executable path changed.
All membership/source checks,100 independent reconstructions,50 pair comparisons,
17 corruption checks, exact100 replay and exact43-row v1 panic remain required.
The frozen originals remain available, including their build-path limitation.

Current verification:

```
cargo run --release --locked -j 1 --example validate026_c1_isolated
```

Native build targets are disjoint:

- v1: `target/026-v1-isolated`
- C1: `target/026-c1-isolated`

The current native C1 command uses the same frozen manifest with
`--target-dir target/026-c1-isolated`. The old shared-target instructions in the
frozen README are historical; do not use them for cross-workspace provenance.
Source identity remains7039076f147e173359e4c3099e35de9ccd4e9a590fd54ad2960c2b64f76f21c5.

This is a post-outcome engineering correction, not preregistered validation
transport. It must be disclosed. Exact replay from isolated frozen sources must
confirm the existing dataset before scientific completion is claimed. No native
binary release, GUI acceptance, general optimization or new experiment follows.
