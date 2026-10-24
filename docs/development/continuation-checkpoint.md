# First persistent world — active continuation checkpoint

Objective: attachment2ac9c96d. Starting HEAD
`77707def0e13936c192a3bc987b8971acb3898d1`; implementation HEAD
`e969eb5` (81b5ce1 audit,7351464 continuation,e969eb5 workbench). Changes are related
isolated continuation, workbench integration and documentation; protected sources
and frozen archives remain unchanged. No026 or025 adoption.

Architecture: [engineering audit](persistent-world-engineering.md). Complete typed
023/024 state, original unchanged coordinator/policies, stable IDs rebuilding ECS
mapping, fixed horizon and charged drain. Native CBOR checkpoint plus immutable
commit generations bind state and append-only actual journal prefix hashes.
One exclusive OS writer lock; no replay or archive recovery. Incomplete files are
preserved; a journal tail beyond the committed checkpoint is rejected explicitly.

Current evidence and limitations: [acceptance report](persistent-world-readiness.md).
Root fmt/Clippy/217tests and full read-only001–024 gate passed (408 artifacts,
425 raw originals). Separate025 gate passed214bindings/50replays. Current release
suite10 passed18tests: exact original histories,228long splits, real process
restarts,024cases, sensitivity, semantic corruption, observer nonrecovery,
writer exclusion and metadata/journal binding. Final component formatting and
desktop/portable all-target Clippy pass. No code changed since these checks.

Actual static candidate native acceptance: seed43 paired023 H1024/P11, saved/reopened
pending invitations tick7 and active resource commitments tick9, relocated into a
second application path with spaces, advanced to281, inspected actual local input
and executed transfers. Computer Use was stopped by human Escape before Pause and
terminal Finalize acceptance; do not resume UI automation without human direction.
Application subsequently absent; checkpoint reopened successfully at281. A separate
copy continued/finalized through packaged backend exactly matched independent
original023 executable; separate024 MealRelief backend process continuation also
matched original024 executable. Original GUI world remains untouched at281.

Candidate (dirty build, NOT a release):
`target/releases/woi-workbench-0.3.0-rc.1-windows-x64-preflight.zip`.
Machine-local evidence/logs: `target/persistent-world-validation/`,
`target/persistent-world-*.log`. Actual external app/data locations are in
`target/persistent-world-validation/gui-launch.json`.
H128/H1024 wall-clock and serialized growth measured; see acceptance report.
No single-world native memory/precise UI latency or clean-VM/power-loss claim.

Workbench all-target release suite with `--features desktop,portable` completed
successfully:21tests, no failures/ignored tests, including legacy execution,
replay/notes/relocation, original transport parity and separate025developer cases.
Log:`target/persistent-world-workbench-all-tests.log`. Session38686 is complete.
The prior command failed before compilation because `portable-app` was an incorrect
feature name. Clippy session11204 completed successfully. No builds remain live.

Remaining: native Pause/terminal and024GUI acceptance
when user permits UI work; memory/responsiveness evidence; final requirement audit,
diff review, validated commits/push, clean-source static rebuild, final package
verification and publication. Do not publish the dirty preflight ZIP or mark the
goal complete while these gates remain unproven. The pending human question asks
whether desktop automation may resume after Escape; no response is assumed.

## Preceding delivered portable milestone

Objective: attachmenta5b07201. Deliver portable Rust Windows023/024 human
operation; preserve developer025, all original producers/evidence and V2 work.
Protected001–024 and separate unadopted025 remain unchanged. No026.

Implementation commits:798c647(audit),b2b9efd(runner),ff3284c(installed backend),
79a1283(native moments/notes),273ae46(packaging), followed by acceptance docs.
Use the delivered release.json for the exact clean build revision and binary
checksums; use Git/actual remote verification rather than assuming this note is
current. Six requested engineering boundaries are kept in separate commits.

Validated: root fmt/Clippy/217tests; full read-only001–024 gate408artifacts/425raw
originals; separate025 acceptance214bindings/50exact replays;19explicit workbench
tests; all12public023/024/025 cases;36original/portable byte comparisons; static
CRT/import inspection; two relocation paths/tool-free PATH; missing/corrupt
assets, schema/hash/path/junction controls; exact reproduction and old/new notes.
Actual extracted GUI: seed43, genuine execution, person/moment/local alternatives,
execution/resource consequences, note save, exact replay, close/reopen and note
recovery into the form. Historyda8611ce53eb8b51db17c500a6e94e4cdde443107388c905ca1e0f8b78332230.

Delivery paths:target/releases/woi-workbench-0.2.0-rc.1-windows-x64.zip and adjacent
SHA256SUMS.txt. Machine-local final byte/source/remote/status receipt:
target/portable-workbench-validation/final-publication.json. Those ignored local
artifacts are not present in a fresh clone; the engineering report gives exact
rebuild/package/test commands. Do not infer publication from a filename alone.

Boundaries:023default,024explicit Baseline/MealRelief;025requires full developer
preflight and is unavailable in the portable runner. Python Explorer stays an
optional developer bridge. User data is outside installation/checkout. Native
readers stream a compact index and selected detail; human notes never feed actors.
Clean VM and firewall-isolated offline acceptance were unavailable and are not
claimed. Windows x64/system graphics is the tested platform. License inventory
includes actual desktop dependencies/fonts; no first-party license was invented.

[Portable guide](PORTABLE_WORKBENCH_QUICKSTART.md),
[engineering/acceptance](portable-workbench-engineering.md),
[preceding V2 evidence](rust-workbench-v2.md).
Next development: hands-on independent observation and usability feedback. A new
behavioral experiment requires separate evidence and preregistration; no automatic
026 or policy adoption follows from this delivery. Preserve all archived mistakes.

Post-publication follow-up:actual browser acceptance found the sole-option
listbox was not selected before auto-opening. The unpackaged workbench bridge
now selects that verified option explicitly; real024 browser opening and the
selected-source regression pass. Original Explorer assets are unchanged. The
portable ZIP remains unchanged/pinned to4f38a85; main includes this later bridge
fix. The publication receipt distinguishes current main from binary build source.
