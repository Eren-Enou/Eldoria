# Four-person research preview acceptance

Actual browser acceptance on 2026-10-18/19, using the native 100-member original
022 archive. This is a separate read-only demonstration, **not full 022 integration
into the main Explorer**. The main Explorer supports 018–021. No simulation was
run by the preview and no archive was rewritten.

## Checks performed

| Original ordinal / tick | Observed result |
|---|---|
| 1 / 6 | Four local choices; 0→1,1→0,2→3,3→2; other partners are candidates |
| 1 / 7 | Four independent recipient decisions, scores and Continue=0 |
| 1 / 8 | Two admitted resource commitments8/9; reciprocal proposals separate |
| 1 / 9 | Simultaneous Offer events in scenes2/3; four lanes on one time axis |
| 1 / 10 | Two transfers, separate releases, one hunger pulse20→25; later choices |
| 5 / 10 | First pair released; second scene remains until11; free actors send |
| 0 / 15 | Resource23(2/3),Claim24(0),Meal26(1) occupy all four people |
| 15 / 10 | Stock1→0 grants one claim; competitor is Depleted; releases at10 |
| 11 / 5 | Two invitations to busy actor0 are Unavailable; no invented response |
| 13 / 14, actor3 | Own food0,hunger5,receipt1; masked Direct candidate, selected Acquire |
| 13 / 14, actor2 | No ordinary choice invented while busy; previous reference cleared |
| 18 / 8 | Query6–19; no interior local input; one need pulse for all four |
| 18 / 19 | Query release and actual information receipt, not a food benefit |
| 31 / 40 | Empty stock/food, continuing contact; null means capacity null |
| 33 / 196 | Distinct hunger/dated receipts; late Wait due affordability |
| 34 / 6 | Largest4.18MB original loads with native identity |

Source buttons were exercised for local inputs, resource events, admissions and
recipient replies, including original1 `boundaries[1].responses[1]` and original11
`boundaries[3].responses[0]`. Actor3's claim/query mask points to original13
`boundaries[8].choices[2].input`. The source panel displays original ordinal,
member/archive SHA256 and schema `experiment-022-trial/v1`.

Changing tick, actor, lens or world clears the previous original record. Actor
Perspective hides observer inventory/stock, other commitments and event history;
it displays only that person's captured choice. Busy/interior absence is explicit,
not reconstructed from the archive. Observer Perspective shows physical lanes.
Local cards precede reservation; lanes follow starts. Bootstrap is labeled
pre-period, before the declared initial physical intervention.

## Automated and visual QA

Four new Python tests passed, including native identity/hash checks, bounded cache,
invalid input/source rejection, read-only HTTP and the actual JS interval/ownership
model. The existing41Python tests and all4legacy JS suites passed separately.
New JS syntax checks passed. Final browser console inspection returned no warnings
or errors. Real layout was inspected, including long and largest histories.
Screenshots are actual rendered records:

- [Two simultaneous scenes](simultaneous-scenes.png): original1,tick9.
- [Different completion](different-completion.png): original5,tick10.
- [Three independent activities](three-activities.png): original0,tick15.
- [Owned inquiry mask](actor-query-mask.png): original13,tick14,actor3.
- [Long history](long-history.png): original33,tick196.

## Limits and startup

```powershell
python -B tools/history-explorer/concurrent_preview.py --port 8768
```

Open <http://127.0.0.1:8768/>. Select an original, click **Open recorded world**,
enter a world tick, then **Inspect tick**. **Next recorded decision** follows the
selected person's actual choices. Buttons change only presentation.

There is no main-viewer022 catalog adapter, paired-comparison panel, cross-trial
navigation, play animation, semantic trust explorer or mobile acceptance. The
shared-axis diagram becomes dense over192ticks; exact tick cards and references
remain usable. No browser heap/performance baseline is claimed. Full integration
needs a four-person/multiple-scene schema, not coercion into021's export. This is
the authorized authentic fallback; full integration remains a separate deliverable.
