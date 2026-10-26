# First persistent world — acceptance report

The headless recovery/storage milestone is validated:35 prototype tests,
243 real saved process cycles,21 headless workbench regressions,217 root tests,
unchanged408 foundation artifacts/425 raw originals and the separate025 gate.
All12 measured raw/zlib recovery workloads match exact state and history.
See [final evidence and limits](persistent-world-recovery.md) and the
[headless CLI guide](PERSISTENT_WORLD_QUICKSTART.md).

Native mid-batch Pause, terminal Finalize,024 interaction and GUI responsiveness/
memory acceptance remain **DEFERRED**. No desktop control, GUI-session modification
or new Windows release occurred in this milestone. The earlier native acceptance
record below remains historical evidence, separate from the headless result.

Status: native/portable acceptance and final publication are still pending.
Starting revision `77707def0e13936c192a3bc987b8971acb3898d1`. Validated implementation
commits:81b5ce1(state audit),7351464(typed continuation/journal/tests),
e969eb5(native sessions/portable integration). This is an intermediate acceptance
report; source commits do not establish the remaining GUI/release gates.
These commits and the initial acceptance documentation are pushed to `origin/main`.

The [engineering audit](persistent-world-engineering.md) defines ownership, the
safe iteration boundary, exact state/journal formats and limitations. The
[human guide](PERSISTENT_WORLD_QUICKSTART.md) describes the intended workflow.
Neither document extends the protected001–024 foundation or adopts025/026.

## Verified so far

- Root formatting, locked all-target Clippy and217 tests passed.
- Read-only001–024 gate passed:408 artifacts and425 raw originals unchanged,
  including original replay and independent physics/cognition/causality readers.
- Separate025 custody/acceptance gate passed:214 bindings and50 exact replays.
- Seven64-tick023 cases passed full-state restore at every legal iteration and
  complete original-coordinator history equality.
- Twelve1024-tick023 cases (four fixtures, seeds0/42/u64MAX) passed228 full-state
  split points and complete original histories, including null/cutoff/drain.
- A1024 world passed four genuine process sessions and exact terminal export.
- Explicit024 Baseline/MealRelief passed all12 public cases and long1024 cases
  against the unchanged original024 coordinator.
- Descending/Rotating order, Epoch/Absolute need phase, claim2/4/8, reporting1/11/128
  and seed sensitivities passed without policy changes.
- Semantic corruption, unknown identity/schema, incomplete writes, missing/mismatched
  journal, unavailable episode/observation, pending-reply loss and exclusive-writer
  controls are covered. Current-source release suite10 passed18 tests, including
  transport metadata binding, writer exclusion and exact process continuation.
- Workbench session integration tests passed, including both024 modes, separate
  finished-run storage and explicit025 deferral. Desktop/portable Clippy passed;
  final desktop/portable all-target Clippy and explicit formatting checks passed.
- Full workbench release/all-target suite passed21 tests with all ignored tests
  enabled: genuine developer runners, installed tool-free execution/replay/notes,
  relocation/failure cases, original023/024 transport parity, recorded history,
  corrected025 developer-only cases and both new world-session tests. The first
  command used an invalid feature name and exited before compilation; the corrected
  `--features desktop,portable` run completed with no failures or ignored tests.

## Native and packaged acceptance evidence

The static-CRT0.3.0 candidate was extracted into two application directories,
including a relocated path containing spaces. Runtime PATH contained only Windows
System32. The checkout still existed on the host; this is not a clean-machine test.
Actual native operation created a paired023 world (seed43,H1024,P11), saved four
pending invitations at tick7, closed the process, and reopened them unchanged.
At tick9, two actual resource commitments (IDs8/9, deadlines14) survived another
process close and relocation/reopen. Subsequent native advancement reached tick281.
Recorded actor-local input and linked executed transfers were inspected in the GUI;
observer step records were explicitly labelled separately from actor knowledge.

