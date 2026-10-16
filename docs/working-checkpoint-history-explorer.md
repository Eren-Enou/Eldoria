# History Explorer checkpoint

Objective: review/protect019 conditionally, deliver working local read-only viewer.
No020,behavior,canon,acquisition or live-control changes. Branch main.
Reference/reviewed fd8e1b08cd2f96390a5e042517e9a18395e25908.
A review commit8c964a1 pushed; ACCEPT WITH QUALIFICATION, no defect/retuning.
A checks fmt/Clippy/197tests/001–018 normal/019 replay/scope/slices all0.

B Gate PASS:296 frozen entries (269 inherited+27 original019), raw guard296 unchanged.
Normal001–019,standalone009–019,fmt/Clippy/197tests all0. Empty-cwd built019 with
Python writes/target reads denied exits0/creates0files; rules self-tested. Shared
contract019 reuses original replay/readers and adds scope/exact21slices; no viewer
imports/cache dependency. B audit/contract/handoff/navigation complete, commit pending.

Viewer code under tools/history-explorer; Python stdlib loopback server + static
HTML/CSS/JS. Normal startup derived catalog unverified until open; selected full
archive/member hashes checked, source-preserving v1 trial/projections/cache. Fast
first open~1.15–1.43s. --prepare full336 scan~54s; --include018 optional602 scan95.21s.
Main server session79001 port8765 (latest core except deferred-demo fix); optional
server38600 port8766 indexes602 (older process code, restart if runtime changes needed).
Default source019 useful_contact seed42 Baseline core103, pairedScale110107.
Launch python -B tools/history-explorer/explorer.py; no-browser optional.
All16 data tests passed including private twins, real eviction, source/hash/schema,
consent/execution,query/gap/reconvergence/cache/import/HTTP. New deferred-demo integrity
fix needs final test rerun; threaded listener + archive lock avoids idle preconnection
blocking; tests cover idle connection. No simulation source changed.

Browser tab1 (cua handle tab): default,actor selection,score table,replay/play/pause,
meal/resource links,resource local view,matchedA,privateB,reconvergentC,missedD,timingE,
nullF all checked. G requested, verify pending; long,keyboard,import/error/responsive/
screenshot and018 UI remain. Full combined smoke hit browser-control execution deadline;
use segmented tests/browser-smoke.js request/verify in separate tool calls. D tick45
first shows need; Next decision reaches actual period0step3/point7; script corrected.

Engineering measure succeeded: default1.43s/2.96MB export,largest17.12→29.43MB/3.85s,
Python peak198.7MB; nondeterministic concurrent-validation observations, not browser heap
or simulation evidence. Rerun once at final viewer source and retain separate record.
New README/user-guide/architecture docs exist; acceptance.md/screenshot/C navigation
and final docs still pending. No files in experiments changed. Logs/guard/scripts in
%TEMP%/history-explorer. Final stages: commit/push validated B only; complete UI acceptance,
final viewer/data checks and measurement, final docs/navigation; archive/link/diff guard;
commit/push C, verify remote/clean and goal complete. No new scientific experiment.
