# Causal navigation: engineering Gate B

Reference revision: `74c3f5a2c2484a7b83da080ca92dc84721979d0a`.
This milestone adds observer infrastructure only; no020 behavioral implementation
belongs to this commit. The original019 scientific interpretation remains qualified.

## Implemented scope and original witnesses

The additive extractor2 protocol reader creates exact-source nodes and typed edges
for selected activities, actual invitations, independent responses, arbitration,
commitments, resource actions, subjective interpretations, captured after-points,
explicit later memory use, queries, separate delivery channels and assessment/progress.
Nodes carry trial identity, source path, original ID/ordinal, tick, actor/recipient,
outcome and actual point anchors. Missing explicit IDs/paths reject extraction.
Edges never arise from adjacency alone. A same-step meal links to a recipient input
only when actor, captured time and resulting food/hunger explicitly match.

| Actual019 source (seed42, one-based record) | Verified distinction |
|---|---|
| Core103/useful_contact/Baseline | ChoiceConsume0→meal1; incoming1→0 acceptance; Offer3/Accept4; one actual transfer, interpretations, point5/trust and recorded later memory cause in Offer7. |
| Core107/Scale110 | Reciprocal proposals; actual0→1 Request3/Offer4/Accept5. Every same-tick arbitration disposition precedes execution in the chain. |
| Held7/information/Descending | At45,p0/s3,planned0→2 Inquiry is Contention; actual2→0 resource executes. Later actual query belongs to2,source1,51–63,empty/Open. |
| Held10/Scale90 | Actual owner0/source2 inquiry51–64; native fieldnull, provenance receipt1, assessment record1/support20→60,Partial→Resolved. Later67–79 query is empty. |
| Core85/ongoing | After258,65 completed Request→Leave scenes,zero transfer;130 actions and195 allocation dispositions remain separate. Scene46 opens terminal event112/tick268 with before312/after315 links. |
| Held182/183 | Tick16 outgoing0→1 differs from actual2→0; source2 Leaves19. Material/Agent reconvergence does not mean equal final Snapshot. |
| Core7/10/no_food | No choice/execution difference;complete final Snapshot equal. No invented causal advantage. |
| Core43/55 | Support20→−30 and redundant10→10 are preserved, not relabeled useful because a response occurred. |

Completed-scene filters distinguish transfers,zero yield,Request→Leave,Request→Refuse
and other actual outcomes. Separate disposition filters distinguish Declined,
Reciprocal and unexecuted proposals; reciprocal proposals are not extra meetings.
After-tick is exclusive, applied to scene termination/disposition capture. Counts
cover captured replay periods,excluding pre-epoch preparation. At most80 result rows
are rendered; full matching counts remain visible. Personal counts distinguish
selection, actual meals, resource actions, inquiry completions and participated scenes.

## Validation

All commands exit0:

* `python -B -m unittest discover -s tools/history-explorer/tests -v`:24 tests,
  final run90.797s. Existing16 tests include original source membership, phase
  ordering, projection privacy,cache/import forgery,HTTP constraints,nulls/reconvergence.
  Eight new tests use immutable real records for typed linkage,exact source paths,
  distinct namespaces,missing IDs,actual reply targets,foreign actual edges,completed
  scene counts,empty/provenance delivery,wrong/redundant evidence and historical nulls.
* `python -B tools/history-explorer/tests/test_causal.py --all-sources`:602 original
  complete018/019 histories,723418 nodes at the final link rules; no source files written.
  This traversal is reference coverage,not602 independent scientific replications.
* `node --check tools/history-explorer/app.js` and `node --check tools/history-explorer/causal.js`.
* `node tools/history-explorer/tests/causal.test.cjs`:actual browser helper checks
 65 scenes versus130 unexecuted dispositions,zero transfers,large95 filtering,
 source namespace resolution,invalid paths and same-tick107 ordering.
* `node tools/history-explorer/tests/perspective.test.cjs`:unchanged owned-state
 allowlist,private twins,missing inputs,no episode recovery and legacy persistence.
* Original296 working-byte guard,source/Markdown links and diff scope are checked
 before commit. No protected contract,archive,policy or Rust runtime is changed.

## Actual browser inspection

Executed in the real local in-app browser on Windows,not replaced by terminal JSON.
Source verification,actor selection,observer switching,tick/next-event navigation,
all requiredA–F witnesses,paired comparison and new chain controls were exercised.
Clicks on Accept reach original `events[1]`; a later memory-use node reaches
`events[2]`; provenance delivery reaches `provenance.receipts[1]`; retained assessment
reaches `assessment.records[1]`. Completed scene0 opens original `scenes[0]`,and its
after button opens `points[5]`. The C85 filter's first result opens `scenes[46]` at
terminal `events[112]`,with exact before312/after315 navigation. Changing its filter
to Declined shows65 dispositions rather than65 additional completed meetings.

Switching to Actor Perspective hides the whole observer panel and clears rendered
chain/source/personal-summary contents. The original local inspection remains intact.
H7's chain shows every47-tick disposition before actual resource execution,not a
fake inquiry. H10 visibly distinguishes native-null from provenance and retained
assessment. The genuine null retains equal complete final snapshots. Real long-history
filters and their counts remain independent of the45-row nearby original timeline.
Largest core95/Scale110 loads all2643 replay moments; its completed-scene filter
shows116 matches while rendering at most80 results. Switching back to Actor view
hides the observer section. No browser console errors occurred in completed flows.

Actual screenshots: [resource chain](causal-navigation/resource-chain.jpg),
[provenance assessment](causal-navigation/provenance-delivery.jpg),
[zero-transfer scene filter](causal-navigation/zero-transfer-filter.jpg).
They are authentic viewport captures,not synthetic illustrations. Browser console
errors must remain absent in the completed acceptance flows.

## Cost and limitations

[Engineering measurement](causal-navigation-performance.json),one concurrent-workload
run,not deterministic simulation evidence: cold default1.307s,largest3.961s,Python
peak209453056 bytes. Default export3493044 bytes;largest31615044 bytes. The delivered
baseline's corresponding packages were2970858/29472127 bytes,so new typed metadata
adds522186/2142917 bytes respectively. Cross-run wall-clock differences are not a
controlled speed comparison; browser heap/render time remains unmeasured. No full
bounded state is copied into causal nodes; original values are resolved on click.

The chain is narrow018/019 protocol navigation,not a universal causal graph or a
counterfactual explanation. Memory-cause IDs show recorded contributions,not sole
causality,truth or restoration of unavailable cognition. Original explicit after-points
can include other same-cohort changes. Missing actor assessment context stays missing.
Pre-epoch causes have source-only inspection; no extra historical replay is invented.
Large pair loads still take time and must reach ready status before interpretation.
This stage does not add020 ingestion or make the viewer a scientific validator.

During engineering,actual browser use exposed two derived presentation issues:
arbitration/execution ordering at equal ticks and response arrows when a recipient
had several invitations. Both were corrected and protected with focused tests.
No original scientific outcome was altered. Gate B completion is recorded with the
separate viewer commit/push in the program checkpoint before020 implementation.