Desktop automation was stopped with the physical Escape key before mid-batch Pause
and terminal Finalize acceptance. Those GUI gates remain unproven. The application
was subsequently absent; its checkpoint reopened at tick281. A separate copy of
that world was continued and finalized through the packaged backend, leaving the
human's GUI world untouched. Its complete terminal JSONL matched the independent
original023 executable byte for byte (SHA256
`2385df70767063878e005e4ffbff9bf9203c9c6ba8bf4d86f7cc2943a45e8c3a`).
An explicit024 MealRelief consumption world (seed43,H64,P11) also crossed separate
packaged create/advance/reopen/resume/finalize processes and matched the independent
original024 executable exactly (SHA256
`1acd1fdbf245ce034f438f758a723fe1af69732f3ef6d81c0b60911589727e40`).
This is backend acceptance; it does not replace the remaining024 native GUI check.
The backend correctly rejected an attempted user-data location inside the checkout;
the successful copy used an external user-data directory.

## Observed persistence cost

Local release measurements, paired seed42,P11, are wall-clock observations under
host load, not deterministic evidence or controlled comparative benchmarks.

| Mode | Horizon | Boundaries | Checkpoint bytes | Journal bytes | Journal execution ms | Save/readback ms | Open/validate ms |
|---|---:|---:|---:|---:|---:|---:|---:|
|023 Continuous|128|161|1,775,655|2,390,545|339.4|39.8|99.5|
|024 Baseline|128|161|1,827,338|2,459,327|260.2|37.5|103.0|
|024 MealRelief|128|161|1,786,789|2,404,961|273.0|37.4|90.1|
|023 Continuous|1024|1057|13,719,717|18,309,019|1685.3|209.6|656.9|
|024 Baseline|1024|1057|14,062,720|18,765,822|2162.4|256.6|809.8|
|024 MealRelief|1024|1057|14,257,788|19,039,095|2046.4|241.0|780.2|

Serialized audit/state grows with history despite bounded cognition. Each immutable
save generation adds another complete checkpoint; no compaction policy is claimed.
The measurement process holds comparison worlds simultaneously, so its working set
would not establish the heap cost of one live world. Two separate read-only packaged
CLI inspection processes sampled about81.2MB peak working set at tick281 and271.5MB
at terminal tick1063 for the same paired023 world lineage. End-to-end process times
were384ms and846ms. These include executable startup, checkpoint/journal decoding,
validation and JSON status output, and do not measure steady-state GUI memory or
pure restore latency. Native GUI working-set and precise UI responsiveness
measurements remain outstanding.

## Remaining acceptance work

Finish native mid-batch Pause, terminal Finalize and024GUI acceptance, and measure
native memory/responsiveness. Then review the final acceptance diff and publish a
verified binary release. Current-source tests, original executable comparisons,
checkpoint/journal/CLI working-set measurements, relocated-package checks and
legacy workflows have passed. Do not infer the remaining GUI gates from code.

The clean candidate was built from `b0741230603cca655a791e1a0971bb67970c6369`
with both identities reporting `dirty:false`. Application and runner share source
fingerprint `8ba7f97421314186991d38969ebe38feec3b31778f51373e5173c74ac564b003`;
continuation runtime remains
`1fca6d6846d75cf40af3743d6be790cd5934c56501020da9cb7b1799a77798d7`.
Its machine-local ZIP is
`target/releases/woi-workbench-0.3.0-rc.1-windows-x64-clean-candidate.zip`, SHA256
`55d29709bd4da9e2780980f6fc58a7f2aee7dcac5bbe365789da5113ce5c0a03`.
The installed-package acceptance test passed again against these exact binaries.
Extraction into another path containing spaces and a System32-only PATH reopened
the original GUI world's tick281 status byte-identically. Static PE inspection
found no Visual C++ redistributable imports. This remains a candidate, with no new
GitHub binary release published. The earlier0.2 ZIP retains SHA256
`5d18525a1a76ee61f0e3eaca3a4593f85f26a2f33b2f5b5fc9dba4a8a4044fc4`.

