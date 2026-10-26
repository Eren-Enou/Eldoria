# 026-C1 verification — completed with explicit qualifications

Current complete read-only gate:

```
cargo run --release --locked -j 1 --example validate026_c1_isolated
```

It passed100 independent full reconstructions,50 matched comparisons,17 corruption
categories,exact100-row C1 replay,and exact43-row v1 failure reproduction from
separate native builds. All original frozen algorithms/assertions are retained.
The I1 projection changes three transport paths and one subordinate executable
path,with counted substitution and exact reverse checks. It changes no scoring,
metric,denominator,configuration,source,threshold or assertion.

The original shared-target wrapper failed because generated host output crossed
between identically named Cargo packages. That log/artifact evidence is preserved;
it is not a poor behavioral outcome or a completed v1 run. The untrusted mixed
output is never promoted. The original C1 verifier/source remains frozen. See
docs/development/experiment-026-build-isolation.md for the post-run correction.
Cold isolated builds reproduce the registered C1 bytes exactly. After both warm
builds,v1 still fails identically at43 and C1 still replays all100 exactly.

Before unknown C1 treatment:50 original025 Baseline bodies,56 feature-off bodies,
all43 observed v1 bodies,6509 original/4858 prefix response equalities,independent
50+43 reconstructions and17 corruptions pass. Four old policy tests and three
new response tests join nine inherited tests:16 prototype tests. Root fmt,locked
all-target Clippy and217 tests pass;private model/generated responder formatting
is explicit. All16 C1 runtime/reader files match their published freeze. All16
original-v1 files and every v1 failure member remain unchanged.

Required historical full gate previously passed:
`cargo run --release --locked -j 1 --example validate025_foundation`,which runs the
unmodified historical001–024 archive/replay gate first,then025 acceptance/review.
Its source/fixtures remain unchanged,so the long suite was not repeated without
reason. Current I1 verification rechecks455 portable artifacts,472 raw originals,
214 candidate custody bindings and C1/v1 source identities before and after.

Publication order:001–025 protection0a33e38;026 registration759385d;v1 runtime
42d3ad3/activationd2321f1;preserved failure+C1 amendment6f44686;C1 runtime22b5148;
C1 activation2a5bec1;then the100-row recorded invocation. All publication identities
were matched to origin/main before the next phase. I1 is a disclosed post-run
engineering transport correction,not preregistered reader logic or new behavior.

Primary96 rows and four reporting rows are retained. Raw332,411,125 bytes and
gzip8,861,475 bytes are bound by archive-manifest.json. Findings/comparisons/
corruptions are independently regenerated and checked,not accepted merely because
the manifest reproduces. Wall-clock/peak data are separate from simulation JSON.
The normal v1 validator remains incomplete and is not weakened to accept43 worlds.
No default adoption,foundation026 extension,GUI acceptance or next experiment.