## Completion audit against the requested milestone

| Requirement | Current evidence/status |
|---|---|
|Capture actual unfinished full state; preserve resources, cognition, IDs, clock and unfinished work|Typed resource inventory; full-state split equality; pending/active native restart observations|
|No observer recovery, future execution or changed behavioral law|Unavailable episode/observation rejection; original history parity; unchanged source bindings|
|Pause duration is external; safe original iteration boundary|Runtime/session tests and worker ownership; actual native mid-batch Pause acceptance pending|
|Exact segmented equivalence|Seven short cases, twelve long worlds/228splits,024matrix, real processes, independent executable comparisons|
|Human create/save/reopen/resume/inspect|Native023 workflow observed through tick281; native terminal Finalize and024 workflow pending|
|Finished-run/reproduce/notes compatibility|All21 workbench tests passed, including ignored package/runner/notes/replay tests|
|Protected archives unchanged;025 separate|Read-only foundation and separate025 gates passed; no frozen files changed|
|Rust-first implementation and practical quickstart|New Rust runtime/session/UI; Python only guarded compile-time source adaptation; linked human guide|
|Performance, storage and portability|H128/H1024 measurements, CLI working set, two relocated candidate installations; GUI latency/memory and final clean package pending|
|Committed and published exact artifact|Pending; dirty candidate must not be published|
|Evidence-based next step|Fixed horizon and growing audit motivate a separate prospective continuity/storage contract; no horizon extension implemented|

The milestone is not yet complete. A backend result is not substituted for a
remaining native acceptance step, and the preflight archive is not a publication.

## Failures preserved during development

The initial test incorrectly expected a live send commitment after its one-tick
completion; its pending invitation was the actual unfinished state. That assertion
was corrected. Overlapping edits/builds caused a stale CLI interface compile error;
an unconverted private-field test access was then corrected. Clippy found a
collapsible conditional, generated helper placement and an unused test binding.
All diagnostics remain in ignored `target/persistent-world-*.log` files.

The first journal design serialized complete state after every step solely for a
hash. A1024 process test took roughly40 seconds under host load. It was replaced by
one complete state/journal binding at checkpoint publication, with append-only
step deltas. Record final timings separately; no wall-clock result belongs in
deterministic simulation evidence. No actor policy or protected output was changed.

## Limits and next decision

Finite fixed horizon1–1024, charged33-tick drain, four people, finite food, capped
hunger and existing heuristic behavior remain. An atomic inquiry cannot be paused
mid-call. Ordinary restart tests do not establish power-loss durability. Exact
runtime identity is required. No indefinite life, sustainability, population
scaling, social institutions, psychological realism or consciousness is established.

025 would need an additional independently inventoried Material resource: raw
stock, ownership, active extraction/conversion phase, escrow, waste, deadlines,
shared sensory slots and corrected observer validation. Its existing preflight
and214-binding custody boundary must survive before any such support. It is not
implicitly portable/resumable in this release.

The paired horizon control shows the same pre-cutoff inputs for different initially
declared Continuous horizons; the public budget is now+33. A completed shorter
world has nevertheless already executed its cutoff/drain history. Extending a
terminal world cannot erase those consequences. Horizon extension needs a separate
prospective contract and broader prefix checks; it is not implemented here.

The strongest immediate engineering follow-up is failure recovery and storage
budgeting for durable multi-session operation: an unsaved journal tail is preserved
but currently prevents reopening, and repeated immutable generations duplicate
growing audit state. Those observed limits justify work before widening horizons
or adding environmental renewal. Neither food use nor saved history establishes
that delayed work valuation, broader social opportunities or a new physical
ontology should change. Such behavioral work needs its own prospective experiment.
